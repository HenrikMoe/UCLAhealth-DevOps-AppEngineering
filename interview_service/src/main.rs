use reqwest::Error;
use std::collections::HashMap;
use csv::Writer;


// Import the structs from the separate file
use crate::structs::{PopulationData, Record, YearlyRecord, StateData};
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
        println!("State Data Map Bulk Calculation of the prime factorization: {:?}", prime_factors_map);

        let mut state_data_vec: Vec<StateData> = Vec::new();

        for (state_name, records) in state_data_map {
            let prime_factors_for_state = prime_factors_map.get(&state_name).cloned().unwrap_or_default();
            state_data_vec.push(StateData {
                 state_name: state_name.clone(),
                 records: records.clone(),
                 prime_factors: prime_factors_for_state, // Get prime factors for the state
                 latest_year: get_latest_year_string(&records),
             });
        }
        

         println!("State Data Map Bulk Calculation of the prime factorization with latest year: {:?}", state_data_vec);
       
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

    








        // let mut state_data_vec: Vec<StateData> = Vec::new();

        // for (state_name, records) in state_data_map {
        //     state_data_vec.push(StateData {
        //         state_name,
        //         records,
        //     });
        // }

        // // Serialize to CSV
        // let file_path = "output.csv";
        // let mut wtr = Writer::from_path(file_path)?;

        // // Write headers
        // wtr.write_record(&["StateName", "Year", "Population", "PercentChange", "PrimeNumbers"])?;

        // // Write records
        // for state_data in state_data_vec {
        //     for record in state_data.records {
        //         let prime_numbers = record.prime_number.map_or(String::new(), |prime| prime.to_string());
        //         wtr.write_record(&[
        //             &state_data.state_name,
        //             &record.year,
        //             &record.population.to_string(),
        //             &record.percent_change.unwrap_or(0.0).to_string(),
        //             &prime_numbers,
        //         ])?;
        //     }
        // }

        // wtr.flush()?;
 
        // Uncomment this section to serialize to CSV if needed
        /*
        let mut state_data_vec: Vec<StateData> = Vec::new();

        for (state_name, records) in state_data_map {
            state_data_vec.push(StateData {
                state_name,
                records,
            });
        }

        // Serialize to CSV
        let file_path = "output.csv";
        let mut wtr = Writer::from_path(file_path)?;

        // Write headers
        wtr.write_record(&["StateName", "Record", "PrimeNumbers"])?;

        // Write records
        for state_data in state_data_vec {
            let mut record_line = String::new();
            let mut prime_numbers = String::new();

            for record in state_data.records {
                if !record_line.is_empty() {
                    record_line.push(';');
                }
                record_line.push_str(&format!(
                    "{} {} {}",
                    record.year, record.population, record.percent_change.unwrap_or(0.0)
                ));
                if let Some(prime) = record.prime_number {
                    if !prime_numbers.is_empty() {
                        prime_numbers.push_str("; ");
                    }
                    prime_numbers.push_str(&prime.to_string());
                }
            }

            wtr.write_record(&[&state_data.state_name, &record_line, &prime_numbers])?;
        }

        wtr.flush()?;
        println!("Data successfully written to {}", file_path);
        */
        
        //println!("Yearly record object: {:?}", state_data_map);

