use std::fs::File;
use daemonize::{Daemonize, DaemonizeError};
use std::env;
use std::net::{TcpListener, SocketAddr, Ipv6Addr, IpAddr, TcpStream};
use std::io::prelude::*;
use std::io;

// TODO: Change port
static PORT: u16 = 6969;

fn main() {
    let stdout = File::create("/tmp/daemon.out").unwrap();
    let stderr = File::create("/tmp/daemon.err").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("/tmp/test.pid") // Every method except `new` and `start`
        .chown_pid_file(true)      // is optional, see `Daemonize` documentation
        .working_directory("/tmp") // for default behaviour.
        // .user("nobody")
        // .group("daemon") // Group name
        // .group(2)        // or group id.
        .umask(0o777)    // Set umask, `0o027` by default.
        .stdout(stdout)  // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr)  // Redirect stderr to `/tmp/daemon.err`.
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => start(),
        Err(e) => {
            match e {
                DaemonizeError::LockPidfile(_) => eprintln!("Process already running, exiting"),
                _ => eprintln!("Error, {error}. DaemonizeError::{error:?}", error = e)
            }
        },
    }
}

fn start() {
    println!("Success, daemonized");
    println!("This is the daemon that manages docker instances of minecraft");

    println!("Running on port {}", port);
    
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0,0,0,0,0,0,0,1)), port)).expect("Unable to bind to socket");

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
    stream.write(b"penis")?;

    Ok(())
}