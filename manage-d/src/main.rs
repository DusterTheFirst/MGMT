use std::net::{TcpListener, SocketAddr, Ipv4Addr, IpAddr, TcpStream};
use std::io::prelude::*;
use std::io;
use core::protocol;

// use daemonize::{Daemonize, DaemonizeError};
// use std::fs::File;

static PORT: u16 = 9895;

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
    println!("This is the daemon that manages docker instances of minecraft");

    println!("Running on port {}", PORT);
    
    let listener = TcpListener::bind(
        SocketAddr::new(
            IpAddr::V4(
                Ipv4Addr::new(127,0,0,1)
            ),
            PORT
        )
    ).expect("Unable to bind to socket");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                std::thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            },
            Err(e) => eprintln!("{:?}", e)
        }
    }
}

fn handle_client(mut stream: TcpStream) -> Result<(), io::Error> {
    let data: Vec<u8> = bincode::serialize(&protocol::Local::Welcome).unwrap();
    stream.write(&data)?;

    let data: Vec<u8> = bincode::serialize(&protocol::Local::Ping {
        time: 1000
    }).unwrap();
    stream.write(&data)?;

    Ok(())
}