use crate::util::file::read_bytes;
use std::fs::File;
use std::io;
use std::io::BufReader;

pub struct Integer {
    bytes: u32,
}

impl Integer {
    pub fn new() -> Integer {
        Integer { bytes: 0 }
    }

    pub fn from(reader: &mut BufReader<File>) -> Result<Integer, io::Error> {
        let mut integer = Integer::new();

        let mut buffer: [u8; 4] = [0; 4];
        read_bytes(reader, &mut buffer, 4)?;

        integer.bytes = u32::from_be_bytes(buffer);

        Ok(integer)
    }
}
