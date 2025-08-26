pub fn vec_to_u8(buffer: &Vec<u8>) -> u8 {
    buffer[0]
}

pub fn vec_to_u16(buffer: &Vec<u8>) -> u16 {
    let mut bytes = [0; 2];
    bytes.copy_from_slice(&buffer[0..2]);
    u16::from_be_bytes(bytes)
}

pub fn vec_to_u32(buffer: &Vec<u8>) -> u32 {
    let mut bytes = [0; 4];
    bytes.copy_from_slice(&buffer[0..4]);
    u32::from_be_bytes(bytes)
}

pub fn vec_to_u64(buffer: &Vec<u8>) -> u64 {
    let mut bytes = [0; 8];
    bytes.copy_from_slice(&buffer[0..8]);
    u64::from_be_bytes(bytes)
}
