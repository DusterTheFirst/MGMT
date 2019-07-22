use std::io::{self, Write, Read, BufWriter, BufReader, BufRead};
use std::convert::TryInto;
use std::marker::PhantomData;
use serde::{Serialize, de::DeserializeOwned};
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

/// A read/write stream for sending and recieving packets
pub struct PacketStream<PR, PW, R, W> 
    where
        PR: DeserializeOwned + std::fmt::Debug,
        PW: Serialize + std::fmt::Debug,
        R: Read,
        W: Write {
    writer: BufWriter<W>,
    reader: BufReader<R>,
    __phantom_read: PhantomData<PR>,
    __phantom_write: PhantomData<PW>
}

impl<PR, PW, R, W> PacketStream<PR, PW, R, W> 
    where
        PW: Serialize + std::fmt::Debug,
        PR: DeserializeOwned + std::fmt::Debug,
        R: Read,
        W: Write {
    
    /// Create a new PacketStream from a reader and writer
    pub fn new(reader: BufReader<R>, writer: BufWriter<W>) -> PacketStream<PR, PW, R, W>
        {
        PacketStream {
            writer,
            reader,
            __phantom_read: PhantomData,
            __phantom_write: PhantomData
        }
    }
}

impl<PR, PW, R, W> PacketStream<PR, PW, R, W> 
    where
        PW: Serialize + std::fmt::Debug,
        PR: DeserializeOwned + std::fmt::Debug,
        R: Read,
        W: Write {
    
    /// Write a packet to a stream
    pub fn write_packet(&mut self, packet: PW) -> Result<(), io::Error> {
        // Serialize the packet
        let data = bincode::serialize(&packet).unwrap();
        // Get the length of the packet
        let len = data.len();

        // Write the length to the stream
        self.writer.write_u32::<BigEndian>(len.try_into().unwrap())?;
        // Write the data to the stream
        self.writer.write(&data)?;
        // Flush the data out
        self.writer.flush()?;

        Ok(())
    }

    pub fn read_packet(&mut self) -> Result<PR, io::Error> {
        // Read the packet size
        let size = self.reader.read_u32::<BigEndian>()?;

        // Allocate a buffer with the correct length for the packet
        let mut buf = vec![0u8; size.try_into().unwrap()];
        // Read the packet into the buffer
        self.reader.read_exact(&mut buf)?;

        // Deserialize the data
        let data = bincode::deserialize(buf.as_slice()).unwrap();

        Ok(data)
    }

    /// Check if the read stream is still open
    pub fn is_open(&mut self) -> bool {
        self.reader.fill_buf().unwrap().len() != 0
    }
}
