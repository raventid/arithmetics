use criterion::{black_box, criterion_group, criterion_main, Criterion};
use arithmetics::platform::{CrossPlatformTester, DecimalCrossPlatformTester, PlatformInfo};
use rust_decimal::Decimal;
use bigdecimal::BigDecimal;
use std::str::FromStr;

fn benchmark_platform_detection(c: &mut Criterion) {
    c.bench_function("platform_info_detection", |b| {
        b.iter(|| {
            let info = black_box(PlatformInfo::detect());
            black_box(info)
        })
    });
}

fn benchmark_cross_platform_arithmetic_tests(c: &mut Criterion) {
    c.bench_function("cross_platform_test_suite", |b| {
        b.iter(|| {
            let mut tester = black_box(CrossPlatformTester::new());
            tester.run_all_tests();
            black_box(tester.get_test_results())
        })
    });

    c.bench_function("float_behavior_test", |b| {
        b.iter(|| {
            let mut tester = CrossPlatformTester::new();
            tester.test_float_behavior();
            black_box(tester.get_test_results())
        })
    });

    c.bench_function("integer_overflow_test", |b| {
        b.iter(|| {
            let mut tester = CrossPlatformTester::new();
            tester.test_integer_overflow();
            black_box(tester.get_test_results())
        })
    });

    c.bench_function("endianness_test", |b| {
        b.iter(|| {
            let mut tester = CrossPlatformTester::new();
            tester.test_endianness_consistency();
            black_box(tester.get_test_results())
        })
    });
}

fn benchmark_decimal_cross_platform_behavior(c: &mut Criterion) {
    c.bench_function("decimal_serialization_test", |b| {
        b.iter(|| {
            let tester = DecimalCrossPlatformTester::new();
            let result = black_box(tester.test_decimal_serialization());
            black_box(result)
        })
    });

    c.bench_function("decimal_precision_consistency", |b| {
        b.iter(|| {
            let tester = DecimalCrossPlatformTester::new();
            let issues = black_box(tester.test_decimal_precision_consistency());
            black_box(issues)
        })
    });

    c.bench_function("bigdecimal_cross_platform", |b| {
        b.iter(|| {
            let tester = DecimalCrossPlatformTester::new();
            let issues = black_box(tester.test_bigdecimal_cross_platform());
            black_box(issues)
        })
    });
}

fn benchmark_endianness_specific_operations(c: &mut Criterion) {
    let test_values: Vec<u64> = (0..1000).map(|i| i * 0x123456789ABCDEF0 + i).collect();

    c.bench_function("big_endian_conversion", |b| {
        b.iter(|| {
            let converted: Vec<[u8; 8]> = black_box(&test_values)
                .iter()
                .map(|&val| val.to_be_bytes())
                .collect();
            black_box(converted)
        })
    });

    c.bench_function("little_endian_conversion", |b| {
        b.iter(|| {
            let converted: Vec<[u8; 8]> = black_box(&test_values)
                .iter()
                .map(|&val| val.to_le_bytes())
                .collect();
            black_box(converted)
        })
    });

    c.bench_function("native_endian_conversion", |b| {
        b.iter(|| {
            let converted: Vec<[u8; 8]> = black_box(&test_values)
                .iter()
                .map(|&val| val.to_ne_bytes())
                .collect();
            black_box(converted)
        })
    });

    c.bench_function("endian_swap_performance", |b| {
        b.iter(|| {
            let swapped: Vec<u64> = black_box(&test_values)
                .iter()
                .map(|&val| val.swap_bytes())
                .collect();
            black_box(swapped)
        })
    });
}

fn benchmark_alignment_sensitive_operations(c: &mut Criterion) {
    // Test structures with different alignment requirements
    #[repr(C)]
    struct AlignedStruct {
        a: u8,
        b: u64,
        c: u8,
    }

    #[repr(C, packed)]
    struct PackedStruct {
        a: u8,
        b: u64,
        c: u8,
    }

    c.bench_function("aligned_struct_operations", |b| {
        b.iter(|| {
            let data: Vec<AlignedStruct> = (0..1000)
                .map(|i| AlignedStruct {
                    a: (i % 256) as u8,
                    b: i as u64,
                    c: ((i * 2) % 256) as u8,
                })
                .collect();
            
            let sum: u64 = data.iter().map(|s| s.b).sum();
            black_box((data, sum))
        })
    });

    c.bench_function("packed_struct_operations", |b| {
        b.iter(|| {
            let data: Vec<PackedStruct> = (0..1000)
                .map(|i| PackedStruct {
                    a: (i % 256) as u8,
                    b: i as u64,
                    c: ((i * 2) % 256) as u8,
                })
                .collect();
            
            let sum: u64 = data.iter().map(|s| s.b).sum();
            black_box((data, sum))
        })
    });
}

fn benchmark_platform_specific_optimizations(c: &mut Criterion) {
    let data: Vec<f64> = (0..1000).map(|i| i as f64 * 0.1).collect();

    // Test different approaches that might be optimized differently on different platforms
    c.bench_function("platform_specific_math_naive", |b| {
        b.iter(|| {
            let mut results = Vec::new();
            for &val in black_box(&data) {
                let result = val.sin() * val.cos() + val.sqrt();
                results.push(result);
            }
            black_box(results)
        })
    });

    c.bench_function("platform_specific_math_optimized", |b| {
        b.iter(|| {
            let mut results = Vec::with_capacity(data.len());
            for &val in black_box(&data) {
                // Use potentially platform-optimized operations
                let sin_cos = val.sin_cos();
                let result = sin_cos.0 * sin_cos.1 + val.sqrt();
                results.push(result);
            }
            black_box(results)
        })
    });

    // Test branch prediction sensitivity
    c.bench_function("platform_branch_prediction_friendly", |b| {
        b.iter(|| {
            let mut positive_count = 0;
            let mut negative_count = 0;
            
            // Sorted data - branch predictor friendly
            let sorted_data: Vec<f64> = {
                let mut data = data.clone();
                data.sort_by(|a, b| a.partial_cmp(b).unwrap());
                data
            };
            
            for &val in black_box(&sorted_data) {
                if val > 50.0 {
                    positive_count += 1;
                } else {
                    negative_count += 1;
                }
            }
            
            black_box((positive_count, negative_count))
        })
    });

    c.bench_function("platform_branch_prediction_unfriendly", |b| {
        b.iter(|| {
            let mut positive_count = 0;
            let mut negative_count = 0;
            
            // Random-ish pattern - branch predictor unfriendly
            let random_data: Vec<f64> = data
                .iter()
                .enumerate()
                .map(|(i, &val)| if i % 2 == 0 { val } else { -val })
                .collect();
            
            for &val in black_box(&random_data) {
                if val > 0.0 {
                    positive_count += 1;
                } else {
                    negative_count += 1;
                }
            }
            
            black_box((positive_count, negative_count))
        })
    });
}

fn benchmark_architecture_specific_features(c: &mut Criterion) {
    let data: Vec<f32> = (0..1024).map(|i| i as f32).collect();

    // Test operations that might use different instruction sets
    c.bench_function("arch_specific_vector_add", |b| {
        b.iter(|| {
            let data1 = black_box(&data);
            let data2 = black_box(&data);
            let mut result = Vec::with_capacity(data.len());
            
            for i in 0..data.len() {
                result.push(data1[i] + data2[i]);
            }
            
            black_box(result)
        })
    });

    c.bench_function("arch_specific_vector_multiply", |b| {
        b.iter(|| {
            let data1 = black_box(&data);
            let data2 = black_box(&data);
            let mut result = Vec::with_capacity(data.len());
            
            for i in 0..data.len() {
                result.push(data1[i] * data2[i]);
            }
            
            black_box(result)
        })
    });

    c.bench_function("arch_specific_fma", |b| {
        b.iter(|| {
            let data1 = black_box(&data);
            let data2 = black_box(&data);
            let data3 = black_box(&data);
            let mut result = Vec::with_capacity(data.len());
            
            for i in 0..data.len() {
                // This might use FMA instruction on supporting architectures
                result.push(data1[i].mul_add(data2[i], data3[i]));
            }
            
            black_box(result)
        })
    });
}

criterion_group!(
    cross_platform_benches,
    benchmark_platform_detection,
    benchmark_cross_platform_arithmetic_tests,
    benchmark_decimal_cross_platform_behavior,
    benchmark_endianness_specific_operations,
    benchmark_alignment_sensitive_operations,
    benchmark_platform_specific_optimizations,
    benchmark_architecture_specific_features
);
criterion_main!(cross_platform_benches);
