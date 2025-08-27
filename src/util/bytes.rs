pub fn buffer_to_u8(buffer: [u8; 1]) -> u8 {
    buffer[0]
}

pub fn buffer_to_u16(buffer: [u8; 2]) -> u16 {
    u16::from_be_bytes(buffer.try_into().unwrap())
}

/// Convert buffer to u32. Assumes buffer is of length 4.
pub fn buffer_to_u32(buffer: [u8; 4]) -> u32 {
    u32::from_be_bytes(buffer.try_into().unwrap())
}

/// Convert buffer to u64. Assumes buffer is of length 8.
pub fn buffer_to_u64(buffer: [u8; 8]) -> u64 {
    u64::from_be_bytes(buffer.try_into().unwrap())
}
