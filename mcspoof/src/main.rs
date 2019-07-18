use std::net::{TcpListener, SocketAddr, IpAddr, Ipv4Addr};
use std::thread;
use ansi_term::Color::*;

mod proto;

use proto::{PacketManipulation, Handshake, response::{Description, Player, Players, Response, Version}};

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

                    // The client follows up with a Request packet. This packet has no fields. 
                    let packet = stream.read_packet().unwrap();
                    println!("{}", RGB(128, 128, 128).paint("REQUEST"));
                    println!("{}", packet);
                    
                    // The server should respond with a Response packet.
                    println!("{}", RGB(128, 128, 128).paint("RESPONSE"));
                    let response = Response {
                        version: Version {
                            name: String::from("Server Alseep"),
                            protocol: 0
                        },
                        players: Players {
                            max: 0,
                            online: 0,
                            sample: vec![
                                Player {
                                    name: String::from("§3The server is currently asleep"),
                                    id: String::from("4566e69f-c907-48ee-8d71-d7ba5aa00d20")
                                },
                                Player {
                                    name: String::from("§2Join now to §dwake it up"),
                                    id: String::from("4566e69f-c907-48ee-8d71-d7ba5aa00d20")
                                },
                                Player {
                                    name: String::from("§9You can change how your server sleeps in the control panel"),
                                    id: String::from("4566e69f-c907-48ee-8d71-d7ba5aa00d20")
                                }
                            ]
                        },
                        // TODO: Keep MOTD and icon of server, just change the version and the players
                        description: Description {
                            text: String::from("                  Join to wake\n"),
                            bold: true,
                            italic: false,
                            underlined: false,
                            strikethrough: false,
                            obfuscated: false,
                            color: Some(String::from("gold")),
                            extra: Some(vec![
                                Description {
                                    text: String::from("       Server has been asleep for 3 hours"),
                                    bold: false,
                                    italic: true,
                                    underlined: false,
                                    strikethrough: false,
                                    obfuscated: false,
                                    color: Some(String::from("green")),
                                    extra: None
                                }
                            ])
                        },
                        favicon: Some("data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAABmJLR0QA/wD/AP+gvaeTAAAACXBIWXMAAAsTAAALEwEAmpwYAAAAB3RJTUUH4wcREyI4l1HM8QAAAPFJREFUeNrt2sENwjAQRNEY0QoNUAINpFYaoAQa4Egh5go+xFoZiC2/uQUpUvZr/yRIXhYRmTkpesMtpfx+fcp51wEe6XOES86hmQ6zbwAAOqCS57LkkQa6F9drZUYKAKAD2t7zre/haKKdVOsECgAweY7lD6M5Xz5P9L8JBQDQAbH05nz5PNHvBAoAoAP2TavzNgAAAMbqgL2dtwEAAPDfDujNeRsAAAC/7YDenbcBAADw3Q4YzXkbAAAAbR3Qm/OtZ5goAIAO2HZ49DNCNgAAADZT9fVaOHfufCBnhSkAwHc7oNYJvWUNzkQBAEREJs4Lb5hUptWWBL4AAAAASUVORK5CYII=".to_owned())
                    };
                    let written_packet = stream.write_response(response).unwrap();
                    println!("{}", written_packet);

                    // If the process is continued, the client will now send a Ping packet containing some payload which is not important.
                    // The server will respond with the Pong packet and then close the connection. 
                    let packet = stream.ping_pong().unwrap(); 
                    println!("{}", RGB(128, 128, 128).paint("PING PONG"));
                    println!("{}", packet);
                
                    // let mut buf = Vec::new();
                    // stream.read_to_end(&mut buf).unwrap();
                    // println!("{:?}", buf);
                }).unwrap();
            },
            Err(e) => eprintln!("{:?}", e)
        }
    }
}