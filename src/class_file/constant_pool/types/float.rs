use crate::util::file::read_bytes;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

pub struct Float {
    bytes: u32,
}

impl Float {
    pub fn new() -> Float {
        Float { bytes: 0 }
    }

    pub fn from(reader: &mut BufReader<impl Read>) -> Result<Float, io::Error> {
        let mut float = Float::new();

        let mut buffer: [u8; 4] = [0; 4];
        read_bytes(reader, &mut buffer, 4)?;

        float.bytes = u32::from_be_bytes(buffer);

        Ok(float)
    }
}
