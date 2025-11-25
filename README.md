# Similarity trait Rust crate

**[documentation](https://docs.rs/similarity-trait/)**
•
**[source](https://github.com/sixarm/similarity-trait-rust-crate/)**
•
**[llms.txt](https://raw.githubusercontent.com/sixarm/similarity-trait/refs/heads/main/llms.txt)**
•
**[crate](https://crates.io/crates/similarity-trait)**
•
**[email](mailto:joel@joelparkerhenderson.com)**

The Similarity trait defines one function with one input and one output, so
you can compare any kinds of input values and return any kind of output
value.

We use this trait in our programs to create multiple kinds of similarity
functionality, such as for trying various similarity algorithms that we want
to use with the same input type and same output type.

For examples, please see the directory [`examples`](examples).

## Similarity of a pair

One way to use this trait is to calculate the similarity of a pair of
values, such as two numbers, or two strings, or two images.

This is sometimes known as pairwise similarity or pair matching.

Example: given two numbers, then return the percent change.

```rust
use similarity_trait::*;
struct MyStruct;

impl SimilarityIO<(i32, i32), Option<i32>> for MyStruct {
    fn similarity(input: (i32, i32)) -> Option<i32> {
        Some(input.1.checked_sub(input.0)?.abs())
    }
}

let absolute_difference = MyStruct::similarity((100, 120));
assert_eq!(absolute_difference, Some(20));
```

## Similarity of a collection

One way to use this trait is to calculate the similarity of a collection of
values, such as an array of numbers, or vector of strings, or set of images.

This is sometimes called intra-group similarity or statistical correlation.

Example: given numbers, then return the population standard deviation.

```rust
use similarity_trait::SimilarityIO;
struct MyStruct;

impl SimilarityIO<&Vec<f64>, Option<f64>> for MyStruct {
    /// Similarity of numbers via population standard deviation
    fn similarity(numbers: &Vec<f64>) -> Option<f64> {
        if numbers.is_empty() { return None }
        let mean = numbers.iter().sum::<f64>() / numbers.len() as f64;
        let variance = numbers.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / numbers.len() as f64;
        Some(variance.sqrt())
    }
}

let numbers = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
let population_standard_deviation = MyStruct::similarity(&numbers).expect("similarity");
assert!(population_standard_deviation > 1.999 && population_standard_deviation < 2.001);
```

For examples, please see the directory [`examples`](examples).

## Similarity of a pair or a collection

You may want to choose whether you prefer to calculate the similarity of a
pair (such as two strings) or a collection (such as a vector of strings).

Example: given a pair of strings, then return the Hamming distance.

```rust
use similarity_trait::SimilarityIO;
struct MyStruct;

impl SimilarityIO<(&str, &str), usize> for MyStruct {
    /// Similarity of a pair of strings via Hamming distance.
    fn similarity(pair: (&str, &str)) -> usize {
        pair.0.chars().zip(pair.1.chars()).filter(|(c1, c2)| c1 != c2).count()
    }
}

let pair = ("information", "informatics");
let hamming_distance = MyStruct::similarity(pair);
assert_eq!(hamming_distance, 2);
```

Example: given a collection of strings, then return the maximum Hamming
distance.

```rust
use similarity_trait::SimilarityIO;
struct MyStruct;

impl SimilarityIO<Vec<&str>, usize> for MyStruct {
    /// Similarity of a collection of strings via maximum Hamming distance.
    fn similarity(collection: Vec<&str>) -> usize {
        let mut max = 0;
        for i in 0..collection.len() {
            for j in (i + 1)..collection.len() {
                max = std::cmp::max(max, collection[i].chars().zip(collection[j].chars()).filter(|(c1, c2)| c1 != c2).count())
            }
        }
        max
    }
}

let collection = vec!["information", "informatics", "affirmation"];
let maximum_hamming_distance = MyStruct::similarity(collection);
assert_eq!(maximum_hamming_distance, 5);
```

## How to learn more

Wikipedia links:

- [Item-item collaborative filtering](https://en.wikipedia.org/wiki/Item-item_collaborative_filtering)

- [Edit distance](https://en.wikipedia.org/wiki/Edit_distance)

- [Hamming distance](https://en.wikipedia.org/wiki/Hamming_distance)

- [Levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance)

- [Paired difference test](https://en.wikipedia.org/wiki/Paired_difference_test)

- [Cosine similarity](https://en.wikipedia.org/wiki/Cosine_similarity)

- [Euclidean_distance](https://en.wikipedia.org/wiki/Euclidean_distance)

- [Correlation coefficient](https://en.wikipedia.org/wiki/Correlation_coefficient)

- [Intraclass correlation](https://en.wikipedia.org/wiki/Intraclass_correlation)

- [Rank correlation](https://en.wikipedia.org/wiki/Rank_correlation)

- [Polychoric correlation](https://en.wikipedia.org/wiki/Polychoric_correlation)

- [Goodman and Kruskal's gamma](https://en.wikipedia.org/wiki/Goodman_and_Kruskal%27s_gamma)

- [Pearson correlation coefficient](https://en.wikipedia.org/wiki/Pearson_correlation_coefficient) also known as product-moment correlation coefficient.

- [Jaccard index](https://en.wikipedia.org/wiki/Jaccard_index) also known as coefficient of community, intersection over union, ratio of verification, critical success index, Tanimoto index.

Similarity research papers about patient matching:

- [Patient Matching within a Health Information Exchange](https://pmc.ncbi.nlm.nih.gov/articles/PMC4696093/)

- [Patient Identification Techniques – Approaches, Implications, and Findings](https://pmc.ncbi.nlm.nih.gov/articles/PMC7442501/)

- [Patient Matching, Aggregation, and Linking (PMAL) Project](https://www.healthit.gov/sites/default/files/page/2019-09/PMAL%20Final%20Report-08162019v2.pdf)
