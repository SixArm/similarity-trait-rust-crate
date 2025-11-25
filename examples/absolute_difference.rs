//! Example:
//! 
//! ```sh
//! cargo run --example absolute_difference -- 100 120
//! ```
//! 
//! Output:
//! 
//! ```
//! value 1: 100
//! value 2: 120
//! absolute difference: 20
//! ```

use similarity_trait::SimilarityIO;
struct AbsoluteDifference;

impl SimilarityIO<(i32, i32), Option<i32>> for AbsoluteDifference {
    /// Similarity of numbers via absolute difference.
    fn similarity(input: (i32, i32)) -> Option<i32> {
        Some(input.1.checked_sub(input.0)?.abs())
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let value1 = args[0].parse::<i32>().unwrap();
    let value2 = args[1].parse::<i32>().unwrap();
    let absolute_difference = AbsoluteDifference::similarity((value1, value2));
    println!("value 1: {}", value1);
    println!("value 2: {}", value2);
    println!("absolute difference: {}", absolute_difference.unwrap())
}
