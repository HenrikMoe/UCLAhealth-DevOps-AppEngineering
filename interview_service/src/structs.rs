use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PopulationData {
    pub data: Vec<Record>,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    #[serde(rename = "ID State")]
    pub id_state: String,
    #[serde(rename = "State")]
    pub state: String,
    #[serde(rename = "ID Year")]
    pub id_year: u16,
    #[serde(rename = "Year")]
    pub year: String,
    #[serde(rename = "Population")]
    pub population: u64,
    #[serde(rename = "Slug State")]
    pub slug_state: String,
}

#[derive(Debug)]
pub struct YearlyRecord {
    pub year: String,
    pub population: u64,
    pub percent_change: Option<f64>,
    pub prime_number: Option<u64>,
}

#[derive(Debug)]
pub struct StateData {
    pub state_name: String,
    pub records: Vec<YearlyRecord>,
}
