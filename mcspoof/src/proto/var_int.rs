use std::io::{self, ErrorKind, Read};

/// Parse in a var int and return the value and its length
pub fn deserialize<T>(stream: &mut T) -> Result<(i32, i32), io::Error>
    where T: Read {
    let mut num_read: i32 = 0;
    let mut result = 0i32;
    let mut read: u8;

    while {
        let mut buf = [0];
        stream.read(&mut buf).unwrap();
        read = buf[0];
        let value = read & 0b0111_1111;
        result |= (value as i32) << (7 * num_read);

        num_read += 1;
        if num_read > 5 {
            return Err(io::Error::new(ErrorKind::InvalidInput, "VarInt is too big"));
        }

        (read & 0b1000_0000) != 0
    } {}

    Ok((result, num_read))
}

/// Convert an integer to a var_int
pub fn serialize(value: i32) -> Vec<u8> {
    let mut buf = Vec::new();
    let mut mut_val = value.clone();

    while {
        let mut temp = (mut_val & 0b0111_1111) as u8;
        // Note: >>> means that the sign bit is shifted with the rest of the number rather than being left alone
        mut_val >>= 7;
        if mut_val != 0 {
            temp |= 0b1000_0000;
        }
        buf.push(temp);
        mut_val != 0
    } {};

    buf
}