#[macro_use] extern crate lazy_static;

use std::process::{Stdio, Command, Child};
// use std::fs::File;
use ansi_term::Colour::*;
use std::io::{prelude::*, ErrorKind, self};
use std::net::{SocketAddr, Ipv6Addr, IpAddr, TcpStream};
use futures::future::{self, Loop};
use futures::prelude::*;

lazy_static! {
    static ref DAEMON_ADDRESS: SocketAddr = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0,0,0,0,0,0,0,1)), 6969);
}

fn main() {
    #[cfg(windows)]
    unimplemented!();

    println!("{}", Green.paint("This is the command line util for managing MCMGMT"));

    println!("{}", Yellow.paint("Connecting to manage-d daemon process"));
    let mut stream = connect_to_daemon(10);
    match stream {
        Some(stream) => {
            println!("{}", Green.paint("Connected"));
        },
        None => {
            println!("{}", Yellow.paint("Daemon not running"));
            println!("{}", Yellow.paint("Starting manage-d"));
            let daemon = start_daemon().expect("Child process failed to start.");

            println!("{}", Green.paint("Started daemon"));
            println!("pid: {}", daemon.id());

            // Replace with listener for stdout of managed saying "ready"
            std::thread::sleep_ms(20000);

            println!("{}", Yellow.paint("Connecting to daemon"));
            let mut stream = connect_to_daemon(20).expect("Failed to connect to running daemon");
            println!("{}", Green.paint("Connected"));
        }
    };

    /*

    let mut out = String::new();
    stream.read_to_string(&mut out).unwrap();//?;

    println!("{}", out);

    let mut stdout = String::new();
    child.stdout.unwrap().read_to_string(&mut stdout).unwrap();

    println!("[manage-d] [LOG] {}", stdout);

    let mut stderr = String::new();
    child.stderr.unwrap().read_to_string(&mut stderr).unwrap();

    println!("[manage-d] [ERROR] {}", stderr); */

    // {
    //     let mut sysout = String::new();
    //     File::open("/tmp/daemon.out").expect("Outfile not found")
    //         .read_to_string(&mut sysout).expect("Unable to read outfile");
    //     println!("{}\n\t{}", RGB(50, 150, 50).paint("/tmp/daemon.out:"), sysout.replace("\n", "\n\t"));
    // }
    // {
    //     let mut syserr = String::new();
    //     File::open("/tmp/daemon.err").expect("Errfile not found")
    //         .read_to_string(&mut syserr).expect("Unable to read errfile");
    //     println!("{}\n\t{}", RGB(150, 50, 50).paint("/tmp/daemon.err:"), syserr.replace("\n", "\n\t"));
    // }
    
    // child.forget() No Child Left Behind
}

fn connect_to_daemon(tries: u8) -> Option<TcpStream> {
    for n in 0..tries {
        match TcpStream::connect(*DAEMON_ADDRESS) {
            Ok(stream) => return Some(stream),
            Err(e) => eprintln!("{}", e)
        }
    }

    None
}

fn start_daemon() -> io::Result<Child> {
    Command::new("/Users/DusterTheFirst/Documents/Codes/mcmgmt/target/debug/managed")
        .arg("6969")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::null())
        .spawn()
}