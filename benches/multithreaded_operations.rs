use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use rayon::prelude::*;
use rust_decimal::Decimal;
use bigdecimal::BigDecimal;
use std::str::FromStr;
use std::sync::Mutex;
use std::thread;
use std::sync::Arc;

fn benchmark_single_threaded_operations(c: &mut Criterion) {
    let data: Vec<f64> = (0..100_000).map(|i| i as f64 * 0.123456).collect();
    
    c.bench_function("single_thread_f64_sum", |b| {
        b.iter(|| {
            let sum = black_box(&data).iter().sum::<f64>();
            black_box(sum)
        })
    });

    c.bench_function("single_thread_f64_map", |b| {
        b.iter(|| {
            let result: Vec<f64> = black_box(&data)
                .iter()
                .map(|&x| x * 2.0 + 1.0)
                .collect();
            black_box(result)
        })
    });

    c.bench_function("single_thread_f64_filter_map", |b| {
        b.iter(|| {
            let result: Vec<f64> = black_box(&data)
                .iter()
                .filter(|&&x| x > 1000.0)
                .map(|&x| x.sqrt())
                .collect();
            black_box(result)
        })
    });

    let decimals: Vec<Decimal> = (0..10_000)
        .map(|i| Decimal::from_str(&format!("{}.123456", i)).unwrap())
        .collect();

    c.bench_function("single_thread_decimal_sum", |b| {
        b.iter(|| {
            let sum = black_box(&decimals)
                .iter()
                .fold(Decimal::ZERO, |acc, &x| acc + x);
            black_box(sum)
        })
    });
}

fn benchmark_rayon_parallel_operations(c: &mut Criterion) {
    let data: Vec<f64> = (0..100_000).map(|i| i as f64 * 0.123456).collect();
    
    c.bench_function("rayon_parallel_f64_sum", |b| {
        b.iter(|| {
            let sum = black_box(&data).par_iter().sum::<f64>();
            black_box(sum)
        })
    });

    c.bench_function("rayon_parallel_f64_map", |b| {
        b.iter(|| {
            let result: Vec<f64> = black_box(&data)
                .par_iter()
                .map(|&x| x * 2.0 + 1.0)
                .collect();
            black_box(result)
        })
    });

    c.bench_function("rayon_parallel_f64_filter_map", |b| {
        b.iter(|| {
            let result: Vec<f64> = black_box(&data)
                .par_iter()
                .filter(|&&x| x > 1000.0)
                .map(|&x| x.sqrt())
                .collect();
            black_box(result)
        })
    });

    let decimals: Vec<Decimal> = (0..10_000)
        .map(|i| Decimal::from_str(&format!("{}.123456", i)).unwrap())
        .collect();

    c.bench_function("rayon_parallel_decimal_sum", |b| {
        b.iter(|| {
            let sum = black_box(&decimals)
                .par_iter()
                .fold(|| Decimal::ZERO, |acc, &x| acc + x)
                .reduce(|| Decimal::ZERO, |a, b| a + b);
            black_box(sum)
        })
    });
}

fn benchmark_manual_threading(c: &mut Criterion) {
    let data: Vec<f64> = (0..100_000).map(|i| i as f64 * 0.123456).collect();
    let num_threads = num_cpus::get();
    
    c.bench_function("manual_thread_f64_sum", |b| {
        b.iter(|| {
            let data = black_box(&data);
            let chunk_size = data.len() / num_threads;
            let result = Arc::new(Mutex::new(0.0_f64));
            
            let handles: Vec<_> = (0..num_threads)
                .map(|i| {
                    let start = i * chunk_size;
                    let end = if i == num_threads - 1 { data.len() } else { (i + 1) * chunk_size };
                    let chunk = &data[start..end];
                    let result = Arc::clone(&result);
                    
                    thread::spawn(move || {
                        let partial_sum: f64 = chunk.iter().sum();
                        let mut total = result.lock().unwrap();
                        *total += partial_sum;
                    })
                })
                .collect();
            
            for handle in handles {
                handle.join().unwrap();
            }
            
            let final_result = *result.lock().unwrap();
            black_box(final_result)
        })
    });
}

fn benchmark_parallel_matrix_operations(c: &mut Criterion) {
    let sizes = [100, 500, 1000];
    
    for size in sizes {
        let matrix_a: Vec<Vec<f64>> = (0..size)
            .map(|i| (0..size).map(|j| (i * size + j) as f64 * 0.001).collect())
            .collect();
        
        let matrix_b: Vec<Vec<f64>> = (0..size)
            .map(|i| (0..size).map(|j| ((i + j) * size) as f64 * 0.002).collect())
            .collect();

        c.bench_with_input(
            BenchmarkId::new("single_thread_matrix_multiply", size),
            &size,
            |bench, _| {
                bench.iter(|| {
                    let mut result = vec![vec![0.0; size]; size];
                    for i in 0..size {
                        for j in 0..size {
                            for k in 0..size {
                                result[i][j] += black_box(matrix_a[i][k]) * black_box(matrix_b[k][j]);
                            }
                        }
                    }
                    black_box(result)
                })
            },
        );

        c.bench_with_input(
            BenchmarkId::new("rayon_parallel_matrix_multiply", size),
            &size,
            |bench, _| {
                bench.iter(|| {
                    let result: Vec<Vec<f64>> = (0..size)
                        .into_par_iter()
                        .map(|i| {
                            (0..size)
                                .map(|j| {
                                    (0..size)
                                        .map(|k| black_box(matrix_a[i][k]) * black_box(matrix_b[k][j]))
                                        .sum()
                                })
                                .collect()
                        })
                        .collect();
                    black_box(result)
                })
            },
        );
    }
}

fn benchmark_parallel_statistical_operations(c: &mut Criterion) {
    let data: Vec<f64> = (0..1_000_000).map(|i| (i as f64 * 0.001).sin()).collect();
    
    c.bench_function("single_thread_mean_variance", |b| {
        b.iter(|| {
            let data = black_box(&data);
            let mean = data.iter().sum::<f64>() / data.len() as f64;
            let variance = data.iter()
                .map(|x| (x - mean).powi(2))
                .sum::<f64>() / data.len() as f64;
            black_box((mean, variance))
        })
    });

    c.bench_function("rayon_parallel_mean_variance", |b| {
        b.iter(|| {
            let data = black_box(&data);
            let mean = data.par_iter().sum::<f64>() / data.len() as f64;
            let variance = data.par_iter()
                .map(|x| (x - mean).powi(2))
                .sum::<f64>() / data.len() as f64;
            black_box((mean, variance))
        })
    });

    c.bench_function("single_thread_histogram", |b| {
        b.iter(|| {
            let data = black_box(&data);
            let mut histogram = vec![0; 100];
            for &value in data {
                let bin = ((value + 1.0) * 50.0) as usize;
                if bin < histogram.len() {
                    histogram[bin] += 1;
                }
            }
            black_box(histogram)
        })
    });

    c.bench_function("rayon_parallel_histogram", |b| {
        b.iter(|| {
            let data = black_box(&data);
            let histogram = data.par_iter()
                .fold(|| vec![0; 100], |mut hist, &value| {
                    let bin = ((value + 1.0) * 50.0) as usize;
                    if bin < hist.len() {
                        hist[bin] += 1;
                    }
                    hist
                })
                .reduce(|| vec![0; 100], |mut a, b| {
                    for (i, &count) in b.iter().enumerate() {
                        a[i] += count;
                    }
                    a
                });
            black_box(histogram)
        })
    });
}

fn benchmark_parallel_decimal_operations(c: &mut Criterion) {
    let decimals: Vec<Decimal> = (0..50_000)
        .map(|i| Decimal::from_str(&format!("{}.{:06}", i, i % 1_000_000)).unwrap())
        .collect();

    c.bench_function("single_thread_decimal_operations", |b| {
        b.iter(|| {
            let result: Vec<Decimal> = black_box(&decimals)
                .iter()
                .map(|&x| x * Decimal::from(2) + Decimal::ONE)
                .collect();
            black_box(result)
        })
    });

    c.bench_function("rayon_parallel_decimal_operations", |b| {
        b.iter(|| {
            let result: Vec<Decimal> = black_box(&decimals)
                .par_iter()
                .map(|&x| x * Decimal::from(2) + Decimal::ONE)
                .collect();
            black_box(result)
        })
    });

    let bigdecimals: Vec<BigDecimal> = (0..10_000)
        .map(|i| BigDecimal::from_str(&format!("{}.{:06}", i, i % 1_000_000)).unwrap())
        .collect();

    c.bench_function("single_thread_bigdecimal_operations", |b| {
        b.iter(|| {
            let result: Vec<BigDecimal> = black_box(&bigdecimals)
                .iter()
                .map(|x| x * &BigDecimal::from(2) + &BigDecimal::from(1))
                .collect();
            black_box(result)
        })
    });

    c.bench_function("rayon_parallel_bigdecimal_operations", |b| {
        b.iter(|| {
            let result: Vec<BigDecimal> = black_box(&bigdecimals)
                .par_iter()
                .map(|x| x * &BigDecimal::from(2) + &BigDecimal::from(1))
                .collect();
            black_box(result)
        })
    });
}

fn benchmark_scalability(c: &mut Criterion) {
    let data: Vec<f64> = (0..1_000_000).map(|i| i as f64 * 0.001).collect();
    let thread_counts = [1, 2, 4, 8, 16];
    
    for thread_count in thread_counts {
        c.bench_with_input(
            BenchmarkId::new("parallel_sum_by_thread_count", thread_count),
            &thread_count,
            |bench, &threads| {
                bench.iter(|| {
                    let pool = rayon::ThreadPoolBuilder::new()
                        .num_threads(threads)
                        .build()
                        .unwrap();
                    
                    let result = pool.install(|| {
                        black_box(&data).par_iter().sum::<f64>()
                    });
                    
                    black_box(result)
                })
            },
        );

        c.bench_with_input(
            BenchmarkId::new("parallel_complex_by_thread_count", thread_count),
            &thread_count,
            |bench, &threads| {
                bench.iter(|| {
                    let pool = rayon::ThreadPoolBuilder::new()
                        .num_threads(threads)
                        .build()
                        .unwrap();
                    
                    let result = pool.install(|| {
                        black_box(&data)
                            .par_iter()
                            .map(|&x| x.sin() * x.cos() + x.sqrt())
                            .sum::<f64>()
                    });
                    
                    black_box(result)
                })
            },
        );
    }
}

fn benchmark_work_stealing(c: &mut Criterion) {
    // Create uneven workload
    let work_sizes: Vec<usize> = vec![1000, 100, 5000, 200, 10000, 50, 2000, 800, 15000, 300];
    
    c.bench_function("single_thread_uneven_work", |b| {
        b.iter(|| {
            let mut results = Vec::new();
            for &size in black_box(&work_sizes) {
                let work_result: f64 = (0..size)
                    .map(|i| (i as f64 * 0.001).sin())
                    .sum();
                results.push(work_result);
            }
            black_box(results)
        })
    });

    c.bench_function("rayon_parallel_uneven_work", |b| {
        b.iter(|| {
            let results: Vec<f64> = black_box(&work_sizes)
                .par_iter()
                .map(|&size| {
                    (0..size)
                        .map(|i| (i as f64 * 0.001).sin())
                        .sum()
                })
                .collect();
            black_box(results)
        })
    });
}

criterion_group!(
    multithreaded_benches,
    benchmark_single_threaded_operations,
    benchmark_rayon_parallel_operations,
    benchmark_manual_threading,
    benchmark_parallel_matrix_operations,
    benchmark_parallel_statistical_operations,
    benchmark_parallel_decimal_operations,
    benchmark_scalability,
    benchmark_work_stealing
);
criterion_main!(multithreaded_benches);
