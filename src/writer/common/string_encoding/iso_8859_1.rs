pub(super) fn encode_iso_8859_1(c: char) -> u8 {
    if (c as u32) <= 0xFF { c as u8 } else { b'?' }
}
