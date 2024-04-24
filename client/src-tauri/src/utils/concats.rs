pub fn concat_u8_to_u16(bytes: &[u8]) -> Result<u16, ()> {
    if bytes.len() != 2 {
        return Err(());
    }
    Ok(((bytes[0] as u16) << 8) | (bytes[1] as u16))
}

pub fn concat_u8_to_u32(bytes: &[u8]) -> Result<u32, ()> {
    if bytes.len() != 4 {
        return Err(());
    }

    Ok(((concat_u8_to_u16(&bytes[0..2])? as u32) << 16) | (concat_u8_to_u16(&bytes[2..4])? as u32))
}

pub fn concat_u8_to_u64(bytes: &[u8]) -> Result<u64, ()> {
    if bytes.len() != 8 {
        return Err(());
    }

    Ok(((concat_u8_to_u32(&bytes[0..4])? as u64) << 32) | (concat_u8_to_u32(&bytes[4..8])? as u64))
}
