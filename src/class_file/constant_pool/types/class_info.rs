use crate::util::file::read_bytes;
use log::{debug, info};
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

#[derive(Copy, Clone, Debug)]
pub struct ClassInfo {
    name_index: u16,
}

impl ClassInfo {
    pub fn new() -> ClassInfo {
        ClassInfo { name_index: 0 }
    }

    pub fn name_index(&self) -> u16 {
        self.name_index
    }

    pub fn from(reader: &mut BufReader<impl Read>) -> Result<ClassInfo, io::Error> {
        info!("Now Parsing ClassInfo");
        let mut class_info = ClassInfo::new();
        let mut buffer: [u8; 2] = [0; 2];
        read_bytes(reader, &mut buffer, 2)?;
        class_info.name_index = u16::from_be_bytes(buffer);
        info!("Finished parsing ClassInfo");
        debug!("ClassInfo: {:?}", class_info);

        Ok(class_info)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn read_valid_u16() {
        let bytes = [0x02, 0x0A]; // 522
        let mut reader = BufReader::new(bytes.as_ref());
        let class_info = ClassInfo::from(&mut reader).unwrap();
        assert_eq!(class_info.name_index(), 522);
    }

    #[test]
    pub fn read_valid_u8() {
        let bytes = [0x00, 0x05]; // 5
        let mut reader = BufReader::new(bytes.as_ref());
        let class_info = ClassInfo::from(&mut reader).unwrap();
        assert_eq!(class_info.name_index(), 5);
    }
}
