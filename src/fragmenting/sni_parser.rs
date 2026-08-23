pub fn find_sni(buf: &[u8]) -> Option<usize>{
    let mut pos = 0usize;
    pos += 5;  
    pos += 4;
    pos += 2;
    pos += 32;

    let session_id_len = *buf.get(pos)? as usize;
    pos += 1 + session_id_len;

    let cipher_suites_len = u16::from_be_bytes([*buf.get(pos)?, *buf.get(pos + 1)?]) as usize;
    pos += 2 + cipher_suites_len;

    let compression_len = *buf.get(pos)? as usize;
    pos += 1 + compression_len;

    let _extensions_total_len = u16::from_be_bytes([*buf.get(pos)?, *buf.get(pos + 1)?]) as usize;
    pos += 2;

    loop {
        let ext_type = u16::from_be_bytes([*buf.get(pos)?, *buf.get(pos + 1)?]) as usize;
        let ext_len = u16::from_be_bytes([*buf.get(pos + 2)?, *buf.get(pos + 3)?]) as usize;
        let ext_data_start = pos + 4;

        if ext_type == 0x0000 {
            let name_len_pos = ext_data_start + 3;
            let name_len = u16::from_be_bytes([
                *buf.get(name_len_pos)?,
                *buf.get(name_len_pos + 1)?
            ]) as usize;
            
            let name_start = name_len_pos + 2;

            return Some(name_start + name_len / 2);
        }

        if ext_len == 0 && ext_type == 0 {
            break;
        }

        pos = ext_data_start + ext_len;
    }
    None
}