use crate::util::file::read_bytes;
use std::fs::File;
use std::io;
use std::io::BufReader;

pub struct MethodType {
    descriptor_index: u16,
}

impl MethodType {
    pub fn new() -> MethodType {
        MethodType {
            descriptor_index: 0,
        }
    }

    pub fn from(reader: &mut BufReader<File>) -> Result<MethodType, io::Error> {
        let mut method_type: MethodType = MethodType::new();

        let mut buffer: [u8; 2] = [0; 2];

        read_bytes(reader, &mut buffer, 2)?;
        method_type.descriptor_index = u16::from_be_bytes(buffer);

        Ok(method_type)
    }
}
