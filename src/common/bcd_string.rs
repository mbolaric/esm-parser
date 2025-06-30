#[derive(Debug)]
pub struct BCDString {}

impl BCDString {
    pub fn decode(bcd: &[u8]) -> String {
        let mut result = String::new();
        for byte in bcd {
            result.push(char::from(b'0' + ((byte >> 4) & 0x0F)));
            result.push(char::from(b'0' + (byte & 0x0F)));
        }
        result
    }
}
