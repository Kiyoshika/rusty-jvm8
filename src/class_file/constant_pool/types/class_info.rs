use crate::util::bytes::vec_to_u16;
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
        let mut buffer = vec![0; 2];
        read_bytes(reader, &mut buffer, 2)?;
        class_info.name_index = vec_to_u16(&buffer);

        Ok(class_info)
    }
}
