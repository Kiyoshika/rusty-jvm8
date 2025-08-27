use crate::class_file::constant_pool::constant_pool::ConstantPool;
use crate::util::file::read_bytes;
use log::info;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

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

    fn parse_magic_number(&mut self, reader: &mut BufReader<impl Read>) -> Result<(), io::Error> {
        let mut buffer: [u8; 4] = [0; 4];
        read_bytes(reader, &mut buffer, 4)?;
        self.magic_number = u32::from_be_bytes(buffer);
        if self.magic_number != 0xCAFEBABE {
            return Err(io::Error::new(io::ErrorKind::Other, "Invalid magic number"));
        }

        Ok(())
    }

    fn parse_minor_version(&mut self, reader: &mut BufReader<impl Read>) -> Result<(), io::Error> {
        let mut buffer: [u8; 2] = [0; 2];
        read_bytes(reader, &mut buffer, 2)?;
        self.minor_version = u16::from_be_bytes(buffer);
        Ok(())
    }

    fn parse_major_version(&mut self, reader: &mut BufReader<impl Read>) -> Result<(), io::Error> {
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

    fn parse_constant_pool(&mut self, reader: &mut BufReader<impl Read>) -> Result<(), io::Error> {
        // first get size of constant pool
        let mut buffer: [u8; 2] = [0; 2];
        read_bytes(reader, &mut buffer, 2)?;
        let constant_pool_count = u16::from_be_bytes(buffer);
        self.constant_pool.set_size(constant_pool_count);

        // parse all constant pool items
        // constant pool starts at index 1 up to count - 1 (described in section 4.1)
        for _i in 1..constant_pool_count {
            self.constant_pool.parse_item_from_class_file(reader)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::class_file::constant_pool::item::{ConstantPoolData, ConstantPoolItem};
    use crate::class_file::constant_pool::tag::ConstantPoolTag;
    use std::ops::Deref;

    #[test]
    fn read_bad_file() {
        let mut class_file = ClassFile::new();
        assert!(class_file.read_file("does not exist").is_err());
    }

    #[test]
    fn parse_valid_file() {
        let mut class_file = ClassFile::new();
        class_file
            .read_file("tests/java/helloworld/HelloWorld.class")
            .unwrap();
        assert_eq!(class_file.magic_number, 0xCAFEBABE);
        assert_eq!(class_file.major_version, 52);
        assert_eq!(class_file.minor_version, 0);
        assert_eq!(class_file.constant_pool.count(), 29);
        // constant pool starts at index 1 up to count - 1 (described in section 4.1)
        // so we expect 28 items
        assert_eq!(
            class_file.constant_pool.items().len(),
            class_file.constant_pool.count() - 1
        );

        // verify constant pool items in order
        let constant_pool_items = class_file.constant_pool.items();

        // #1 MethodRef class_index=6 name_and_type_index=15 ("java/lang/Object")
        let mut item: &ConstantPoolItem = constant_pool_items.get(0).unwrap();
        assert_eq!(*item.tag(), ConstantPoolTag::MethodRef);
        let mut data: &ConstantPoolData = item.data();
        match data {
            ConstantPoolData::MethodRef(method_ref) => {
                assert_eq!(method_ref.class_index(), 6);
                assert_eq!(method_ref.name_and_type_index(), 15);
            }
            _ => panic!("Expected MethodRef at constant pool index 0"),
        }

        // #2 FieldRef class_index=16 name_and_type_index=17 ("java/lang/System")
        item = constant_pool_items.get(1).unwrap();
        assert_eq!(*item.tag(), ConstantPoolTag::FieldRef);
        data = item.data();
        match data {
            ConstantPoolData::FieldRef(field_ref) => {
                assert_eq!(field_ref.class_index(), 16);
                assert_eq!(field_ref.name_and_type_index(), 17);
            }
            _ => panic!("Expected FieldRef at constant pool index 1"),
        }

        // #3 String string_index=18 ("Hello, World!")
        item = constant_pool_items.get(2).unwrap();
        assert_eq!(*item.tag(), ConstantPoolTag::String);
        data = item.data();
        match data {
            ConstantPoolData::String(string) => {
                assert_eq!(string.string_index(), 18);
            }
            _ => panic!("Expected JvmString at constant pool index 2"),
        }

        // #4 MethodRef class_index=19 ("java/io/PrintStream")
        item = constant_pool_items.get(3).unwrap();
        assert_eq!(*item.tag(), ConstantPoolTag::MethodRef);
        data = item.data();
        match data {
            ConstantPoolData::MethodRef(method_ref) => {
                assert_eq!(method_ref.class_index(), 19);
                assert_eq!(method_ref.name_and_type_index(), 20);
            }
            _ => panic!("Expected MethodRef at constant pool index 3"),
        }

        // #5 Class name_index=21 ("HelloWorld")
        item = constant_pool_items.get(4).unwrap();
        assert_eq!(*item.tag(), ConstantPoolTag::Class);
        data = item.data();
        match data {
            ConstantPoolData::ClassInfo(ci) => {
                assert_eq!(ci.name_index(), 21);
            }
            _ => panic!("Expected Class at constant pool index 4"),
        }

        // #6 Class name_index=22 ("java/lang/Object")
        item = constant_pool_items.get(5).unwrap();
        assert_eq!(*item.tag(), ConstantPoolTag::Class);
        data = item.data();
        match data {
            ConstantPoolData::ClassInfo(ci) => {
                assert_eq!(ci.name_index(), 22);
            }
            _ => panic!("Expected Class at constant pool index 5"),
        }

        // #7 Utf8 "<init>"
        item = constant_pool_items.get(6).unwrap();
        assert_eq!(*item.tag(), ConstantPoolTag::Utf8);
        data = item.data();
        match data {
            ConstantPoolData::Utf8(u) => assert_eq!(u.value(), "<init>"),
            _ => panic!("Expected Utf8 at constant pool index 6"),
        }

        // #8 Utf8 "()V"
        item = constant_pool_items.get(7).unwrap();
        assert_eq!(*item.tag(), ConstantPoolTag::Utf8);
        data = item.data();
        match data {
            ConstantPoolData::Utf8(u) => assert_eq!(u.value(), "()V"),
            _ => panic!("Expected Utf8 at constant pool index 7"),
        }

        // #9 Utf8 "Code"
        item = constant_pool_items.get(8).unwrap();
        assert_eq!(*item.tag(), ConstantPoolTag::Utf8);
        data = item.data();
        match data {
            ConstantPoolData::Utf8(u) => assert_eq!(u.value(), "Code"),
            _ => panic!("Expected Utf8 at constant pool index 8"),
        }

        // #10 Utf8 "LineNumberTable"
        item = constant_pool_items.get(9).unwrap();
        assert_eq!(*item.tag(), ConstantPoolTag::Utf8);
        data = item.data();
        match data {
            ConstantPoolData::Utf8(u) => assert_eq!(u.value(), "LineNumberTable"),
            _ => panic!("Expected Utf8 at constant pool index 9"),
        }

        // #11 Utf8 "main"
        item = constant_pool_items.get(10).unwrap();
        assert_eq!(*item.tag(), ConstantPoolTag::Utf8);
        data = item.data();
        match data {
            ConstantPoolData::Utf8(u) => assert_eq!(u.value(), "main"),
            _ => panic!("Expected Utf8 at constant pool index 10"),
        }

        // #12 Utf8 "([Ljava/lang/String;)V"
        item = constant_pool_items.get(11).unwrap();
        assert_eq!(*item.tag(), ConstantPoolTag::Utf8);
        data = item.data();
        match data {
            ConstantPoolData::Utf8(u) => assert_eq!(u.value(), "([Ljava/lang/String;)V"),
            _ => panic!("Expected Utf8 at constant pool index 11"),
        }

        // #13 Utf8 "SourceFile"
        item = constant_pool_items.get(12).unwrap();
        assert_eq!(*item.tag(), ConstantPoolTag::Utf8);
        data = item.data();
        match data {
            ConstantPoolData::Utf8(u) => assert_eq!(u.value(), "SourceFile"),
            _ => panic!("Expected Utf8 at constant pool index 12"),
        }

        // #14 Utf8 "HelloWorld.java"
        item = constant_pool_items.get(13).unwrap();
        assert_eq!(*item.tag(), ConstantPoolTag::Utf8);
        data = item.data();
        match data {
            ConstantPoolData::Utf8(u) => assert_eq!(u.value(), "HelloWorld.java"),
            _ => panic!("Expected Utf8 at constant pool index 13"),
        }

        // #15 NameAndType name_index=7, descriptor_index=8
        item = constant_pool_items.get(14).unwrap();
        assert_eq!(*item.tag(), ConstantPoolTag::NameAndType);
        data = item.data();
        match data {
            ConstantPoolData::NameAndType(nt) => {
                assert_eq!(nt.name_index(), 7);
                assert_eq!(nt.descriptor_index(), 8);
            }
            _ => panic!("Expected NameAndType at constant pool index 14"),
        }

        // #16 Class name_index=23
        item = constant_pool_items.get(15).unwrap();
        assert_eq!(*item.tag(), ConstantPoolTag::Class);
        data = item.data();
        match data {
            ConstantPoolData::ClassInfo(ci) => assert_eq!(ci.name_index(), 23),
            _ => panic!("Expected Class at constant pool index 15"),
        }

        // #17 NameAndType name_index=24, descriptor_index=25
        item = constant_pool_items.get(16).unwrap();
        assert_eq!(*item.tag(), ConstantPoolTag::NameAndType);
        data = item.data();
        match data {
            ConstantPoolData::NameAndType(nt) => {
                assert_eq!(nt.name_index(), 24);
                assert_eq!(nt.descriptor_index(), 25);
            }
            _ => panic!("Expected NameAndType at constant pool index 16"),
        }

        // #18 Utf8 "Hello, World!"
        item = constant_pool_items.get(17).unwrap();
        assert_eq!(*item.tag(), ConstantPoolTag::Utf8);
        data = item.data();
        match data {
            ConstantPoolData::Utf8(u) => assert_eq!(u.value(), "Hello, World!"),
            _ => panic!("Expected Utf8 at constant pool index 17"),
        }

        // #19 Class name_index=26
        item = constant_pool_items.get(18).unwrap();
        assert_eq!(*item.tag(), ConstantPoolTag::Class);
        data = item.data();
        match data {
            ConstantPoolData::ClassInfo(ci) => assert_eq!(ci.name_index(), 26),
            _ => panic!("Expected Class at constant pool index 18"),
        }

        // #20 NameAndType name_index=27, descriptor_index=28
        item = constant_pool_items.get(19).unwrap();
        assert_eq!(*item.tag(), ConstantPoolTag::NameAndType);
        data = item.data();
        match data {
            ConstantPoolData::NameAndType(nt) => {
                assert_eq!(nt.name_index(), 27);
                assert_eq!(nt.descriptor_index(), 28);
            }
            _ => panic!("Expected NameAndType at constant pool index 19"),
        }

        // #21 Utf8 "HelloWorld"
        item = constant_pool_items.get(20).unwrap();
        assert_eq!(*item.tag(), ConstantPoolTag::Utf8);
        data = item.data();
        match data {
            ConstantPoolData::Utf8(u) => assert_eq!(u.value(), "HelloWorld"),
            _ => panic!("Expected Utf8 at constant pool index 20"),
        }

        // #22 Utf8 "java/lang/Object"
        item = constant_pool_items.get(21).unwrap();
        assert_eq!(*item.tag(), ConstantPoolTag::Utf8);
        data = item.data();
        match data {
            ConstantPoolData::Utf8(u) => assert_eq!(u.value(), "java/lang/Object"),
            _ => panic!("Expected Utf8 at constant pool index 21"),
        }

        // #23 Utf8 "java/lang/System"
        item = constant_pool_items.get(22).unwrap();
        assert_eq!(*item.tag(), ConstantPoolTag::Utf8);
        data = item.data();
        match data {
            ConstantPoolData::Utf8(u) => assert_eq!(u.value(), "java/lang/System"),
            _ => panic!("Expected Utf8 at constant pool index 22"),
        }

        // #24 Utf8 "out"
        item = constant_pool_items.get(23).unwrap();
        assert_eq!(*item.tag(), ConstantPoolTag::Utf8);
        data = item.data();
        match data {
            ConstantPoolData::Utf8(u) => assert_eq!(u.value(), "out"),
            _ => panic!("Expected Utf8 at constant pool index 23"),
        }

        // #25 Utf8 "Ljava/io/PrintStream;"
        item = constant_pool_items.get(24).unwrap();
        assert_eq!(*item.tag(), ConstantPoolTag::Utf8);
        data = item.data();
        match data {
            ConstantPoolData::Utf8(u) => assert_eq!(u.value(), "Ljava/io/PrintStream;"),
            _ => panic!("Expected Utf8 at constant pool index 24"),
        }

        // #26 Utf8 "java/io/PrintStream"
        item = constant_pool_items.get(25).unwrap();
        assert_eq!(*item.tag(), ConstantPoolTag::Utf8);
        data = item.data();
        match data {
            ConstantPoolData::Utf8(u) => assert_eq!(u.value(), "java/io/PrintStream"),
            _ => panic!("Expected Utf8 at constant pool index 25"),
        }

        // #27 Utf8 "println"
        item = constant_pool_items.get(26).unwrap();
        assert_eq!(*item.tag(), ConstantPoolTag::Utf8);
        data = item.data();
        match data {
            ConstantPoolData::Utf8(u) => assert_eq!(u.value(), "println"),
            _ => panic!("Expected Utf8 at constant pool index 26"),
        }

        // #28 Utf8 "(Ljava/lang/String;)V"
        item = constant_pool_items.get(27).unwrap();
        assert_eq!(*item.tag(), ConstantPoolTag::Utf8);
        data = item.data();
        match data {
            ConstantPoolData::Utf8(u) => assert_eq!(u.value(), "(Ljava/lang/String;)V"),
            _ => panic!("Expected Utf8 at constant pool index 27"),
        }
    }

    #[test]
    fn fail_to_read_class_thats_too_new() {
        let mut class_file = ClassFile::new();
        assert!(class_file
            .read_file("tests/java/toonew/TooNew.class")
            .is_err());
    }
}
