#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// mod iperf_bindings;
pub mod iperf_reports;

pub mod api;

// use color_eyre::{eyre::eyre, Report};
// use iperf_reports::TestResults;
// use std::ffi::{CStr, CString};
// use std::fmt;
// pub enum TestRole {
//     Client,
//     Server,
// }

// pub struct IperfTest {
//     inner: *mut iperf_bindings::iperf_test,
// }

// impl IperfTest {
//     pub fn new() -> Result<Self, Report> {
//         unsafe {
//             let test = iperf_bindings::iperf_new_test();
//             if !test.is_null() {
//                 // Set defaults
//                 iperf_bindings::iperf_defaults(test);
//                 Ok(Self { inner: test })
//             } else {
//                 Err(eyre!("Failed to allocate Iperf Client"))
//             }
//         }
//     }

//     /// Use this to configure the test client as if called from the command line
//     ///
//     /// This function also overrides json output formatting to true
//     pub fn new_from_arguments<T, U>(args: T) -> Result<Self, Report>
//     where
//         T: IntoIterator<Item = U>,
//         U: Into<Vec<u8>>,
//     {
//         let test = Self::new()?;

//         // Construct a temporary array of CStrings
//         let mut arg_buffer: Vec<CString> =
//             args.into_iter().map(|a| CString::new(a).unwrap()).collect();

//         let mut argv: Vec<*mut i8> = arg_buffer
//             .iter_mut()
//             .map(|a| a.as_ptr() as *mut i8) // This feels illegal
//             .collect();

//         let ret = unsafe {
//             iperf_bindings::iperf_parse_arguments(
//                 test.inner,
//                 argv.len().try_into().unwrap(),
//                 argv.as_mut_ptr(),
//             )
//         };

//         if ret == 0 {
//             test.set_json_output(true);
//             Ok(test)
//         } else {
//             Err(eyre!("Bad arguments passed to iperf"))
//         }
//     }

//     pub fn set_verbose(&mut self, v: bool) {
//         unsafe {
//             iperf_bindings::iperf_set_verbose(self.inner, if v { 1 } else { 0 });
//         }
//     }

//     pub fn get_verbose(&mut self) -> bool {
//         unsafe { !matches!(iperf_bindings::iperf_get_verbose(self.inner), 0) }
//     }

//     pub fn get_control_socket(&mut self) -> i32 {
//         unsafe { iperf_bindings::iperf_get_control_socket(self.inner) }
//     }

//     pub fn get_omit(&mut self) -> i32 {
//         unsafe { iperf_bindings::iperf_get_test_omit(self.inner) }
//     }
//     pub fn get_duration(&mut self) -> i32 {
//         unsafe { iperf_bindings::iperf_get_test_duration(self.inner) }
//     }

//     /// Configure client or server operation
//     pub fn set_test_role(&mut self, role: TestRole) {
//         unsafe {
//             match role {
//                 TestRole::Client => iperf_bindings::iperf_set_test_role(self.inner, 'c' as i8),
//                 TestRole::Server => iperf_bindings::iperf_set_test_role(self.inner, 's' as i8),
//             }
//         }
//     }

//     /// Set hostname (or IP?) of server
//     pub fn set_test_server_hostname(&mut self, host: &str) {
//         unsafe {
//             let host = std::ffi::CString::new(host).unwrap();
//             iperf_bindings::iperf_set_test_server_hostname(self.inner, host.as_ptr())
//         }
//     }

//     /// Set port of server to connect to (default 5201)
//     pub fn set_test_server_port(&mut self, port: i32) {
//         unsafe {
//             iperf_bindings::iperf_set_test_server_port(self.inner, port);
//         }
//     }

//     pub fn set_test_omit(&mut self, omit: i32) {
//         unsafe {
//             iperf_bindings::iperf_set_test_omit(self.inner, omit);
//         }
//     }

//     /// Set the duration of the test in seconds
//     pub fn set_test_duration(&mut self, duration: i32) {
//         unsafe {
//             iperf_bindings::iperf_set_test_duration(self.inner, duration);
//         }
//     }

//     pub fn set_test_reporter_interval(&mut self, interval: f64) {
//         unsafe {
//             iperf_bindings::iperf_set_test_reporter_interval(self.inner, interval);
//         }
//     }
//     pub fn set_test_stats_interval(&mut self, interval: f64) {
//         unsafe {
//             iperf_bindings::iperf_set_test_stats_interval(self.inner, interval);
//         }
//     }

//     /// Run the client, and attempt to parse the results
//     pub fn run_client(&mut self) -> Result<TestResults, Report> {
//         let res = unsafe { iperf_bindings::iperf_run_client(self.inner) };

//         if res < 0 {
//             Err(Report::new(IperfError { value: res }))
//         } else {
//             let res = serde_json::from_str(&self.get_json_output_string()?)?;
//             Ok(res)
//         }
//     }

//     /// Run the test as a server (eventually, this should be in a different type)
//     pub fn run_server(&mut self) -> Result<(), Report> {
//         let res = unsafe { iperf_bindings::iperf_run_server(self.inner) };

//         match res {
//             0 => Ok(()),
//             error_code => Err(Report::new(IperfError { value: error_code })),
//         }
//     }

//     pub fn get_json_output_string(&self) -> Result<String, Report> {
//         unsafe {
//             let output = iperf_bindings::iperf_get_test_json_output_string(self.inner);

//             if output.is_null() {
//                 Err(eyre!("Error retreiving JSON output"))
//             } else {
//                 Ok(CStr::from_ptr(output).to_string_lossy().to_string())
//             }
//         }
//     }

//     pub fn set_json_output(&self, json: bool) {
//         unsafe { iperf_bindings::iperf_set_test_json_output(self.inner, if json { 1 } else { 0 }) }
//     }
// }

// impl Drop for IperfTest {
//     fn drop(&mut self) {
//         unsafe {
//             iperf_bindings::iperf_free_test(self.inner);
//         }
//     }
// }

// #[derive(Debug)]
// pub struct IperfError {
//     value: i32,
// }

// impl fmt::Display for IperfError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // It should be safe to unwrap here, as we can be fairly certain this type is only created by valid producers
//         let error_message = unsafe {
//             CStr::from_ptr(iperf_bindings::iperf_strerror(self.value)).to_string_lossy()
//             // Go to an owned type, because this data is in a static buffer in the function
//         };
//         write!(f, "{}", error_message)
//     }
// }

// impl std::error::Error for IperfError {}
