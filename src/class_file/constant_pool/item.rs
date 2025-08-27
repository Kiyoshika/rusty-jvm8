use crate::class_file::constant_pool::tag::ConstantPoolTag;
use crate::class_file::constant_pool::types::class_info::ClassInfo;
use crate::class_file::constant_pool::types::field_ref::FieldRef;
use std::fs::File;
use std::io;
use std::io::BufReader;

enum ConstantPoolData {
    Uninit, // only for uninitialized data
    ClassInfo(ClassInfo),
    FieldRef(FieldRef),
}

pub struct ConstantPoolItem {
    tag: ConstantPoolTag,
    data: ConstantPoolData,
}

impl ConstantPoolItem {
    pub fn new(tag: ConstantPoolTag) -> ConstantPoolItem {
        ConstantPoolItem {
            tag,
            data: ConstantPoolData::Uninit,
        }
    }

    pub fn parse(&mut self, reader: &mut BufReader<File>) -> Result<(), io::Error> {
        // delegate the parser based on the tag
        match self.tag {
            ConstantPoolTag::Class => {
                self.data = ConstantPoolData::ClassInfo(ClassInfo::from(reader)?);
            }
            ConstantPoolTag::FieldRef => {
                self.data = ConstantPoolData::FieldRef(FieldRef::from(reader)?);
            }
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Unsupported constant pool tag {:?}", self.tag),
            ))?,
        }
        Ok(())
    }
}
