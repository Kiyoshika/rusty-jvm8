use crate::util::file::read_bytes;
use std::fs::File;
use std::io;
use std::io::BufReader;

#[derive(Copy, Clone)]
pub struct ClassInfo {
    name_index: u16,
}

impl ClassInfo {
    pub fn new() -> ClassInfo {
        ClassInfo { name_index: 0 }
    }

    pub fn from(reader: &mut BufReader<File>) -> Result<ClassInfo, io::Error> {
        let mut class_info = ClassInfo::new();
        let mut buffer: [u8; 2] = [0; 2];
        read_bytes(reader, &mut buffer, 2)?;
        class_info.name_index = u16::from_be_bytes(buffer);

        Ok(class_info)
    }
}
