use crate::util::file::read_bytes;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

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

    pub fn length(&self) -> u16 {
        self.length
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the UTF-8 decoded string value. For the current use-cases,
    /// class file Modified UTF-8 specifics don't affect these literals.
    pub fn value(&self) -> String {
        String::from_utf8_lossy(&self.bytes).to_string()
    }

    pub fn from(reader: &mut BufReader<impl Read>) -> Result<Utf8, io::Error> {
        let mut utf8: Utf8 = Utf8::new();

        let mut buffer: [u8; 2] = [0; 2];
        read_bytes(reader, &mut buffer, 2)?;
        utf8.length = u16::from_be_bytes(buffer);

        // allocate space for bytes and read content
        utf8.bytes.resize(utf8.length as usize, 0);
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_zero_length() {
        // length = 0, no bytes follow
        let bytes = [0x00, 0x00];
        let mut reader = BufReader::new(bytes.as_ref());
        let utf8 = Utf8::from(&mut reader).unwrap();
        assert_eq!(utf8.length(), 0);
        assert_eq!(utf8.bytes(), &[]);
        assert_eq!(utf8.value(), "");
    }

    #[test]
    fn read_small_length_one() {
        // length = 1, byte = 0x01 (valid: not 0x00 and < 0xF0)
        let bytes = [0x00, 0x01, 0x01];
        let mut reader = BufReader::new(bytes.as_ref());
        let utf8 = Utf8::from(&mut reader).unwrap();
        assert_eq!(utf8.length(), 1);
        assert_eq!(utf8.bytes(), &[0x01]);
    }

    #[test]
    fn read_large_length_three_edge_bytes() {
        // length = 3, bytes near upper allowed boundary: 0xEE, 0xEF, 0x7F
        let bytes = [0x00, 0x03, 0xEE, 0xEF, 0x7F];
        let mut reader = BufReader::new(bytes.as_ref());
        let utf8 = Utf8::from(&mut reader).unwrap();
        assert_eq!(utf8.length(), 3);
        assert_eq!(utf8.bytes(), &[0xEE, 0xEF, 0x7F]);
    }

    #[test]
    fn invalid_zero_byte_fails() {
        // length = 1, byte = 0x00 is invalid per check
        let bytes = [0x00, 0x01, 0x00];
        let mut reader = BufReader::new(bytes.as_ref());
        assert!(Utf8::from(&mut reader).is_err());
    }

    #[test]
    fn invalid_high_range_fails() {
        // length = 1, byte = 0xF0 is invalid per check
        let bytes = [0x00, 0x01, 0xF0];
        let mut reader = BufReader::new(bytes.as_ref());
        assert!(Utf8::from(&mut reader).is_err());
    }
}
