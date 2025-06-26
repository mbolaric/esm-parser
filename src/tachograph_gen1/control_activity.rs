use crate::{
    Readable,
    gen1::FullCardNumber,
    tacho::{ControlTypeCode, TimeReal},
};

#[derive(Debug)]
pub struct ControlActivityRecord {
    pub control_type: ControlTypeCode,
    pub control_time: TimeReal,
    pub full_card_number: FullCardNumber,
    pub download_period_begin_time: TimeReal,
    pub download_period_end_time: TimeReal,
}

impl Readable<ControlActivityRecord> for ControlActivityRecord {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(
        reader: &mut R,
    ) -> crate::Result<ControlActivityRecord> {
        let ct = reader.read_u8()?;
        let control_type: ControlTypeCode = ct.into();
        let control_time = TimeReal::read(reader)?;

        let full_card_number = FullCardNumber::read(reader)?;
        let download_period_begin_time = TimeReal::read(reader)?;
        let download_period_end_time = TimeReal::read(reader)?;

        Ok(Self {
            control_type,
            control_time,
            full_card_number,
            download_period_begin_time,
            download_period_end_time,
        })
    }
}

#[derive(Debug)]
pub struct ControlActivity {
    pub no_of_controls: u8,
    pub control_activities: Vec<ControlActivityRecord>,
}

impl Readable<ControlActivity> for ControlActivity {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(
        reader: &mut R,
    ) -> crate::Result<ControlActivity> {
        let no_of_controls = reader.read_u8()?;
        let mut control_activities: Vec<ControlActivityRecord> = Vec::new();
        for _ in 0..no_of_controls {
            control_activities.push(ControlActivityRecord::read(reader)?);
        }

        Ok(Self { no_of_controls, control_activities })
    }
}
