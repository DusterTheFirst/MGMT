use serde::{Serialize, Deserialize};

/// Packets sent and recieved between the cli and manage-d
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Local {
    Welcome,
    Ping {
        time: u64
    }
}