//! # Similarity trait
//!
//! **[documentation](https://docs.rs/similarity-trait/)** •
//! **[source](https://github.com/sixarm/similarity-trait-rust-crate/)** •
//! **[llms.txt](https://raw.githubusercontent.com/sixarm/similarity-trait/refs/heads/main/llms.txt)**
//! • **[crate](https://crates.io/crates/similarity-trait)** •
//! **[email](mailto:joel@joelparkerhenderson.com)**
//!
//! The Similarity trait crate defines a function `similarity` in various ways,
//! so you can compare of any kind of input and return any kind of output.
//!
//! We use these traits in our programs to create multiple kinds of similarity
//! functionality, such as for trying various similarity algorithms that we want
//! to use with the same input type and same output type.
//!
//! For examples, please see the directory [`examples`](examples).
//! 
//! ## Similarity of a pair
//!
//! One way to use this trait is to calculate the similarity of a pair of
//! values, such as two numbers, or two strings, or two images.
//! 
//! This is sometimes known as pairwise similarity or pair matching.
//! 
//! Example: given two numbers, then return the absolute difference,
//! and also handle the special case of an integer overflow.
//!
//! ```rust
//! use similarity_trait::*;
//! struct MyStruct;
//!
//! impl SimilarityIO<(i32, i32), Option<i32>> for MyStruct {
//!     fn similarity(input: (i32, i32)) -> Option<i32> {
//!         Some(input.1.checked_sub(input.0)?.abs())
//!     }
//! }
//!
//! let absolute_difference = MyStruct::similarity((100, 120));
//! assert_eq!(absolute_difference, Some(20));
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
//! use similarity_trait::*;
//! struct MyStruct;
//!
//! impl SimilarityIO<&Vec<f64>, Option<f64>> for MyStruct {
//!     /// Similarity of numbers via population standard deviation
//!     fn similarity(numbers: &Vec<f64>) -> Option<f64> {
//!         if numbers.is_empty() { return None }
//!         let mean = numbers.iter().sum::<f64>() / numbers.len() as f64;
//!         let variance = numbers.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / numbers.len() as f64;
//!         Some(variance.sqrt())
//!     }
//! }
//!
//! let numbers = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
//! let population_standard_deviation = MyStruct::similarity(&numbers).expect("similarity");
//! assert!(population_standard_deviation > 1.999 && population_standard_deviation < 2.001);
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
//! use similarity_trait::*;
//! struct MyStruct;
//!
//! impl SimilarityIO<(&str, &str), usize> for MyStruct {
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
//! use similarity_trait::*;
//! struct MyStruct;
//!
//! impl SimilarityIO<Vec<&str>, usize> for MyStruct {
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

/// SimilarityIO trait for Input, Output.
/// 
/// SimilarityIO defines one input, which is typically a tuple, and one output.
/// 
/// ```no_run
/// pub trait SimilarityIO<Input, Output> {
///     fn similarity(input: Input) -> Output;
/// }
/// ```
/// 
/// Example: calculate similarity of two strings by counting equivalent characters.
/// 
/// ```no_run
/// use similarity_trait::*;
/// 
/// struct S;
/// 
/// impl SimilarityIO<(&str, &str), usize> for S {
///     fn similarity((a, b): (&str, &str)) -> usize {
///         a.chars().zip(b.chars()).filter(|(x, y)| x == y).count()
///     }
/// }
/// ```
/// 
pub trait SimilarityIO<Input, Output> {
    fn similarity(input: Input) -> Output;
}

/// SimilarityIIO trait for Input0, Input1, Output.
/// 
/// Example: calculate pairwise similarity.
/// 
/// ```no_run
/// pub trait SimilarityIIO<Input0, Input1, Output> {
///     fn similarity(input0: Input0, input1: Input1) -> Output;
/// }
/// ```
/// 
/// Example: calculate similarity of two strings by counting equivalent characters.
/// 
/// ```no_run
/// use similarity_trait::*;
/// 
/// struct S;
/// 
/// impl SimilarityIIO<&str, &str, usize> for S {
///     fn similarity(a: &str, b: &str) -> usize {
///         a.chars().zip(b.chars()).filter(|(x, y)| x == y).count()
///     }
/// }
/// ```
/// 
pub trait SimilarityIIO<Input0, Input1, Output> {
    fn similarity(input0: Input0, input1: Input1) -> Output;
}

/// SimilaritySO trait for Self, Output.
/// 
/// Typical use is to compare a struct's attributes to themselves.
///
pub trait SimilaritySO<Output> {
    fn similarity(&self) -> Output;
}

/// SimilaritySIO trait for Self, Input, Output.
/// 
/// Typical use is to compare self to any other type.
///
/// ```no_run
/// pub trait SimilaritySIO<Input, Output> {
///     fn similarity(&self, input: Input) -> Output;
/// }
/// ```
/// 
/// Example: calculate similarity of two strings by counting equivalent characters.
/// 
/// ```
/// use similarity_trait::*;
/// 
/// struct S {
///     my_string: String
/// }
/// 
/// impl SimilaritySIO<&str, usize> for S {
///     fn similarity(&self, other: &str) -> usize {
///         self.my_string.chars().zip(other.chars()).filter(|(x, y)| x == y).count()
///     }
/// }
/// ```
/// 
pub trait SimilaritySIO<Input, Output> {
    fn similarity(&self, input: Input) -> Output;
}

#[cfg(test)]
mod tests {
    use super::*;
    mod percent_change {
        use super::*;

        mod similarity_io {
            use super::*;
            struct MyStruct {}

            impl SimilarityIO<(i32, i32), Option<f64>> for MyStruct {
                /// Similarity of two numbers via percent change.
                fn similarity(input: (i32, i32)) -> Option<f64> {
                    Some(100.0 * (input.1.checked_sub(input.0)?) as f64) / i32::abs(input.0) as f64)
                }
            }

            #[test]
            fn test() {
                let percent_change = MyStruct::similarity((100, 120));
                assert_eq!(percent_change, Some(20.0));
            }

        }


        mod similarity_iio {
            use super::*;
            struct MyStruct {}

            impl SimilarityIIO<i32, i32, f64> for MyStruct {
                /// Similarity of two numbers via percent change.
                fn similarity(input0: i32, input1: i32) -> f64 {
                    (100.0 * (input1 - input0) as f64) / i32::abs(input0) as f64
                }
            }

            #[test]
            fn test() {
                let percent_change = MyStruct::similarity(100, 120);
                assert_eq!(percent_change, 20.0);
            }

        }

        mod similarity_sio {
            use super::*;

            impl SimilaritySIO<i32, f64> for i32 {
                /// Similarity of two numbers via percent change.
                fn similarity(&self, input: i32) -> f64 {
                    (100.0 * (input - self) as f64) / i32::abs(*self) as f64
                }
            }

            #[test]
            fn test() {
                let percent_change = 100.similarity( 120);
                assert_eq!(percent_change, 20.0);
            }

        }
    }

    mod population_standard_deviation {
        use super::*;

        mod similarity_io {
            use super::*;
            struct MyStruct {}

            impl SimilarityIO<&Vec<f64>, Option<f64>> for MyStruct {
                /// Similarity of numbers via population standard deviation.
                fn similarity(numbers: &Vec<f64>) -> Option<f64> {
                    if numbers.is_empty() { return None }
                    let mean = numbers.iter().sum::<f64>() / numbers.len() as f64;
                    let variance = numbers.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / numbers.len() as f64;
                    Some(variance.sqrt())
                }
            }

            #[test]
            fn test() {
                let numbers = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
                let population_standard_deviation = MyStruct::similarity(&numbers).expect("similarity");
                assert!(population_standard_deviation > 1.999 && population_standard_deviation < 2.001);
            }

        }

    }

    mod hamming_distance_for_a_pair_of_strings {
        use super::*;

        mod similarity_io {
            use super::*;
            struct MyStruct {}

            impl SimilarityIO<(&str, &str), usize> for MyStruct {
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

        mod similarity_sio {
            use super::*;

            impl SimilaritySIO<&str, usize> for str {
                /// Similarity of two strings via Hamming distance.
                fn similarity(&self, input: &str) -> usize {
                    self.chars().zip(input.chars()).filter(|(c1, c2)| c1 != c2).count()
                }
            }

            #[test]
            fn test() {
                let hamming_distance = "information".similarity("informatics");
                assert_eq!(hamming_distance, 2);
            }
        }

    }

    mod hamming_distance_for_a_collection_of_strings {
        use super::*;
        struct MyStruct {}

        impl SimilarityIO<Vec<&str>, usize> for MyStruct {
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
