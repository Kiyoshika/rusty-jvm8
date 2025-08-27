use crate::util::file::read_bytes;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

pub struct InvokeDynamic {
    bootstrap_method_attr_index: u16,
    name_and_type_index: u16,
}

impl InvokeDynamic {
    pub fn new() -> InvokeDynamic {
        InvokeDynamic {
            bootstrap_method_attr_index: 0,
            name_and_type_index: 0,
        }
    }

    pub fn from(reader: &mut BufReader<impl Read>) -> Result<InvokeDynamic, io::Error> {
        let mut invoke_dynamic = InvokeDynamic::new();

        let mut buffer: [u8; 2] = [0; 2];

        read_bytes(reader, &mut buffer, 2)?;
        invoke_dynamic.bootstrap_method_attr_index = u16::from_be_bytes(buffer);

        read_bytes(reader, &mut buffer, 2)?;
        invoke_dynamic.name_and_type_index = u16::from_be_bytes(buffer);

        Ok(invoke_dynamic)
    }
}
