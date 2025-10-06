pub fn get_sub_array(data: &[u8], offset: usize, len: usize) -> &[u8] {
    &data[offset..offset + len]
}
