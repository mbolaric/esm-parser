use binary_data::BigEndian;
use chrono::Utc;

use crate::Readable;

#[derive(Debug)]
pub struct TimeReal {
    pub data: u32,
    date_time: Option<chrono::DateTime<Utc>>,
}

impl TimeReal {
    pub fn get_date_str(&self) -> String {
        self.date_time
            .map_or("".to_owned(), |data| data.format("%Y-%m-%d").to_string())
    }

    pub fn get_date_time_str(&self) -> String {
        self.date_time.map_or("".to_owned(), |data| {
            data.format("%Y-%m-%d %H:%M:%S").to_string()
        })
    }

    pub fn get_time_str(&self) -> String {
        self.date_time
            .map_or("".to_owned(), |data| data.format("%H:%M:%S").to_string())
    }
}

impl Readable<TimeReal> for TimeReal {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(
        reader: &mut R,
    ) -> crate::Result<TimeReal> {
        let data = reader.read_u32::<BigEndian>()?;

        let date_time = chrono::DateTime::from_timestamp(data as i64, 0);

        println!("{:?}", date_time);
        Ok(Self { data, date_time })
    }
}
