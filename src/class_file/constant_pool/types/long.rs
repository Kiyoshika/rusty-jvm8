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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_small_long_parts() {
        // high = 0x00000000, low = 0x00000001
        let bytes = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01];
        let mut reader = BufReader::new(bytes.as_ref());
        let l = Long::from(&mut reader).unwrap();
        assert_eq!(l.high_bytes, 0x00000000);
        assert_eq!(l.low_bytes, 0x00000001);
    }

    #[test]
    fn read_large_long_parts() {
        // high = 0x89ABCDEF, low = 0x01234567
        let bytes = [0x89, 0xAB, 0xCD, 0xEF, 0x01, 0x23, 0x45, 0x67];
        let mut reader = BufReader::new(bytes.as_ref());
        let l = Long::from(&mut reader).unwrap();
        assert_eq!(l.high_bytes, 0x89ABCDEF);
        assert_eq!(l.low_bytes, 0x01234567);
    }
}
