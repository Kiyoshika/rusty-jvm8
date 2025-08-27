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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_small_indices() {
        // bootstrap_method_attr_index = 1, name_and_type_index = 2
        let bytes = [0x00, 0x01, 0x00, 0x02];
        let mut reader = BufReader::new(bytes.as_ref());
        let indy = InvokeDynamic::from(&mut reader).unwrap();
        // Private fields; accessible from child module
        assert_eq!(indy.bootstrap_method_attr_index, 1);
        assert_eq!(indy.name_and_type_index, 2);
    }

    #[test]
    fn read_large_indices() {
        // bootstrap_method_attr_index = 0xC0DE, name_and_type_index = 0xBEEF
        let bytes = [0xC0, 0xDE, 0xBE, 0xEF];
        let mut reader = BufReader::new(bytes.as_ref());
        let indy = InvokeDynamic::from(&mut reader).unwrap();
        assert_eq!(indy.bootstrap_method_attr_index, 0xC0DE);
        assert_eq!(indy.name_and_type_index, 0xBEEF);
    }
}
