use crate::util::file::read_bytes;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

#[derive(Eq, PartialEq)]
enum ReferenceKind {
    Uninit = -1, // only for uninitialized method handles
    GetField = 1,
    GetStatic = 2,
    PutField = 3,
    PutStatic = 4,
    InvokeVirtual = 5,
    InvokeStatic = 6,
    InvokeSpecial = 7,
    NewInvokeSpecial = 8,
    InvokeInterface = 9,
}
pub struct MethodHandle {
    reference_kind: ReferenceKind,
    reference_index: u16,
}

impl MethodHandle {
    pub fn new() -> MethodHandle {
        MethodHandle {
            reference_kind: ReferenceKind::Uninit,
            reference_index: 0,
        }
    }

    pub fn from(reader: &mut BufReader<impl Read>) -> Result<MethodHandle, io::Error> {
        let mut method_handle = MethodHandle::new();

        // read 1 byte for reference_kind
        let mut kind_buf: [u8; 1] = [0; 1];
        read_bytes(reader, &mut kind_buf, 1)?;
        method_handle.reference_kind = match kind_buf[0] {
            1 => ReferenceKind::GetField,
            2 => ReferenceKind::GetStatic,
            3 => ReferenceKind::PutField,
            4 => ReferenceKind::PutStatic,
            5 => ReferenceKind::InvokeVirtual,
            6 => ReferenceKind::InvokeStatic,
            7 => ReferenceKind::InvokeSpecial,
            8 => ReferenceKind::NewInvokeSpecial,
            9 => ReferenceKind::InvokeInterface,
            _ => ReferenceKind::Uninit,
        };

        // read 2 bytes for reference_index
        let mut idx_buf: [u8; 2] = [0; 2];
        read_bytes(reader, &mut idx_buf, 2)?;
        method_handle.reference_index = u16::from_be_bytes(idx_buf);

        Ok(method_handle)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_small_kind_and_index() {
        // reference_kind = 1 (GetField), index = 0x0005
        let bytes = [0x01, 0x00, 0x05];
        let mut reader = BufReader::new(bytes.as_ref());
        let mh = MethodHandle::from(&mut reader).unwrap();
        assert!(matches!(mh.reference_kind, ReferenceKind::GetField));
        assert_eq!(mh.reference_index, 5);
    }

    #[test]
    fn read_large_kind_and_index() {
        // reference_kind = 9 (InvokeInterface), index = 0xFFFE
        let bytes = [0x09, 0xFF, 0xFE];
        let mut reader = BufReader::new(bytes.as_ref());
        let mh = MethodHandle::from(&mut reader).unwrap();
        assert!(matches!(mh.reference_kind, ReferenceKind::InvokeInterface));
        assert_eq!(mh.reference_index, 0xFFFE);
    }

    #[test]
    fn read_unrecognized_kind_defaults_uninit() {
        // reference_kind = 0 -> Uninit by implementation, index arbitrary
        let bytes = [0x00, 0x12, 0x34];
        let mut reader = BufReader::new(bytes.as_ref());
        let mh = MethodHandle::from(&mut reader).unwrap();
        assert!(matches!(mh.reference_kind, ReferenceKind::Uninit));
        assert_eq!(mh.reference_index, 0x1234);
    }
}
