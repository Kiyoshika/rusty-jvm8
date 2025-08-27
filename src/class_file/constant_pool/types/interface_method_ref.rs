use crate::util::file::read_bytes;
use std::fs::File;
use std::io;
use std::io::BufReader;

pub struct InterfaceMethodRef {
    class_index: u16,
    name_and_type_index: u16,
}

impl InterfaceMethodRef {
    pub fn new() -> InterfaceMethodRef {
        InterfaceMethodRef {
            class_index: 0,
            name_and_type_index: 0,
        }
    }

    pub fn from(reader: &mut BufReader<File>) -> Result<InterfaceMethodRef, io::Error> {
        let mut interface_method_ref: InterfaceMethodRef = InterfaceMethodRef::new();

        let mut buffer: [u8; 2] = [0; 2];

        read_bytes(reader, &mut buffer, 2)?;
        interface_method_ref.class_index = u16::from_be_bytes(buffer);

        read_bytes(reader, &mut buffer, 2)?;
        interface_method_ref.name_and_type_index = u16::from_be_bytes(buffer);

        Ok(interface_method_ref)
    }
}
