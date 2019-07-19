use std::net::{TcpListener, SocketAddr, IpAddr, Ipv4Addr, TcpStream};
use std::thread;
use ansi_term::Color::*;

mod proto;

use proto::{PacketManipulation, Handshake, NextState, Chat, response::{Player, Players, Response, Version}};

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
                    println!("{} {}", Green.paint("New connection:"), stream.peer_addr().unwrap());
                                    
                    // First, the client sends a Handshake packet with its state set to 1.                     
                    let handshake: Handshake = stream.read_handshake().unwrap();
                    println!("{}", RGB(128, 128, 128).paint("HANDSHAKE"));
                    println!("{}\n\n", handshake);

                    match handshake.next_state {
                        NextState::Ping => handle_ping(&mut stream),
                        NextState::Connect => handle_connect(&mut stream),
                        NextState::Unknown(x) => println!("Unknown next_state: {}", x)
                    };

                    
                
                    // let mut buf = Vec::new();
                    // stream.read_to_end(&mut buf).unwrap();
                    // println!("{:?}", buf);
                }).unwrap();
            },
            Err(e) => eprintln!("{:?}", e)
        }
    }
}

fn handle_connect(stream: &mut TcpStream) {
    println!("Handling connection");

    let login_start = stream.read_packet().unwrap();
    let username = proto::string::read(&mut login_start.data.as_slice()).unwrap();

    println!("{}\nUsername: {}", login_start, username);

    let chat = proto::string::write(&serde_json::to_string(&Chat {
            text: String::from("The server is waking up\n\n"),
            bold: true,
            italic: false,
            underlined: false,
            strikethrough: false,
            obfuscated: false,
            color: Some(String::from("green")),
            extra: Some(vec![
                Chat {
                    text: format!("Please wait {} seconds before recconecting", 10),
                    bold: false,
                    italic: true,
                    underlined: false,
                    strikethrough: false,
                    obfuscated: false,
                    color: Some(String::from("gold")),
                    extra: None
                }
            ])
        },).unwrap());

    stream.write_packet(0x00, &chat).unwrap();
}

fn handle_ping(stream: &mut TcpStream) {
    // The client follows up with a Request packet. This packet has no fields. 
    let packet = stream.read_packet().unwrap();
    println!("{}", RGB(128, 128, 128).paint("REQUEST"));
    println!("{}", packet);
    
    // The server should respond with a Response packet.
    println!("{}", RGB(128, 128, 128).paint("RESPONSE"));
    let response = Response {
        version: Version {
            name: String::from("Server Asleep"),
            protocol: 0
        },
        players: Players {
            max: 0,
            online: 0,
            sample: vec![
                Player {
                    name: format!("§3The server has been asleep for §d{} {}", 30, "minutes"),
                    id: String::from("4566e69f-c907-48ee-8d71-d7ba5aa00d20")
                },
                Player {
                    name: String::from("§2Join now to wake it up"),
                    id: String::from("4566e69f-c907-48ee-8d71-d7ba5aa00d20")
                },
                Player {
                    name: String::from("§9You can change how your server sleeps in the control panel"),
                    id: String::from("4566e69f-c907-48ee-8d71-d7ba5aa00d20")
                },
                Player {
                    name: String::from("§7For more info, visit https://mgmt.dusterthefirst.com/about#sleep"),
                    id: String::from("4566e69f-c907-48ee-8d71-d7ba5aa00d20")
                }
            ]
        },
        // TODO: Keep MOTD and icon of server, just change the version and the players
        description: Chat {
            text: String::from("A faked Minecraft Server"),
            bold: false,
            italic: false,
            underlined: false,
            strikethrough: false,
            obfuscated: false,
            color: None,
            extra: None
        },
        favicon: Some("data:image/png;base64,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".to_owned())
    };
    let written_packet = stream.write_response(response).unwrap();
    println!("{}", written_packet);

    // If the process is continued, the client will now send a Ping packet containing some payload which is not important.
    // The server will respond with the Pong packet and then close the connection. 
    let packet = stream.ping_pong().unwrap(); 
    println!("{}", RGB(128, 128, 128).paint("PING PONG"));
    println!("{}", packet);
}