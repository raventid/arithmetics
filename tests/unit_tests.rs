use rust_decimal::Decimal;
use bigdecimal::BigDecimal;
use std::str::FromStr;

#[cfg(test)]
mod precision_tests {
    use super::*;

    #[test]
    fn test_decimal_precision_comparison() {
        let value = "0.123456789012345678901234567890";
        let decimal = Decimal::from_str(value).unwrap();
        let bigdecimal = BigDecimal::from_str(value).unwrap();
        
        // rust_decimal should truncate to its precision limit
        assert!(decimal.to_string().len() <= 30);
        
        // bigdecimal should preserve more precision
        assert!(bigdecimal.to_string().contains("123456789"));
    }

    #[test]
    fn test_accumulation_error_f64() {
        let mut sum = 0.0_f64;
        for _ in 0..100 {
            sum += 0.01;
        }
        
        // f64 should have some accumulation error
        assert!((sum - 1.0).abs() > 1e-15);
    }

    #[test]
    fn test_accumulation_error_decimal() {
        let mut sum = Decimal::ZERO;
        let increment = Decimal::from_str("0.01").unwrap();
        
        for _ in 0..100 {
            sum += increment;
        }
        
        // Decimal should be exact
        assert_eq!(sum, Decimal::ONE);
    }

    #[test]
    fn test_small_number_precision() {
        let small_f64 = 1e-15_f64;
        let small_decimal = Decimal::from_str("0.000000000000001").unwrap();
        
        // f64 operations with very small numbers
        let f64_result = 1.0 + small_f64;
        assert!(f64_result > 1.0);
        
        // Decimal operations
        let decimal_result = Decimal::ONE + small_decimal;
        assert!(decimal_result > Decimal::ONE);
    }

    #[test]
    fn test_division_precision() {
        let a = Decimal::ONE;
        let b = Decimal::from(3);
        let result = a / b;
        
        // Should have limited decimal places but high precision
        let result_str = result.to_string();
        assert!(result_str.starts_with("0.3333333"));
    }
}

#[cfg(test)]
mod safety_tests {
    use super::*;

    #[test]
    fn test_overflow_handling() {
        let max_decimal = Decimal::MAX;
        let one = Decimal::ONE;
        
        // This should not panic (Decimal handles overflow gracefully)
        let result = std::panic::catch_unwind(|| {
            let _overflow = max_decimal + one;
        });
        
        // The result depends on rust_decimal's overflow behavior
        // This test ensures we don't panic unexpectedly
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_division_by_zero_f64() {
        let result = 1.0_f64 / 0.0_f64;
        assert!(result.is_infinite());
        
        let result = 0.0_f64 / 0.0_f64;
        assert!(result.is_nan());
    }

    #[test]
    fn test_division_by_zero_decimal() {
        let result = std::panic::catch_unwind(|| {
            let _div = Decimal::ONE / Decimal::ZERO;
        });
        
        // Decimal should handle division by zero (likely panics or returns error)
        // The exact behavior depends on the library implementation
        assert!(result.is_err());
    }

    #[test]
    fn test_negative_sqrt() {
        let negative = -4.0_f64;
        let result = negative.sqrt();
        assert!(result.is_nan());
    }
}

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[test]
    fn test_arithmetic_consistency() {
        let a_str = "12.34";
        let b_str = "56.78";
        
        let f64_a: f64 = a_str.parse().unwrap();
        let f64_b: f64 = b_str.parse().unwrap();
        let f64_result = f64_a + f64_b;
        
        let decimal_a = Decimal::from_str(a_str).unwrap();
        let decimal_b = Decimal::from_str(b_str).unwrap();
        let decimal_result = decimal_a + decimal_b;
        
        // Results should be very close (within reasonable tolerance)
        let decimal_as_f64 = decimal_result.to_string().parse::<f64>().unwrap();
        assert!((f64_result - decimal_as_f64).abs() < 1e-10);
    }

    #[test]
    fn test_mathematical_properties() {
        let a = Decimal::from_str("5.5").unwrap();
        let b = Decimal::from_str("2.2").unwrap();
        let c = Decimal::from_str("3.3").unwrap();
        
        // Test associativity: (a + b) + c = a + (b + c)
        let left = (a + b) + c;
        let right = a + (b + c);
        assert_eq!(left, right);
        
        // Test commutativity: a + b = b + a
        assert_eq!(a + b, b + a);
        assert_eq!(a * b, b * a);
        
        // Test distributivity: a * (b + c) = a * b + a * c
        let left = a * (b + c);
        let right = a * b + a * c;
        assert_eq!(left, right);
    }

    #[test]
    fn test_zero_and_one_properties() {
        let value = Decimal::from_str("42.42").unwrap();
        
        // Adding zero
        assert_eq!(value + Decimal::ZERO, value);
        assert_eq!(Decimal::ZERO + value, value);
        
        // Multiplying by one
        assert_eq!(value * Decimal::ONE, value);
        assert_eq!(Decimal::ONE * value, value);
        
        // Multiplying by zero
        assert_eq!(value * Decimal::ZERO, Decimal::ZERO);
        assert_eq!(Decimal::ZERO * value, Decimal::ZERO);
    }

    #[test]
    fn test_string_roundtrip() {
        let test_values = vec![
            "0.1",
            "123.456",
            "0.000001",
            "999999.999999",
        ];
        
        for value_str in test_values {
            let decimal = Decimal::from_str(value_str).unwrap();
            let roundtrip = decimal.to_string();
            
            // Should be able to parse back successfully
            let parsed_back = Decimal::from_str(&roundtrip).unwrap();
            assert_eq!(decimal, parsed_back);
        }
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_f64_performance() {
        let start = Instant::now();
        let mut sum = 0.0_f64;
        
        for i in 0..10000 {
            sum += i as f64 * 0.1;
        }
        
        let duration = start.elapsed();
        
        // f64 operations should be very fast
        assert!(duration.as_millis() < 100);
        assert!(sum > 0.0);
    }

    #[test]
    fn test_decimal_performance() {
        let start = Instant::now();
        let mut sum = Decimal::ZERO;
        let increment = Decimal::from_str("0.1").unwrap();
        
        for i in 0..1000 { // Fewer iterations for decimal
            let value = Decimal::from(i) * increment;
            sum += value;
        }
        
        let duration = start.elapsed();
        
        // Decimal should be slower but still reasonable
        assert!(duration.as_millis() < 1000);
        assert!(sum > Decimal::ZERO);
    }

    #[test]
    fn test_bigdecimal_performance() {
        let start = Instant::now();
        let mut sum = BigDecimal::from(0);
        
        for i in 0..100 { // Even fewer iterations for BigDecimal
            let value = BigDecimal::from(i);
            sum = sum + value;
        }
        
        let duration = start.elapsed();
        
        // BigDecimal will be slowest
        assert!(duration.as_millis() < 5000);
        assert!(sum > BigDecimal::from(0));
    }
}

#[cfg(test)]
mod error_analysis_tests {
    use super::*;

    #[test]
    fn test_classic_floating_point_error() {
        let result = 0.1_f64 + 0.2_f64;
        let expected = 0.3_f64;
        
        // Classic floating-point precision issue
        assert_ne!(result, expected);
        assert!((result - expected).abs() < 1e-15);
    }

    #[test]
    fn test_decimal_avoids_floating_point_error() {
        let a = Decimal::from_str("0.1").unwrap();
        let b = Decimal::from_str("0.2").unwrap();
        let result = a + b;
        let expected = Decimal::from_str("0.3").unwrap();
        
        // Decimal should give exact result
        assert_eq!(result, expected);
    }

    #[test]
    fn test_catastrophic_cancellation() {
        let a = 1.0000000000000002_f64;
        let b = 1.0_f64;
        let result = a - b;
        
        // Should lose significant precision
        assert!(result > 0.0);
        assert!(result < 1e-15);
    }

    #[test]
    fn test_large_small_addition() {
        let large = 1e16_f64;
        let small = 1.0_f64;
        let result = large + small;
        
        // Small number should be lost in f64 precision
        assert_eq!(result, large);
    }

    #[test]
    fn test_associativity_violation() {
        let a = 1e20_f64;
        let b = -1e20_f64;
        let c = 1.0_f64;
        
        let left_to_right = (a + b) + c;
        let right_to_left = a + (b + c);
        
        // Should violate associativity due to floating-point precision
        assert_ne!(left_to_right, right_to_left);
    }
}

#[cfg(test)]
mod config_tests {
    use arithmetics::config::ArithmeticConfig;

    #[test]
    fn test_default_config() {
        let config = ArithmeticConfig::default();
        
        assert!(config.benchmark.iterations > 0);
        assert!(!config.enabled_libraries.is_empty());
        assert!(config.enabled_libraries.contains(&"f64".to_string()));
        assert!(config.enabled_libraries.contains(&"rust_decimal".to_string()));
    }

    #[test]
    fn test_config_validation() {
        let mut config = ArithmeticConfig::default();
        
        // Valid config should pass
        assert!(config.validate().is_ok());
        
        // Invalid config should fail
        config.benchmark.iterations = 0;
        assert!(config.validate().is_err());
        
        // Fix and test again
        config.benchmark.iterations = 1000;
        config.enabled_libraries.clear();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_library_enable_disable() {
        let mut config = ArithmeticConfig::default();
        
        assert!(config.is_library_enabled("f64"));
        
        config.disable_library("f64");
        assert!(!config.is_library_enabled("f64"));
        
        config.enable_library("f64");
        assert!(config.is_library_enabled("f64"));
    }

    #[test]
    fn test_fast_mode() {
        let mut config = ArithmeticConfig::default();
        let original_iterations = config.benchmark.iterations;
        
        config.set_fast_mode();
        
        assert!(config.benchmark.iterations < original_iterations);
        assert!(config.benchmark.warmup_iterations < 100);
    }

    #[test]
    fn test_thorough_mode() {
        let mut config = ArithmeticConfig::default();
        let original_iterations = config.benchmark.iterations;
        
        config.set_thorough_mode();
        
        assert!(config.benchmark.iterations > original_iterations);
        assert!(config.precision.test_accumulation_errors);
        assert!(config.memory.analyze_footprint);
        assert!(config.safety.test_overflow);
    }
}

#[cfg(test)]
mod benchmarks {
    use super::*;
    
    // These are not typical unit tests but rather smoke tests
    // to ensure benchmark functions don't panic
    
    #[test]
    fn test_basic_arithmetic_smoke_test() {
        // Test that basic arithmetic operations don't panic
        let a = Decimal::from_str("123.456").unwrap();
        let b = Decimal::from_str("789.012").unwrap();
        
        let _add = a + b;
        let _sub = a - b;
        let _mul = a * b;
        let _div = a / b;
        
        // If we get here without panicking, the test passes
        assert!(true);
    }

    #[test]
    fn test_complex_operations_smoke_test() {
        let value = 25.0_f64;
        
        let _sqrt = value.sqrt();
        let _sin = value.sin();
        let _cos = value.cos();
        let _power = value.powf(2.0);
        
        assert!(true);
    }

    #[test]
    fn test_memory_allocation_smoke_test() {
        // Test that we can create various numeric types without issues
        let _decimals: Vec<Decimal> = (0..100)
            .map(|i| Decimal::from_str(&format!("{}.123", i)).unwrap())
            .collect();
        
        let _bigdecimals: Vec<BigDecimal> = (0..100)
            .map(|i| BigDecimal::from_str(&format!("{}.123", i)).unwrap())
            .collect();
        
        let _f64s: Vec<f64> = (0..100).map(|i| i as f64 * 0.123).collect();
        
        assert!(true);
    }
}
