use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::{Value};

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
