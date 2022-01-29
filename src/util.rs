pub fn bytes_to_u16(bytes: &[u8]) -> u16 {
    let arr = bytes[..2].try_into().unwrap();
    u16::from_le_bytes(arr)
}

pub fn bytes_to_u32(bytes: &[u8]) -> u32 {
    let arr = bytes[..4].try_into().unwrap();
    u32::from_le_bytes(arr)
}

pub fn bytes_to_u64(bytes: &[u8]) -> u64 {
    let arr = bytes[..8].try_into().unwrap();
    u64::from_le_bytes(arr)
}
