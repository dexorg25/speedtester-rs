use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::ffi::{CStr, CString, NulError};
use std::fmt::{self, Display};

#[derive(Serialize, Deserialize, Debug)]
pub struct IperfInterval {}

#[derive(Serialize, Deserialize, Debug)]
pub struct IperfEnd {
    pub streams: Option<Vec<IperfEndStream>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IperfEndStream {
    udp: Option<UdpEndStreamItem>,
    sender: Option<HashMap<String, Value>>,
    receiver: Option<HashMap<String, Value>>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UdpEndStreamItem {
    pub socket: i32,
    pub start: f64,
    pub end: f64,
    pub seconds: f64,
    pub bytes: u64,
    pub bits_per_second: f64,
    pub jitter_ms: f32,
    pub lost_packets: u64,
    pub packets: u64,
    pub lost_percent: f64,
    pub out_of_order: u32,
    pub sender: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IperfConnected {
    pub socket: u32,
    pub local_host: String,
    pub local_port: u16,
    pub remote_host: String,
    pub remote_port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IperfTimestamp {
    pub time: String,
    pub timesecs: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IperfHost {
    pub host: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IperfTestStart {
    pub protocol: String,
    pub num_streams: u32,
    pub blksize: u32,
    pub omit: u32,
    pub duration: u32,
    pub bytes: u32,
    pub blocks: u32,
    pub reverse: u32,
    pub tos: u32,
    pub target_bitrate: u32,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct IperfStart {
    pub connected: Vec<IperfConnected>,
    pub version: String,
    pub system_info: String,
    pub timestamp: IperfTimestamp,
    /// Remote host, not present for server reports
    pub connecting_to: Option<IperfHost>,
    pub cookie: String,
    // pub target_bitrate: u64,
    //pub sock_bufsize: u64,
    // pub sndbuf_actual: u32,
    // pub rcvbuf_actual: u32,
    pub test_start: IperfTestStart,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestResults {
    pub start: IperfStart,
    // This field is less useful for this specific application so it's not filled in
    // The test client only needs to know the summary stats, and _really_ not even that.
    // The whole JSON text gets converted to BSON for the log anyway, and those are not actually interpreted by anything that depends on this module at time of writing
    pub intervals: Vec<HashMap<String, Value>>,
    pub end: IperfEnd,
    // Server output will only be present if the right flag was passed
    pub server_output_text: Option<String>,

    pub error: Option<String>,
}

pub enum TestRole {
    Client,
    Server,
}
#[derive(Debug)]
pub enum IperfError {
    Alloc,
    Args,
}
impl Display for IperfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for IperfError {}

mod iperf_bindings;

pub struct IperfTest {
    inner: *mut iperf_bindings::iperf_test,
}

impl IperfTest {
    /// Create a new test context with default settings
    ///
    /// # Errors
    /// Iperf internally allocates storage for this handle. `new` errors if this process fails due to (presumably) an allocation error
    pub fn new() -> Result<Self, IperfError> {
        unsafe {
            let test = iperf_bindings::iperf_new_test();
            if test.is_null() {
                Err(IperfError::Alloc)
            } else {
                // Set defaults
                iperf_bindings::iperf_defaults(test);
                Ok(Self { inner: test })
            }
        }
    }

    /// Use this to configure the test client as if called from the command line
    ///
    /// This function also overrides json output formatting to true to enable returning `IperfTest` directly
    ///
    /// # Errors
    /// If iperf's arguments are invalid it's internal error will be propagated
    /// If any args are null, which cannot happen
    ///
    pub fn new_from_arguments<T, U>(args: T) -> Result<Self, Box<dyn Error + Send + Sync>>
    where
        T: IntoIterator<Item = U>,
        U: Into<Vec<u8>>,
    {
        let mut test = Self::new()?;

        // Construct a temporary array of CStrings
        let arg_buffer: Result<Vec<CString>, NulError> =
            args.into_iter().map(|a| CString::new(a)).collect();

        // rebind and early-return any error
        let mut arg_buffer = arg_buffer?;

        #[cfg(target_os = "macos")]
        let mut arg_raw_ptrs: Vec<*mut i8> = arg_buffer
            .iter_mut()
            .map(|a| a.as_ptr() as *mut i8) // dependent lib uses a dumb signed string type
            .collect();

        #[cfg(target_os = "linux")]
        let mut arg_raw_ptrs: Vec<*mut u8> = arg_buffer
            .iter_mut()
            .map(|a| a.as_ptr() as *mut u8) // dependent lib uses a dumb signed string type
            .collect();

        let ret = unsafe {
            iperf_bindings::iperf_parse_arguments(
                test.inner,
                arg_raw_ptrs.len().try_into()?,
                arg_raw_ptrs.as_mut_ptr() as *mut *mut i8,
            )
        };

        if ret == 0 {
            test.set_json_output(true);
            Ok(test)
        } else {
            Err(Box::new(IperfError::Args))
        }
    }

    pub fn set_verbose(&mut self, v: bool) {
        unsafe {
            iperf_bindings::iperf_set_verbose(self.inner, if v { 1 } else { 0 });
        }
    }

    pub fn get_verbose(&mut self) -> bool {
        unsafe { !matches!(iperf_bindings::iperf_get_verbose(self.inner), 0) }
    }

    pub fn get_control_socket(&mut self) -> i32 {
        unsafe { iperf_bindings::iperf_get_control_socket(self.inner) }
    }

    pub fn get_omit(&mut self) -> i32 {
        unsafe { iperf_bindings::iperf_get_test_omit(self.inner) }
    }

    /// Get the configured test duration
    ///
    /// # Panics
    /// - If the underlying test duratoin cannot be converted to u32 (IE if it is negative)
    pub fn get_duration(&mut self) -> i32 {
        unsafe { iperf_bindings::iperf_get_test_duration(self.inner) }
    }

    /// Configure client or server operation
    pub fn set_role(&mut self, role: &TestRole) {
        unsafe {
            match role {
                TestRole::Client => {
                    iperf_bindings::iperf_set_test_role(self.inner, 'c' as std::os::raw::c_char);
                }

                TestRole::Server => {
                    iperf_bindings::iperf_set_test_role(self.inner, 's' as std::os::raw::c_char);
                }
            }
        }
    }

    /// Set hostname (or IP?) of server
    ///
    /// # Panics
    /// If there are null bytes in the host arg, this should be easy to avoid
    #[allow(clippy::expect_used)]
    pub fn set_test_server_hostname(&mut self, host: &str) {
        unsafe {
            let host: CString = CString::new(host).expect("null bytes in host");
            iperf_bindings::iperf_set_test_server_hostname(self.inner, host.as_ptr());
        }
    }

    /// Set port of server to connect to (default 5201)
    pub fn set_server_port(&mut self, port: i32) {
        unsafe {
            iperf_bindings::iperf_set_test_server_port(self.inner, port);
        }
    }

    pub fn set_test_omit(&mut self, omit: i32) {
        unsafe {
            iperf_bindings::iperf_set_test_omit(self.inner, omit);
        }
    }

    /// Set the duration of the test in seconds
    pub fn set_test_duration(&mut self, duration: i32) {
        unsafe {
            iperf_bindings::iperf_set_test_duration(self.inner, duration);
        }
    }

    pub fn set_test_reporter_interval(&mut self, interval: f64) {
        unsafe {
            iperf_bindings::iperf_set_test_reporter_interval(self.inner, interval);
        }
    }
    pub fn set_test_stats_interval(&mut self, interval: f64) {
        unsafe {
            iperf_bindings::iperf_set_test_stats_interval(self.inner, interval);
        }
    }

    /// Run the client, and attempt to parse the results
    ///
    /// # Errors
    /// - If iperf fails to run
    /// - If the results from iperf fail to parse
    pub fn run_client(&mut self) -> Result<TestResults, Box<dyn Error + Send + Sync>> {
        let res = unsafe { iperf_bindings::iperf_run_client(self.inner) };

        if res < 0 {
            Err(IperfFFIError::from(res).into())
        } else {
            let res = self.get_test_results()?;
            Ok(res)
        }
    }

    /// Run the test as a server (eventually, this should be in a different type)
    ///
    /// # Errors
    /// - If iperf experienced an error internally
    /// - Server output fails to parse
    pub fn run_server(&mut self) -> Result<TestResults, Box<dyn Error + Send + Sync>> {
        let res = unsafe { iperf_bindings::iperf_run_server(self.inner) };

        if res == 0 {
            let res = self.get_test_results()?;
            Ok(res)
        } else {
            Err(Box::new(IperfFFIError::from(res)))
        }
    }

    /// Get the results from a completed test
    ///
    /// # Errors
    /// - Internal iPerf errors
    ///
    /// # Panics
    /// - If iperf returns a string with negative values
    pub fn get_test_results(&self) -> Result<TestResults, Box<dyn Error + Send + Sync>> {
        let exec_output = unsafe {
            // This is a ptr to local json string
            let output = iperf_bindings::iperf_get_test_json_output_string(self.inner);

            // If not null, then treat as a string (but just construct into a u8 byte slice for now)
            // Since this is stored in an internal data structure, copy to an owned type
            if output.is_null() {
                Err("ptr for JSON results was NULL")
            } else {
                let mut len = 0;
                let mut baseptr = output;
                // Loop to calculate the length of this CString
                loop {
                    if *baseptr == 0 {
                        break;
                    }

                    // Necessary because at time of writing macos C Int type is signed???
                    #[cfg(target_os = "macos")]
                    assert!(*baseptr >= 0, "negative value found in string");

                    len += 1;
                    // Check next byte
                    baseptr = baseptr.add(1);
                }
                Ok(std::slice::from_raw_parts(output.cast::<u8>(), len))
            }
        }?;
        Ok(serde_json::from_slice(exec_output)?)
    }

    pub fn set_one_off(&mut self, one_off: bool) {
        unsafe {
            iperf_bindings::iperf_set_test_one_off(self.inner, if one_off { 1 } else { 0 });
        }
    }

    #[must_use]
    pub fn get_one_off(&self) -> bool {
        unsafe { iperf_bindings::iperf_get_test_one_off(self.inner) == 1 }
    }

    pub fn set_json_output(&mut self, json: bool) {
        unsafe { iperf_bindings::iperf_set_test_json_output(self.inner, if json { 1 } else { 0 }) }
    }

    /// Set test idle timeout
    ///
    /// Internally this only uses seconds so the granularity is not as small as suggested by the arg type.
    ///
    /// # Panics
    /// - if the integer seconds representation does not fit into an i32
    #[allow(clippy::expect_used)]
    pub fn set_idle_timeout(&mut self, timeout: std::time::Duration) {
        unsafe {
            iperf_bindings::iperf_set_test_idle_timeout(
                self.inner,
                timeout.as_secs().try_into().expect("timeout too big"),
            );
        }
    }
}

impl Drop for IperfTest {
    fn drop(&mut self) {
        unsafe {
            iperf_bindings::iperf_free_test(self.inner);
        }
    }
}

#[derive(Debug)]
pub struct IperfFFIError {
    code: i32,
    _message: Option<String>,
}

impl From<i32> for IperfFFIError {
    fn from(code: i32) -> Self {
        Self {
            code,
            _message: None,
        }
    }
}

impl fmt::Display for IperfFFIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // It should be safe to unwrap here, as we can be fairly certain this type is only created by valid producers
        let error_message = unsafe {
            CStr::from_ptr(iperf_bindings::iperf_strerror(self.code)).to_string_lossy()
            // Go to an owned type, because this data is in a static buffer in the function
        };
        write!(f, "{}", error_message)
    }
}

impl std::error::Error for IperfFFIError {}
