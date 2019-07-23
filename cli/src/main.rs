#[macro_use] extern crate lazy_static;

use ansi_term::Colour::*;
use std::net::{SocketAddr, Ipv4Addr, IpAddr, TcpStream};
use std::io::{BufReader, BufWriter};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

use dialoguer::{Select, theme::ColorfulTheme};

use core::{protocol::local, PacketStream};

/// The port to use to communicate with manage-d
static PORT_LOCAL: u16 = 9895;
/// The protocol version used to communicate with manage-d
static PROTOCOL_VERSION_LOCAL: u8 = 0;

lazy_static! {
    static ref DAEMON_ADDRESS: SocketAddr = SocketAddr::new(
        IpAddr::V4(
            Ipv4Addr::new(127,0,0,1)
        ), 
        PORT_LOCAL
    );
    static ref PING_PONG_PAYLOAD: u64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
}

fn main() {
    println!("{}", Yellow.paint("Connecting to manage-d daemon process"));

    match TcpStream::connect(*DAEMON_ADDRESS) {
        Ok(stream) => {
            println!("{}", Green.paint("Connected"));

            let reader = BufReader::new(&stream);
            let writer = BufWriter::new(&stream);

            let mut packet_stream = PacketStream::new(reader, writer);

            while packet_stream.is_open() {
                let packet: local::ToCLI = packet_stream.read_packet().unwrap();

                println!("{} {}",
                    RGB(50, 150, 150).paint("[Packet Recieved]"),
                    RGB(150, 150, 150).paint(format!("{:#?}", packet)));

                match packet {
                    local::ToCLI::Welcome {
                        protocol_version
                    } => {
                        if protocol_version < PROTOCOL_VERSION_LOCAL {
                            println!("{}", Yellow.paint("Manage-d too old"));
                            break;
                        } else if protocol_version > PROTOCOL_VERSION_LOCAL {
                            println!("{}", Yellow.paint("CLI too old"));
                            break;
                        }

                        println!("{}", Green.paint("Protocol versions match"));
                        
                        packet_stream.write_packet(local::ToManageD::Ping(*PING_PONG_PAYLOAD)).unwrap();
                    },
                    local::ToCLI::Pong(payload) => {
                        if payload == *PING_PONG_PAYLOAD {
                            println!("{}", Green.paint("Ping success"));

                            thread::spawn(move || {
                                loop {
                                    let options = [
                                        "Manage containers",
                                        "Manage server connection"
                                    ];
                                    let choice = Select::with_theme(&ColorfulTheme::default()).items(&options).interact().unwrap();
                                    println!("{}, {}", choice, options[choice]);
                                    // let input = Input::<String>::new().with_prompt("manage-d >")
                                    //     .interact().unwrap();
                                    // let input = input.to_lowercase();


                                    // let mut input_parts = input.split(" ");

                                    // let command = input_parts.next().unwrap();

                                    // let args: Vec<_> = input_parts.collect();

                                    // match command.as_ref() {
                                    //     "?" | "help" if args.len() == 0 => println!("{}\n\n{}",
                                    //         RGB(100, 100, 100).paint("Manage-d management daemon configuration CLI"),
                                    //         vec![
                                    //             "Command        Description",
                                    //             "-------        -----------",
                                    //             "help [command]      This list of commands"
                                    //         ].join("\n")),
                                    //     "?" | "help" if args.len() == 1 => println!("Help for {}\n{}", command, match args[0] {
                                    //         "help" => "This list of commands",
                                    //         _ => "Command not found"
                                    //     }),
                                    //     "?" | "help" if args.len() > 1 => println!("Usage: help [command]"),
                                    //     _ => println!("{}", Red.paint("Command not found... use `?` for help"))
                                    // };
                                }
                            });
                        } else {
                            println!("{}", Red.paint("Invalid payload"));
                            break;
                        }
                    }
                };
            }

            println!("{}", Red.paint("Disconnected"));
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