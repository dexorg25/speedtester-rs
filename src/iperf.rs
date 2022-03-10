#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fmt;
use thiserror::Error;

use tracing::{debug, info};

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/iperf_bindings.rs"));
}
pub enum TestRole {
    Client,
    Server,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IperfInterval {}

#[derive(Serialize, Deserialize, Debug)]
pub struct IperfEnd {
    pub streams: Vec<IperfEndStream>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum IperfEndStream {
    udp {
        socket: i32,
        start: f64,
        end: f64,
        seconds: f64,
        bytes: u64,
        bits_per_second: f64,
        jitter_ms: f32,
        lost_packets: u64,
        packets: u64,
        lost_percent: f64,
        out_of_order: u32,
        sender: bool,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestResults {
    // May fill in these sections later, for now only fully parse the end stats section
    pub start: HashMap<String, Value>,
    pub intervals: Vec<HashMap<String, Value>>,
    pub end: IperfEnd,
    // Server output will only be present if the right flag was passed
    pub server_output_text: Option<String>,
}

pub struct IperfTest {
    inner: *mut bindings::iperf_test,
}

impl Default for IperfTest {
    fn default() -> Self {
        let test = Self::new();

        unsafe {
            bindings::iperf_defaults(test.inner);
        }

        test
    }
}

impl IperfTest {
    pub fn new() -> Self {
        unsafe {
            let test = bindings::iperf_new_test();
            assert!(!test.is_null());
            Self { inner: test }
        }
    }

    /// Use this to configure the test client as if you called it from the command line
    pub fn new_from_arguments(args: std::env::Args) -> Self {
        let test = Self::default();

        // Construct a temporary array of CStrings
        let mut arg_buffer: Vec<CString> = args.map(|a| CString::new(a).unwrap()).collect();

        let mut argv: Vec<*mut i8> = arg_buffer
            .iter_mut()
            .map(|a| a.as_ptr() as *mut i8) // This feels illegal
            .collect();

        let ret = unsafe {
            bindings::iperf_parse_arguments(
                test.inner,
                argv.len().try_into().unwrap(),
                argv.as_mut_ptr(),
            )
        };

        assert!(ret == 0);

        test
    }

    pub fn set_verbose(&mut self, v: bool) {
        unsafe {
            bindings::iperf_set_verbose(self.inner, if v { 1 } else { 0 });
        }
    }

    pub fn get_verbose(&mut self) -> bool {
        unsafe { !matches!(bindings::iperf_get_verbose(self.inner), 0) }
    }

    pub fn get_control_socket(&mut self) -> i32 {
        unsafe { bindings::iperf_get_control_socket(self.inner) }
    }

    pub fn get_omit(&mut self) -> i32 {
        unsafe { bindings::iperf_get_test_omit(self.inner) }
    }
    pub fn get_duration(&mut self) -> i32 {
        unsafe { bindings::iperf_get_test_duration(self.inner) }
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

    pub fn get_json_output_string(&self) -> Result<String, IperfStringError> {
        unsafe {
            let output = bindings::iperf_get_test_json_output_string(self.inner);

            if output.is_null() {
                Err(IperfStringError {
                    message: "Error getting JSON output",
                })
            } else {
                Ok(CStr::from_ptr(output).to_string_lossy().to_string())
            }
        }
    }

    pub fn set_json_output(&self, json: bool) {
        unsafe { bindings::iperf_set_test_json_output(self.inner, if json { 1 } else { 0 }) }
    }
}

impl Drop for IperfTest {
    fn drop(&mut self) {
        unsafe {
            bindings::iperf_free_test(self.inner);
        }
    }
}

#[derive(Error, Debug)]
pub struct IperfStringError {
    message: &'static str,
}

impl fmt::Display for IperfStringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
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
