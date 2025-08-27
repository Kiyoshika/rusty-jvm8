use crate::util::file::read_bytes;
use std::fs::File;
use std::io;
use std::io::BufReader;

pub struct FieldRef {
    class_index: u16,
    name_and_type_index: u16,
}

impl FieldRef {
    pub fn new() -> FieldRef {
        FieldRef {
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

    pub fn from(reader: &mut BufReader<File>) -> Result<FieldRef, io::Error> {
        let mut field_ref: FieldRef = FieldRef::new();

        let mut buffer: [u8; 2] = [0; 2];

        read_bytes(reader, &mut buffer, 2)?;
        field_ref.class_index = u16::from_be_bytes(buffer);

        read_bytes(reader, &mut buffer, 2)?;
        field_ref.name_and_type_index = u16::from_be_bytes(buffer);

        Ok(field_ref)
    }
}
