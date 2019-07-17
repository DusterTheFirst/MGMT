// use hematite_server::proto::slp;
use std::net::{TcpListener, SocketAddr, IpAddr, Ipv4Addr};
use std::io::prelude::*;
use std::thread;
use std::convert::TryInto;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use serde::Serialize;
use serde_json::ser;
use ansi_term::Color::*;

// TODO: CLEAN UP
// TODO: DETECT PLAYER JOIN
fn main() {
    let listener = TcpListener::bind(
        SocketAddr::new(
            IpAddr::V4(
                Ipv4Addr::new(127,0,0,1)
            ),
            25565
        )
    ).expect("Unable to bind to socket");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    println!("{} {}", Green.paint("New connection:"), stream.peer_addr().unwrap());

                    let mut reader = std::io::BufReader::new(&stream);
                    let mut writer = std::io::BufWriter::new(&stream);

                    // connection succeeded
                
                    println!("{}", RGB(128, 128, 128).paint("START HANDSHAKE"));
                    
                    // First, the client sends a Handshake packet with its state set to 1. 
                    let length = read_var_int(&mut reader).unwrap();
                    if length == 254 {
                        println!("{}", Red.paint("LEGACY HANDSHAKE, RETURNING\n\n"));
                        return;
                    }
                    println!("Length: {:?} bytes", length);
                    println!("Packet ID: 0x{:02X}", read_var_int(&mut reader).unwrap());
                    println!("-- DATA --");
                    println!("Protocol version: {}", read_var_int(&mut reader).unwrap());
                    let address_len = read_var_int(&mut reader).unwrap();
                    println!("Address Len: {}", address_len);
                    let mut buf = vec![0u8; address_len.try_into().unwrap()];
                    reader.read_exact(&mut buf).unwrap();
                    println!("Address: {}", String::from_utf8_lossy(&buf));
                    println!("Port: {}", reader.read_u16::<BigEndian>().unwrap());
                    println!("Next State (should be 1): {:?}", read_var_int(&mut reader).unwrap());

                    println!("{}", RGB(128, 128, 128).paint("END HANDSHAKE\n\n"));

                    println!("{}", RGB(128, 128, 128).paint("START REQUEST"));
                    // The client follows up with a Request packet. This packet has no fields. 
                    println!("Length: {:?} bytes", read_var_int(&mut reader).unwrap());
                    println!("Packet ID: 0x{:02X}", read_var_int(&mut reader).unwrap());

                    println!("{}", RGB(128, 128, 128).paint("END REQUEST\n\n"));
                    
                    // The server should respond with a Response packet.
                    println!("{}", RGB(128, 128, 128).paint("START RESPONSE"));
                    let response = ser::to_string(&Response {
                        version: Version {
                            name: String::from("Server Alseep"),
                            protocol: 0
                        },
                        players: Players {
                            max: 100,
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
                        favicon: None
                    }).unwrap();
                    let packet_id = write_var_int(0);
                    let response_len = write_var_int(response.len().try_into().unwrap());
                    let size = packet_id.len() + response.len() + response_len.len();
                    println!("Size: {} bytes", size);
                    println!("Packet ID: 0x{:02X}", 0);
                    println!("Response size: {} bytes", response.len());
                    println!("Response body: {}", response);
                    writer.write_all(&write_var_int(size.try_into().unwrap())).unwrap();
                    writer.write_all(&packet_id).unwrap();
                    // utf 8 string with size prefix
                    writer.write_all(&response_len).unwrap();
                    writer.write_all(&response.as_bytes()).unwrap();
                    writer.flush().unwrap();

                    println!("{}", RGB(128, 128, 128).paint("END RESPONSE\n\n"));

                    println!("{}", RGB(128, 128, 128).paint("START PING"));
                    
                    // If the process is continued, the client will now send a Ping packet containing some payload which is not important. 
                    println!("Length: {:?} bytes", read_var_int(&mut reader).unwrap());
                    let packet_id = read_var_int(&mut reader).unwrap();
                    println!("Packet ID: 0x{:02X}", packet_id);
                    let payload = reader.read_i64::<BigEndian>().unwrap();
                    println!("Payload: 0x{:08X}", payload);

                    println!("{}", RGB(128, 128, 128).paint("END PING\n\n"));

                    println!("{}", RGB(128, 128, 128).paint("START PONG"));
                    
                    // The server will respond with the Pong packet and then close the connection. 
                    let packet_id = write_var_int(1);
                    let length = packet_id.len() + 8;
                    writer.write_all(&write_var_int(length.try_into().unwrap())).unwrap();
                    writer.write_all(&packet_id).unwrap();
                    writer.write_i64::<BigEndian>(payload).unwrap();
                    println!("Length: {:?} bytes", length);
                    println!("Packet ID: 0x{:02X}", 1);
                    println!("Payload: 0x{:08X}", payload);

                    println!("{}", RGB(128, 128, 128).paint("END PONG\n\n"));

                    // let mut buf = Vec::new();
                    // stream.read_to_end(&mut buf).unwrap();
                    // println!("{:?}", buf);
                });
            },
            Err(e) => eprintln!("{:?}", e)
        }
    }
}

fn read_var_int<T>(stream: &mut T) -> Result<i32, &'static str> where T: Read {
    let mut num_read = 0i32;
    let mut result = 0i32;
    let mut read: u8;

    while {
        let mut buf = [0];
        stream.read(&mut buf).unwrap();
        read = buf[0];
        let value = read & 0b01111111;
        result |= (value as i32) << (7 * num_read);

        num_read += 1;
        if num_read > 5 {
            return Err("VarInt is too big");
        }

        (read & 0b10000000) != 0
    } {}

    return Ok(result);
}

fn write_var_int(value: i32) -> Vec<u8> {
    let mut buf = Vec::new();
    let mut mut_val = value.clone();
    while {
        let mut temp = (mut_val & 0b01111111) as u8;
        // Note: >>> means that the sign bit is shifted with the rest of the number rather than being left alone
        mut_val >>= 7;
        if mut_val != 0 {
            temp |= 0b10000000;
        }
        buf.push(temp);
        mut_val != 0
    } {};
    buf
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