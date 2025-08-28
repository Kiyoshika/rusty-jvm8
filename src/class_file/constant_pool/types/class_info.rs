use crate::class_file::constant_pool::constant_pool::ConstantPool;
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

    pub fn from(
        reader: &mut BufReader<impl Read>,
        constant_pool: &ConstantPool,
    ) -> Result<ClassInfo, io::Error> {
        info!("Now Parsing ClassInfo");
        let mut class_info = ClassInfo::new();
        let mut buffer: [u8; 2] = [0; 2];
        read_bytes(reader, &mut buffer, 2)?;
        class_info.name_index = u16::from_be_bytes(buffer);
        if class_info.name_index == 0
            || class_info.name_index > constant_pool.max_constant_pool_index()
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "name_index '{:?}' must be a valid index into the constant pool (1-{:?})",
                    class_info.name_index,
                    constant_pool.max_constant_pool_index()
                ),
            ));
        }
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
        let mut constant_pool = ConstantPool::new();
        constant_pool.set_count(600);

        let bytes = [0x02, 0x0A]; // 522
        let mut reader = BufReader::new(bytes.as_ref());

        let class_info = ClassInfo::from(&mut reader, &constant_pool).unwrap();
        assert_eq!(class_info.name_index(), 522);
    }

    #[test]
    pub fn read_valid_u8() {
        let mut constant_pool = ConstantPool::new();
        constant_pool.set_count(10);

        let bytes = [0x00, 0x05]; // 5
        let mut reader = BufReader::new(bytes.as_ref());

        let class_info = ClassInfo::from(&mut reader, &constant_pool).unwrap();
        assert_eq!(class_info.name_index(), 5);
    }

    #[test]
    pub fn name_index_invalid_constant_pool_index() {
        let mut constant_pool = ConstantPool::new();
        constant_pool.set_count(2);

        let bytes = [0x00, 0x05]; // 5
        let mut reader = BufReader::new(bytes.as_ref());

        let class_info = ClassInfo::from(&mut reader, &constant_pool);
        assert_eq!(class_info.is_err(), true);
    }
}
