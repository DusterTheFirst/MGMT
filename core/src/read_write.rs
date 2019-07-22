use std::io::{self, Write, Read, BufWriter, BufReader, BufRead};
use std::convert::TryInto;
use std::marker::PhantomData;
use serde::{Serialize, de::DeserializeOwned};
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

pub struct PacketReadWriter<PR, PW, R, W> 
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

impl<PR, PW, R, W> PacketReadWriter<PR, PW, R, W> 
    where
        PW: Serialize + std::fmt::Debug,
        PR: DeserializeOwned + std::fmt::Debug,
        R: Read,
        W: Write{
    pub fn new(reader: BufReader<R>, writer: BufWriter<W>) -> PacketReadWriter<PR, PW, R, W>
        {
        PacketReadWriter {
            writer,
            reader,
            __phantom_read: PhantomData,
            __phantom_write: PhantomData
        }
    }
}

impl<PR, PW, R, W> PacketReadWriter<PR, PW, R, W> 
    where
        PW: Serialize + std::fmt::Debug,
        PR: DeserializeOwned + std::fmt::Debug,
        R: Read,
        W: Write {
    

    pub fn write_packet(&mut self, packet: PW) -> Result<(), io::Error> {
        let data = bincode::serialize(&packet).unwrap();
        let len = data.len();

        println!("Writing {} bytes:\n {:?}\nRAW:\n{:X?}", len, packet, data);

        self.writer.write_u32::<BigEndian>(len.try_into().unwrap())?;
        self.writer.write(&data)?;
        self.writer.flush()?;

        Ok(())
    }

    pub fn read_packet(&mut self) -> Result<PR, io::Error> {
        let size = self.reader.read_u32::<BigEndian>().unwrap();
        println!("Reading {} bytes:", size);
        let mut buf = vec![0u8; size.try_into().unwrap()];
        self.reader.read_exact(&mut buf)?;
        println!("{:X?}", buf);

        let data = bincode::deserialize(buf.as_slice()).unwrap();

        Ok(data)
    }

    pub fn is_open(&mut self) -> bool {
        self.reader.fill_buf().unwrap().len() != 0
    }
}
