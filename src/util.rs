use crate::Error;

pub fn bytes_to_u16(bytes: &[u8]) -> Result<u16, Error> {
    let arr_res = bytes[..2].try_into();
    match arr_res {
        Ok(arr) => Ok(u16::from_le_bytes(arr)),
        Err(_) => Err(Error::InvalidConversion),
    }
}

pub fn bytes_to_u32(bytes: &[u8]) -> Result<u32, Error> {
    let arr_res = bytes[..4].try_into();
    match arr_res {
        Ok(arr) => Ok(u32::from_le_bytes(arr)),
        Err(_) => Err(Error::InvalidConversion),
    }
}

pub fn bytes_to_u64(bytes: &[u8]) -> Result<u64, Error> {
    let arr_res = bytes[..8].try_into();
    match arr_res {
        Ok(arr) => Ok(u64::from_le_bytes(arr)),
        Err(_) => Err(Error::InvalidConversion),
    }
}
