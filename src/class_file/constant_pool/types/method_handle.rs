use crate::util::file::read_bytes;
use std::fs::File;
use std::io;
use std::io::BufReader;

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

    pub fn from(reader: &mut BufReader<File>) -> Result<MethodHandle, io::Error> {
        let mut method_handle = MethodHandle::new();

        let mut buffer: [u8; 2] = [0; 2];

        read_bytes(reader, &mut buffer, 1)?;
        method_handle.reference_kind = match buffer[0] {
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

        read_bytes(reader, &mut buffer, 2)?;
        method_handle.reference_index = u16::from_be_bytes(buffer);

        Ok(method_handle)
    }
}
