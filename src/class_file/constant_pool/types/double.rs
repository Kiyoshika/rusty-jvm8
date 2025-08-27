use crate::util::file::read_bytes;
use std::fs::File;
use std::io;
use std::io::BufReader;

pub struct Double {
    high_bytes: u32,
    low_bytes: u32,
}

impl Double {
    pub fn new() -> Double {
        Double {
            high_bytes: 0,
            low_bytes: 0,
        }
    }

    pub fn from(reader: &mut BufReader<File>) -> Result<Double, io::Error> {
        let mut double: Double = Double::new();

        let mut buffer: [u8; 4] = [0; 4];

        read_bytes(reader, &mut buffer, 4)?;
        double.high_bytes = u32::from_be_bytes(buffer);

        read_bytes(reader, &mut buffer, 4)?;
        double.low_bytes = u32::from_be_bytes(buffer);

        Ok(double)
    }
}
