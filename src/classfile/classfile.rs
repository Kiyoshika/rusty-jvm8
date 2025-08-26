use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

pub struct ClassFile {
    magic_number: u32,
    minor_version: u16,
    major_version: u16,
}

impl ClassFile {
    pub fn new() -> ClassFile {
        ClassFile {
            magic_number: 0,
            minor_version: 0,
            major_version: 0,
        }
    }

    /// Reads a file and attempts to parse it as a ClassFile
    ///
    /// # Examples
    /// ```rust
    /// let class_file = ClassFile::new();
    /// class_file.read_file("MyClass.class");
    /// match class_file {
    ///     Ok(f) => println!(f.magic_number),
    ///     Err(e) => println!("Error reading classfile: {e:?}"),
    /// }
    /// ```
    pub fn read_file(&mut self, file_path: &str) -> Result<(), io::Error> {
        let file = File::open(file_path)?;
        let mut reader = BufReader::new(file);

        self.parse_magic_number(&mut reader)?;
        self.parse_minor_version(&mut reader)?;
        self.parse_major_version(&mut reader)?;

        Ok(())
    }

    fn parse_magic_number(&mut self, reader: &mut BufReader<File>) -> Result<(), io::Error> {
        let mut buffer = vec![0; 4];
        self.read_bytes(reader, &mut buffer, 4)?;
        self.magic_number = self.vec_to_u32(&buffer);
        if self.magic_number != 0xCAFEBABE {
            return Err(io::Error::new(io::ErrorKind::Other, "Invalid magic number"));
        }

        Ok(())
    }

    fn parse_minor_version(&mut self, reader: &mut BufReader<File>) -> Result<(), io::Error> {
        let mut buffer = vec![0; 2];
        self.read_bytes(reader, &mut buffer, 2)?;
        self.minor_version = self.vec_to_u16(&mut buffer);
        Ok(())
    }

    fn parse_major_version(&mut self, reader: &mut BufReader<File>) -> Result<(), io::Error> {
        let mut buffer = vec![0; 2];
        self.read_bytes(reader, &mut buffer, 2)?;
        self.major_version = self.vec_to_u16(&mut buffer);
        if self.major_version > 52 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Unsupported major class version - must be <= 52",
            ));
        }
        Ok(())
    }

    fn read_bytes(
        &mut self,
        reader: &mut BufReader<File>,
        buffer: &mut Vec<u8>,
        n_bytes: usize,
    ) -> Result<(), io::Error> {
        let bytes_read = reader.read(buffer)?;
        if bytes_read != n_bytes {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Expected to read {n_bytes} bytes but instead only read {bytes_read}"),
            ));
        }

        Ok(())
    }

    fn vec_to_u8(&self, buffer: &Vec<u8>) -> u8 {
        buffer[0]
    }

    fn vec_to_u16(&self, buffer: &Vec<u8>) -> u16 {
        let mut bytes = [0; 2];
        bytes.copy_from_slice(&buffer[0..2]);
        u16::from_be_bytes(bytes)
    }

    fn vec_to_u32(&self, buffer: &Vec<u8>) -> u32 {
        let mut bytes = [0; 4];
        bytes.copy_from_slice(&buffer[0..4]);
        u32::from_be_bytes(bytes)
    }

    fn vec_to_u64(&self, buffer: &Vec<u8>) -> u64 {
        let mut bytes = [0; 8];
        bytes.copy_from_slice(&buffer[0..8]);
        u64::from_be_bytes(bytes)
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
        class_file
            .read_file("tests/java/helloworld/HelloWorld.class")
            .unwrap();
        assert_eq!(class_file.magic_number, 0xCAFEBABE);
    }

    #[test]
    fn parse_major_version() {
        let mut class_file = ClassFile::new();
        class_file
            .read_file("tests/java/helloworld/HelloWorld.class")
            .unwrap();
        assert_eq!(class_file.major_version, 52);
    }

    #[test]
    fn fail_to_read_class_thats_too_new() {
        let mut class_file = ClassFile::new();
        assert!(class_file
            .read_file("tests/java/toonew/TooNew.class")
            .is_err());
    }
}
