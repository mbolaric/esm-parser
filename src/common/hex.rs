use std::fmt;

pub struct HexHelper<'a>(&'a [u8]);

impl<'a> HexHelper<'a> {
    fn new<T>(data: &'a T) -> HexHelper<'a>
    where
        T: ?Sized + AsRef<[u8]> + 'a,
    {
        HexHelper(data.as_ref())
    }

    pub fn to_lower_hex_string(&self) -> String {
        self.0.iter().map(|c| format!("{c:02x}")).collect()
    }

    pub fn to_upper_hex_string(&self) -> String {
        self.0.iter().map(|c| format!("{c:02X}")).collect()
    }
}

impl fmt::Display for HexHelper<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0 {
            write!(f, "{byte:2X}")?;
        }
        Ok(())
    }
}

pub trait HexDisplay {
    fn to_hex(&self) -> HexHelper<'_>;
    fn to_hex_string(&self) -> String;
}

impl<T> HexDisplay for T
where
    T: ?Sized + AsRef<[u8]>,
{
    fn to_hex(&self) -> HexHelper<'_> {
        HexHelper::new(self)
    }

    fn to_hex_string(&self) -> String {
        HexHelper::new(self).to_upper_hex_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_lower_hex_string() {
        let data1: &[u8] = &[0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
        assert_eq!(data1.to_hex().to_lower_hex_string(), "0123456789abcdef");

        let data2: &[u8] = &[0xFF, 0x00, 0xAA, 0x55];
        assert_eq!(data2.to_hex().to_lower_hex_string(), "ff00aa55");

        let data3: &[u8] = &[];
        assert_eq!(data3.to_hex().to_lower_hex_string(), "");
    }

    #[test]
    fn test_to_upper_hex_string() {
        let data1: &[u8] = &[0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
        assert_eq!(data1.to_hex().to_upper_hex_string(), "0123456789ABCDEF");

        let data2: &[u8] = &[0xFF, 0x00, 0xAA, 0x55];
        assert_eq!(data2.to_hex().to_upper_hex_string(), "FF00AA55");

        let data3: &[u8] = &[];
        assert_eq!(data3.to_hex().to_upper_hex_string(), "");
    }

    #[test]
    fn test_trait_to_hex_string() {
        // The trait method defaults to upper case
        let data1: &[u8] = &[0x01, 0x23, 0xAB, 0xCD];
        assert_eq!(data1.to_hex_string(), "0123ABCD");

        let vec1 = vec![0xDE, 0xAD, 0xBE, 0xEF];
        assert_eq!(vec1.to_hex_string(), "DEADBEEF");
    }

    #[test]
    fn test_fmt_display() {
        let data: &[u8] = &[0xDE, 0xAD, 0xBE, 0xEF];
        let hex_display = data.to_hex();
        assert_eq!(format!("{}", hex_display), "DEADBEEF");

        let data_empty: &[u8] = &[];
        let hex_display_empty = data_empty.to_hex();
        assert_eq!(format!("{}", hex_display_empty), "");
    }
}
