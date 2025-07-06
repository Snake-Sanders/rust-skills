//! # Grinder - Focused Rust Exercises
//! 
//! This crate contains focused exercises to practice specific Rust concepts in depth.
//! Each module focuses on a particular topic with progressive difficulty.

pub mod iterators {
    //! Iterator exercises focusing on transformation, filtering, and collection patterns.

    /// **Exercise 1: Data Transformation Pipeline**
    /// 
    /// **Goal:** Practice chaining iterator methods to transform and filter data.
    /// 
    /// **Requirements:**
    /// - Take a vector of integers
    /// - Filter out negative numbers
    /// - Double each remaining number
    /// - Keep only numbers greater than 10
    /// - Collect into a new vector
    /// 
    /// **Learning Objectives:**
    /// - Understand iterator chaining
    /// - Practice filter() and map() operations
    /// - Learn when to collect() vs iterate lazily
    /// 
    /// **Example:**
    /// Input: vec![-2, 1, 3, -5, 8, 15, 0]
    /// Output: vec![16, 30] // (8*2=16, 15*2=30, others filtered out)
    pub fn transform_pipeline(numbers: Vec<i32>) -> Vec<i32> {
        // TODO: Implement this function
        // Hint: Use iter(), filter(), map(), and collect()
        todo!("Implement the transformation pipeline")
    }

    /// **Exercise 2: Statistical Analysis**
    /// 
    /// **Goal:** Practice reduction operations and statistical calculations using iterators.
    /// 
    /// **Requirements:**
    /// - Calculate the sum, count, and average of positive numbers in a slice
    /// - Return None if there are no positive numbers
    /// - Return Some((sum, count, average)) otherwise
    /// 
    /// **Learning Objectives:**
    /// - Understand fold() and reduce() operations
    /// - Practice working with Option types in iterator context
    /// - Learn to compute multiple statistics in one pass
    /// 
    /// **Example:**
    /// Input: &[1, -2, 3, 4, -5, 6]
    /// Output: Some((14, 4, 3.5)) // sum=14, count=4, avg=3.5
    /// 
    /// Input: &[-1, -2, -3]
    /// Output: None // no positive numbers
    pub fn analyze_positive_numbers(numbers: &[i32]) -> Option<(i32, usize, f64)> {
        // TODO: Implement this function
        // Hint: Filter positives first, then use fold() or collect and calculate
        todo!("Implement statistical analysis")
    }
}

#[cfg(test)]
mod tests {
    use super::iterators::*;

    mod transform_pipeline_tests {
        use super::*;

        #[test]
        fn test_basic_transformation() {
            let input = vec![-2, 1, 3, -5, 8, 15, 0];
            let result = transform_pipeline(input);
            assert_eq!(result, vec![16, 30]);
        }

        #[test]
        fn test_all_negative_numbers() {
            let input = vec![-1, -2, -3, -4];
            let result = transform_pipeline(input);
            assert_eq!(result, vec![]);
        }

        #[test]
        fn test_all_positive_but_small() {
            let input = vec![1, 2, 3, 4]; // After doubling: [2, 4, 6, 8], all <= 10
            let result = transform_pipeline(input);
            assert_eq!(result, vec![]);
        }

        #[test]
        fn test_mixed_with_zeros() {
            let input = vec![0, 5, 0, 10, -1]; // After processing: 0->0, 5->10, 0->0, 10->20, -1 filtered
            let result = transform_pipeline(input);
            assert_eq!(result, vec![20]); // Only 10*2=20 > 10
        }

        #[test]
        fn test_empty_vector() {
            let input = vec![];
            let result = transform_pipeline(input);
            assert_eq!(result, vec![]);
        }

        #[test]
        fn test_large_numbers() {
            let input = vec![50, 100, -50];
            let result = transform_pipeline(input);
            assert_eq!(result, vec![100, 200]); // 50*2=100, 100*2=200
        }
    }

    mod analyze_positive_numbers_tests {
        use super::*;

        #[test]
        fn test_mixed_positive_and_negative() {
            let input = &[1, -2, 3, 4, -5, 6];
            let result = analyze_positive_numbers(input);
            assert_eq!(result, Some((14, 4, 3.5))); // sum=1+3+4+6=14, count=4, avg=14/4=3.5
        }

        #[test]
        fn test_all_negative() {
            let input = &[-1, -2, -3];
            let result = analyze_positive_numbers(input);
            assert_eq!(result, None);
        }

        #[test]
        fn test_all_positive() {
            let input = &[2, 4, 6, 8];
            let result = analyze_positive_numbers(input);
            assert_eq!(result, Some((20, 4, 5.0))); // sum=20, count=4, avg=5.0
        }

        #[test]
        fn test_single_positive() {
            let input = &[5];
            let result = analyze_positive_numbers(input);
            assert_eq!(result, Some((5, 1, 5.0)));
        }

        #[test]
        fn test_with_zeros() {
            let input = &[0, 1, 0, 2, 0];
            let result = analyze_positive_numbers(input);
            assert_eq!(result, Some((3, 2, 1.5))); // Only 1 and 2 are positive
        }

        #[test]
        fn test_empty_slice() {
            let input = &[];
            let result = analyze_positive_numbers(input);
            assert_eq!(result, None);
        }

        #[test]
        fn test_large_numbers() {
            let input = &[100, 200, -50, 300];
            let result = analyze_positive_numbers(input);
            assert_eq!(result, Some((600, 3, 200.0))); // sum=600, count=3, avg=200.0
        }

        #[test]
        fn test_fractional_average() {
            let input = &[1, 2, 3];
            let result = analyze_positive_numbers(input);
            assert_eq!(result, Some((6, 3, 2.0))); // sum=6, count=3, avg=2.0
        }
    }
} 
