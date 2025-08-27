use crate::util::file::read_bytes;
use std::fs::File;
use std::io;
use std::io::BufReader;

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

    pub fn from(reader: &mut BufReader<File>) -> Result<NameAndType, io::Error> {
        let mut name_and_type = NameAndType::new();

        let mut buffer: [u8; 2] = [0; 2];
        read_bytes(reader, &mut buffer, 2)?;
        name_and_type.name_index = u16::from_le_bytes(buffer);

        read_bytes(reader, &mut buffer, 2)?;
        name_and_type.descriptor_index = u16::from_le_bytes(buffer);

        Ok(name_and_type)
    }
}
