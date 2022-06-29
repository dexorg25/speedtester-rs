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
pub struct TestResults {
    // May fill in these sections later, for now only fully parse the end stats section
    pub start: HashMap<String, Value>,
    pub intervals: Vec<HashMap<String, Value>>,
    pub end: IperfEnd,
    // Server output will only be present if the right flag was passed
    pub server_output_text: Option<String>,

    pub error: Option<String>,
}
