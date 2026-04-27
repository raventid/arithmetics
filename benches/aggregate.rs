//! Array reductions: sum and dot product, folded with each type's own
//! accumulator. Single-threaded on purpose — this suite measures
//! arithmetic, not thread-pool overhead.

use std::hint::black_box;
use std::time::Duration;

use bigdecimal::{BigDecimal, Zero};
use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use fastnum::D128;
use fixed::types::{I32F32, I64F64};
use half::{bf16, f16};
use rust_decimal::Decimal;

use arithmetics::{decimal_strings, parse_all};

const N: usize = 1_000;

/// R2 operands, 0.500..=2.000 with three decimal places: a 1000-element sum
/// stays <= 2 000 and a dot product <= 4 000, inside f16 range (max 65 504).
fn operands() -> (Vec<String>, Vec<String>) {
    (
        decimal_strings(0xD07, N, 3, 500, 2_000),
        decimal_strings(0x5EED, N, 3, 500, 2_000),
    )
}

macro_rules! bench_sum {
    ($group:expr, $label:expr, $xs:expr, $zero:expr) => {{
        let xs = &$xs;
        $group.bench_function($label, |b| {
            b.iter(|| black_box(black_box(xs).iter().fold($zero, |acc, x| acc + *x)))
        });
    }};
    (ref $group:expr, $label:expr, $xs:expr, $zero:expr) => {{
        let xs = &$xs;
        $group.bench_function($label, |b| {
            b.iter(|| black_box(black_box(xs).iter().fold($zero, |acc, x| acc + x)))
        });
    }};
}

macro_rules! bench_dot {
    ($group:expr, $label:expr, $xs:expr, $ys:expr, $zero:expr) => {{
        let (xs, ys) = (&$xs, &$ys);
        $group.bench_function($label, |b| {
            b.iter(|| {
                black_box(
                    black_box(xs)
                        .iter()
                        .zip(black_box(ys).iter())
                        .fold($zero, |acc, (x, y)| acc + *x * *y),
                )
            })
        });
    }};
    (ref $group:expr, $label:expr, $xs:expr, $ys:expr, $zero:expr) => {{
        let (xs, ys) = (&$xs, &$ys);
        $group.bench_function($label, |b| {
            b.iter(|| {
                black_box(
                    black_box(xs)
                        .iter()
                        .zip(black_box(ys).iter())
                        .fold($zero, |acc, (x, y)| acc + x * y),
                )
            })
        });
    }};
}

fn sum(c: &mut Criterion) {
    let (sa, _) = operands();
    let mut group = c.benchmark_group("sum");
    group.throughput(Throughput::Elements(N as u64));
    bench_sum!(group, "f32", parse_all::<f32>(&sa), 0.0f32);
    bench_sum!(group, "f64", parse_all::<f64>(&sa), 0.0f64);
    bench_sum!(group, "f16", parse_all::<f16>(&sa), f16::ZERO);
    bench_sum!(group, "bf16", parse_all::<bf16>(&sa), bf16::ZERO);
    bench_sum!(group, "i32f32", parse_all::<I32F32>(&sa), I32F32::ZERO);
    bench_sum!(group, "i64f64", parse_all::<I64F64>(&sa), I64F64::ZERO);
    bench_sum!(
        group,
        "rust_decimal",
        parse_all::<Decimal>(&sa),
        Decimal::ZERO
    );
    bench_sum!(
        ref group,
        "bigdecimal",
        parse_all::<BigDecimal>(&sa),
        BigDecimal::zero()
    );
    bench_sum!(group, "fastnum_d128", parse_all::<D128>(&sa), D128::ZERO);

    // The pattern ML code actually uses: 16-bit storage, f32 accumulator.
    // Contrast with the plain "f16"/"bf16" rows, whose 16-bit accumulator
    // both costs conversions per step and stalls once the sum outgrows the
    // element magnitude.
    let f16s: Vec<f16> = parse_all(&sa);
    group.bench_function("f16_f32acc", |b| {
        b.iter(|| {
            black_box(
                black_box(&f16s)
                    .iter()
                    .fold(0.0f32, |acc, x| acc + x.to_f32()),
            )
        })
    });
    let bf16s: Vec<bf16> = parse_all(&sa);
    group.bench_function("bf16_f32acc", |b| {
        b.iter(|| {
            black_box(
                black_box(&bf16s)
                    .iter()
                    .fold(0.0f32, |acc, x| acc + x.to_f32()),
            )
        })
    });
    group.finish();
}

fn dot(c: &mut Criterion) {
    let (sa, sb) = operands();
    let mut group = c.benchmark_group("dot");
    group.throughput(Throughput::Elements(N as u64));
    bench_dot!(
        group,
        "f32",
        parse_all::<f32>(&sa),
        parse_all::<f32>(&sb),
        0.0f32
    );
    bench_dot!(
        group,
        "f64",
        parse_all::<f64>(&sa),
        parse_all::<f64>(&sb),
        0.0f64
    );
    bench_dot!(
        group,
        "f16",
        parse_all::<f16>(&sa),
        parse_all::<f16>(&sb),
        f16::ZERO
    );
    bench_dot!(
        group,
        "bf16",
        parse_all::<bf16>(&sa),
        parse_all::<bf16>(&sb),
        bf16::ZERO
    );
    bench_dot!(
        group,
        "i32f32",
        parse_all::<I32F32>(&sa),
        parse_all::<I32F32>(&sb),
        I32F32::ZERO
    );
    bench_dot!(
        group,
        "i64f64",
        parse_all::<I64F64>(&sa),
        parse_all::<I64F64>(&sb),
        I64F64::ZERO
    );
    bench_dot!(
        group,
        "rust_decimal",
        parse_all::<Decimal>(&sa),
        parse_all::<Decimal>(&sb),
        Decimal::ZERO
    );
    bench_dot!(
        ref group,
        "bigdecimal",
        parse_all::<BigDecimal>(&sa),
        parse_all::<BigDecimal>(&sb),
        BigDecimal::zero()
    );
    bench_dot!(
        group,
        "fastnum_d128",
        parse_all::<D128>(&sa),
        parse_all::<D128>(&sb),
        D128::ZERO
    );
    group.finish();
}

fn config() -> Criterion {
    Criterion::default()
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(2))
}

criterion_group! {
    name = benches;
    config = config();
    targets = sum, dot
}
criterion_main!(benches);
