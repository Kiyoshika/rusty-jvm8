use crate::class_file::constant_pool::tag::ConstantPoolTag;
use crate::class_file::constant_pool::types::class_info::ClassInfo;
use crate::class_file::constant_pool::types::double::Double;
use crate::class_file::constant_pool::types::field_ref::FieldRef;
use crate::class_file::constant_pool::types::float::Float;
use crate::class_file::constant_pool::types::integer::Integer;
use crate::class_file::constant_pool::types::interface_method_ref::InterfaceMethodRef;
use crate::class_file::constant_pool::types::invoke_dynamic::InvokeDynamic;
use crate::class_file::constant_pool::types::jvm_string::JvmString;
use crate::class_file::constant_pool::types::long::Long;
use crate::class_file::constant_pool::types::method_handle::MethodHandle;
use crate::class_file::constant_pool::types::method_ref::MethodRef;
use crate::class_file::constant_pool::types::method_type::MethodType;
use crate::class_file::constant_pool::types::name_and_type::NameAndType;
use crate::class_file::constant_pool::types::utf8::Utf8;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

pub enum ConstantPoolData {
    Uninit, // only for uninitialized data
    ClassInfo(ClassInfo),
    FieldRef(FieldRef),
    MethodRef(MethodRef),
    InterfaceMethodRef(InterfaceMethodRef),
    String(JvmString),
    Float(Float),
    Integer(Integer),
    Long(Long),
    Double(Double),
    NameAndType(NameAndType),
    Utf8(Utf8),
    MethodHandle(MethodHandle),
    MethodType(MethodType),
    InvokeDynamic(InvokeDynamic),
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

    pub fn tag(&self) -> &ConstantPoolTag {
        &self.tag
    }

    pub fn data(&self) -> &ConstantPoolData {
        &self.data
    }

    pub fn parse(&mut self, reader: &mut BufReader<impl Read>) -> Result<(), io::Error> {
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
                self.data = ConstantPoolData::String(JvmString::from(reader)?);
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
            ConstantPoolTag::NameAndType => {
                self.data = ConstantPoolData::NameAndType(NameAndType::from(reader)?);
            }
            ConstantPoolTag::Utf8 => {
                self.data = ConstantPoolData::Utf8(Utf8::from(reader)?);
            }
            ConstantPoolTag::MethodHandle => {
                self.data = ConstantPoolData::MethodHandle(MethodHandle::from(reader)?);
            }
            ConstantPoolTag::MethodType => {
                self.data = ConstantPoolData::MethodType(MethodType::from(reader)?);
            }
            ConstantPoolTag::InvokeDynamic => {
                self.data = ConstantPoolData::InvokeDynamic(InvokeDynamic::from(reader)?);
            }
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Unsupported constant pool tag {:?}", self.tag),
            ))?,
        }
        Ok(())
    }
}
