pub fn concat_u8_to_u16(byte1: &u8, byte2: &u8) -> u16 {
    ((*byte1 as u16) << 8) | (*byte2 as u16)
}

pub fn concat_u8_to_u32(bytes: &[u8]) -> Result<u32, ()> {
    if bytes.len() != 4 {
        return Err(());
    }

    let first_u16 = concat_u8_to_u16(&bytes[0], &bytes[1]);
    let second_u16 = concat_u8_to_u16(&bytes[2], &bytes[3]);
    Ok(((first_u16 as u32) << 16) | (second_u16 as u32))
}

pub fn concat_u8_to_u64(bytes: &[u8]) -> Result<u64, ()> {
    if bytes.len() != 8 {
        return Err(());
    }

    let first_u32 = concat_u8_to_u32(&bytes[0..4])?;
    let second_u32 = concat_u8_to_u32(&bytes[4..8])?;
    Ok(((first_u32 as u64) << 32) | (second_u32 as u64))
}
