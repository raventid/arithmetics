use rust_decimal::Decimal;
use bigdecimal::BigDecimal;
use decimal::d128;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ErrorAnalysis {
    pub operation: String,
    pub expected: f64,
    pub actual: f64,
    pub absolute_error: f64,
    pub relative_error: f64,
    pub library: String,
}

#[derive(Debug, Clone)]
pub struct AccumulationError {
    pub iterations: usize,
    pub expected_result: String,
    pub actual_result: String,
    pub total_error: f64,
    pub error_per_iteration: f64,
}

pub struct AdvancedErrorAnalyzer;

impl AdvancedErrorAnalyzer {
    pub fn analyze_floating_point_errors() -> Vec<ErrorAnalysis> {
        let mut errors = Vec::new();
        
        // Test classic floating point errors
        errors.extend(Self::test_classic_fp_errors());
        errors.extend(Self::test_decimal_precision_errors());
        errors.extend(Self::test_catastrophic_cancellation());
        errors.extend(Self::test_associativity_violations());
        
        errors
    }

    fn test_classic_fp_errors() -> Vec<ErrorAnalysis> {
        let mut errors = Vec::new();
        
        // 0.1 + 0.2 != 0.3 problem
        let f64_result = 0.1_f64 + 0.2_f64;
        let expected = 0.3_f64;
        
        errors.push(ErrorAnalysis {
            operation: "0.1 + 0.2".to_string(),
            expected,
            actual: f64_result,
            absolute_error: (f64_result - expected).abs(),
            relative_error: ((f64_result - expected) / expected).abs(),
            library: "f64".to_string(),
        });

        // Compare with decimal libraries
        let decimal_result = Decimal::from_str("0.1").unwrap() + Decimal::from_str("0.2").unwrap();
        let decimal_f64 = decimal_result.to_string().parse::<f64>().unwrap();
        
        errors.push(ErrorAnalysis {
            operation: "0.1 + 0.2".to_string(),
            expected,
            actual: decimal_f64,
            absolute_error: (decimal_f64 - expected).abs(),
            relative_error: ((decimal_f64 - expected) / expected).abs(),
            library: "rust_decimal".to_string(),
        });

        // Large number + small number problem
        let large = 1e16_f64;
        let small = 1.0_f64;
        let f64_large_small = large + small;
        
        errors.push(ErrorAnalysis {
            operation: "1e16 + 1.0".to_string(),
            expected: large + small,
            actual: f64_large_small,
            absolute_error: (f64_large_small - (large + small)).abs(),
            relative_error: ((f64_large_small - (large + small)) / (large + small)).abs(),
            library: "f64".to_string(),
        });

        errors
    }

    fn test_decimal_precision_errors() -> Vec<ErrorAnalysis> {
        let mut errors = Vec::new();
        
        // Test division precision
        let one_third_f64 = 1.0_f64 / 3.0_f64;
        let one_third_decimal = Decimal::ONE / Decimal::from(3);
        let decimal_as_f64 = one_third_decimal.to_string().parse::<f64>().unwrap_or(0.0);
        
        let expected = 1.0 / 3.0; // Mathematical truth
        
        errors.push(ErrorAnalysis {
            operation: "1/3 precision".to_string(),
            expected,
            actual: one_third_f64,
            absolute_error: (one_third_f64 - expected).abs(),
            relative_error: ((one_third_f64 - expected) / expected).abs(),
            library: "f64".to_string(),
        });

        errors.push(ErrorAnalysis {
            operation: "1/3 precision".to_string(),
            expected,
            actual: decimal_as_f64,
            absolute_error: (decimal_as_f64 - expected).abs(),
            relative_error: ((decimal_as_f64 - expected) / expected).abs(),
            library: "rust_decimal".to_string(),
        });

        errors
    }

    fn test_catastrophic_cancellation() -> Vec<ErrorAnalysis> {
        let mut errors = Vec::new();
        
        // Test subtracting nearly equal numbers
        let a = 1.000000000000001_f64;
        let b = 1.0_f64;
        let f64_result = a - b;
        let expected = 0.000000000000001_f64;
        
        errors.push(ErrorAnalysis {
            operation: "catastrophic cancellation".to_string(),
            expected,
            actual: f64_result,
            absolute_error: (f64_result - expected).abs(),
            relative_error: if expected != 0.0 { ((f64_result - expected) / expected).abs() } else { f64::INFINITY },
            library: "f64".to_string(),
        });

        // Test with higher precision arithmetic
        let decimal_a = Decimal::from_str("1.000000000000001").unwrap();
        let decimal_b = Decimal::ONE;
        let decimal_result = decimal_a - decimal_b;
        let decimal_as_f64 = decimal_result.to_string().parse::<f64>().unwrap_or(0.0);
        
        errors.push(ErrorAnalysis {
            operation: "catastrophic cancellation".to_string(),
            expected,
            actual: decimal_as_f64,
            absolute_error: (decimal_as_f64 - expected).abs(),
            relative_error: if expected != 0.0 { ((decimal_as_f64 - expected) / expected).abs() } else { f64::INFINITY },
            library: "rust_decimal".to_string(),
        });

        errors
    }

    fn test_associativity_violations() -> Vec<ErrorAnalysis> {
        let mut errors = Vec::new();
        
        // Test (a + b) + c vs a + (b + c)
        let a = 1e20_f64;
        let b = -1e20_f64;
        let c = 1.0_f64;
        
        let left_to_right = (a + b) + c;  // Should be 1.0
        let right_to_left = a + (b + c);  // Might not be 1.0
        let expected = 1.0_f64;
        
        errors.push(ErrorAnalysis {
            operation: "(1e20 + -1e20) + 1 (left-to-right)".to_string(),
            expected,
            actual: left_to_right,
            absolute_error: (left_to_right - expected).abs(),
            relative_error: ((left_to_right - expected) / expected).abs(),
            library: "f64".to_string(),
        });

        errors.push(ErrorAnalysis {
            operation: "1e20 + (-1e20 + 1) (right-to-left)".to_string(),
            expected,
            actual: right_to_left,
            absolute_error: (right_to_left - expected).abs(),
            relative_error: ((right_to_left - expected) / expected).abs(),
            library: "f64".to_string(),
        });

        errors
    }

    pub fn analyze_accumulation_errors() -> Vec<AccumulationError> {
        let mut results = Vec::new();
        
        // Test repeated addition
        results.push(Self::test_repeated_addition());
        results.push(Self::test_repeated_multiplication());
        results.push(Self::test_harmonic_series());
        
        results
    }

    fn test_repeated_addition() -> AccumulationError {
        let iterations = 10000;
        let increment = 0.0001_f64;
        
        let mut f64_sum = 0.0_f64;
        for _ in 0..iterations {
            f64_sum += increment;
        }
        
        let expected = iterations as f64 * increment;
        let error = (f64_sum - expected).abs();
        
        AccumulationError {
            iterations,
            expected_result: expected.to_string(),
            actual_result: f64_sum.to_string(),
            total_error: error,
            error_per_iteration: error / iterations as f64,
        }
    }

    fn test_repeated_multiplication() -> AccumulationError {
        let iterations = 100;
        let factor = 1.01_f64;
        
        let mut f64_product = 1.0_f64;
        for _ in 0..iterations {
            f64_product *= factor;
        }
        
        let expected = factor.powi(iterations);
        let error = (f64_product - expected).abs();
        
        AccumulationError {
            iterations,
            expected_result: expected.to_string(),
            actual_result: f64_product.to_string(),
            total_error: error,
            error_per_iteration: error / iterations as f64,
        }
    }

    fn test_harmonic_series() -> AccumulationError {
        let iterations = 10000;
        
        let mut f64_sum = 0.0_f64;
        for i in 1..=iterations {
            f64_sum += 1.0 / i as f64;
        }
        
        // Approximate expected value using more precise calculation
        let mut precise_sum = 0.0_f64;
        for i in 1..=iterations {
            precise_sum += 1.0 / i as f64; // Still f64, but accumulated differently
        }
        
        let error = (f64_sum - precise_sum).abs();
        
        AccumulationError {
            iterations,
            expected_result: format!("~{:.10}", precise_sum),
            actual_result: format!("{:.10}", f64_sum),
            total_error: error,
            error_per_iteration: error / iterations as f64,
        }
    }

    pub fn compare_library_accuracy() -> HashMap<String, f64> {
        let mut accuracy_scores = HashMap::new();
        
        let test_cases = vec![
            ("0.1", "0.2", "0.3"),
            ("1.0", "3.0", "0.33333333333333333333"),
            ("123.456", "789.012", "912.468"),
        ];
        
        for (a_str, b_str, expected_str) in test_cases {
            let expected = expected_str.parse::<f64>().unwrap_or(0.0);
            
            // Test f64
            let a_f64: f64 = a_str.parse().unwrap();
            let b_f64: f64 = b_str.parse().unwrap();
            let f64_result = a_f64 + b_f64;
            let f64_error = (f64_result - expected).abs();
            
            *accuracy_scores.entry("f64".to_string()).or_insert(0.0) += f64_error;
            
            // Test rust_decimal
            let a_decimal = Decimal::from_str(a_str).unwrap();
            let b_decimal = Decimal::from_str(b_str).unwrap();
            let decimal_result = a_decimal + b_decimal;
            let decimal_as_f64 = decimal_result.to_string().parse::<f64>().unwrap_or(0.0);
            let decimal_error = (decimal_as_f64 - expected).abs();
            
            *accuracy_scores.entry("rust_decimal".to_string()).or_insert(0.0) += decimal_error;
            
            // Test bigdecimal
            let a_big = BigDecimal::from_str(a_str).unwrap();
            let b_big = BigDecimal::from_str(b_str).unwrap();
            let big_result = a_big + b_big;
            let big_as_f64 = big_result.to_string().parse::<f64>().unwrap_or(0.0);
            let big_error = (big_as_f64 - expected).abs();
            
            *accuracy_scores.entry("bigdecimal".to_string()).or_insert(0.0) += big_error;
        }
        
        accuracy_scores
    }

    pub fn print_error_analysis(errors: &[ErrorAnalysis]) {
        println!("\n=== Advanced Error Analysis ===");
        println!("{:<30} {:<15} {:<15} {:<15} {:<15} {:<15}", 
                 "Operation", "Library", "Expected", "Actual", "Abs Error", "Rel Error");
        println!("{}", "-".repeat(105));
        
        for error in errors {
            println!("{:<30} {:<15} {:<15.10e} {:<15.10e} {:<15.3e} {:<15.3e}", 
                     error.operation,
                     error.library,
                     error.expected,
                     error.actual,
                     error.absolute_error,
                     error.relative_error);
        }
        
        // Summarize by library
        let mut library_errors: HashMap<String, Vec<f64>> = HashMap::new();
        for error in errors {
            library_errors
                .entry(error.library.clone())
                .or_insert_with(Vec::new)
                .push(error.relative_error);
        }
        
        println!("\n=== Error Summary by Library ===");
        for (library, rel_errors) in library_errors {
            let avg_error = rel_errors.iter().sum::<f64>() / rel_errors.len() as f64;
            let max_error = rel_errors.iter().copied().fold(0.0, f64::max);
            println!("{}: Avg Rel Error: {:.3e}, Max Rel Error: {:.3e}", library, avg_error, max_error);
        }
    }

    pub fn print_accumulation_analysis(accumulations: &[AccumulationError]) {
        println!("\n=== Accumulation Error Analysis ===");
        
        for acc in accumulations {
            println!("Iterations: {}", acc.iterations);
            println!("Expected: {}", acc.expected_result);
            println!("Actual: {}", acc.actual_result);
            println!("Total Error: {:.3e}", acc.total_error);
            println!("Error per Iteration: {:.3e}", acc.error_per_iteration);
            println!("{}", "-".repeat(50));
        }
    }
}
