use serde::Deserialize;
use std::collections::HashMap;

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

#[derive(Debug, Clone, Deserialize)]
pub struct YearlyRecord {
    pub year: String,
    pub population: u64,
    pub percent_change: Option<String>,
    pub prime_number: Option<u64>,
}

#[derive(Debug)]
pub struct StateData {
    pub state_name: String,
    pub records: Vec<YearlyRecord>,
    pub latest_year: String,
    //pub placeholder: String, // Add a field for the placeholder value
    // pub population_and_percentage: String, // Include a field for population and percentage
    pub prime_factors: HashMap<String, String>, // HashMap with String keys and values
}
