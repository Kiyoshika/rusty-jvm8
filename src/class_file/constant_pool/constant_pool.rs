use crate::class_file::constant_pool::item::ConstantPoolItem;
use crate::class_file::constant_pool::tag::ConstantPoolTag;
use crate::util::file::read_bytes;

use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

pub struct ConstantPool {
    count: u16,
    items: Vec<ConstantPoolItem>,
}

impl ConstantPool {
    pub fn new() -> ConstantPool {
        ConstantPool {
            count: 0,
            items: Vec::new(),
        }
    }

    pub fn set_size(&mut self, size: u16) {
        self.count = size;
    }

    pub fn count(&self) -> usize {
        self.count as usize
    }

    pub fn items(&self) -> &[ConstantPoolItem] {
        &self.items
    }

    pub fn parse_item_from_class_file(
        &mut self,
        reader: &mut BufReader<impl Read>,
    ) -> Result<(), io::Error> {
        // read the tag
        // TODO: convert this to stack buffer
        let mut buffer = vec![0; 1];
        read_bytes(reader, &mut buffer, 1)?;
        let tag = match buffer[0] {
            7 => ConstantPoolTag::Class,
            9 => ConstantPoolTag::FieldRef,
            10 => ConstantPoolTag::MethodRef,
            15 => ConstantPoolTag::MethodHandle,
            16 => ConstantPoolTag::MethodType,
            18 => ConstantPoolTag::InvokeDynamic,
            11 => ConstantPoolTag::InterfaceMethodRef,
            8 => ConstantPoolTag::String,
            3 => ConstantPoolTag::Integer,
            4 => ConstantPoolTag::Float,
            5 => ConstantPoolTag::Long,
            6 => ConstantPoolTag::Double,
            12 => ConstantPoolTag::NameAndType,
            1 => ConstantPoolTag::Utf8,
            _ => ConstantPoolTag::Unknown,
        };

        if tag == ConstantPoolTag::Unknown {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Unknown tag with value {:?} encountered while reading constant pool.",
                    buffer[0]
                ),
            ));
        }

        let mut item = ConstantPoolItem::new(tag);
        item.parse(reader)?;

        self.items.push(item);
        Ok(())
    }
}
