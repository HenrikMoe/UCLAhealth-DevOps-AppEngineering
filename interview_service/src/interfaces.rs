(/* 
   
This module contains functions for returing desired analysis attributes from the dataset. 
The functions below are applied in the main module to the structs module. 

====================

- author: Henrik Moe
- version: 0
- date: 08 June 2024

====================


 */)

use std::collections::HashMap;
use crate::structs::YearlyRecord;


pub fn calculate_percent_change(current: u64, previous: u64) -> String {
    if previous == 0 {
        return String::from("0%"); // Avoid division by zero
    }
    let percent_change = ((current as f64 - previous as f64) / previous as f64) * 100.0;
    format!("({:.3}%)", percent_change)
}



pub fn prime_factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut number = n;
    let mut divisor = 2;

    while number >= divisor * divisor {
        while number % divisor == 0 {
            factors.push(divisor);
            number /= divisor;
        }
        divisor += 1;
    }
    if number > 1 {
        factors.push(number);
    }

    factors
}


pub fn get_latest_year_prime_factors(state_data_map: &HashMap<String, Vec<YearlyRecord>>) -> HashMap<String, HashMap<String, String>> {
    let mut prime_factors_map: HashMap<String, HashMap<String, String>> = HashMap::new();

    for (state, records) in state_data_map {
        let mut yearly_factors: HashMap<String, String> = HashMap::new();

        if let Some(latest_record) = records.iter().max_by_key(|record| record.year.parse::<u16>().unwrap_or(0)) {
            let factors = prime_factors(latest_record.population);
            let factors_str = factors.iter().map(|&f| f.to_string()).collect::<Vec<String>>().join(";");
            let result_str = format!("{}", factors_str);
            yearly_factors.insert(latest_record.year.clone(), result_str);
        }

        prime_factors_map.insert(state.clone(), yearly_factors);
    }

    prime_factors_map
}


pub fn get_latest_year_string(records: &[YearlyRecord]) -> String {
    records.iter()
        .max_by_key(|record| record.year.parse::<u16>().unwrap_or(0))
        .map(|record| format!("{} Factors", record.year))
        .unwrap_or_else(|| String::from("N/A"))
}

