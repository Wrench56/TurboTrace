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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat_u8_to_u16_valid() {
        let bytes: [u8; 2] = [0xAB, 0xCD];
        assert_eq!(concat_u8_to_u16(&bytes), Ok(0xABCD));
    }

    #[test]
    fn test_concat_u8_to_u16_invalid() {
        let bytes_short: [u8; 1] = [0xAB];
        assert_eq!(concat_u8_to_u16(&bytes_short), Err(()));

        let bytes_long: [u8; 3] = [0xAB, 0xCD, 0xEF];
        assert_eq!(concat_u8_to_u16(&bytes_long), Err(()));
    }

    #[test]
    fn test_concat_u8_to_u32_valid() {
        let bytes: [u8; 4] = [0xAB, 0xCD, 0xEF, 0x12];
        assert_eq!(concat_u8_to_u32(&bytes), Ok(0xABCDEF12));
    }

    #[test]
    fn test_concat_u8_to_u32_invalid() {
        let bytes_short: [u8; 3] = [0xAB, 0xCD, 0xEF];
        assert_eq!(concat_u8_to_u32(&bytes_short), Err(()));

        let bytes_long: [u8; 5] = [0xAB, 0xCD, 0xEF, 0x12, 0x34];
        assert_eq!(concat_u8_to_u32(&bytes_long), Err(()));
    }

    #[test]
    fn test_concat_u8_to_u64_valid() {
        let bytes: [u8; 8] = [0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x9A];
        assert_eq!(concat_u8_to_u64(&bytes), Ok(0xABCDEF123456789A));
    }

    #[test]
    fn test_concat_u8_to_u64_invalid() {
        let bytes_short: [u8; 7] = [0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78];
        assert_eq!(concat_u8_to_u64(&bytes_short), Err(()));

        let bytes_long: [u8; 9] = [0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC];
        assert_eq!(concat_u8_to_u64(&bytes_long), Err(()));
    }
}