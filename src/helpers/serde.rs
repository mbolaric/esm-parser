use serde::{Serialize, Serializer};
use serde_json::json;

#[allow(dead_code)]
pub fn from_obj_to_string<T: Serialize>(message: &T) -> String {
    let json_obj = json!(message);
    json_obj.to_string()
}

#[allow(dead_code)]
pub fn serialize_option_utc_date_time<S>(date: &Option<time::OffsetDateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match date.is_some() {
        true => {
            let fmt = time::macros::format_description!("[year]-[month]-[day] [hour]:[minute]:[second] UTC");
            let s = date.as_ref().unwrap().format(&fmt).unwrap_or_default();
            serializer.serialize_str(&s)
        }
        false => serializer.serialize_none(),
    }
}
