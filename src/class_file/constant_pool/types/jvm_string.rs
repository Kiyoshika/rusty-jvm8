use crate::util::file::read_bytes;
use std::fs::File;
use std::io;
use std::io::BufReader;

/// Named JvmString to avoid ambiguity with String.
/// Represents constant objects of String type.
pub struct JvmString {
    string_index: u16,
}

impl JvmString {
    pub fn new() -> JvmString {
        JvmString { string_index: 0 }
    }

    pub fn from(reader: &mut BufReader<File>) -> Result<JvmString, io::Error> {
        let mut string = JvmString::new();

        let mut buffer: [u8; 2] = [0; 2];
        read_bytes(reader, &mut buffer, 2)?;
        string.string_index = u16::from_be_bytes(buffer);

        Ok(string)
    }
}
