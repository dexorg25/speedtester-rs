#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::ffi::CStr;
use std::fmt;

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/iperf_bindings.rs"));
}
pub enum TestRole {
    Client,
    Server,
}

pub struct IperfTest {
    inner: *mut bindings::iperf_test,
}

impl IperfTest {
    pub fn new() -> Self {
        unsafe {
            let test = bindings::iperf_new_test();
            assert!(!test.is_null());
            Self { inner: test }
        }
    }

    pub fn defaults(&mut self) {
        unsafe {
            bindings::iperf_defaults(self.inner);
        }
    }

    pub fn set_verbose(&mut self, v: i32) {
        unsafe {
            bindings::iperf_set_verbose(self.inner, v);
        }
    }

    /// Configure client or server operation
    pub fn set_test_role(&mut self, role: TestRole) {
        unsafe {
            match role {
                TestRole::Client => bindings::iperf_set_test_role(self.inner, 'c' as i8),
                TestRole::Server => bindings::iperf_set_test_role(self.inner, 's' as i8),
            }
        }
    }

    /// Set fully hostname (or IP?) of server
    pub fn set_test_server_hostname(&mut self, host: &str) {
        unsafe {
            let host = std::ffi::CString::new(host).unwrap();
            bindings::iperf_set_test_server_hostname(self.inner, host.as_ptr())
        }
    }

    /// Set port of server to connect to (default 5201)
    pub fn set_test_server_port(&mut self, port: i32) {
        unsafe {
            bindings::iperf_set_test_server_port(self.inner, port);
        }
    }

    pub fn set_test_omit(&mut self, omit: i32) {
        unsafe {
            bindings::iperf_set_test_omit(self.inner, omit);
        }
    }

    /// Set the duration of the test in seconds
    pub fn set_test_duration(&mut self, duration: i32) {
        unsafe {
            bindings::iperf_set_test_duration(self.inner, duration);
        }
    }

    pub fn set_test_reporter_interval(&mut self, interval: f64) {
        unsafe {
            bindings::iperf_set_test_reporter_interval(self.inner, interval);
        }
    }
    pub fn set_test_stats_interval(&mut self, interval: f64) {
        unsafe {
            bindings::iperf_set_test_stats_interval(self.inner, interval);
        }
    }

    pub fn run_client(&mut self) -> Result<(), IperfError> {
        let res = unsafe { bindings::iperf_run_client(self.inner) };

        if res < 0 {
            Err(IperfError { value: res })
        } else {
            Ok(())
        }
    }

    pub fn get_test_json_output_string(&self) -> Result<String, &'static str> {
        unsafe {
            let output = bindings::iperf_get_test_json_output_string(self.inner);

            if output.is_null() {
                Err("Error getting JSON test output")
            } else {
                Ok(CStr::from_ptr(output).to_string_lossy().to_string())
            }
        }
    }
}

impl Drop for IperfTest {
    fn drop(&mut self) {
        unsafe {
            bindings::iperf_free_test(self.inner);
        }
    }
}

#[derive(Debug)]
pub struct IperfError {
    value: i32,
}

impl fmt::Display for IperfError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // It should be safe to unwrap here, as we can be fairly certain this type is only created by valid producers
        let error_message = unsafe {
            CStr::from_ptr(bindings::iperf_strerror(self.value)).to_string_lossy()
            // Go to an owned type, because this data is in a static buffer in the function
        };
        write!(f, "{}", error_message)
    }
}

impl std::error::Error for IperfError {}
