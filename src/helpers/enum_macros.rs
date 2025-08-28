#[macro_export]
macro_rules! impl_enum_from_u8 {
    ($enum_name:ident { $($variant:ident = $value:expr),+ $(,)? }) => {
        impl From<u8> for $enum_name {
            fn from(value: u8) -> Self {
                match value {
                    $( $value => $enum_name::$variant, )+
                    _ =>  $enum_name::Unknown,
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_enum_from_u16 {
    ($enum_name:ident { $($variant:ident = $value:expr),+ $(,)? }) => {
        impl From<u16> for $enum_name {
            fn from(value: u16) -> Self {
                match value {
                    $( $value => $enum_name::$variant, )+
                    _ =>  $enum_name::Unknown,
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    // Define a test enum for the u8 macro
    #[derive(Debug, PartialEq)]
    #[repr(u8)]
    enum TestEnumU8 {
        A,
        B,
        C,
        Unknown,
    }

    // Use the macro to implement From<u8>
    impl_enum_from_u8!(TestEnumU8 {
        A = 0x01,
        B = 0x02,
        C = 0x05,
    });

    #[test]
    fn test_impl_enum_from_u8() {
        assert_eq!(TestEnumU8::from(0x01), TestEnumU8::A);
        assert_eq!(TestEnumU8::from(0x02), TestEnumU8::B);
        assert_eq!(TestEnumU8::from(0x05), TestEnumU8::C);
        assert_eq!(TestEnumU8::from(0x00), TestEnumU8::Unknown); // Test the unknown case
        assert_eq!(TestEnumU8::from(0xFF), TestEnumU8::Unknown); // Test another unknown case
    }

    // Define a test enum for the u16 macro
    #[derive(Debug, PartialEq)]
    #[repr(u16)]
    enum TestEnumU16 {
        X,
        Y,
        Z,
        Unknown,
    }

    // Use the macro to implement From<u16>
    impl_enum_from_u16!(TestEnumU16 {
        X = 0x1000,
        Y = 0x2000,
        Z = 0x3000,
    });

    #[test]
    fn test_impl_enum_from_u16() {
        assert_eq!(TestEnumU16::from(0x1000), TestEnumU16::X);
        assert_eq!(TestEnumU16::from(0x2000), TestEnumU16::Y);
        assert_eq!(TestEnumU16::from(0x3000), TestEnumU16::Z);
        assert_eq!(TestEnumU16::from(0x0000), TestEnumU16::Unknown); // Test the unknown case
        assert_eq!(TestEnumU16::from(0xFFFF), TestEnumU16::Unknown); // Test another unknown case
    }
}
