//! Boundary crossings: string parsing/formatting and f64 round-trips.
//!
//! Conversion is measured here as the operation under test, on hoisted
//! inputs — never as hidden setup inside another benchmark's loop.

use std::hint::black_box;
use std::time::Duration;

use bigdecimal::BigDecimal;
use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use fastnum::D128;
use fixed::types::I32F32;
use half::{bf16, f16};
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;

use arithmetics::{decimal_strings, parse_all};

const N: usize = 1_000;

/// R1 inputs, 1.00..=100.00 with two decimal places.
fn strings() -> Vec<String> {
    decimal_strings(0xCAFE, N, 2, 100, 10_000)
}

macro_rules! bench_parse {
    ($group:expr, $label:expr, $t:ty, $strs:expr) => {{
        let strs = &$strs;
        $group.bench_function($label, |b| {
            b.iter(|| {
                for s in black_box(strs) {
                    black_box(s.parse::<$t>().unwrap());
                }
            })
        });
    }};
}

macro_rules! bench_display {
    ($group:expr, $label:expr, $vals:expr) => {{
        let vals = &$vals;
        $group.bench_function($label, |b| {
            b.iter(|| {
                for v in black_box(vals) {
                    black_box(v.to_string());
                }
            })
        });
    }};
}

/// Apply a conversion closure to every element of a hoisted vector.
macro_rules! bench_conv {
    ($group:expr, $label:expr, $vals:expr, $f:expr) => {{
        let vals = &$vals;
        let f = $f;
        $group.bench_function($label, |b| {
            b.iter(|| {
                for v in black_box(vals) {
                    black_box(f(v));
                }
            })
        });
    }};
}

fn parse(c: &mut Criterion) {
    let strs = strings();
    let mut group = c.benchmark_group("parse");
    group.throughput(Throughput::Elements(N as u64));
    bench_parse!(group, "f64", f64, strs);
    bench_parse!(group, "f16", f16, strs);
    bench_parse!(group, "bf16", bf16, strs);
    bench_parse!(group, "i32f32", I32F32, strs);
    bench_parse!(group, "rust_decimal", Decimal, strs);
    bench_parse!(group, "bigdecimal", BigDecimal, strs);
    bench_parse!(group, "fastnum_d128", D128, strs);
    group.finish();
}

fn display(c: &mut Criterion) {
    let strs = strings();
    let mut group = c.benchmark_group("display");
    group.throughput(Throughput::Elements(N as u64));
    bench_display!(group, "f64", parse_all::<f64>(&strs));
    bench_display!(group, "f16", parse_all::<f16>(&strs));
    bench_display!(group, "bf16", parse_all::<bf16>(&strs));
    bench_display!(group, "i32f32", parse_all::<I32F32>(&strs));
    bench_display!(group, "rust_decimal", parse_all::<Decimal>(&strs));
    bench_display!(group, "bigdecimal", parse_all::<BigDecimal>(&strs));
    bench_display!(group, "fastnum_d128", parse_all::<D128>(&strs));
    group.finish();
}

fn from_f64(c: &mut Criterion) {
    let vals: Vec<f64> = parse_all(&strings());
    let mut group = c.benchmark_group("from_f64");
    group.throughput(Throughput::Elements(N as u64));
    bench_conv!(group, "f16", vals, |x: &f64| f16::from_f64(*x));
    bench_conv!(group, "bf16", vals, |x: &f64| bf16::from_f64(*x));
    bench_conv!(group, "i32f32", vals, |x: &f64| I32F32::from_num(*x));
    bench_conv!(group, "rust_decimal", vals, |x: &f64| Decimal::from_f64(*x).unwrap());
    bench_conv!(group, "bigdecimal", vals, |x: &f64| BigDecimal::try_from(*x).unwrap());
    bench_conv!(group, "fastnum_d128", vals, |x: &f64| D128::from_f64(*x));
    group.finish();
}

fn to_f64(c: &mut Criterion) {
    let strs = strings();
    let mut group = c.benchmark_group("to_f64");
    group.throughput(Throughput::Elements(N as u64));
    bench_conv!(group, "f16", parse_all::<f16>(&strs), |x: &f16| x.to_f64());
    bench_conv!(group, "bf16", parse_all::<bf16>(&strs), |x: &bf16| x.to_f64());
    bench_conv!(group, "i32f32", parse_all::<I32F32>(&strs), |x: &I32F32| {
        x.to_num::<f64>()
    });
    bench_conv!(group, "rust_decimal", parse_all::<Decimal>(&strs), |x: &Decimal| {
        x.to_f64().unwrap()
    });
    bench_conv!(group, "bigdecimal", parse_all::<BigDecimal>(&strs), |x: &BigDecimal| {
        x.to_f64().unwrap()
    });
    bench_conv!(group, "fastnum_d128", parse_all::<D128>(&strs), |x: &D128| x.to_f64());
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
    targets = parse, display, from_f64, to_f64
}
criterion_main!(benches);
