/* 
   
This module contains the main function executed by cargo run. 
Rust Http library 'reqwest' is called for handling the declartion of the JSON api in the program. 
The structs and interfaces modules are called to extract and analyze the data. 
The csv module completes the job in the requested output. 

====================

- author: Henrik Moe
- version: 0
- date: 08 June 2024

====================


 */


use reqwest::Error;
use std::collections::HashMap;


// Import the structs from the separate file
use crate::structs::{PopulationData, YearlyRecord, StateData};
use crate::interfaces::{calculate_percent_change, get_latest_year_prime_factors, get_latest_year_string};

mod structs;
mod interfaces;
mod csv;


#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://datausa.io/api/data?drilldowns=State&measures=Population";

    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let population_data: PopulationData = response.json().await?;

        let mut state_data_map: HashMap<String, Vec<YearlyRecord>> = HashMap::new();

        for record in population_data.data {
            state_data_map
                .entry(record.state.clone())
                .or_insert_with(Vec::new)
                .push(YearlyRecord {
                    year: record.year.clone(),
                    population: record.population,
                    percent_change: None,
                    prime_number: None,
                });
        }

        // Calculate percent change for each state's yearly records
        for records in state_data_map.values_mut() {
            records.sort_by(|a, b| a.year.cmp(&b.year)); // Ensure records are sorted by year

            for i in 1..records.len() {
                let percent_change = calculate_percent_change(records[i].population, records[i - 1].population);
                records[i].percent_change = Some(percent_change);
            }
        }

        // Get prime factors for the latest year's population for each state
        let prime_factors_map = get_latest_year_prime_factors(&state_data_map);
       // println!("State Data Map Bulk Calculation of the prime factorization: {:?}", prime_factors_map);

        let mut state_data_vec: Vec<StateData> = Vec::new();

        for (state_name, records) in state_data_map {
            let prime_factors_for_state = prime_factors_map.get(&state_name).cloned().unwrap_or_default();
            state_data_vec.push(StateData {
                 state_name: state_name.clone(),
                 records: records.clone(),
                //  placeholder: "Squarelandia".to_string(), // Assign the placeholder value
                 prime_factors: prime_factors_for_state, // Get prime factors for the state
                 latest_year: get_latest_year_string(&records),
             });
        }
        

         println!("All done XD ");
       
         match csv::write_state_data_to_csv(&state_data_vec) {
            Ok(_) => (),
            Err(e) => {
                // Handle the error here
                println!("Error writing state data to CSV: {:?}", e);
            }
        }
        
        } else {
            println!("Request failed with status code: {}", response.status());
        }
    
        Ok(())
    }

    



