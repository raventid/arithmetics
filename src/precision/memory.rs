use rust_decimal::Decimal;
use bigdecimal::BigDecimal;
use fixed::types::I32F32;
use half::f16;
use decimal::d128;
use std::mem;

/// Memory usage analysis for different arithmetic types
pub struct MemoryAnalyzer;

impl MemoryAnalyzer {
    /// Analyze memory footprint of different number types
    pub fn analyze_memory_footprint() {
        println!("🧠 Memory Analysis: Type Size Comparison");
        println!("========================================");
        
        println!("f64:         {} bytes", mem::size_of::<f64>());
        println!("f32:         {} bytes", mem::size_of::<f32>());
        println!("f16:         {} bytes", mem::size_of::<f16>());
        println!("I32F32:      {} bytes", mem::size_of::<I32F32>());
        println!("Decimal:     {} bytes", mem::size_of::<Decimal>());
        println!("BigDecimal:  {} bytes", mem::size_of::<BigDecimal>());
        println!("D128:        {} bytes", mem::size_of::<d128>());
        
        // Test with arrays to see impact
        const ARRAY_SIZE: usize = 1000;
        
        println!("\nArray of {} elements:", ARRAY_SIZE);
        println!("f64 array:        {} KB", (mem::size_of::<f64>() * ARRAY_SIZE) / 1024);
        println!("f32 array:        {} KB", (mem::size_of::<f32>() * ARRAY_SIZE) / 1024);
        println!("f16 array:        {} KB", (mem::size_of::<f16>() * ARRAY_SIZE) / 1024);
        println!("I32F32 array:     {} KB", (mem::size_of::<I32F32>() * ARRAY_SIZE) / 1024);
        println!("Decimal array:    {} KB", (mem::size_of::<Decimal>() * ARRAY_SIZE) / 1024);
        println!("BigDecimal array: {} KB", (mem::size_of::<BigDecimal>() * ARRAY_SIZE) / 1024);
        println!("D128 array:       {} KB", (mem::size_of::<d128>() * ARRAY_SIZE) / 1024);
    }
    
    /// Analyze allocation patterns
    pub fn analyze_allocation_patterns() {
        println!("\n🧠 Memory Analysis: Stack vs Heap Allocation");
        println!("===========================================");
        
        println!("Stack allocated (fixed size):");
        println!("  ✅ f64, f32, f16, I32F32, Decimal, D128");
        
        println!("\nPotentially heap allocated (variable size):");
        println!("  ⚠️  BigDecimal (may allocate for large numbers)");
        
        println!("\nRecommendations:");
        println!("  • Use f32/f64 for cache-friendly numerical computing");
        println!("  • Use f16 for memory-constrained applications");
        println!("  • Use Decimal for financial calculations");
        println!("  • Use BigDecimal only when arbitrary precision is required");
    }
}