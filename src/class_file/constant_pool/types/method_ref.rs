use crate::util::file::read_bytes;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

pub struct MethodRef {
    class_index: u16,
    name_and_type_index: u16,
}

impl MethodRef {
    pub fn new() -> MethodRef {
        MethodRef {
            class_index: 0,
            name_and_type_index: 0,
        }
    }

    pub fn class_index(&self) -> u16 {
        self.class_index
    }

    pub fn name_and_type_index(&self) -> u16 {
        self.name_and_type_index
    }

    pub fn from(reader: &mut BufReader<impl Read>) -> Result<MethodRef, io::Error> {
        let mut method_ref: MethodRef = MethodRef::new();

        let mut buffer: [u8; 2] = [0; 2];

        read_bytes(reader, &mut buffer, 2)?;
        method_ref.class_index = u16::from_be_bytes(buffer);

        read_bytes(reader, &mut buffer, 2)?;
        method_ref.name_and_type_index = u16::from_be_bytes(buffer);

        Ok(method_ref)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_small_indices() {
        // class_index = 3, name_and_type_index = 7
        let bytes = [0x00, 0x03, 0x00, 0x07];
        let mut reader = BufReader::new(bytes.as_ref());
        let mr = MethodRef::from(&mut reader).unwrap();
        assert_eq!(mr.class_index(), 3);
        assert_eq!(mr.name_and_type_index(), 7);
    }

    #[test]
    fn read_large_indices() {
        // class_index = 0x8001, name_and_type_index = 0xFFEE
        let bytes = [0x80, 0x01, 0xFF, 0xEE];
        let mut reader = BufReader::new(bytes.as_ref());
        let mr = MethodRef::from(&mut reader).unwrap();
        assert_eq!(mr.class_index(), 0x8001);
        assert_eq!(mr.name_and_type_index(), 0xFFEE);
    }
}
