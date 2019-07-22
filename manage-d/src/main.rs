use std::net::{TcpListener, SocketAddr, Ipv4Addr, IpAddr, TcpStream};
use std::io::{self, BufReader, BufWriter};
use std::thread;
use ansi_term::Colour::*;

use core::protocol::local;
use core::PacketStream;

// use daemonize::{Daemonize, DaemonizeError};
// use std::fs::File;

/// The port used to communicate with the CLI
static PORT_LOCAL: u16 = 9895;
/// The protocol version used to communicate with the CLI
static PROTOCOL_VERSION_LOCAL: u8 = 0;

fn main() {
    // let stdout = File::create("/tmp/daemon.out").unwrap();
    // let stderr = File::create("/tmp/daemon.err").unwrap();

    // let daemonize = Daemonize::new()
    //     .pid_file("/tmp/test.pid") // Every method except `new` and `start`
    //     .chown_pid_file(true)      // is optional, see `Daemonize` documentation
    //     .working_directory("/tmp") // for default behaviour.
    //     // .user("nobody")
    //     // .group("daemon") // Group name
    //     // .group(2)        // or group id.
    //     .umask(0o777)    // Set umask, `0o027` by default.
    //     .stdout(stdout)  // Redirect stdout to `/tmp/daemon.out`.
    //     .stderr(stderr)  // Redirect stderr to `/tmp/daemon.err`.
    //     .privileged_action(|| "Executed before drop privileges");

    // match daemonize.start() {
    //     Ok(_) => start(),
    //     Err(e) => {
    //         match e {
    //             DaemonizeError::LockPidfile(_) => eprintln!("Process already running, exiting"),
    //             _ => eprintln!("Error, {error}. DaemonizeError::{error:?}", error = e)
    //         }
    //     },
    // }
    start()
}

fn start() {
    // println!("Success, daemonized");

    println!("{}", Blue.paint(format!("Listening for CLI connections on port {}", PORT_LOCAL)));
    
    let listener = TcpListener::bind(
        SocketAddr::new(
            IpAddr::V4(
                Ipv4Addr::new(127,0,0,1)
            ),
            PORT_LOCAL
        )
    ).expect("Unable to bind to socket");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            },
            Err(e) => eprintln!("{}", Red.paint(format!("{:?}", e)))
        }
    }
}

fn handle_client(stream: TcpStream) -> Result<(), io::Error> {
    println!("{} {}", Blue.paint("New CLI connection:"), stream.peer_addr().unwrap());

    let mut packet_stream = PacketStream::new(BufReader::new(&stream), BufWriter::new(&stream));

    // Welcome client
    packet_stream.write_packet(local::ToCLI::Welcome {
        protocol_version: PROTOCOL_VERSION_LOCAL
    })?;

    while packet_stream.is_open() {
        let packet: local::ToManageD = packet_stream.read_packet()?;

        match packet {
            local::ToManageD::Ping(x) => packet_stream.write_packet(local::ToCLI::Pong(x))?
        };

        println!("{} {}",
            RGB(50, 150, 150).paint("[Packet Recieved]"),
            RGB(150, 150, 150).paint(format!("{:#?}", packet)));
    }
    println!("{} {}", Yellow.paint("Closed CLI connection: "), stream.peer_addr()?);

    Ok(())
}