use std::error::Error;
use std::fs::File;
pub use csv::Writer;
use crate::structs::StateData;

pub fn write_state_data_to_csv(state_data: &[StateData]) -> Result<(), Box<dyn Error>> {
    // Create a new file named output.csv
    let mut writer = Writer::from_path("output.csv")?;

    // Write header row
    writer.write_record(&["State Name", "Population and Percentage Change", "Years", "Latest Year", "Prime Factors"])?;

    // Write data for each state
    for data in state_data {
        let mut population_and_percentage = Vec::new();
        for (i, record) in data.records.iter().enumerate() {
            let mut pop_percent_str = record.population.to_string();
            if let Some(percent_change) = &record.percent_change {
                pop_percent_str.push_str(&format!(" {:}", percent_change)); // Append percentage change if available
            }
            population_and_percentage.push(pop_percent_str);
        }

        let prime_factors_str = data.prime_factors.values().cloned().collect::<Vec<_>>().join(";");


        
        writer.write_record(&[
            &data.state_name,
            &data.records.iter().map(|r| r.year.clone()).collect::<Vec<_>>().join(";"),
            &data.latest_year,
            //new row here even tho we are still looking at the same state 
            &population_and_percentage.join(";"), // Join population and percentage strings
            // &data.placeholder,
            &prime_factors_str, // Write the prime factors string
        ])?;
    }

    // Flush the writer to ensure all data is written to the file
    writer.flush()?;

    Ok(())
}