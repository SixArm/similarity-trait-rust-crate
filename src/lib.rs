//! # Similarity trait
//!
//! The Similarity trait defines one function with one input and one output.
//!
//! ```no_run
//! pub trait Similarity<InputType, OutputType> {
//!     fn similarity(input: InputType) -> OutputType;
//! }
//! ```
//! 
//! This trait is purposefully very generic so you can use it as you wish.
//!
//! We use this trait in our programs to create multiple kinds of similarity
//! functionality, such as for trying various similarity algorithms that we want
//! to use with the same input type and same output type.
//!
//! ## Similarity of a pair
//!
//! One way to use this trait is to calculate the similarity of a pair of
//! values, such as two numbers, or two strings, or two images.
//! 
//! This is sometimes known as pairwise similarity or pair matching.
//! 
//! Example: given two numbers, then return the percent change.
//!
//! ```rust
//! use similarity_trait::Similarity;
//! struct MyStruct;
//!
//! impl Similarity<(i32, i32), f64> for MyStruct {
//!     /// Similarity of numbers via percent change.
//!     fn similarity(input: (i32, i32)) -> f64 {
//!         (100.0 * (input.1 - input.0) as f64) / i32::abs(input.0) as f64
//!     }
//! }
//!
//! let percent_change = MyStruct::similarity((100, 120));
//! assert_eq!(percent_change, 20.0);
//! ```
//!
//! ## Similarity of a collection
//!
//! One way to use this trait is to calculate the similarity of a collection of
//! values, such as an array of numbers, or vector of strings, or set of images.
//! 
//! This is sometimes called intra-group similarity or statistical correlation.
//!
//! Example: given numbers, then return the population standard deviation.
//!
//! ```rust
//! use similarity_trait::Similarity;
//! struct MyStruct;
//!
//! impl Similarity<Vec<f64>, Option<f64>> for MyStruct {
//!     /// Similarity of numbers via population standard deviation
//!     fn similarity(numbers: Vec<f64>) -> Option<f64> {
//!         if numbers.is_empty() { return None }
//!         let mean = numbers.iter().sum::<f64>() / numbers.len() as f64;
//!         let variance = numbers.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / numbers.len() as f64;
//!         Some(variance.sqrt())
//!     }
//! }
//!
//! let numbers = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
//! let standard_deviation = MyStruct::similarity(numbers).expect("similarity");
//! assert!(standard_deviation > 1.999 && standard_deviation < 2.001);
//! ```
//!
//! ## Similarity of a pair or a collection
//!
//! You may want to choose whether you prefer to calculate the similarity of a
//! pair (such as two strings) or a collection (such as a vector of strings).
//!
//! Example: given a pair of strings, then return the Hamming distance.
//!
//! ```rust
//! use similarity_trait::Similarity;
//! struct MyStruct;
//!
//! impl Similarity<(&str, &str), usize> for MyStruct {
//!     /// Similarity of a pair of strings via Hamming distance.
//!     fn similarity(pair: (&str, &str)) -> usize {
//!         pair.0.chars().zip(pair.1.chars()).filter(|(c1, c2)| c1 != c2).count()
//!     }
//! }
//!
//! let pair = ("information", "informatics");
//! let hamming_distance = MyStruct::similarity(pair);
//! assert_eq!(hamming_distance, 2);
//! ```
//!
//! Example: given a collection of strings, then return the maximum Hamming
//! distance.
//!
//! ```rust
//! use similarity_trait::Similarity;
//! struct MyStruct;
//!
//! impl Similarity<Vec<&str>, usize> for MyStruct {
//!     /// Similarity of a collection of strings via maximum Hamming distance.
//!     fn similarity(collection: Vec<&str>) -> usize {
//!         let mut max = 0;
//!         for i in 0..collection.len() {
//!             for j in (i + 1)..collection.len() {
//!                 max = std::cmp::max(max, collection[i].chars().zip(collection[j].chars()).filter(|(c1, c2)| c1 != c2).count())
//!             }
//!         }
//!         max
//!     }
//! }
//!
//! let collection = vec!["information", "informatics", "affirmation"];
//! let maximum_hamming_distance = MyStruct::similarity(collection);
//! assert_eq!(maximum_hamming_distance, 5);
//! ```

pub trait Similarity<InputType, OutputType> {
    fn similarity(input: InputType) -> OutputType;
}

#[cfg(test)]
mod tests {
    use super::*;
    struct MyStruct {}

    mod percent_change {
        use super::*;

        impl Similarity<(i32, i32), f64> for MyStruct {
            /// Similarity of two numbers via percent change.
            fn similarity(input: (i32, i32)) -> f64 {
                (100.0 * (input.1 - input.0) as f64) / i32::abs(input.0) as f64
            }
        }

        #[test]
        fn test() {
            let percent_change = MyStruct::similarity((100, 120));
            assert_eq!(percent_change, 20.0);
        }

    }

    mod population_standard_deviation {
        use super::*;

        impl Similarity<Vec<f64>, Option<f64>> for MyStruct {
            /// Similarity of numbers via population standard deviation.
            fn similarity(numbers: Vec<f64>) -> Option<f64> {
                if numbers.is_empty() { return None }
                let mean = numbers.iter().sum::<f64>() / numbers.len() as f64;
                let variance = numbers.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / numbers.len() as f64;
                Some(variance.sqrt())
            }
        }

        #[test]
        fn test() {
            let numbers = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
            let standard_deviation = MyStruct::similarity(numbers).expect("similarity");
            assert!(standard_deviation > 1.999 && standard_deviation < 2.001);
        }

    }

    mod hamming_distance_for_a_pair_of_strings {
        use super::*;

        impl Similarity<(&str, &str), usize> for MyStruct {
            /// Similarity of two strings via Hamming distance.
            fn similarity(input: (&str, &str)) -> usize {
                input.0.chars().zip(input.1.chars()).filter(|(c1, c2)| c1 != c2).count()
            }
        }

        #[test]
        fn test() {
            let pair = ("information", "informatics");
            let hamming_distance = MyStruct::similarity(pair);
            assert_eq!(hamming_distance, 2);
        }

    }

    mod hamming_distance_for_a_collection_of_strings {
        use super::*;

        impl Similarity<Vec<&str>, usize> for MyStruct {
            /// Similarity of a collection strings via maximum Hamming distance.
            fn similarity(strings: Vec<&str>) -> usize {
                let mut max = 0;
                for i in 0..strings.len() {
                    for j in (i + 1)..strings.len() {
                        max = std::cmp::max(max, strings[i].chars().zip(strings[j].chars()).filter(|(c1, c2)| c1 != c2).count())
                    }
                }
                max
            }
        }

        #[test]
        fn test() {
            let collection = vec!["information", "informatics", "affirmation"];
            let maximum_hamming_distance = MyStruct::similarity(collection);
            assert_eq!(maximum_hamming_distance, 5);
        }

    }

}
