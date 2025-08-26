use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

pub struct ClassFile {
    magic_number: u32
}

impl ClassFile {
    pub fn new() -> ClassFile {
        ClassFile {
            magic_number: 0
        }
    }

    /// Reads a file and attempts to parse it as a ClassFile
    ///
    /// # Examples
    /// ```rust
    /// ClassFile class_file;
    /// class_file.read_file("MyClass.class");
    /// match class_file {
    ///     Ok(f) => println!(f.magic_number),
    ///     Err(e) => println!("Error reading classfile: {e:?}"),
    /// }
    /// ```
    pub fn read_file(&mut self, file_path: &str) -> Result<(), io::Error>{
        let file = File::open(file_path)?;
        let mut reader = BufReader::new(file);

        self.parse_magic_number(&mut reader)?;

        Ok(())
    }

    fn read_bytes(&mut self, reader: &mut BufReader<File>, buffer: &mut Vec<u8>, n_bytes: usize) -> Result<(), io::Error> {
        let bytes_read = reader.read(buffer)?;
        if bytes_read != n_bytes {
            io::Error::new(io::ErrorKind::Other, format!("Expected to read {n_bytes} bytes but instead only read {bytes_read}"));
        }

        Ok(())
    }

    fn vec_to_u32(&self, buffer: &Vec<u8>) -> u32 {
        let mut bytes = [0; 4];
        bytes.copy_from_slice(&buffer[0..4]);
        u32::from_be_bytes(bytes)
    }

    fn parse_magic_number(&mut self, reader: &mut BufReader<File>) -> Result<(), io::Error>{
        let mut buffer = vec![0; 4];
        self.read_bytes(reader, &mut buffer, 4)?;
        self.magic_number = self.vec_to_u32(&buffer);
        if self.magic_number != 0xCAFEBABE {
            io::Error::new(io::ErrorKind::Other, "Invalid magic number");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_bad_file() {
        let mut class_file = ClassFile::new();
        assert!(class_file.read_file("does not exist").is_err());
    }

    #[test]
    fn parse_magic_number() {
        let mut class_file = ClassFile::new();
        class_file.read_file("tests/java/helloworld/HelloWorld.class").unwrap();
        assert_eq!(class_file.magic_number, 0xCAFEBABE);
    }
}