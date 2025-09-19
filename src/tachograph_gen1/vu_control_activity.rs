use serde::Serialize;

use crate::{
    Readable,
    tacho::{ControlType, FullCardNumber, TimeReal},
};

/// Information, stored in a vehicle unit, related to a control performed using
/// this VU (Annex 1B requirement 102 and Annex 1C requirement 126).
#[derive(Debug, Serialize)]
pub struct VuControlActivityRecord {
    #[serde(rename = "controlType")]
    pub control_type: ControlType,
    #[serde(rename = "controlTime")]
    pub control_time: TimeReal,
    #[serde(rename = "controlCardNumber")]
    pub control_card_number: FullCardNumber,
    #[serde(rename = "downloadPeriodBeginTime")]
    pub download_period_begin_time: TimeReal,
    #[serde(rename = "downloadPeriodEndTime")]
    pub download_period_end_time: TimeReal,
}

impl Readable<VuControlActivityRecord> for VuControlActivityRecord {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VuControlActivityRecord> {
        let ct = reader.read_u8()?;
        let control_type: ControlType = ct.into();
        let control_time = TimeReal::read(reader)?;

        let full_card_number = FullCardNumber::read(reader)?;
        let download_period_begin_time = TimeReal::read(reader)?;
        let download_period_end_time = TimeReal::read(reader)?;

        Ok(Self {
            control_type,
            control_time,
            control_card_number: full_card_number,
            download_period_begin_time,
            download_period_end_time,
        })
    }
}

/// Collection of Information, stored in a vehicle unit, related to a control performed using
/// this VU (Annex 1B requirement 102 and Annex 1C requirement 126).
#[derive(Debug, Serialize)]
pub struct VuControlActivity {
    #[serde(rename = "noOfControls")]
    pub no_of_controls: u8,
    #[serde(rename = "vuControlActivities")]
    pub vu_control_activities: Vec<VuControlActivityRecord>,
}

impl Readable<VuControlActivity> for VuControlActivity {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VuControlActivity> {
        let no_of_controls = reader.read_u8()?;
        let mut vu_control_activities: Vec<VuControlActivityRecord> = Vec::new();
        for _ in 0..no_of_controls {
            vu_control_activities.push(VuControlActivityRecord::read(reader)?);
        }

        Ok(Self { no_of_controls, vu_control_activities })
    }
}
