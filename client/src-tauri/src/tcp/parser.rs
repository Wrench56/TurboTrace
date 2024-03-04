pub fn concat_u8_to_u16(byte1: u8, byte2: u8) -> u16 {
    ((byte1 as u16) << 8) | (byte2 as u16)
}

pub fn concat_u8_to_u32(bytes: &[u8]) -> Result<u32, ()> {
    if bytes.len() != 4 {
        return Err(());
    }

    let first_u16 = concat_u8_to_u16(bytes[0], bytes[1]);
    let second_u16 = concat_u8_to_u16(bytes[2], bytes[3]);
    Ok(((first_u16 as u32) << 16) | (second_u16 as u32))
}

pub fn concat_u8_to_u128(bytes: &[u8]) -> Result<u128, ()> {
    if bytes.len() != 16 {
        return Err(());
    }

    let mut result: u128 = 0;
    for i in 0..16 {
        result |= (bytes[i] as u128) << ((15 - i) * 8);
    }

    Ok(result)
}

pub fn char_to_level(chr_u8: &u8) -> &str {
    match *chr_u8 as char {
        'D' => "DEBUG",
        'I' => "INFO",
        'W' => "WARN",
        'E' => "ERROR",
        'C' => "CRIT",
        /* Raise internal error */
        _ => "PRGME",
    }
}

pub fn module_name(_: &[u8]) -> &str {
    "std::default"
}
