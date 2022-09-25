use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

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
