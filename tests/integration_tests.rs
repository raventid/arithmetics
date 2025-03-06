use rust_decimal::Decimal;
use bigdecimal::BigDecimal;
use decimal::d128;
use std::str::FromStr;

#[test]
fn test_cross_library_arithmetic_consistency() {
    let a_str = "123.456";
    let b_str = "789.012";
    
    // Test addition
    let decimal_sum = Decimal::from_str(a_str).unwrap() + Decimal::from_str(b_str).unwrap();
    let bigdecimal_sum = BigDecimal::from_str(a_str).unwrap() + BigDecimal::from_str(b_str).unwrap();
    let d128_sum = d128::from_str(a_str).unwrap() + d128::from_str(b_str).unwrap();
    
    // All should give same result for this precision
    assert_eq!(decimal_sum.to_string(), "912.468");
    assert_eq!(bigdecimal_sum.to_string(), "912.468");
    assert_eq!(d128_sum.to_string(), "912.468");
    
    // Test multiplication
    let decimal_mult = Decimal::from_str("2.5").unwrap() * Decimal::from_str("4.0").unwrap();
    let bigdecimal_mult = BigDecimal::from_str("2.5").unwrap() * BigDecimal::from_str("4.0").unwrap();
    
    assert_eq!(decimal_mult.to_string(), "10.0");
    assert_eq!(bigdecimal_mult.to_string(), "10.0");
}

#[test]
fn test_precision_preservation() {
    // Test that decimal arithmetic preserves exact values
    let mut sum = Decimal::ZERO;
    for _ in 0..100 {
        sum += Decimal::from_str("0.01").unwrap();
    }
    
    assert_eq!(sum, Decimal::ONE);
    assert_eq!(sum.to_string(), "1.00");
}

#[test]
fn test_large_number_handling() {
    let large_decimal = Decimal::from_str("99999999999999999999999999.999999999999999999999999").unwrap();
    let small_decimal = Decimal::from_str("0.000000000000000000000001").unwrap();
    
    // Should not panic and maintain precision
    let result = large_decimal + small_decimal;
    assert!(result > large_decimal);
}

#[test]
fn test_division_by_small_numbers() {
    let dividend = Decimal::from_str("1.0").unwrap();
    let divisor = Decimal::from_str("0.000001").unwrap();
    
    let result = dividend / divisor;
    assert_eq!(result.to_string(), "1000000");
}

#[test]
fn test_rounding_behavior() {
    use rust_decimal::RoundingStrategy;
    
    let value = Decimal::from_str("1.2345").unwrap();
    
    // Test different rounding strategies
    assert_eq!(value.round_dp(2).to_string(), "1.23");
    assert_eq!(value.round_dp_with_strategy(2, RoundingStrategy::MidpointAwayFromZero).to_string(), "1.23");
}

#[test]
fn test_string_conversion_roundtrip() {
    let test_values = vec![
        "0.1",
        "123.456789",
        "0.000000001",
        "999999999.999999999",
        "1.23456789012345678901234567890",
    ];
    
    for value_str in test_values {
        let decimal = Decimal::from_str(value_str).unwrap();
        let _bigdecimal = BigDecimal::from_str(value_str).unwrap();
        
        // Should be able to parse and convert back (within precision limits)
        assert!(decimal.to_string().starts_with(&value_str[..std::cmp::min(value_str.len(), 10)]));
    }
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
fn test_comparison_operations() {
    let a = Decimal::from_str("1.23").unwrap();
    let b = Decimal::from_str("1.24").unwrap();
    let c = Decimal::from_str("1.23").unwrap();
    
    assert!(a < b);
    assert!(b > a);
    assert_eq!(a, c);
    assert!(a <= b);
    assert!(a <= c);
    assert!(b >= a);
    assert!(c >= a);
}

#[test]
fn test_edge_case_operations() {
    // Test very small additions
    let tiny = Decimal::from_str("0.000000000000000001").unwrap();
    let one = Decimal::ONE;
    let result = one + tiny;
    assert!(result > one);
    
    // Test subtraction resulting in zero
    let value = Decimal::from_str("123.456").unwrap();
    assert_eq!(value - value, Decimal::ZERO);
    
    // Test division resulting in one
    let dividend = Decimal::from_str("987.654").unwrap();
    assert_eq!(dividend / dividend, Decimal::ONE);
}
