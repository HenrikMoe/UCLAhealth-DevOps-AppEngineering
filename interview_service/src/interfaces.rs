// make simple interface with 2+2 f 

// pub fn add_two_to_array(arr: &mut [f32]) {
//     for element in arr {
//         *element += 2.0;
//     }
// }

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
            let result_str = format!("{} ({})", latest_record.year, factors_str);
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

// use serde::Deserialize;

// #[derive(Debug, Deserialize, Clone)] // Add Clone trait
// pub struct Record {
//     #[serde(rename = "State")]
//     pub state: String,
//     #[serde(rename = "Year")]
//     pub year: String,
//     #[serde(rename = "Population")]
//     pub population: u64,
// }

// use std::collections::HashMap;

// #[derive(Debug, Deserialize)]
// pub struct PopulationData {
//     pub data: Vec<Record>,
// }


// pub struct PrimeFactorOfLastYear;

// impl PrimeFactorOfLastYear {
//     pub fn declare_prime_factors(population: u64) -> String {
//         let factors = prime_factors(population);
//         factors.iter().map(|&f| f.to_string()).collect::<Vec<String>>().join("; ")
//     }
// }

// pub struct PerStatePopulationChange;

// impl PerStatePopulationChange {
//     pub fn percent_change(prev_population: u64, current_population: u64) -> f64 {
//         ((current_population as f64 - prev_population as f64) / prev_population as f64) * 100.0
//     }

//     pub fn declare_years(data: &[Record]) -> Vec<String> {
//         data.iter().map(|r| r.year.clone()).collect()
//     }

//     pub fn declare_populations(data: &[Record]) -> Vec<u64> {
//         data.iter().map(|r| r.population).collect()
//     }
// }

// // Helper function to calculate prime factors
// fn prime_factors(mut n: u64) -> Vec<u64> {
//     let mut factors = Vec::new();
//     let mut divisor = 2;
//     while n > 1 {
//         while n % divisor == 0 {
//             factors.push(divisor);
//             n /= divisor;
//         }
//         divisor += 1;
//     }
//     factors
// }

