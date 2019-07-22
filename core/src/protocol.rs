/// These packets are used for local (CLI) management of
/// the manage-d service
pub mod local {
    use serde::{Serialize, Deserialize};

    /// Packets sent from the cli to manage-d 
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    pub enum ToManageD {
        /// A ping to the server to assure the client and server both can understand eachother
        Ping(u64)
    }

    /// Packets sent from manage-d to the cli
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    pub enum ToCLI {
        /// Welcoming packet with information about the connection
        Welcome {
            protocol_version: u8
        },
        /// A response to the client's ping, returning the same payload as recieved
        Pong(u64),
    }
}
