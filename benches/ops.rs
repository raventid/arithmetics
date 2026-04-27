//! Scalar arithmetic: the same N element pairs pushed through each type.
//!
//! Times are per element (`Throughput::Elements`), which keeps the measured
//! work well above criterion's per-iteration overhead — a single float add
//! is sub-nanosecond and unmeasurable on its own.

use std::hint::black_box;
use std::time::Duration;

use bigdecimal::BigDecimal;
use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use fastnum::D128;
use fixed::types::{I32F32, I64F64};
use half::{bf16, f16};
use rust_decimal::Decimal;

use arithmetics::{decimal_strings, parse_all};

const N: usize = 1_000;

/// R1 operands, 1.00..=100.00 with two decimal places: sums stay <= 200,
/// products <= 10 000 and divisors >= 1, so every op is in range for every
/// type (f16 tops out at 65 504) and division-by-zero cannot occur.
fn operands() -> (Vec<String>, Vec<String>) {
    (
        decimal_strings(0xA11CE, N, 2, 100, 10_000),
        decimal_strings(0xB0B, N, 2, 100, 10_000),
    )
}

/// Stamp one benchmark: apply `op` to N hoisted element pairs.
macro_rules! bench_binop {
    // Copy types operate by value.
    ($group:expr, $label:expr, $xs:expr, $ys:expr, $op:tt) => {{
        let (xs, ys) = (&$xs, &$ys);
        $group.bench_function($label, |b| {
            b.iter(|| {
                for (x, y) in black_box(xs).iter().zip(black_box(ys).iter()) {
                    black_box(*x $op *y);
                }
            })
        });
    }};
    // Heap-backed types (BigDecimal) operate by reference.
    (ref $group:expr, $label:expr, $xs:expr, $ys:expr, $op:tt) => {{
        let (xs, ys) = (&$xs, &$ys);
        $group.bench_function($label, |b| {
            b.iter(|| {
                for (x, y) in black_box(xs).iter().zip(black_box(ys).iter()) {
                    black_box(x $op y);
                }
            })
        });
    }};
}

/// One benchmark group covering every type under test.
macro_rules! bench_all_types {
    ($c:expr, $name:expr, $op:tt) => {{
        let (sa, sb) = operands();
        let mut group = $c.benchmark_group($name);
        group.throughput(Throughput::Elements(N as u64));
        bench_binop!(
            group,
            "f32",
            parse_all::<f32>(&sa),
            parse_all::<f32>(&sb),
            $op
        );
        bench_binop!(
            group,
            "f64",
            parse_all::<f64>(&sa),
            parse_all::<f64>(&sb),
            $op
        );
        bench_binop!(
            group,
            "f16",
            parse_all::<f16>(&sa),
            parse_all::<f16>(&sb),
            $op
        );
        bench_binop!(
            group,
            "bf16",
            parse_all::<bf16>(&sa),
            parse_all::<bf16>(&sb),
            $op
        );
        bench_binop!(
            group,
            "i32f32",
            parse_all::<I32F32>(&sa),
            parse_all::<I32F32>(&sb),
            $op
        );
        bench_binop!(
            group,
            "i64f64",
            parse_all::<I64F64>(&sa),
            parse_all::<I64F64>(&sb),
            $op
        );
        bench_binop!(
            group,
            "rust_decimal",
            parse_all::<Decimal>(&sa),
            parse_all::<Decimal>(&sb),
            $op
        );
        bench_binop!(
            ref group,
            "bigdecimal",
            parse_all::<BigDecimal>(&sa),
            parse_all::<BigDecimal>(&sb),
            $op
        );
        bench_binop!(
            group,
            "fastnum_d128",
            parse_all::<D128>(&sa),
            parse_all::<D128>(&sb),
            $op
        );
        group.finish();
    }};
}

fn add(c: &mut Criterion) {
    bench_all_types!(c, "add", +);
}

fn mul(c: &mut Criterion) {
    bench_all_types!(c, "mul", *);
}

/// Division cost depends on how many digits each type computes: floats and
/// fixed-point round to their fixed width, rust_decimal to 28 significant
/// digits, fastnum to its 128-bit context, BigDecimal to a default of 100
/// significant digits. That asymmetry is the honest out-of-the-box cost.
fn div(c: &mut Criterion) {
    bench_all_types!(c, "div", /);
}

fn config() -> Criterion {
    Criterion::default()
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(2))
}

criterion_group! {
    name = benches;
    config = config();
    targets = add, mul, div
}
criterion_main!(benches);
