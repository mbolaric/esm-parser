use crate::{
    Readable,
    gen1::FullCardNumber,
    tacho::{Name, TimeReal},
};

#[derive(Debug)]
pub struct DownloadActivity {
    pub downloading_time: TimeReal,
    pub full_card_number: FullCardNumber,
    pub company_or_workshop_name: Name,
}

impl Readable<DownloadActivity> for DownloadActivity {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<DownloadActivity> {
        let downloading_time = TimeReal::read(reader)?;
        let full_card_number = FullCardNumber::read(reader)?;
        let company_or_workshop_name = Name::read(reader)?;

        Ok(Self { downloading_time, full_card_number, company_or_workshop_name })
    }
}
