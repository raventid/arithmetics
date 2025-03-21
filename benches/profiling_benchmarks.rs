use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_decimal::Decimal;
use bigdecimal::BigDecimal;
use std::str::FromStr;
use std::sync::Arc;
use arithmetics::profiler::{TrackingAllocator, CombinedProfiler, CacheAnalyzer};

// Global allocator for memory tracking
#[global_allocator]
static ALLOCATOR: TrackingAllocator = TrackingAllocator::new();

fn benchmark_with_profiling(c: &mut Criterion) {
    let allocator = Arc::new(TrackingAllocator::new());
    let mut profiler = CombinedProfiler::new(allocator.clone());
    
    // Reset allocator stats before benchmarking
    ALLOCATOR.reset_stats();
    
    c.bench_function("decimal_operations_with_profiling", |b| {
        b.iter(|| {
            let (result, profile) = profiler.profile_operation("decimal_arithmetic", || {
                let mut sum = Decimal::ZERO;
                for i in 0..1000 {
                    let val = Decimal::from(i);
                    sum += val * Decimal::from(2);
                    sum = sum / Decimal::from(3);
                }
                black_box(sum)
            });
            
            black_box((result, profile))
        })
    });

    c.bench_function("bigdecimal_operations_with_profiling", |b| {
        b.iter(|| {
            let (result, profile) = profiler.profile_operation("bigdecimal_arithmetic", || {
                let mut sum = BigDecimal::from(0);
                for i in 0..1000 {
                    let val = BigDecimal::from(i);
                    sum += &val * &BigDecimal::from(2);
                    sum = &sum / &BigDecimal::from(3);
                }
                black_box(sum)
            });
            
            black_box((result, profile))
        })
    });

    c.bench_function("f64_operations_with_profiling", |b| {
        b.iter(|| {
            let (result, profile) = profiler.profile_operation("f64_arithmetic", || {
                let mut sum = 0.0f64;
                for i in 0..1000 {
                    let val = i as f64;
                    sum += val * 2.0;
                    sum = sum / 3.0;
                }
                black_box(sum)
            });
            
            black_box((result, profile))
        })
    });
    
    // Print comprehensive analysis at the end
    profiler.print_comprehensive_summary();
}

fn benchmark_cache_performance(c: &mut Criterion) {
    let cache_analyzer = CacheAnalyzer::new();
    
    // Large array for cache analysis
    let data: Vec<f64> = (0..10_000).map(|i| i as f64).collect();
    
    c.bench_function("sequential_access_pattern", |b| {
        b.iter(|| {
            let access_pattern: Vec<usize> = (0..1000).collect();
            let stats = cache_analyzer.analyze_access_pattern(black_box(&data), &access_pattern);
            
            let mut sum = 0.0;
            for &index in &access_pattern {
                sum += data[index];
            }
            
            black_box((sum, stats))
        })
    });

    c.bench_function("random_access_pattern", |b| {
        b.iter(|| {
            let access_pattern: Vec<usize> = (0..1000)
                .map(|i| (i * 1103515245 + 12345) % data.len()) // Linear congruential generator
                .collect();
            let stats = cache_analyzer.analyze_access_pattern(black_box(&data), &access_pattern);
            
            let mut sum = 0.0;
            for &index in &access_pattern {
                sum += data[index];
            }
            
            black_box((sum, stats))
        })
    });

    c.bench_function("strided_access_pattern", |b| {
        b.iter(|| {
            let stride = 16;
            let access_pattern: Vec<usize> = (0..1000)
                .map(|i| (i * stride) % data.len())
                .collect();
            let stats = cache_analyzer.analyze_access_pattern(black_box(&data), &access_pattern);
            
            let mut sum = 0.0;
            for &index in &access_pattern {
                sum += data[index];
            }
            
            black_box((sum, stats))
        })
    });
}

fn benchmark_memory_allocation_patterns(c: &mut Criterion) {
    let allocator = Arc::new(TrackingAllocator::new());
    
    c.bench_function("vector_growth_pattern", |b| {
        b.iter(|| {
            ALLOCATOR.reset_stats();
            let initial_usage = ALLOCATOR.current_usage();
            
            let mut vec = Vec::new();
            for i in 0..1000 {
                vec.push(i as f64);
            }
            
            let final_usage = ALLOCATOR.current_usage();
            let peak_usage = ALLOCATOR.peak_usage();
            let allocations = ALLOCATOR.allocation_count();
            
            black_box((vec, final_usage - initial_usage, peak_usage, allocations))
        })
    });

    c.bench_function("pre_allocated_vector", |b| {
        b.iter(|| {
            ALLOCATOR.reset_stats();
            let initial_usage = ALLOCATOR.current_usage();
            
            let mut vec = Vec::with_capacity(1000);
            for i in 0..1000 {
                vec.push(i as f64);
            }
            
            let final_usage = ALLOCATOR.current_usage();
            let peak_usage = ALLOCATOR.peak_usage();
            let allocations = ALLOCATOR.allocation_count();
            
            black_box((vec, final_usage - initial_usage, peak_usage, allocations))
        })
    });

    c.bench_function("decimal_vector_allocation", |b| {
        b.iter(|| {
            ALLOCATOR.reset_stats();
            let initial_usage = ALLOCATOR.current_usage();
            
            let mut vec = Vec::new();
            for i in 0..1000 {
                vec.push(Decimal::from(i));
            }
            
            let final_usage = ALLOCATOR.current_usage();
            let peak_usage = ALLOCATOR.peak_usage();
            let allocations = ALLOCATOR.allocation_count();
            
            black_box((vec, final_usage - initial_usage, peak_usage, allocations))
        })
    });

    c.bench_function("bigdecimal_vector_allocation", |b| {
        b.iter(|| {
            ALLOCATOR.reset_stats();
            let initial_usage = ALLOCATOR.current_usage();
            
            let mut vec = Vec::new();
            for i in 0..1000 {
                vec.push(BigDecimal::from(i));
            }
            
            let final_usage = ALLOCATOR.current_usage();
            let peak_usage = ALLOCATOR.peak_usage();
            let allocations = ALLOCATOR.allocation_count();
            
            black_box((vec, final_usage - initial_usage, peak_usage, allocations))
        })
    });
}

fn benchmark_arithmetic_precision_vs_performance(c: &mut Criterion) {
    let values_f64: Vec<f64> = (0..1000).map(|i| i as f64 * 0.1).collect();
    let values_decimal: Vec<Decimal> = values_f64
        .iter()
        .map(|&v| Decimal::from_str(&v.to_string()).unwrap())
        .collect();
    let values_bigdecimal: Vec<BigDecimal> = values_f64
        .iter()
        .map(|&v| BigDecimal::from_str(&v.to_string()).unwrap())
        .collect();

    // Complex calculation that accumulates precision errors
    c.bench_function("precision_test_f64", |b| {
        b.iter(|| {
            let mut result = 0.0f64;
            for &val in black_box(&values_f64) {
                result += val;
                result *= 1.000001; // Small multiplier that accumulates error
                result -= val * 0.999999;
            }
            black_box(result)
        })
    });

    c.bench_function("precision_test_decimal", |b| {
        b.iter(|| {
            let mut result = Decimal::ZERO;
            let multiplier = Decimal::from_str("1.000001").unwrap();
            let subtractor = Decimal::from_str("0.999999").unwrap();
            
            for val in black_box(&values_decimal) {
                result += val;
                result *= multiplier;
                result -= val * subtractor;
            }
            black_box(result)
        })
    });

    c.bench_function("precision_test_bigdecimal", |b| {
        b.iter(|| {
            let mut result = BigDecimal::from(0);
            let multiplier = BigDecimal::from_str("1.000001").unwrap();
            let subtractor = BigDecimal::from_str("0.999999").unwrap();
            
            for val in black_box(&values_bigdecimal) {
                result += val;
                result = &result * &multiplier;
                result -= val * &subtractor;
            }
            black_box(result)
        })
    });
}

criterion_group!(
    profiling_benches,
    benchmark_with_profiling,
    benchmark_cache_performance,
    benchmark_memory_allocation_patterns,
    benchmark_arithmetic_precision_vs_performance
);
criterion_main!(profiling_benches);
