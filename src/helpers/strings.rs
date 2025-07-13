use log::trace;

use crate::Result;

pub fn vec_u8_to_string(bytes: Vec<u8>) -> Result<String> {
    trace!("vec_u8_to_string = {:?}", bytes);
    // TODO: we need to use proper decoding.
    let tem_str = String::from_utf8_lossy(&bytes);
    Ok(tem_str.trim().to_string())
}

#[allow(dead_code)]
pub fn time_u16_to_string(time_min: u16) -> String {
    let hours: u16 = time_min / 60;
    let mins: u16 = time_min % 60;
    format!("{:2}:{:2}", hours, mins)
}
