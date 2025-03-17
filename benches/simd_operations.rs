use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use std::simd::{f64x4, f64x8, f32x8, f32x16};

fn benchmark_scalar_operations(c: &mut Criterion) {
    let data: Vec<f64> = (0..1000).map(|i| i as f64 * 0.123456).collect();
    let other: Vec<f64> = (0..1000).map(|i| (i + 1) as f64 * 0.654321).collect();
    
    c.bench_function("scalar_f64_addition", |b| {
        b.iter(|| {
            let mut result = Vec::with_capacity(data.len());
            for i in 0..data.len() {
                result.push(black_box(data[i]) + black_box(other[i]));
            }
            black_box(result)
        })
    });

    c.bench_function("scalar_f64_multiplication", |b| {
        b.iter(|| {
            let mut result = Vec::with_capacity(data.len());
            for i in 0..data.len() {
                result.push(black_box(data[i]) * black_box(other[i]));
            }
            black_box(result)
        })
    });

    c.bench_function("scalar_f64_fma", |b| {
        b.iter(|| {
            let mut result = Vec::with_capacity(data.len());
            for i in 0..data.len() {
                result.push(black_box(data[i]).mul_add(black_box(other[i]), black_box(1.0)));
            }
            black_box(result)
        })
    });
}

fn benchmark_simd_f64x4_operations(c: &mut Criterion) {
    let data: Vec<f64> = (0..1000).map(|i| i as f64 * 0.123456).collect();
    let other: Vec<f64> = (0..1000).map(|i| (i + 1) as f64 * 0.654321).collect();
    
    c.bench_function("simd_f64x4_addition", |b| {
        b.iter(|| {
            let mut result = Vec::with_capacity(data.len());
            let chunks = data.chunks_exact(4).zip(other.chunks_exact(4));
            
            for (a_chunk, b_chunk) in chunks {
                let a_simd = f64x4::from_slice(a_chunk);
                let b_simd = f64x4::from_slice(b_chunk);
                let sum = black_box(a_simd) + black_box(b_simd);
                result.extend_from_slice(&sum.to_array());
            }
            black_box(result)
        })
    });

    c.bench_function("simd_f64x4_multiplication", |b| {
        b.iter(|| {
            let mut result = Vec::with_capacity(data.len());
            let chunks = data.chunks_exact(4).zip(other.chunks_exact(4));
            
            for (a_chunk, b_chunk) in chunks {
                let a_simd = f64x4::from_slice(a_chunk);
                let b_simd = f64x4::from_slice(b_chunk);
                let product = black_box(a_simd) * black_box(b_simd);
                result.extend_from_slice(&product.to_array());
            }
            black_box(result)
        })
    });
}

fn benchmark_simd_f64x8_operations(c: &mut Criterion) {
    let data: Vec<f64> = (0..1000).map(|i| i as f64 * 0.123456).collect();
    let other: Vec<f64> = (0..1000).map(|i| (i + 1) as f64 * 0.654321).collect();
    
    c.bench_function("simd_f64x8_addition", |b| {
        b.iter(|| {
            let mut result = Vec::with_capacity(data.len());
            let chunks = data.chunks_exact(8).zip(other.chunks_exact(8));
            
            for (a_chunk, b_chunk) in chunks {
                let a_simd = f64x8::from_slice(a_chunk);
                let b_simd = f64x8::from_slice(b_chunk);
                let sum = black_box(a_simd) + black_box(b_simd);
                result.extend_from_slice(&sum.to_array());
            }
            black_box(result)
        })
    });

    c.bench_function("simd_f64x8_multiplication", |b| {
        b.iter(|| {
            let mut result = Vec::with_capacity(data.len());
            let chunks = data.chunks_exact(8).zip(other.chunks_exact(8));
            
            for (a_chunk, b_chunk) in chunks {
                let a_simd = f64x8::from_slice(a_chunk);
                let b_simd = f64x8::from_slice(b_chunk);
                let product = black_box(a_simd) * black_box(b_simd);
                result.extend_from_slice(&product.to_array());
            }
            black_box(result)
        })
    });
}

fn benchmark_simd_f32_operations(c: &mut Criterion) {
    let data: Vec<f32> = (0..1000).map(|i| i as f32 * 0.123456).collect();
    let other: Vec<f32> = (0..1000).map(|i| (i + 1) as f32 * 0.654321).collect();
    
    c.bench_function("scalar_f32_addition", |b| {
        b.iter(|| {
            let mut result = Vec::with_capacity(data.len());
            for i in 0..data.len() {
                result.push(black_box(data[i]) + black_box(other[i]));
            }
            black_box(result)
        })
    });

    c.bench_function("simd_f32x8_addition", |b| {
        b.iter(|| {
            let mut result = Vec::with_capacity(data.len());
            let chunks = data.chunks_exact(8).zip(other.chunks_exact(8));
            
            for (a_chunk, b_chunk) in chunks {
                let a_simd = f32x8::from_slice(a_chunk);
                let b_simd = f32x8::from_slice(b_chunk);
                let sum = black_box(a_simd) + black_box(b_simd);
                result.extend_from_slice(&sum.to_array());
            }
            black_box(result)
        })
    });

    c.bench_function("simd_f32x16_addition", |b| {
        b.iter(|| {
            let mut result = Vec::with_capacity(data.len());
            let chunks = data.chunks_exact(16).zip(other.chunks_exact(16));
            
            for (a_chunk, b_chunk) in chunks {
                let a_simd = f32x16::from_slice(a_chunk);
                let b_simd = f32x16::from_slice(b_chunk);
                let sum = black_box(a_simd) + black_box(b_simd);
                result.extend_from_slice(&sum.to_array());
            }
            black_box(result)
        })
    });
}

fn benchmark_reduction_operations(c: &mut Criterion) {
    let data: Vec<f64> = (0..1000).map(|i| i as f64 * 0.001).collect();
    
    c.bench_function("scalar_sum", |b| {
        b.iter(|| {
            let sum = black_box(&data).iter().sum::<f64>();
            black_box(sum)
        })
    });

    c.bench_function("simd_f64x4_sum", |b| {
        b.iter(|| {
            let mut sum = f64x4::splat(0.0);
            for chunk in black_box(&data).chunks_exact(4) {
                let simd_chunk = f64x4::from_slice(chunk);
                sum += simd_chunk;
            }
            let result = sum.reduce_sum();
            black_box(result)
        })
    });

    c.bench_function("simd_f64x8_sum", |b| {
        b.iter(|| {
            let mut sum = f64x8::splat(0.0);
            for chunk in black_box(&data).chunks_exact(8) {
                let simd_chunk = f64x8::from_slice(chunk);
                sum += simd_chunk;
            }
            let result = sum.reduce_sum();
            black_box(result)
        })
    });
}

fn benchmark_dot_product(c: &mut Criterion) {
    let sizes = [100, 1000, 10000];
    
    for size in sizes {
        let a: Vec<f64> = (0..size).map(|i| i as f64 * 0.001).collect();
        let b: Vec<f64> = (0..size).map(|i| (i + 1) as f64 * 0.002).collect();
        
        c.bench_with_input(
            BenchmarkId::new("scalar_dot_product", size),
            &(a.clone(), b.clone()),
            |bench, (a, b)| {
                bench.iter(|| {
                    let dot_product = a.iter()
                        .zip(b.iter())
                        .map(|(x, y)| black_box(*x) * black_box(*y))
                        .sum::<f64>();
                    black_box(dot_product)
                })
            },
        );

        c.bench_with_input(
            BenchmarkId::new("simd_f64x4_dot_product", size),
            &(a.clone(), b.clone()),
            |bench, (a, b)| {
                bench.iter(|| {
                    let mut sum = f64x4::splat(0.0);
                    let chunks = black_box(a).chunks_exact(4).zip(black_box(b).chunks_exact(4));
                    
                    for (a_chunk, b_chunk) in chunks {
                        let a_simd = f64x4::from_slice(a_chunk);
                        let b_simd = f64x4::from_slice(b_chunk);
                        sum += a_simd * b_simd;
                    }
                    
                    let result = sum.reduce_sum();
                    black_box(result)
                })
            },
        );

        c.bench_with_input(
            BenchmarkId::new("simd_f64x8_dot_product", size),
            &(a.clone(), b.clone()),
            |bench, (a, b)| {
                bench.iter(|| {
                    let mut sum = f64x8::splat(0.0);
                    let chunks = black_box(a).chunks_exact(8).zip(black_box(b).chunks_exact(8));
                    
                    for (a_chunk, b_chunk) in chunks {
                        let a_simd = f64x8::from_slice(a_chunk);
                        let b_simd = f64x8::from_slice(b_chunk);
                        sum += a_simd * b_simd;
                    }
                    
                    let result = sum.reduce_sum();
                    black_box(result)
                })
            },
        );
    }
}

fn benchmark_mathematical_functions(c: &mut Criterion) {
    let data: Vec<f64> = (1..=1000).map(|i| i as f64 * 0.01).collect();
    
    c.bench_function("scalar_sqrt", |b| {
        b.iter(|| {
            let mut result = Vec::with_capacity(data.len());
            for &value in black_box(&data) {
                result.push(value.sqrt());
            }
            black_box(result)
        })
    });

    // Note: SIMD sqrt would require platform-specific intrinsics
    // This is a simplified version showing the pattern
    c.bench_function("manual_simd_sqrt_f64x4", |b| {
        b.iter(|| {
            let mut result = Vec::with_capacity(data.len());
            for chunk in black_box(&data).chunks_exact(4) {
                // Manual sqrt for each element (not true SIMD sqrt)
                let sqrt_values: Vec<f64> = chunk.iter().map(|x| x.sqrt()).collect();
                result.extend_from_slice(&sqrt_values);
            }
            black_box(result)
        })
    });

    c.bench_function("scalar_sin", |b| {
        b.iter(|| {
            let mut result = Vec::with_capacity(data.len());
            for &value in black_box(&data) {
                result.push(value.sin());
            }
            black_box(result)
        })
    });
}

fn benchmark_memory_patterns(c: &mut Criterion) {
    let data: Vec<f64> = (0..1000).map(|i| i as f64 * 0.123456).collect();
    
    // Sequential access
    c.bench_function("sequential_access_scalar", |b| {
        b.iter(|| {
            let mut sum = 0.0;
            for &value in black_box(&data) {
                sum += value;
            }
            black_box(sum)
        })
    });

    c.bench_function("sequential_access_simd_f64x4", |b| {
        b.iter(|| {
            let mut sum = f64x4::splat(0.0);
            for chunk in black_box(&data).chunks_exact(4) {
                let simd_chunk = f64x4::from_slice(chunk);
                sum += simd_chunk;
            }
            let result = sum.reduce_sum();
            black_box(result)
        })
    });

    // Strided access (every 2nd element)
    c.bench_function("strided_access_scalar", |b| {
        b.iter(|| {
            let mut sum = 0.0;
            for i in (0..black_box(&data).len()).step_by(2) {
                sum += data[i];
            }
            black_box(sum)
        })
    });
}

criterion_group!(
    simd_benches,
    benchmark_scalar_operations,
    benchmark_simd_f64x4_operations,
    benchmark_simd_f64x8_operations,
    benchmark_simd_f32_operations,
    benchmark_reduction_operations,
    benchmark_dot_product,
    benchmark_mathematical_functions,
    benchmark_memory_patterns
);
criterion_main!(simd_benches);
