(/* 
   
This module contains the write_state_data_to_csv function. 
The function is applied as the last step in the data processing in the parent main module. 
The result of this function is an exported CSV after iterating through the StateData struct for data elements. 

====================

- author: Henrik Moe
- version: 0
- date: 08 June 2024

====================


 */)

use std::error::Error;
use std::fs::File;
pub use csv::Writer;
use crate::structs::StateData;

pub fn write_state_data_to_csv(state_data: &[StateData]) -> Result<(), Box<dyn Error>> {
    // Create a new file named output.csv
    let mut writer = Writer::from_path("output.csv")?;

    // Write header row
    // Uncomment and modify if header row is needed
    // writer.write_record(&["State Name", "Years", "Population and Percentage", "Latest Year", "Squarelandia", "Prime Factors"])?;

    println!("State data before csv write record: {:?}", state_data);

    // Write data for each state
    for data in state_data {
        let mut population_and_percentage = Vec::new();
        for record in &data.records {
            let mut pop_percent_str = record.population.to_string();
            if let Some(percent_change) = &record.percent_change {
                pop_percent_str.push_str(&format!(" {}", percent_change)); // Append percentage change if available
            }
            population_and_percentage.push(pop_percent_str);
        }

        println!("population and percentage debug {:?}", population_and_percentage);
        println!(
            "year debug {:?}",
            data.records.iter().map(|r| r.year.clone()).collect::<Vec<_>>().join(","),
        );

        let prime_factors_str = data.prime_factors.values().cloned().collect::<Vec<_>>().join(";");

        let squarelandia_str: String = "Squarelandia".to_string();
        let years: Vec<String> = data.records.iter().map(|r| r.year.clone()).collect();

        println!("year debug2 {:?}", years);

        // Write the first part of the record
        let mut first_record = vec![data.state_name.clone()];
        first_record.extend(years.iter().cloned());
        first_record.push("2022 Factors".to_string());

        writer.write_record(&first_record)?;
        println!("First record debug {:?}", &first_record);

        // Write the second part of the record
        let mut second_record = vec![squarelandia_str.clone()];
        second_record.extend(population_and_percentage.iter().cloned());
        second_record.push(prime_factors_str.clone());

        writer.write_record(&second_record)?;
        println!("Second record debug {:?}", &second_record);
    }

    // Flush the writer to ensure all data is written to the file
    writer.flush()?;

    Ok(())
}