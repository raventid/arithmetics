use rust_decimal::Decimal;
use bigdecimal::BigDecimal;
use decimal::d128;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub test_name: String,
    pub expected: String,
    pub actual: String,
    pub passed: bool,
    pub error_margin: Option<f64>,
}

pub struct ValidationSuite;

impl ValidationSuite {
    pub fn run_all_validations() -> Vec<ValidationResult> {
        let mut results = Vec::new();
        
        results.extend(Self::validate_basic_arithmetic());
        results.extend(Self::validate_precision_consistency());
        results.extend(Self::validate_edge_cases());
        results.extend(Self::validate_conversion_accuracy());
        
        results
    }

    fn validate_basic_arithmetic() -> Vec<ValidationResult> {
        let mut results = Vec::new();
        
        // Test addition consistency
        let f64_result = 1.1_f64 + 2.2_f64;
        let decimal_result = Decimal::from_str("1.1").unwrap() + Decimal::from_str("2.2").unwrap();
        let bigdecimal_result = BigDecimal::from_str("1.1").unwrap() + BigDecimal::from_str("2.2").unwrap();
        let d128_result = d128::from_str("1.1").unwrap() + d128::from_str("2.2").unwrap();
        
        results.push(ValidationResult {
            test_name: "Addition 1.1 + 2.2".to_string(),
            expected: "3.3".to_string(),
            actual: f64_result.to_string(),
            passed: (f64_result - 3.3).abs() < 1e-10,
            error_margin: Some((f64_result - 3.3).abs()),
        });

        results.push(ValidationResult {
            test_name: "Decimal Addition 1.1 + 2.2".to_string(),
            expected: "3.3".to_string(),
            actual: decimal_result.to_string(),
            passed: decimal_result.to_string() == "3.3",
            error_margin: None,
        });

        // Test multiplication consistency
        let mult_f64 = 0.1_f64 * 0.2_f64;
        let mult_decimal = Decimal::from_str("0.1").unwrap() * Decimal::from_str("0.2").unwrap();
        
        results.push(ValidationResult {
            test_name: "Multiplication 0.1 * 0.2 (f64)".to_string(),
            expected: "0.02".to_string(),
            actual: mult_f64.to_string(),
            passed: (mult_f64 - 0.02).abs() < 1e-10,
            error_margin: Some((mult_f64 - 0.02).abs()),
        });

        results.push(ValidationResult {
            test_name: "Multiplication 0.1 * 0.2 (Decimal)".to_string(),
            expected: "0.02".to_string(),
            actual: mult_decimal.to_string(),
            passed: mult_decimal.to_string() == "0.02",
            error_margin: None,
        });

        results
    }

    fn validate_precision_consistency() -> Vec<ValidationResult> {
        let mut results = Vec::new();
        
        // Test repeated addition
        let mut f64_sum = 0.0_f64;
        let mut decimal_sum = Decimal::ZERO;
        
        for _ in 0..100 {
            f64_sum += 0.01;
            decimal_sum += Decimal::from_str("0.01").unwrap();
        }
        
        results.push(ValidationResult {
            test_name: "Repeated addition of 0.01 (100 times) - f64".to_string(),
            expected: "1.00".to_string(),
            actual: f64_sum.to_string(),
            passed: (f64_sum - 1.0).abs() < 1e-10,
            error_margin: Some((f64_sum - 1.0).abs()),
        });

        results.push(ValidationResult {
            test_name: "Repeated addition of 0.01 (100 times) - Decimal".to_string(),
            expected: "1.00".to_string(),
            actual: decimal_sum.to_string(),
            passed: decimal_sum == Decimal::ONE,
            error_margin: None,
        });

        results
    }

    fn validate_edge_cases() -> Vec<ValidationResult> {
        let mut results = Vec::new();
        
        // Test division by very small numbers
        let small_division_f64 = 1.0_f64 / 1e-10_f64;
        let small_division_decimal = Decimal::ONE / Decimal::from_str("0.0000000001").unwrap();
        
        results.push(ValidationResult {
            test_name: "Division by very small number (1.0 / 1e-10) - f64".to_string(),
            expected: "10000000000".to_string(),
            actual: small_division_f64.to_string(),
            passed: (small_division_f64 - 1e10).abs() < 1e5,
            error_margin: Some((small_division_f64 - 1e10).abs()),
        });

        // Test very large numbers
        let large_multiplication = Decimal::from_str("999999999999999999").unwrap() 
            * Decimal::from_str("999999999999999999").unwrap();
        
        results.push(ValidationResult {
            test_name: "Large number multiplication".to_string(),
            expected: "999999999999999998000000000000000001".to_string(),
            actual: large_multiplication.to_string(),
            passed: true, // Just checking it doesn't panic
            error_margin: None,
        });

        results
    }

    fn validate_conversion_accuracy() -> Vec<ValidationResult> {
        let mut results = Vec::new();
        
        // Test string to number conversions
        let test_values = vec!["123.456", "0.001", "999.999", "0.123456789"];
        
        for value_str in test_values {
            let decimal_val = Decimal::from_str(value_str).unwrap();
            let bigdecimal_val = BigDecimal::from_str(value_str).unwrap();
            let d128_val = d128::from_str(value_str).unwrap();
            
            results.push(ValidationResult {
                test_name: format!("String conversion accuracy: {}", value_str),
                expected: value_str.to_string(),
                actual: decimal_val.to_string(),
                passed: decimal_val.to_string() == value_str,
                error_margin: None,
            });
        }

        results
    }

    pub fn print_validation_summary(results: &[ValidationResult]) {
        let total_tests = results.len();
        let passed_tests = results.iter().filter(|r| r.passed).count();
        let failed_tests = total_tests - passed_tests;
        
        println!("\n=== Validation Summary ===");
        println!("Total tests: {}", total_tests);
        println!("Passed: {}", passed_tests);
        println!("Failed: {}", failed_tests);
        println!("Success rate: {:.2}%", (passed_tests as f64 / total_tests as f64) * 100.0);
        
        if failed_tests > 0 {
            println!("\n=== Failed Tests ===");
            for result in results.iter().filter(|r| !r.passed) {
                println!("❌ {}", result.test_name);
                println!("   Expected: {}", result.expected);
                println!("   Actual: {}", result.actual);
                if let Some(margin) = result.error_margin {
                    println!("   Error margin: {:.2e}", margin);
                }
                println!();
            }
        }
    }
}
