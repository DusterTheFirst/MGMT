use std::net::{TcpListener, SocketAddr, IpAddr, Ipv4Addr, TcpStream};
use std::io::{BufReader, BufWriter};
use std::thread;
use core::PacketStream;
use core::protocol::managed;
use termion::{color, style};

static DAEMON_PORT: u16 = 3376;

fn main() {
    println!("{}Listening for Manage-D connections on port {}{}{}", color::Fg(color::Blue), color::Bg(color::Green), DAEMON_PORT, style::Reset);
    
    let listener = TcpListener::bind(
        SocketAddr::new(
            IpAddr::V4(
                Ipv4Addr::new(127,0,0,1)
            ),
            DAEMON_PORT
        )
    ).expect("Unable to bind to socket");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    let stream = PacketStream::new(BufReader::new(&stream), BufWriter::new(&stream));
                    // connection succeeded
                    handle_daemon_connect(stream);
                });
            },
            Err(e) => eprintln!("{}{:?}", color::Fg(color::Red),  e)
        }
    }
}

fn handle_daemon_connect(mut stream: PacketStream<managed::Incoming, managed::Outgoing, &TcpStream, &TcpStream>) {
    println!("{}New connection{}", style::Italic, style::Reset);
}