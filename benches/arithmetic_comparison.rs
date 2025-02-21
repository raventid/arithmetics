use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_decimal::Decimal;
use bigdecimal::BigDecimal;
use std::str::FromStr;

fn benchmark_addition(c: &mut Criterion) {
    let mut group = c.benchmark_group("addition");
    
    // f64 baseline
    group.bench_function("f64", |b| {
        b.iter(|| {
            let a = black_box(1.23);
            let b = black_box(4.56);
            black_box(a + b)
        })
    });
    
    // rust_decimal
    group.bench_function("rust_decimal", |b| {
        let a = Decimal::from_str("1.23").unwrap();
        let b = Decimal::from_str("4.56").unwrap();
        b.iter(|| {
            black_box(black_box(a) + black_box(b))
        })
    });
    
    // bigdecimal
    group.bench_function("bigdecimal", |b| {
        let a = BigDecimal::from_str("1.23").unwrap();
        let b = BigDecimal::from_str("4.56").unwrap();
        b.iter(|| {
            black_box(&black_box(&a) + &black_box(&b))
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
            let b = black_box(4.56);
            black_box(a * b)
        })
    });
    
    // rust_decimal
    group.bench_function("rust_decimal", |b| {
        let a = Decimal::from_str("1.23").unwrap();
        let b = Decimal::from_str("4.56").unwrap();
        b.iter(|| {
            black_box(black_box(a) * black_box(b))
        })
    });
    
    // bigdecimal
    group.bench_function("bigdecimal", |b| {
        let a = BigDecimal::from_str("1.23").unwrap();
        let b = BigDecimal::from_str("4.56").unwrap();
        b.iter(|| {
            black_box(&black_box(&a) * &black_box(&b))
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
            let b = black_box(5.67);
            black_box(a / b)
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
            black_box(&black_box(&a) / &black_box(&b_val))
        })
    });
    
    group.finish();
}

criterion_group!(benches, benchmark_addition, benchmark_multiplication, benchmark_division);
criterion_main!(benches);