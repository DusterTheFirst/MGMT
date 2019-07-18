use std::io::{self, Read, Write};
use std::convert::TryInto;
use byteorder::{BigEndian, ReadBytesExt};

pub mod var_int;
pub mod string;

/// Additions of writing of MC packets to any Read + Sized
impl<R: Write + Sized> WritePacket for R {}

pub trait WritePacket: Write {
    /// Write a packet and output its data
    fn write_packet(&mut self, id: i32, data: &[u8]) -> Result<Packet, io::Error> {
        let ser_id = var_int::write(id);
        let length = (data.len() + ser_id.len()).try_into().unwrap();
        let ser_length = var_int::write(length);

        self.write_all(&ser_length)?;
        self.write_all(&ser_id)?;
        self.write_all(&data)?;

        Ok(Packet {
            id,
            length,
            data: Vec::from(data)
        })
    }
}

/// Additions of reading of MC packets from any Read + Sized
impl<R: Read + Sized> ReadPacket for R {}

pub trait ReadPacket: Read where Self: std::marker::Sized {
    /// Read a packet and output its data
    fn read_packet<'a>(&mut self) -> Result<Packet, io::Error> {
        let length = var_int::read(self)?.value;
        let id = var_int::read(self)?;
        let mut data = vec![0u8; (length - id.length).try_into().unwrap()];

        self.read_exact(&mut data)?;

        Ok(Packet {
            length,
            id: id.value,
            data
        })
    }

    /// Read the handshake packet in and return the data from it
    fn read_handshake(&mut self) -> Result<Handshake, io::Error> {
        let packet = self.read_packet()?;
        let mut data_buf = packet.data.as_slice();

        // if packet.length == 254 {
        //     println!("{}", Red.paint("LEGACY HANDSHAKE, RETURNING\n\n"));
        //     return;
        // }

        // Get the protocol version
        let protocol_version = var_int::read(&mut data_buf)?.value;
        let address = string::read(&mut data_buf)?;
        let port = data_buf.read_u16::<BigEndian>()?;
        let next_state = var_int::read(&mut data_buf)?.value;

        Ok(Handshake {
            packet,
            protocol_version,
            address,
            port,
            next_state
        })
    }
}

pub struct Packet {
    pub length: i32,
    pub id: i32,
    pub data: Vec<u8>
}

pub struct Handshake {
    pub packet: Packet,
    pub protocol_version: i32,
    pub address: String,
    pub port: u16,
    pub next_state: i32
}
