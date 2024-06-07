use reqwest::Error;
use std::collections::HashMap;

// Import the structs from the separate file
mod structs;
use structs::{PopulationData, YearlyRecord};

//mod interface import for the maths
mod interfaces;

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
                    percent_change: None, // Calculate this if needed
                    prime_number: None,   // Calculate this if needed
                });
        }

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
        
        println!("Yearly record object: {:?}", state_data_map);
    } else {
        println!("Request failed with status code: {}", response.status());
    }

    Ok(())
}
