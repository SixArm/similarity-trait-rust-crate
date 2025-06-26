//! Example:
//! 
//! ```sh
//! cargo run --example percent_change -- 100 120
//! ```
//! 
//! Output:
//! 
//! ```
//! value 1: 100
//! value 2: 120
//! percent change: 20
//! ```

use similarity_trait::Similarity;
struct PercentChange;

impl Similarity<(i32, i32), f64> for PercentChange {
    /// Similarity of numbers via percent change.
    fn similarity(input: (i32, i32)) -> f64 {
        (100.0 * (input.1 - input.0) as f64) / i32::abs(input.0) as f64
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let value1 = args[0].parse::<i32>().unwrap();
    let value2 = args[1].parse::<i32>().unwrap();
    let percent_change = PercentChange::similarity((value1, value2));
    println!("value 1: {}", value1);
    println!("value 2: {}", value2);
    println!("percent change: {}", percent_change)
}
