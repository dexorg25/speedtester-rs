use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct TestReservation {
    pub port_number: u16,
}

pub mod iperf3;
