use crate::util::file::read_bytes;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

pub struct MethodType {
    descriptor_index: u16,
}

impl MethodType {
    pub fn new() -> MethodType {
        MethodType {
            descriptor_index: 0,
        }
    }

    pub fn from(reader: &mut BufReader<impl Read>) -> Result<MethodType, io::Error> {
        let mut method_type: MethodType = MethodType::new();

        let mut buffer: [u8; 2] = [0; 2];

        read_bytes(reader, &mut buffer, 2)?;
        method_type.descriptor_index = u16::from_be_bytes(buffer);

        Ok(method_type)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_small_descriptor_index() {
        let bytes = [0x00, 0x05];
        let mut reader = BufReader::new(bytes.as_ref());
        let mt = MethodType::from(&mut reader).unwrap();
        // access private field from child module allowed
        assert_eq!(mt.descriptor_index, 5);
    }

    #[test]
    fn read_large_descriptor_index() {
        let bytes = [0xFE, 0xDC]; // 0xFEDC
        let mut reader = BufReader::new(bytes.as_ref());
        let mt = MethodType::from(&mut reader).unwrap();
        assert_eq!(mt.descriptor_index, 0xFEDC);
    }
}
