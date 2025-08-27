use crate::class_file::constant_pool::tag::ConstantPoolTag;
use crate::class_file::constant_pool::types::class_info::ClassInfo;
use crate::class_file::constant_pool::types::double::Double;
use crate::class_file::constant_pool::types::field_ref::FieldRef;
use crate::class_file::constant_pool::types::float::Float;
use crate::class_file::constant_pool::types::integer::Integer;
use crate::class_file::constant_pool::types::interface_method_ref::InterfaceMethodRef;
use crate::class_file::constant_pool::types::jvm_string::JvmString;
use crate::class_file::constant_pool::types::long::Long;
use crate::class_file::constant_pool::types::method_ref::MethodRef;
use std::fs::File;
use std::io;
use std::io::BufReader;

enum ConstantPoolData {
    Uninit, // only for uninitialized data
    ClassInfo(ClassInfo),
    FieldRef(FieldRef),
    MethodRef(MethodRef),
    InterfaceMethodRef(InterfaceMethodRef),
    JvmString(JvmString),
    Float(Float),
    Integer(Integer),
    Long(Long),
    Double(Double),
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
            ConstantPoolTag::MethodRef => {
                self.data = ConstantPoolData::MethodRef(MethodRef::from(reader)?);
            }
            ConstantPoolTag::InterfaceMethodRef => {
                self.data = ConstantPoolData::InterfaceMethodRef(InterfaceMethodRef::from(reader)?);
            }
            ConstantPoolTag::String => {
                self.data = ConstantPoolData::JvmString(JvmString::from(reader)?);
            }
            ConstantPoolTag::Float => {
                self.data = ConstantPoolData::Float(Float::from(reader)?);
            }
            ConstantPoolTag::Integer => {
                self.data = ConstantPoolData::Integer(Integer::from(reader)?);
            }
            ConstantPoolTag::Long => {
                self.data = ConstantPoolData::Long(Long::from(reader)?);
            }
            ConstantPoolTag::Double => {
                self.data = ConstantPoolData::Double(Double::from(reader)?);
            }
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Unsupported constant pool tag {:?}", self.tag),
            ))?,
        }
        Ok(())
    }
}
