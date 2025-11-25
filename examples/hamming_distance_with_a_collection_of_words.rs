//! Example:
//! 
//! ```sh
//! cargo run --example hamming_distance_with_a_collection_of_words -- information informatics affirmation
//! ```
//! 
//! Output:
//! 
//! ```
//! words: ["information", "informatics", "affirmation"]
//! maximum hamming distance: 5
//! ```

use similarity_trait::SimilarityIO;
struct HammingDistance;

impl SimilarityIO<&Vec<String>, usize> for HammingDistance {
    /// Similarity of a collection of strings via maximum Hamming distance.
    fn similarity(collection: &Vec<String>) -> usize {
        let mut max = 0;
        for i in 0..collection.len() {
            for j in (i + 1)..collection.len() {
                max = std::cmp::max(max, collection[i].chars().zip(collection[j].chars()).filter(|(c1, c2)| c1 != c2).count())
            }
        }
        max
    }
}


fn main() {
    let words: Vec<String> = std::env::args().skip(1).collect();
    let maximum_hamming_distance = HammingDistance::similarity(&words);
    println!("words: {:?}", words);
    println!("maximum hamming distance: {}", maximum_hamming_distance)
}
