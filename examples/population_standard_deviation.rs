//! Example:
//! 
//! ```sh
//! cargo run --example population_standard_deviation -- 2 4 4 4 5 5 7 9
//! ```
//! 
//! Output:
//! 
//! ```
//! numbers: [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0]
//! population standard deviation: 2
//! ```

use similarity_trait::Similarity;
struct PopulationStandardDeviation;

impl Similarity<&Vec<f64>, Option<f64>> for PopulationStandardDeviation {
    /// Similarity of numbers via population standard deviation
    fn similarity(numbers: &Vec<f64>) -> Option<f64> {
        if numbers.is_empty() { return None }
        let mean = numbers.iter().sum::<f64>() / numbers.len() as f64;
        let variance = numbers.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / numbers.len() as f64;
        Some(variance.sqrt())
    }
}

fn main() {
    let numbers: Vec<f64> = std::env::args().skip(1).map(|x|->f64{x.parse().unwrap()}).collect();
    let population_standard_deviation = PopulationStandardDeviation::similarity(&numbers).expect("similarity");
    println!("numbers: {:?}", &numbers);
    println!("population standard deviation: {}", population_standard_deviation)
}
