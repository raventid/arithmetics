use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_decimal::Decimal;
use bigdecimal::BigDecimal;
use decimal::d128;
use std::str::FromStr;

fn benchmark_addition(c: &mut Criterion) {
    let mut group = c.benchmark_group("addition");
    
    // f64 baseline
    group.bench_function("f64", |b| {
        b.iter(|| {
            let a = black_box(1.23);
            let b_val = black_box(4.56);
            black_box(a + b_val)
        })
    });
    
    // rust_decimal
    group.bench_function("rust_decimal", |b| {
        let a = Decimal::from_str("1.23").unwrap();
        let b_val = Decimal::from_str("4.56").unwrap();
        b.iter(|| {
            black_box(black_box(a) + black_box(b_val))
        })
    });
    
    // bigdecimal
    group.bench_function("bigdecimal", |b| {
        let a = BigDecimal::from_str("1.23").unwrap();
        let b_val = BigDecimal::from_str("4.56").unwrap();
        b.iter(|| {
            black_box(black_box(&a) + black_box(&b_val))
        })
    });
    
    // decimal128
    group.bench_function("decimal128", |b| {
        let a = d128!(1.23);
        let b_val = d128!(4.56);
        b.iter(|| {
            black_box(black_box(a) + black_box(b_val))
        })
    });
    
    group.finish();
}

fn benchmark_multiplication(c: &mut Criterion) {
    let mut group = c.benchmark_group("multiplication");
    
    // f64 baseline
    group.bench_function("f64", |b| {
        b.iter(|| {
            let a = black_box(1.23);
            let b_val = black_box(4.56);
            black_box(a * b_val)
        })
    });
    
    // rust_decimal
    group.bench_function("rust_decimal", |b| {
        let a = Decimal::from_str("1.23").unwrap();
        let b_val = Decimal::from_str("4.56").unwrap();
        b.iter(|| {
            black_box(black_box(a) * black_box(b_val))
        })
    });
    
    // bigdecimal
    group.bench_function("bigdecimal", |b| {
        let a = BigDecimal::from_str("1.23").unwrap();
        let b_val = BigDecimal::from_str("4.56").unwrap();
        b.iter(|| {
            black_box(black_box(&a) * black_box(&b_val))
        })
    });
    
    // decimal128
    group.bench_function("decimal128", |b| {
        let a = d128!(1.23);
        let b_val = d128!(4.56);
        b.iter(|| {
            black_box(black_box(a) * black_box(b_val))
        })
    });
    
    group.finish();
}

fn benchmark_division(c: &mut Criterion) {
    let mut group = c.benchmark_group("division");
    
    // f64 baseline
    group.bench_function("f64", |b| {
        b.iter(|| {
            let a = black_box(12.34);
            let b_val = black_box(5.67);
            black_box(a / b_val)
        })
    });
    
    // rust_decimal
    group.bench_function("rust_decimal", |b| {
        let a = Decimal::from_str("12.34").unwrap();
        let b_val = Decimal::from_str("5.67").unwrap();
        b.iter(|| {
            black_box(black_box(a) / black_box(b_val))
        })
    });
    
    // bigdecimal
    group.bench_function("bigdecimal", |b| {
        let a = BigDecimal::from_str("12.34").unwrap();
        let b_val = BigDecimal::from_str("5.67").unwrap();
        b.iter(|| {
            black_box(black_box(&a) / black_box(&b_val))
        })
    });
    
    // decimal128
    group.bench_function("decimal128", |b| {
        let a = d128!(12.34);
        let b_val = d128!(5.67);
        b.iter(|| {
            black_box(black_box(a) / black_box(b_val))
        })
    });
    
    group.finish();
}

criterion_group!(benches, benchmark_addition, benchmark_multiplication, benchmark_division);
criterion_main!(benches);