use crate::tacho::TimeReal;
use crate::Readable;

#[derive(Debug)]
pub struct DownloadablePeriod {
    pub downloadable_period_min: TimeReal,
    pub downloadable_period_max: TimeReal,
}

impl Readable<DownloadablePeriod> for DownloadablePeriod {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(
        reader: &mut R,
    ) -> crate::Result<DownloadablePeriod> {
        let downloadable_period_min = TimeReal::read(reader)?;
        let downloadable_period_max = TimeReal::read(reader)?;

        Ok(Self {
            downloadable_period_min,
            downloadable_period_max,
        })
    }
}
