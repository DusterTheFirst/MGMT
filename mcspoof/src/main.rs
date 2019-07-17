use std::net::{TcpListener, SocketAddr, IpAddr, Ipv4Addr};
use std::io::prelude::*;
use std::thread;
use std::convert::TryInto;
use byteorder::{BigEndian, ByteOrder, ReadBytesExt};
use serde::Serialize;
use serde_json::ser;
use ansi_term::Color::*;

mod proto;

use proto::{ReadPacket, WritePacket, var_int};

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
            Ok(stream) => {
                thread::Builder::new().name("Stream Handler".to_owned()).spawn(move || {
                    println!("{} {}", Green.paint("New connection:"), stream.peer_addr().unwrap());

                    let mut reader = std::io::BufReader::new(&stream);
                    let mut writer = std::io::BufWriter::new(&stream);

                    // connection succeeded
                
                    println!("{}", RGB(128, 128, 128).paint("START HANDSHAKE"));
                    
                    // First, the client sends a Handshake packet with its state set to 1.                     
                    let packet = reader.read_packet().unwrap();
                    if packet.length == 254 {
                        println!("{}", Red.paint("LEGACY HANDSHAKE, RETURNING\n\n"));
                        return;
                    }
                    println!("Length: {:?} bytes", packet.length);
                    println!("Packet ID: 0x{:02X}", packet.id);
                    // TODO: do not use as reader
                    let mut packet_data_buf = packet.data.as_slice();
                    println!("-- DATA --");
                    println!("Protocol version: {}", var_int::deserialize(&mut packet_data_buf).unwrap().0);
                    let address_len = var_int::deserialize(&mut packet_data_buf).unwrap().0;
                    println!("Address Len: {}", address_len);
                    let mut buf = vec![0u8; address_len.try_into().unwrap()];
                    packet_data_buf.read_exact(&mut buf).unwrap();
                    println!("Address: {}", String::from_utf8_lossy(&buf));
                    println!("Port: {}", packet_data_buf.read_u16::<BigEndian>().unwrap());
                    println!("Next State (1 = ping, 2 = connect): {:?}", var_int::deserialize(&mut packet_data_buf).unwrap().0);

                    println!("{}", RGB(128, 128, 128).paint("END HANDSHAKE\n\n"));

                    println!("{}", RGB(128, 128, 128).paint("START REQUEST"));

                    // The client follows up with a Request packet. This packet has no fields. 
                    let packet = reader.read_packet().unwrap();
                    println!("Length: {:?} bytes", packet.length);
                    println!("Packet ID: 0x{:02X}", packet.id);

                    println!("{}", RGB(128, 128, 128).paint("END REQUEST\n\n"));
                    
                    // The server should respond with a Response packet.
                    println!("{}", RGB(128, 128, 128).paint("START RESPONSE"));
                    let response = ser::to_string(&Response {
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
                    }).unwrap();
                    let response_prefixed = proto::prefix_string(&response);
                    let written_packet = writer.write_packet(0x00, response_prefixed.as_slice()).unwrap();
                    println!("Size: {} bytes", written_packet.length);
                    println!("Packet ID: 0x{:02X}", written_packet.id);
                    println!("Response size: {} bytes", response.len());
                    println!("Response body: {}", response);
                    writer.flush().unwrap();

                    println!("{}", RGB(128, 128, 128).paint("END RESPONSE\n\n"));

                    println!("{}", RGB(128, 128, 128).paint("START PING"));
                    
                    // If the process is continued, the client will now send a Ping packet containing some payload which is not important.
                    let packet = reader.read_packet().unwrap(); 
                    println!("Length: {:?} bytes", packet.length);
                    println!("Packet ID: 0x{:02X}", packet.id);
                    let payload = BigEndian::read_i64(packet.data.as_slice());
                    println!("Payload: 0x{:08X}", payload);

                    println!("{}", RGB(128, 128, 128).paint("END PING\n\n"));

                    println!("{}", RGB(128, 128, 128).paint("START PONG"));
                    
                    // The server will respond with the Pong packet and then close the connection. 
                    let written_packet = writer.write_packet(1, packet.data.as_slice()).unwrap();
                    println!("Length: {:?} bytes", written_packet.length);
                    println!("Packet ID: 0x{:02X}", written_packet.id);
                    println!("Payload: 0x{:08X}", payload);

                    println!("{}", RGB(128, 128, 128).paint("END PONG\n\n"));

                    // let mut buf = Vec::new();
                    // stream.read_to_end(&mut buf).unwrap();
                    // println!("{:?}", buf);
                }).unwrap();
            },
            Err(e) => eprintln!("{:?}", e)
        }
    }
}

#[derive(Serialize)]
struct Response {
    version: Version,
    players: Players,	
    description: Description,
    #[serde(skip_serializing_if = "Option::is_none")]
    favicon: Option<String>
}

#[derive(Serialize)]
struct Version {
    name: String,
    protocol: u16
}

#[derive(Serialize)]
struct Players {
    max: u16,
    online: u16,
    sample: Vec<Player>
}

#[derive(Serialize)]
struct Player {
    name: String,
    id: String
}

#[derive(Serialize)]
struct Description {
    text: String,
    bold: bool,
    italic: bool,
    underlined: bool,
    strikethrough: bool,
    obfuscated: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extra: Option<Vec<Description>>
}