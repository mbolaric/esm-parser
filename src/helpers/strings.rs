use log::trace;

use crate::Result;

pub fn vec_u8_to_string(bytes: Vec<u8>) -> Result<String> {
    trace!("vec_u8_to_string = {:?}", bytes);
    let tem_str = String::from_utf8(bytes)?;
    Ok(tem_str.trim().to_string())
}

pub fn time_u16_to_string(time_min: u16) -> String {
    let hours: u16 = time_min / 60;
    let mins: u16 = time_min % 60;
    format!("{:2}:{:2}", hours, mins)
}
