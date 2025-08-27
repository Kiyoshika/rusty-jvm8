use crate::util::file::read_bytes;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

/// Named JvmString to avoid ambiguity with String.
/// Represents constant objects of String type.
pub struct JvmString {
    string_index: u16,
}

impl JvmString {
    pub fn new() -> JvmString {
        JvmString { string_index: 0 }
    }

    pub fn string_index(&self) -> u16 {
        self.string_index
    }

    pub fn from(reader: &mut BufReader<impl Read>) -> Result<JvmString, io::Error> {
        let mut string = JvmString::new();

        let mut buffer: [u8; 2] = [0; 2];
        read_bytes(reader, &mut buffer, 2)?;
        string.string_index = u16::from_be_bytes(buffer);

        Ok(string)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_small_string_index() {
        let bytes = [0x00, 0x02];
        let mut reader = BufReader::new(bytes.as_ref());
        let s = JvmString::from(&mut reader).unwrap();
        assert_eq!(s.string_index(), 2);
    }

    #[test]
    fn read_large_string_index() {
        let bytes = [0xFE, 0xDC]; // 0xFEDC
        let mut reader = BufReader::new(bytes.as_ref());
        let s = JvmString::from(&mut reader).unwrap();
        assert_eq!(s.string_index(), 0xFEDC);
    }
}
