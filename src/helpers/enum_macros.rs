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
