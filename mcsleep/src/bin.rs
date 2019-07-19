use std::net::{TcpListener, SocketAddr, IpAddr, Ipv4Addr};
use std::thread;

use mcsleep::handle_connection;

// TODO: CLEAN UP
// TODO: DETECT PLAYER JOIN
fn main() {
    let listener = TcpListener::bind(
        SocketAddr::new(
            IpAddr::V4(
                Ipv4Addr::new(0,0,0,0)
            ),
            25565
        )
    ).expect("Unable to bind to socket");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::Builder::new().name("Stream Handler".to_owned()).spawn(move || {
                    handle_connection(&mut stream)
                }).unwrap();
            },
            Err(e) => eprintln!("{:?}", e)
        }
    }
}