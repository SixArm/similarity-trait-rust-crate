//! Example:
//! 
//! ```sh
//! cargo run --example hamming_distance_with_a_pair_of_words -- information informatics
//! ```
//! 
//! Output:
//! 
//! ```
//! word 1: information
//! word 2: informatics
//! hamming distance: 2
//! ```

use similarity_trait::Similarity;
struct HammingDistance;

impl Similarity<(&str, &str), usize> for HammingDistance {
    /// Similarity of a pair of strings via Hamming distance.
    fn similarity(pair: (&str, &str)) -> usize {
        pair.0.chars().zip(pair.1.chars()).filter(|(c1, c2)| c1 != c2).count()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let word1 = &args[0];
    let word2 = &args[1];
    let hamming_distance = HammingDistance::similarity((word1, word2));
    println!("word 1: {}", word1);
    println!("word 2: {}", word2);
    println!("hamming distance: {}", hamming_distance)
}
