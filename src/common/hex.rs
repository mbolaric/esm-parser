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
            write!(f, "{byte:2X} ")?;
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
