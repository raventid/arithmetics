use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fixed::{types::I32F32, types::I16F16, types::I64F64};

fn benchmark_fixed_point_addition(c: &mut Criterion) {
    let a_32f32 = I32F32::from_num(1.23456789);
    let b_32f32 = I32F32::from_num(9.87654321);
    
    let a_16f16 = I16F16::from_num(1.234);
    let b_16f16 = I16F16::from_num(9.876);
    
    let a_64f64 = I64F64::from_num(1.23456789012345);
    let b_64f64 = I64F64::from_num(9.87654321012345);

    c.bench_function("fixed_point_i32f32_addition", |b| {
        b.iter(|| {
            let result = black_box(a_32f32) + black_box(b_32f32);
            black_box(result)
        })
    });

    c.bench_function("fixed_point_i16f16_addition", |b| {
        b.iter(|| {
            let result = black_box(a_16f16) + black_box(b_16f16);
            black_box(result)
        })
    });

    c.bench_function("fixed_point_i64f64_addition", |b| {
        b.iter(|| {
            let result = black_box(a_64f64) + black_box(b_64f64);
            black_box(result)
        })
    });
}

fn benchmark_fixed_point_multiplication(c: &mut Criterion) {
    let a_32f32 = I32F32::from_num(1.234);
    let b_32f32 = I32F32::from_num(5.678);
    
    let a_16f16 = I16F16::from_num(1.23);
    let b_16f16 = I16F16::from_num(5.67);
    
    let a_64f64 = I64F64::from_num(1.234567);
    let b_64f64 = I64F64::from_num(5.678901);

    c.bench_function("fixed_point_i32f32_multiplication", |b| {
        b.iter(|| {
            let result = black_box(a_32f32) * black_box(b_32f32);
            black_box(result)
        })
    });

    c.bench_function("fixed_point_i16f16_multiplication", |b| {
        b.iter(|| {
            let result = black_box(a_16f16) * black_box(b_16f16);
            black_box(result)
        })
    });

    c.bench_function("fixed_point_i64f64_multiplication", |b| {
        b.iter(|| {
            let result = black_box(a_64f64) * black_box(b_64f64);
            black_box(result)
        })
    });
}

fn benchmark_fixed_point_division(c: &mut Criterion) {
    let a_32f32 = I32F32::from_num(123.456);
    let b_32f32 = I32F32::from_num(7.89);
    
    let a_16f16 = I16F16::from_num(123.4);
    let b_16f16 = I16F16::from_num(7.8);
    
    let a_64f64 = I64F64::from_num(123.456789);
    let b_64f64 = I64F64::from_num(7.891234);

    c.bench_function("fixed_point_i32f32_division", |b| {
        b.iter(|| {
            let result = black_box(a_32f32) / black_box(b_32f32);
            black_box(result)
        })
    });

    c.bench_function("fixed_point_i16f16_division", |b| {
        b.iter(|| {
            let result = black_box(a_16f16) / black_box(b_16f16);
            black_box(result)
        })
    });

    c.bench_function("fixed_point_i64f64_division", |b| {
        b.iter(|| {
            let result = black_box(a_64f64) / black_box(b_64f64);
            black_box(result)
        })
    });
}

criterion_group!(
    fixed_point_benches,
    benchmark_fixed_point_addition,
    benchmark_fixed_point_multiplication,
    benchmark_fixed_point_division
);
criterion_main!(fixed_point_benches);
