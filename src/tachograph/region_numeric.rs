use crate::impl_enum_from_u8;

/// Numerical reference to a region within a specified country.
#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum RegionNumeric {
    Unknown = 0,
    Andalucia = 1,
    Aragon = 2,
    Asturias = 3,
    Cantabria = 4,
    Cataluna = 5,
    CastillaLeon = 6,
    CastillaLaMancha = 7,
    Valencia = 8,
    Extremadura = 9,
    Galicia = 10,
    Baleares = 11,
    Canarias = 12,
    LaRioja = 13,
    Madrid = 14,
    Murcia = 15,
    Navarra = 16,
    PaisVasco = 17,
    Ceuta = 18,
    Melilla = 20,
}

impl_enum_from_u8!(
    RegionNumeric {
        Unknown = 0,
        Andalucia = 1,
        Aragon = 2,
        Asturias = 3,
        Cantabria = 4,
        Cataluna = 5,
        CastillaLeon = 6,
        CastillaLaMancha = 7,
        Valencia = 8,
        Extremadura = 9,
        Galicia = 10,
        Baleares = 11,
        Canarias = 12,
        LaRioja = 13,
        Madrid = 14,
        Murcia = 15,
        Navarra = 16,
        PaisVasco = 17,
        Ceuta = 18,
        Melilla = 20,
    }
);
