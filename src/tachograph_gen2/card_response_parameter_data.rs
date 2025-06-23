#[derive(Debug)]
pub enum CardResponseParameterData {
    Unsupported,
    DriverCard,
    CompanyCard,
    WorkshopCard,
    ControlCard,
}
