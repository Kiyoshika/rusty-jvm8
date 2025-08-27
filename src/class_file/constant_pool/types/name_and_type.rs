use crate::util::file::read_bytes;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

pub struct NameAndType {
    name_index: u16,
    descriptor_index: u16,
}

impl NameAndType {
    pub fn new() -> NameAndType {
        NameAndType {
            name_index: 0,
            descriptor_index: 0,
        }
    }

    pub fn name_index(&self) -> u16 {
        self.name_index
    }

    pub fn descriptor_index(&self) -> u16 {
        self.descriptor_index
    }

    pub fn from(reader: &mut BufReader<impl Read>) -> Result<NameAndType, io::Error> {
        let mut name_and_type = NameAndType::new();

        let mut buffer: [u8; 2] = [0; 2];
        read_bytes(reader, &mut buffer, 2)?;
        name_and_type.name_index = u16::from_be_bytes(buffer);

        read_bytes(reader, &mut buffer, 2)?;
        name_and_type.descriptor_index = u16::from_be_bytes(buffer);

        Ok(name_and_type)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_small_values() {
        // name_index = 1, descriptor_index = 2
        let bytes = [0x00, 0x01, 0x00, 0x02];
        let mut reader = BufReader::new(bytes.as_ref());
        let nat = NameAndType::from(&mut reader).unwrap();
        assert_eq!(nat.name_index(), 1);
        assert_eq!(nat.descriptor_index(), 2);
    }

    #[test]
    fn read_large_values() {
        // name_index = 0xFFFE (65534), descriptor_index = 0xABCD (43981)
        let bytes = [0xFF, 0xFE, 0xAB, 0xCD];
        let mut reader = BufReader::new(bytes.as_ref());
        let nat = NameAndType::from(&mut reader).unwrap();
        assert_eq!(nat.name_index(), 65534);
        assert_eq!(nat.descriptor_index(), 0xABCD);
    }
}
