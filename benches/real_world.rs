//! Matched-algorithm application scenarios. Every type runs the *identical*
//! algorithm on identical hoisted inputs, and per-iteration state is
//! re-initialized inside the timed closure so nothing leaks between
//! iterations.

use std::hint::black_box;
use std::time::Duration;

use bigdecimal::BigDecimal;
use criterion::{criterion_group, criterion_main, Criterion};
use fastnum::D128;
use fixed::types::{I32F32, I64F64};
use half::{bf16, f16};
use rust_decimal::Decimal;

/// Compounding periods for the interest scenario.
const PERIODS: usize = 30;

/// 1000.00 at 5% over 30 annual periods, iterated for every type — no
/// closed-form `powi` shortcut for the floats, so all types perform the
/// same 30 mul + 30 add. The final balance (~4321.94) fits f16's range.
macro_rules! bench_compound {
    ($group:expr, $label:expr, $principal:expr, $rate:expr) => {{
        let (principal, rate) = ($principal, $rate);
        $group.bench_function($label, |b| {
            b.iter(|| {
                let mut balance = black_box(principal);
                let rate = black_box(rate);
                for _ in 0..PERIODS {
                    balance = balance + balance * rate;
                }
                black_box(balance)
            })
        });
    }};
    (ref $group:expr, $label:expr, $principal:expr, $rate:expr) => {{
        let (principal, rate) = ($principal, $rate);
        $group.bench_function($label, |b| {
            b.iter(|| {
                let mut balance = black_box(&principal).clone();
                let rate = black_box(&rate);
                for _ in 0..PERIODS {
                    balance = &balance + &balance * rate;
                }
                black_box(balance)
            })
        });
    }};
}

fn compound_interest(c: &mut Criterion) {
    const P: &str = "1000.00";
    const R: &str = "0.05";
    let mut group = c.benchmark_group("compound_interest");
    group.sample_size(50);
    bench_compound!(group, "f32", P.parse::<f32>().unwrap(), R.parse::<f32>().unwrap());
    bench_compound!(group, "f64", P.parse::<f64>().unwrap(), R.parse::<f64>().unwrap());
    bench_compound!(group, "f16", P.parse::<f16>().unwrap(), R.parse::<f16>().unwrap());
    bench_compound!(group, "bf16", P.parse::<bf16>().unwrap(), R.parse::<bf16>().unwrap());
    bench_compound!(group, "i32f32", P.parse::<I32F32>().unwrap(), R.parse::<I32F32>().unwrap());
    bench_compound!(group, "i64f64", P.parse::<I64F64>().unwrap(), R.parse::<I64F64>().unwrap());
    bench_compound!(group, "rust_decimal", P.parse::<Decimal>().unwrap(), R.parse::<Decimal>().unwrap());
    bench_compound!(ref group, "bigdecimal", P.parse::<BigDecimal>().unwrap(), R.parse::<BigDecimal>().unwrap());
    bench_compound!(group, "fastnum_d128", P.parse::<D128>().unwrap(), R.parse::<D128>().unwrap());
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
    targets = compound_interest
}
criterion_main!(benches);
