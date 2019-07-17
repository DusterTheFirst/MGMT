use std::io::{self, Read, Write};
use std::convert::TryInto;

pub mod var_int;

// /// Read the handshake packet in and return the data from it
// pub fn read_handshake() -> Result<Handshake, io::Error> {

// }

impl<R: Write + Sized> WritePacket for R {}

pub trait WritePacket: Write {
    /// Write a packet and output its data
    fn write_packet(&mut self, id: i32, data: &[u8]) -> Result<Packet, io::Error> {
        let ser_id = var_int::serialize(id);
        let length = (data.len() + ser_id.len()).try_into().unwrap();
        let ser_length = var_int::serialize(length);

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

impl<R: Read + Sized> ReadPacket for R {}

pub trait ReadPacket: Read where Self: std::marker::Sized {
    /// Read a packet and output its data
    fn read_packet<'a>(&mut self) -> Result<Packet, io::Error> {
        let (length, _) = var_int::deserialize(self)?;
        let (id, id_len) = var_int::deserialize(self)?;
        let mut data = vec![0u8; (length - id_len).try_into().unwrap()];

        self.read_exact(&mut data)?;

        Ok(Packet {
            length,
            id,
            data
        })
    }
}

pub struct Packet {
    pub length: i32,
    pub id: i32,
    pub data: Vec<u8>
}


// utf 8 string with size prefix
pub fn prefix_string(string: &str) -> Vec<u8> {
    let string_vec = Vec::from(string);
    let mut size_vec = var_int::serialize(string.len().try_into().unwrap());
    
    size_vec.extend(string_vec);

    size_vec
}