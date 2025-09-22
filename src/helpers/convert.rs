/// Converts a u16 value representing minutes into a string formatted as "HH:MM".
///
/// # Arguments
///
/// * `time_min` - A u16 value representing the total number of minutes.
///
/// # Returns
///
/// A string representation of the time in "HH:MM" format.
#[allow(dead_code)]
pub fn time_u16_to_string(time_min: u16) -> String {
    let hours: u16 = time_min / 60;
    let mins: u16 = time_min % 60;
    format!("{hours:2}:{mins:2}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_u16_to_string() {
        assert_eq!(time_u16_to_string(0), " 0: 0");
        assert_eq!(time_u16_to_string(59), " 0:59");
        assert_eq!(time_u16_to_string(60), " 1: 0");
        assert_eq!(time_u16_to_string(61), " 1: 1");
        assert_eq!(time_u16_to_string(1439), "23:59"); // 23 * 60 + 59
        assert_eq!(time_u16_to_string(1440), "24: 0");
    }
}
