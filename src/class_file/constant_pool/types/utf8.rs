use crate::util::file::read_bytes;
use std::fs::File;
use std::io;
use std::io::BufReader;

pub struct Utf8 {
    length: u16,
    bytes: Vec<u8>,
}

impl Utf8 {
    pub fn new() -> Utf8 {
        Utf8 {
            length: 0,
            bytes: Vec::new(),
        }
    }

    pub fn from(reader: &mut BufReader<File>) -> Result<Utf8, io::Error> {
        let mut utf8: Utf8 = Utf8::new();

        let mut buffer: [u8; 2] = [0; 2];
        read_bytes(reader, &mut buffer, 2)?;
        utf8.length = u16::from_be_bytes(buffer);

        read_bytes(reader, &mut utf8.bytes, utf8.length as usize)?;

        // verify byte values
        // 4.4.7: no byte may have value 0x0 or lie in range 0xf0 to 0xff
        for byte in utf8.bytes.iter() {
            if *byte == 0x0 || *byte >= 0xf0 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "UTF8 bytes must not be 0x0 or lie in range 0xf0 to 0xff",
                ));
            }
        }

        Ok(utf8)
    }
}
