use criterion::{black_box, criterion_group, criterion_main, Criterion};
use half::{f16, bf16};

fn benchmark_f16_arithmetic(c: &mut Criterion) {
    let a = f16::from_f32(123.45);
    let b = f16::from_f32(67.89);
    
    c.bench_function("f16_addition", |bench| {
        bench.iter(|| {
            let result = black_box(a) + black_box(b);
            black_box(result)
        })
    });

    c.bench_function("f16_multiplication", |bench| {
        bench.iter(|| {
            let result = black_box(a) * black_box(b);
            black_box(result)
        })
    });

    c.bench_function("f16_division", |bench| {
        bench.iter(|| {
            let result = black_box(a) / black_box(b);
            black_box(result)
        })
    });

    c.bench_function("f16_subtraction", |bench| {
        bench.iter(|| {
            let result = black_box(a) - black_box(b);
            black_box(result)
        })
    });
}

fn benchmark_bf16_arithmetic(c: &mut Criterion) {
    let a = bf16::from_f32(123.45);
    let b = bf16::from_f32(67.89);
    
    c.bench_function("bf16_addition", |bench| {
        bench.iter(|| {
            let result = black_box(a) + black_box(b);
            black_box(result)
        })
    });

    c.bench_function("bf16_multiplication", |bench| {
        bench.iter(|| {
            let result = black_box(a) * black_box(b);
            black_box(result)
        })
    });

    c.bench_function("bf16_division", |bench| {
        bench.iter(|| {
            let result = black_box(a) / black_box(b);
            black_box(result)
        })
    });

    c.bench_function("bf16_subtraction", |bench| {
        bench.iter(|| {
            let result = black_box(a) - black_box(b);
            black_box(result)
        })
    });
}

fn benchmark_precision_comparison(c: &mut Criterion) {
    let f32_val = 123.456789_f32;
    let f16_val = f16::from_f32(f32_val);
    let bf16_val = bf16::from_f32(f32_val);
    
    c.bench_function("f32_to_f16_conversion", |bench| {
        bench.iter(|| {
            let result = f16::from_f32(black_box(f32_val));
            black_box(result)
        })
    });

    c.bench_function("f32_to_bf16_conversion", |bench| {
        bench.iter(|| {
            let result = bf16::from_f32(black_box(f32_val));
            black_box(result)
        })
    });

    c.bench_function("f16_to_f32_conversion", |bench| {
        bench.iter(|| {
            let result = black_box(f16_val).to_f32();
            black_box(result)
        })
    });

    c.bench_function("bf16_to_f32_conversion", |bench| {
        bench.iter(|| {
            let result = black_box(bf16_val).to_f32();
            black_box(result)
        })
    });
}

fn benchmark_half_precision_arrays(c: &mut Criterion) {
    let f16_array: Vec<f16> = (0..1000).map(|i| f16::from_f32(i as f32 * 0.123)).collect();
    let bf16_array: Vec<bf16> = (0..1000).map(|i| bf16::from_f32(i as f32 * 0.123)).collect();
    
    c.bench_function("f16_array_sum", |bench| {
        bench.iter(|| {
            let sum = black_box(&f16_array)
                .iter()
                .fold(f16::ZERO, |acc, &x| acc + x);
            black_box(sum)
        })
    });

    c.bench_function("bf16_array_sum", |bench| {
        bench.iter(|| {
            let sum = black_box(&bf16_array)
                .iter()
                .fold(bf16::ZERO, |acc, &x| acc + x);
            black_box(sum)
        })
    });

    c.bench_function("f16_array_dot_product", |bench| {
        bench.iter(|| {
            let dot_product = black_box(&f16_array)
                .iter()
                .zip(f16_array.iter())
                .fold(f16::ZERO, |acc, (&a, &b)| acc + a * b);
            black_box(dot_product)
        })
    });

    c.bench_function("bf16_array_dot_product", |bench| {
        bench.iter(|| {
            let dot_product = black_box(&bf16_array)
                .iter()
                .zip(bf16_array.iter())
                .fold(bf16::ZERO, |acc, (&a, &b)| acc + a * b);
            black_box(dot_product)
        })
    });
}

fn benchmark_half_precision_math(c: &mut Criterion) {
    let values: Vec<f16> = (1..=100).map(|i| f16::from_f32(i as f32)).collect();
    let bf16_values: Vec<bf16> = (1..=100).map(|i| bf16::from_f32(i as f32)).collect();
    
    c.bench_function("f16_sqrt_approximation", |bench| {
        bench.iter(|| {
            for &val in black_box(&values) {
                // Convert to f32, sqrt, then back to f16
                let result = f16::from_f32(val.to_f32().sqrt());
                black_box(result);
            }
        })
    });

    c.bench_function("bf16_sqrt_approximation", |bench| {
        bench.iter(|| {
            for &val in black_box(&bf16_values) {
                // Convert to f32, sqrt, then back to bf16
                let result = bf16::from_f32(val.to_f32().sqrt());
                black_box(result);
            }
        })
    });

    c.bench_function("f16_min_max_operations", |bench| {
        bench.iter(|| {
            let min_val = black_box(&values).iter().copied().fold(f16::INFINITY, f16::min);
            let max_val = black_box(&values).iter().copied().fold(f16::NEG_INFINITY, f16::max);
            black_box((min_val, max_val))
        })
    });

    c.bench_function("bf16_min_max_operations", |bench| {
        bench.iter(|| {
            let min_val = black_box(&bf16_values).iter().copied().fold(bf16::INFINITY, bf16::min);
            let max_val = black_box(&bf16_values).iter().copied().fold(bf16::NEG_INFINITY, bf16::max);
            black_box((min_val, max_val))
        })
    });
}

criterion_group!(
    half_precision_benches,
    benchmark_f16_arithmetic,
    benchmark_bf16_arithmetic,
    benchmark_precision_comparison,
    benchmark_half_precision_arrays,
    benchmark_half_precision_math
);
criterion_main!(half_precision_benches);
