use log::trace;

use crate::Result;

pub fn vec_u8_to_string(bytes: Vec<u8>) -> Result<String> {
    trace!("vec_u8_to_string = {:?}", bytes);
    let tem_str = String::from_utf8(bytes)?;
    Ok(tem_str.trim().to_string())
}
