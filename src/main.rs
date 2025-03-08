use rust_decimal::Decimal;
use bigdecimal::BigDecimal;
use std::str::FromStr;

mod precision;
mod safety;
mod validation;
mod profiling;
mod error_analysis;
use precision::{PrecisionAnalyzer, memory::MemoryAnalyzer};
use safety::SafetyAnalyzer;
use validation::ValidationSuite;
use profiling::PerformanceProfiler;
use error_analysis::AdvancedErrorAnalyzer;

fn main() {
    println!("Arithmetic Libraries Comparison Project");
    println!("======================================");
    println!();
    
    // Run validation tests first
    println!("=== Running Validation Tests ===");
    let validation_results = ValidationSuite::run_all_validations();
    ValidationSuite::print_validation_summary(&validation_results);
    println!();
    
    // Example calculations with different libraries
    let num1_str = "123.456789";
    let num2_str = "987.654321";
    
    // f64 calculation
    let f64_a: f64 = num1_str.parse().unwrap();
    let f64_b: f64 = num2_str.parse().unwrap();
    println!("f64:         {} + {} = {}", f64_a, f64_b, f64_a + f64_b);
    
    // rust_decimal calculation
    let decimal_a = Decimal::from_str(num1_str).unwrap();
    let decimal_b = Decimal::from_str(num2_str).unwrap();
    println!("rust_decimal: {} + {} = {}", decimal_a, decimal_b, decimal_a + decimal_b);
    
    // bigdecimal calculation
    let big_a = BigDecimal::from_str(num1_str).unwrap();
    let big_b = BigDecimal::from_str(num2_str).unwrap();
    println!("bigdecimal:   {} + {} = {}", big_a, big_b, &big_a + &big_b);
    
    println!();
    
    // Run precision analysis
    PrecisionAnalyzer::analyze_accumulation_error();
    PrecisionAnalyzer::analyze_small_number_precision();
    
    // Run memory analysis
    MemoryAnalyzer::analyze_memory_footprint();
    MemoryAnalyzer::analyze_allocation_patterns();
    
    // Run safety analysis
    SafetyAnalyzer::test_overflow_detection();
    SafetyAnalyzer::test_division_by_zero();
    SafetyAnalyzer::test_precision_loss();
    
    // Run performance profiling
    println!("\n=== Performance Profiling ===");
    let performance_metrics = PerformanceProfiler::run_comprehensive_profiling();
    PerformanceProfiler::print_performance_report(&performance_metrics);
    PerformanceProfiler::benchmark_memory_allocation_speed();
    
    // Run advanced error analysis
    println!("\n=== Advanced Error Analysis ===");
    let error_analyses = AdvancedErrorAnalyzer::analyze_floating_point_errors();
    AdvancedErrorAnalyzer::print_error_analysis(&error_analyses);
    
    let accumulation_errors = AdvancedErrorAnalyzer::analyze_accumulation_errors();
    AdvancedErrorAnalyzer::print_accumulation_analysis(&accumulation_errors);
}