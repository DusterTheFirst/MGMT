use std::net::{TcpListener, SocketAddr, IpAddr, Ipv4Addr, TcpStream};
use std::time::{Instant, Duration};

use ansi_term::Color::*;

pub mod proto;

use proto::{PacketManipulation, Handshake, NextState, response::{Response, Version}};

pub use proto::{Chat, response::{Players, Player}};

pub struct AsleepServer {
    description: Chat,
    kick_msg: KickMessage,
    favicon: Option<String>,
    sleep_mode: SleepMode,
    listener: TcpListener,
    started: Instant
}

impl AsleepServer {
    pub fn new(motd: Motd, favicon: Option<String>, sleep_mode: SleepMode, kick_msg: KickMessage, port: u16) -> AsleepServer {
        AsleepServer {
            description: motd.into_chat(),
            favicon,
            sleep_mode,
            kick_msg,
            listener: TcpListener::bind(
                SocketAddr::new(
                    IpAddr::V4(
                        Ipv4Addr::new(0,0,0,0)
                    ),
                    port
                )
            ).expect("Unable to bind to socket"),
            started: Instant::now()
        }
    }

    pub fn listen_until_wake(self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    println!("{} {}", Green.paint("New connection:"), stream.peer_addr().unwrap());
                                        
                    // First, the client sends a Handshake packet with its state set to 1.                     
                    let handshake: Handshake = stream.read_handshake().unwrap();
                    println!("{}", RGB(128, 128, 128).paint("HANDSHAKE"));
                    println!("{}\n\n", handshake);

                    match handshake.next_state {
                        NextState::Ping => {
                            self.handle_ping(&mut stream);

                            // Break out of loop if set to wake
                            if self.sleep_mode == SleepMode::WakeOnPing {
                                return;
                            }
                        },
                        NextState::Connect => {
                            self.handle_connect(&mut stream);

                            // Break out of loop if set to wake
                            if self.sleep_mode == SleepMode::WakeOnConnect {
                                return;
                            }
                        },
                        NextState::Unknown(x) => eprintln!("Unknown next_state: {}", x)
                    };
                },
                Err(e) => eprintln!("{:?}", e)
            }
        }
    }

    /// Handle incommiing requests to connect
    fn handle_connect(&self, stream: &mut TcpStream) {
        println!("Handling connection");

        let login_start = stream.read_packet().unwrap();
        let username = proto::string::read(&mut login_start.data.as_slice()).unwrap();

        println!("{}\nUsername: {}", login_start, username);

        let kick_msg = match &self.kick_msg {
            KickMessage::None => String::new(),
            KickMessage::Custom(message) => serde_json::to_string(message).unwrap(),
            KickMessage::Default => serde_json::to_string(&Chat {
                text: String::from("The server is waking up\n\n"),
                bold: true,
                italic: false,
                underlined: false,
                strikethrough: false,
                obfuscated: false,
                color: Some(String::from("green")),
                extra: Some(vec![
                    Chat {
                        // TODO: better reconnect time heuristic
                        text: format!("Please wait {} seconds before reconecting", 10),
                        bold: false,
                        italic: true,
                        underlined: false,
                        strikethrough: false,
                        obfuscated: false,
                        color: Some(String::from("gold")),
                        extra: None
                    }
                ])
            }).unwrap(),
        };

        let chat = proto::string::write(&kick_msg);

        stream.write_packet(0x00, &chat).unwrap();
    }


    /// Handle request to ping
    fn handle_ping(&self, stream: &mut TcpStream) {
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
                        name: format!("§cThe server has been asleep for {}", humantime::format_duration(Duration::new(self.started.elapsed().as_secs(), 0))),
                        id: String::from("00000000-0000-0000-0000-000000000000")
                    },
                    Player {
                        name: match self.sleep_mode {
                            SleepMode::WakeOnConnect => String::from("§6Join now to start the server"),
                            SleepMode::WakeOnPing => String::from("§2The server is currently starting")
                        },
                        id: String::from("00000000-0000-0000-0000-000000000000")
                    },
                    Player {
                        name: String::from("§7You can configure server sleep in the webpanel"),
                        id: String::from("00000000-0000-0000-0000-000000000000")
                    }
                ] 
            },
            description: self.description.clone(),
            favicon: self.favicon.clone()
        };
        let written_packet = stream.write_response(response).unwrap();
        println!("{}", written_packet);

        // If the process is continued, the client will now send a Ping packet containing some payload which is not important.
        // The server will respond with the Pong packet and then close the connection. 
        let packet = stream.ping_pong().unwrap(); 
        println!("{}", RGB(128, 128, 128).paint("PING PONG"));
        println!("{}", packet);
    }
}

#[derive(PartialEq)]
pub enum SleepMode {
    WakeOnPing,
    WakeOnConnect
}

pub enum Motd {
    Chat(Chat),
    Raw(String)
}

pub enum KickMessage {
    Default,
    Custom(Chat),
    None
}

impl Motd {
    pub fn into_chat(self) -> Chat {
        match self {
            Motd::Chat(c) => c,
            Motd::Raw(s) => Chat {
                text: s,
                bold: false,
                italic: false,
                underlined: false,
                strikethrough: false,
                obfuscated: false,
                color: None,
                extra: None
            }
        }
    }
}