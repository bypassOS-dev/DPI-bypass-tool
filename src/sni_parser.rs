pub fn find_SNI(buf: &[u8]) -> Option<usize>{
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
    }
    todo!()
}
