use crate::Result;

#[allow(dead_code)]
pub fn time_u16_to_string(time_min: u16) -> String {
    let hours: u16 = time_min / 60;
    let mins: u16 = time_min % 60;
    format!("{:2}:{:2}", hours, mins)
}
