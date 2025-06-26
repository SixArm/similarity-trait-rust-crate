//! Example:
//! 
//! ```sh
//! cargo run --example levenshtein_distance -- inform information
//! ```
//! 
//! Output:
//! 
//! ```
//! word 1: inform
//! word 2: information
//! levenshtein distance: 5
//! ```

use similarity_trait::Similarity;
struct LevenshteinDistance;

impl Similarity<(&str, &str), usize> for LevenshteinDistance {
    fn similarity(input: (&str, &str)) -> usize {
        let (str1, str2) = input;
        let chars1: Vec<char> = str1.chars().collect();
        let chars2: Vec<char> = str2.chars().collect();
        let len1 = chars1.len();
        let len2 = chars2.len();
        
        if len1 == 0 { return len2; }
        if len2 == 0 { return len1; }
        
        let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];
        
        // Initialize first row and column
        for i in 0..=len1 {
            matrix[i][0] = i;
        }
        for j in 0..=len2 {
            matrix[0][j] = j;
        }
        
        // Fill the matrix
        for i in 1..=len1 {
            for j in 1..=len2 {
                let cost = if chars1[i - 1] == chars2[j - 1] { 0 } else { 1 };
                matrix[i][j] = (matrix[i - 1][j] + 1) // deletion
                    .min(matrix[i][j - 1] + 1) // insertion
                    .min(matrix[i - 1][j - 1] + cost); // substitution
            }
        }
        
        matrix[len1][len2]
    }

}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let word1 = &args[0];
    let word2 = &args[1];
    let levenshtein_distance = LevenshteinDistance::similarity((word1, word2));
    println!("word 1: {}", word1);
    println!("word 2: {}", word2);
    println!("levenshtein distance: {}", levenshtein_distance)
}
