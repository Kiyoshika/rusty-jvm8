use crate::util::bytes::buffer_to_u16;
use crate::util::file::read_bytes;
use std::fs::File;
use std::io;
use std::io::BufReader;

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

    pub fn from(reader: &mut BufReader<File>) -> Result<MethodRef, io::Error> {
        let mut method_ref: MethodRef = MethodRef::new();

        let mut buffer: [u8; 2] = [0; 2];

        read_bytes(reader, &mut buffer, 2)?;
        method_ref.class_index = buffer_to_u16(buffer);

        read_bytes(reader, &mut buffer, 2)?;
        method_ref.name_and_type_index = buffer_to_u16(buffer);

        Ok(method_ref)
    }
}
