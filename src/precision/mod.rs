//! Precision analysis module for comparing arithmetic accuracy

pub mod memory;

/// Precision analysis struct for testing arithmetic accuracy
pub struct PrecisionAnalyzer;

impl PrecisionAnalyzer {
    /// Test accumulation error across different arithmetic types
    pub fn analyze_accumulation_error() {
        println!("🔍 Precision Analysis: Accumulation Error Test");
        println!("==============================================");
        
        let iterations = 10000;
        let value = 0.1;
        
        // f64 accumulation
        let mut f64_sum = 0.0f64;
        for _ in 0..iterations {
            f64_sum += value;
        }
        
        println!("f64 accumulation:     {:.10}", f64_sum);
        println!("Expected result:      {:.10}", value * iterations as f64);
        println!("f64 error:            {:.2e}", (f64_sum - (value * iterations as f64)).abs());
        
        // More types to be added...
        println!("📊 Analysis: f64 shows typical floating-point accumulation errors");
    }
    
    /// Test precision with very small numbers
    pub fn analyze_small_number_precision() {
        println!("\n🔍 Precision Analysis: Small Number Handling");
        println!("===========================================");
        
        let small_value = 1e-15;
        
        println!("Testing with value: {:.2e}", small_value);
        println!("f64 representation: {:.20}", small_value);
        
        // Test arithmetic with small numbers
        let result = small_value + small_value;
        println!("f64 addition result: {:.20}", result);
    }
}