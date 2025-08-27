use crate::class_file::constant_pool::constant_pool::ConstantPool;
use crate::util::file::read_bytes;
use std::fs::File;
use std::io;
use std::io::BufReader;
use log::info;

pub struct ClassFile {
    magic_number: u32,
    minor_version: u16,
    major_version: u16,
    constant_pool: ConstantPool,
}

impl ClassFile {
    pub fn new() -> ClassFile {
        ClassFile {
            magic_number: 0,
            minor_version: 0,
            major_version: 0,
            constant_pool: ConstantPool::new(),
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
    ///     Err(e) => println!("Error reading class_file: {e:?}"),
    /// }
    /// ```
    pub fn read_file(&mut self, file_path: &str) -> Result<(), io::Error> {
        info!("Now reading class file {file_path}");

        let file = File::open(file_path)?;
        let mut reader = BufReader::new(file);

        self.parse_magic_number(&mut reader)?;
        self.parse_minor_version(&mut reader)?;
        self.parse_major_version(&mut reader)?;
        self.parse_constant_pool(&mut reader)?;

        info!("Finished reading class file {file_path}");

        Ok(())
    }

    fn parse_magic_number(&mut self, reader: &mut BufReader<File>) -> Result<(), io::Error> {
        let mut buffer: [u8; 4] = [0; 4];
        read_bytes(reader, &mut buffer, 4)?;
        self.magic_number = u32::from_be_bytes(buffer);
        if self.magic_number != 0xCAFEBABE {
            return Err(io::Error::new(io::ErrorKind::Other, "Invalid magic number"));
        }

        Ok(())
    }

    fn parse_minor_version(&mut self, reader: &mut BufReader<File>) -> Result<(), io::Error> {
        let mut buffer: [u8; 2] = [0; 2];
        read_bytes(reader, &mut buffer, 2)?;
        self.minor_version = u16::from_be_bytes(buffer);
        Ok(())
    }

    fn parse_major_version(&mut self, reader: &mut BufReader<File>) -> Result<(), io::Error> {
        let mut buffer: [u8; 2] = [0; 2];
        read_bytes(reader, &mut buffer, 2)?;
        self.major_version = u16::from_be_bytes(buffer);
        if self.major_version > 52 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Unsupported major class version - must be <= 52",
            ));
        }
        Ok(())
    }

    fn parse_constant_pool(&mut self, reader: &mut BufReader<File>) -> Result<(), io::Error> {
        // first get size of constant pool
        let mut buffer: [u8; 2] = [0; 2];
        read_bytes(reader, &mut buffer, 2)?;
        let constant_pool_count = u16::from_be_bytes(buffer);
        self.constant_pool.set_size(constant_pool_count);

        // parse all constant pool items
        // constant pool starts at index 1 up to count - 1 (described in section 4.1)
        for i in 1..constant_pool_count {
            self.constant_pool.parse_item_from_class_file(reader)?;
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
