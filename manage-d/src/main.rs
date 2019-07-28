use std::net::{TcpListener, SocketAddr, Ipv4Addr, IpAddr, TcpStream};
use std::io::{self, BufReader, BufWriter};
use std::thread;
use ansi_term::Colour::*;

use core::protocol::local;
use core::PacketStream;

// use daemonize::{Daemonize, DaemonizeError};
// use std::fs::File;

mod mc;

fn main() {
    // TODO: DETECT DAEMON VS CLI START AND ACT ACCORDINGLY
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
    start();
}

fn start() {
    // Use shortcuts to manage downloaded versions
    let version_listing = mc::get_versions(true, false).unwrap();
    println!("{:?}", version_listing);
    // println!("{:X?}", bincode::serialize(&version_listing).unwrap());   
}