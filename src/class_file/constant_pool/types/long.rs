use crate::util::file::read_bytes;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

pub struct Long {
    high_bytes: u32,
    low_bytes: u32,
}

impl Long {
    pub fn new() -> Long {
        Long {
            high_bytes: 0,
            low_bytes: 0,
        }
    }

    pub fn from(reader: &mut BufReader<impl Read>) -> Result<Long, io::Error> {
        let mut long: Long = Long::new();

        let mut buffer: [u8; 4] = [0; 4];

        read_bytes(reader, &mut buffer, 4)?;
        long.high_bytes = u32::from_be_bytes(buffer);

        read_bytes(reader, &mut buffer, 4)?;
        long.low_bytes = u32::from_be_bytes(buffer);

        Ok(long)
    }
}
