#[macro_use] extern crate lazy_static;

use ansi_term::Colour::*;
use std::net::{SocketAddr, Ipv4Addr, IpAddr, TcpStream};
use std::io::prelude::*;

static PORT: u16 = 9895;

lazy_static! {
    static ref DAEMON_ADDRESS: SocketAddr = SocketAddr::new(
        IpAddr::V4(
            Ipv4Addr::new(127,0,0,1)
        ), 
        PORT
    );
}

fn main() {
    #[cfg(windows)]
    unimplemented!();

    println!("{}", Yellow.paint("Connecting to manage-d daemon process"));
    match TcpStream::connect(*DAEMON_ADDRESS) {
        Ok(mut stream) => {
            println!("{}", Green.paint("Connected"));

            let mut buf = Vec::new();
            stream.read_to_end(&mut buf).unwrap();
            // TODO: Parse and seperate messages
            println!("{:?}", buf);
            println!("{:?}", bincode::deserialize::<core::protocol::Local>(&buf));
        },
        Err(e) => {
            let os = os_type::current_platform();

            eprintln!("{}", Red.paint(format!("{}", e)));
            eprintln!("{} {} {}",
                Yellow.paint("Make sure you have"),
                Blue.paint("manage-d"),
                Yellow.paint("downloaded and running as a system daemon"));
            eprintln!("{} {}",
                Yellow.paint("For information on how to setup manage-d please visit"),
                RGB(66, 155, 245).paint(
                    format!("https://mgmt.dusterthefirst.com/manage-d#{:?}-{}", os.os_type, os.version)));
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