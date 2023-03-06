pub mod reports;

use std::process;

use crate::iperf3::reports::TestResults;

// Start iperf3 command
pub fn test_udp_server(port: u16) -> Result<reports::TestResults, reports::IperfError> {
    let ret = process::Command::new("iperf3")
        .args(["-s", "-p", &port.to_string(), "-J", "-1"])
        .output()
        .expect("Failed to start iperf3 server");

    let test_report: TestResults =
        serde_json::from_str(&String::from_utf8_lossy(&ret.stdout)).unwrap();
    Ok(test_report)
}

// Start iperf3 client to run a UDP test to the given server for 13 seconds
pub fn test_udp_client(host: &str, port: u16) -> Result<reports::TestResults, reports::IperfError> {
    let ret = process::Command::new("iperf3")
        .args(["-c", host, "-p", &port.to_string(), "-u", "-t", "1", "-J"])
        .output()
        .expect("Failed to start iperf3 client");

    let test_report: TestResults =
        serde_json::from_str(&String::from_utf8_lossy(&ret.stdout)).unwrap();

    Ok(test_report)
}

#[cfg(test)]
mod test {
    use super::{test_udp_client, test_udp_server};

    #[test]
    fn test_server() {
        let test_server = std::thread::spawn(|| test_udp_server(5201).unwrap());
        std::process::Command::new("iperf3")
            .args(["-c", "localhost", "-p", "5201", "-u", "-t", "1"])
            .spawn()
            .unwrap()
            .wait()
            .ok();
        let test_report = test_server.join().unwrap();
        assert_eq!(test_report.start.test_start.protocol, "UDP");
    }

    #[test]
    fn test_client() {
        let mut test_server = std::process::Command::new("iperf3")
            .args(["-s", "-p", "5202", "-1"])
            .spawn()
            .unwrap();
        let test_report = test_udp_client("localhost", 5202).unwrap();
        test_server.wait().ok();
        assert_eq!(test_report.start.test_start.protocol, "UDP");
    }
}
