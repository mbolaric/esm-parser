#[derive(Debug)]
pub struct CardAuthenticationRecordsParams {
    pub no_of_records: u32,
}

impl CardAuthenticationRecordsParams {
    pub fn new(no_of_records: u32) -> Self {
        Self { no_of_records }
    }
}
