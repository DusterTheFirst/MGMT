//! These packets are used for the manage-d service
//! to communicate to the backend

use serde::{Serialize, Deserialize};

/// Packets sent from the backend to managed
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Incoming {
    /// A ping to the server to assure the client and server both can understand eachother
    Ping(u64)
}

/// Packets sent from managed to the backend
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Outgoing {
    /// Welcoming packet with information about the connection
    Welcome {
        protocol_version: u8
    },
    /// A response to the client's ping, returning the same payload as recieved
    Pong(u64),
}