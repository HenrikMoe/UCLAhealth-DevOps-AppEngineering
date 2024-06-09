(/* 
   
This module contains the structs for declaring data from the JSON payload into workable rust objects. 
The Record struct is the first to interface with the JSON payload. 
The PopulationData struct contains lists of Record structs. 
The Yearly record struct further organizes the data with placeholders for desired analysis attributes. 
The state data aggreagates and provides further placeholders for desired analysis attributes. 

====================

- author: Henrik Moe
- version: 0
- date: 08 June 2024

====================


 */)

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
