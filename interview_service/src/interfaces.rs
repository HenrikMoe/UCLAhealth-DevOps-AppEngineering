// make simple interface with 2+2 f 

pub fn add_two_to_array(arr: &mut [f32]) {
    for element in arr {
        *element += 2.0;
    }
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

