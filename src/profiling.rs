use std::time::{Duration, Instant};
use std::collections::HashMap;
use rust_decimal::Decimal;
use bigdecimal::BigDecimal;
use decimal::d128;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub operation: String,
    pub library: String,
    pub duration: Duration,
    pub iterations: usize,
    pub avg_ns_per_op: f64,
    pub ops_per_second: f64,
}

pub struct PerformanceProfiler;

impl PerformanceProfiler {
    pub fn run_comprehensive_profiling() -> Vec<PerformanceMetrics> {
        let mut metrics = Vec::new();
        
        metrics.extend(Self::profile_basic_operations());
        metrics.extend(Self::profile_complex_operations());
        metrics.extend(Self::profile_memory_intensive_operations());
        
        metrics
    }

    fn profile_basic_operations() -> Vec<PerformanceMetrics> {
        let mut metrics = Vec::new();
        let iterations = 10_000;
        
        // Profile f64 addition
        let start = Instant::now();
        for i in 0..iterations {
            let a = i as f64 * 0.123456;
            let b = (i + 1) as f64 * 0.654321;
            let _ = a + b;
        }
        let duration = start.elapsed();
        metrics.push(PerformanceMetrics {
            operation: "Addition".to_string(),
            library: "f64".to_string(),
            duration,
            iterations,
            avg_ns_per_op: duration.as_nanos() as f64 / iterations as f64,
            ops_per_second: iterations as f64 / duration.as_secs_f64(),
        });

        // Profile Decimal addition
        let decimals: Vec<Decimal> = (0..iterations)
            .map(|i| Decimal::from_str(&format!("{}.123456", i)).unwrap())
            .collect();
        
        let start = Instant::now();
        for i in 0..iterations - 1 {
            let _ = decimals[i] + decimals[i + 1];
        }
        let duration = start.elapsed();
        metrics.push(PerformanceMetrics {
            operation: "Addition".to_string(),
            library: "rust_decimal".to_string(),
            duration,
            iterations: iterations - 1,
            avg_ns_per_op: duration.as_nanos() as f64 / (iterations - 1) as f64,
            ops_per_second: (iterations - 1) as f64 / duration.as_secs_f64(),
        });

        // Profile BigDecimal addition
        let bigdecimals: Vec<BigDecimal> = (0..1000) // Reduced for performance
            .map(|i| BigDecimal::from_str(&format!("{}.123456", i)).unwrap())
            .collect();
        
        let start = Instant::now();
        for i in 0..999 {
            let _ = &bigdecimals[i] + &bigdecimals[i + 1];
        }
        let duration = start.elapsed();
        metrics.push(PerformanceMetrics {
            operation: "Addition".to_string(),
            library: "bigdecimal".to_string(),
            duration,
            iterations: 999,
            avg_ns_per_op: duration.as_nanos() as f64 / 999.0,
            ops_per_second: 999.0 / duration.as_secs_f64(),
        });

        metrics
    }

    fn profile_complex_operations() -> Vec<PerformanceMetrics> {
        let mut metrics = Vec::new();
        let iterations = 1_000;
        
        // Profile f64 sqrt
        let start = Instant::now();
        for i in 1..=iterations {
            let value = i as f64 * 123.456;
            let _ = value.sqrt();
        }
        let duration = start.elapsed();
        metrics.push(PerformanceMetrics {
            operation: "Square Root".to_string(),
            library: "f64".to_string(),
            duration,
            iterations,
            avg_ns_per_op: duration.as_nanos() as f64 / iterations as f64,
            ops_per_second: iterations as f64 / duration.as_secs_f64(),
        });

        // Profile f64 power
        let start = Instant::now();
        for i in 1..=iterations {
            let base = (i % 10 + 1) as f64;
            let exp = (i % 5 + 1) as f64;
            let _ = base.powf(exp);
        }
        let duration = start.elapsed();
        metrics.push(PerformanceMetrics {
            operation: "Power".to_string(),
            library: "f64".to_string(),
            duration,
            iterations,
            avg_ns_per_op: duration.as_nanos() as f64 / iterations as f64,
            ops_per_second: iterations as f64 / duration.as_secs_f64(),
        });

        metrics
    }

    fn profile_memory_intensive_operations() -> Vec<PerformanceMetrics> {
        let mut metrics = Vec::new();
        let iterations = 1_000;
        
        // Profile string parsing
        let start = Instant::now();
        for i in 0..iterations {
            let _ = Decimal::from_str(&format!("{}.{:06}", i, i * 123456 % 1_000_000)).unwrap();
        }
        let duration = start.elapsed();
        metrics.push(PerformanceMetrics {
            operation: "String Parsing".to_string(),
            library: "rust_decimal".to_string(),
            duration,
            iterations,
            avg_ns_per_op: duration.as_nanos() as f64 / iterations as f64,
            ops_per_second: iterations as f64 / duration.as_secs_f64(),
        });

        // Profile vector operations
        let values: Vec<f64> = (0..iterations).map(|i| i as f64 * 0.123456).collect();
        let start = Instant::now();
        let _sum: f64 = values.iter().sum();
        let duration = start.elapsed();
        metrics.push(PerformanceMetrics {
            operation: "Vector Sum".to_string(),
            library: "f64".to_string(),
            duration,
            iterations,
            avg_ns_per_op: duration.as_nanos() as f64 / iterations as f64,
            ops_per_second: iterations as f64 / duration.as_secs_f64(),
        });

        metrics
    }

    pub fn analyze_performance_trends(metrics: &[PerformanceMetrics]) {
        let mut library_performance: HashMap<String, Vec<f64>> = HashMap::new();
        
        for metric in metrics {
            library_performance
                .entry(metric.library.clone())
                .or_insert_with(Vec::new)
                .push(metric.ops_per_second);
        }
        
        println!("\n=== Performance Analysis ===");
        for (library, performances) in library_performance {
            let avg_performance = performances.iter().sum::<f64>() / performances.len() as f64;
            let max_performance = performances.iter().copied().fold(0.0, f64::max);
            let min_performance = performances.iter().copied().fold(f64::INFINITY, f64::min);
            
            println!("Library: {}", library);
            println!("  Average ops/sec: {:.2e}", avg_performance);
            println!("  Max ops/sec: {:.2e}", max_performance);
            println!("  Min ops/sec: {:.2e}", min_performance);
            println!("  Performance range: {:.2e}", max_performance - min_performance);
            println!();
        }
    }

    pub fn print_performance_report(metrics: &[PerformanceMetrics]) {
        println!("\n=== Detailed Performance Report ===");
        println!("{:<20} {:<15} {:<12} {:<15} {:<15}", 
                 "Operation", "Library", "Iterations", "Avg ns/op", "Ops/sec");
        println!("{}", "-".repeat(77));
        
        for metric in metrics {
            println!("{:<20} {:<15} {:<12} {:<15.2} {:<15.2e}", 
                     metric.operation,
                     metric.library,
                     metric.iterations,
                     metric.avg_ns_per_op,
                     metric.ops_per_second);
        }
        
        Self::analyze_performance_trends(metrics);
    }

    pub fn benchmark_memory_allocation_speed() {
        println!("\n=== Memory Allocation Benchmarks ===");
        let iterations = 10_000;
        
        // Benchmark vector allocation
        let start = Instant::now();
        for _ in 0..iterations {
            let _vec: Vec<f64> = Vec::with_capacity(100);
        }
        let vec_duration = start.elapsed();
        
        // Benchmark decimal creation
        let start = Instant::now();
        for i in 0..iterations {
            let _ = Decimal::from_str(&format!("{}.123", i)).unwrap();
        }
        let decimal_duration = start.elapsed();
        
        println!("Vector allocation ({}): {:?}", iterations, vec_duration);
        println!("Decimal creation ({}): {:?}", iterations, decimal_duration);
        println!("Ratio (Decimal/Vector): {:.2}x", 
                 decimal_duration.as_nanos() as f64 / vec_duration.as_nanos() as f64);
    }
}
