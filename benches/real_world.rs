//! Matched-algorithm application scenarios. Every type runs the *identical*
//! algorithm on identical hoisted inputs, and per-iteration state is
//! re-initialized inside the timed closure so nothing leaks between
//! iterations.

use std::hint::black_box;
use std::time::Duration;

use bigdecimal::{BigDecimal, Zero};
use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use fastnum::D128;
use fixed::types::{I32F32, I64F64};
use half::{bf16, f16};
use rust_decimal::Decimal;

use arithmetics::{decimal_strings, parse_all, Lcg};

/// Compounding periods for the interest scenario.
const PERIODS: usize = 30;

/// Line items on the invoice.
const ITEMS: usize = 200;

/// FIR filter dimensions: TAPS-tap kernel over SAMPLES input samples.
const SAMPLES: usize = 256;
const TAPS: usize = 32;

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
    bench_compound!(
        group,
        "f32",
        P.parse::<f32>().unwrap(),
        R.parse::<f32>().unwrap()
    );
    bench_compound!(
        group,
        "f64",
        P.parse::<f64>().unwrap(),
        R.parse::<f64>().unwrap()
    );
    bench_compound!(
        group,
        "f16",
        P.parse::<f16>().unwrap(),
        R.parse::<f16>().unwrap()
    );
    bench_compound!(
        group,
        "bf16",
        P.parse::<bf16>().unwrap(),
        R.parse::<bf16>().unwrap()
    );
    bench_compound!(
        group,
        "i32f32",
        P.parse::<I32F32>().unwrap(),
        R.parse::<I32F32>().unwrap()
    );
    bench_compound!(
        group,
        "i64f64",
        P.parse::<I64F64>().unwrap(),
        R.parse::<I64F64>().unwrap()
    );
    bench_compound!(
        group,
        "rust_decimal",
        P.parse::<Decimal>().unwrap(),
        R.parse::<Decimal>().unwrap()
    );
    bench_compound!(
        ref group,
        "bigdecimal",
        P.parse::<BigDecimal>().unwrap(),
        R.parse::<BigDecimal>().unwrap()
    );
    bench_compound!(
        group,
        "fastnum_d128",
        P.parse::<D128>().unwrap(),
        R.parse::<D128>().unwrap()
    );
    group.finish();
}

/// Sum of 200 qty × unit-price line items plus one final division for the
/// average line value. Quantities are small integers (1..=5), prices are
/// 1.00..=20.00, so the grand total tops out at 20 000 — still f16 range.
macro_rules! bench_invoice {
    ($group:expr, $label:expr, $qtys:expr, $prices:expr, $zero:expr, $count:expr) => {{
        let (qtys, prices, count) = (&$qtys, &$prices, $count);
        $group.bench_function($label, |b| {
            b.iter(|| {
                let mut total = $zero;
                for (q, p) in black_box(qtys).iter().zip(black_box(prices).iter()) {
                    total += *q * *p;
                }
                black_box(total / count)
            })
        });
    }};
    (ref $group:expr, $label:expr, $qtys:expr, $prices:expr, $zero:expr, $count:expr) => {{
        let (qtys, prices, count) = (&$qtys, &$prices, &$count);
        $group.bench_function($label, |b| {
            b.iter(|| {
                let mut total = $zero;
                for (q, p) in black_box(qtys).iter().zip(black_box(prices).iter()) {
                    total += q * p;
                }
                black_box(&total / count)
            })
        });
    }};
}

fn invoice_total(c: &mut Criterion) {
    let mut rng = Lcg::new(0x1CE);
    let qty_strings: Vec<String> = (0..ITEMS)
        .map(|_| (1 + rng.next_u64() % 5).to_string())
        .collect();
    let price_strings = decimal_strings(0xF00D, ITEMS, 2, 100, 2_000);
    let count = ITEMS.to_string();

    let mut group = c.benchmark_group("invoice_total");
    group.sample_size(50);
    group.throughput(Throughput::Elements(ITEMS as u64));
    macro_rules! row {
        ($label:expr, $t:ty, $zero:expr) => {
            bench_invoice!(
                group,
                $label,
                parse_all::<$t>(&qty_strings),
                parse_all::<$t>(&price_strings),
                $zero,
                count.parse::<$t>().unwrap()
            )
        };
        (ref $label:expr, $t:ty, $zero:expr) => {
            bench_invoice!(
                ref group,
                $label,
                parse_all::<$t>(&qty_strings),
                parse_all::<$t>(&price_strings),
                $zero,
                count.parse::<$t>().unwrap()
            )
        };
    }
    row!("f32", f32, 0.0f32);
    row!("f64", f64, 0.0f64);
    row!("f16", f16, f16::ZERO);
    row!("bf16", bf16, bf16::ZERO);
    row!("i32f32", I32F32, I32F32::ZERO);
    row!("i64f64", I64F64, I64F64::ZERO);
    row!("rust_decimal", Decimal, Decimal::ZERO);
    row!(ref "bigdecimal", BigDecimal, BigDecimal::zero());
    row!("fastnum_d128", D128, D128::ZERO);
    group.finish();
}

/// 32-tap FIR filter over a 256-sample signal — the classic fixed-point
/// versus float DSP workload; the decimal types are included to quantify
/// what exact arithmetic costs in a kernel like this. Signal values sit in
/// 0.001..=0.999 and taps in 0.001..=0.062 (kernel sum <= 2), so every
/// accumulator stays around 2 — comfortably in range for every type.
macro_rules! bench_fir {
    ($group:expr, $label:expr, $signal:expr, $taps:expr, $zero:expr) => {{
        let (signal, taps) = (&$signal, &$taps);
        $group.bench_function($label, |b| {
            b.iter(|| {
                let signal = black_box(signal);
                let taps = black_box(taps);
                for i in 0..(SAMPLES - TAPS) {
                    let mut acc = $zero;
                    for (k, t) in taps.iter().enumerate() {
                        acc += signal[i + k] * *t;
                    }
                    black_box(acc);
                }
            })
        });
    }};
    (ref $group:expr, $label:expr, $signal:expr, $taps:expr, $zero:expr) => {{
        let (signal, taps) = (&$signal, &$taps);
        $group.bench_function($label, |b| {
            b.iter(|| {
                let signal = black_box(signal);
                let taps = black_box(taps);
                for i in 0..(SAMPLES - TAPS) {
                    let mut acc = $zero;
                    for (k, t) in taps.iter().enumerate() {
                        acc += &signal[i + k] * t;
                    }
                    black_box(acc);
                }
            })
        });
    }};
}

fn fir_filter(c: &mut Criterion) {
    let signal_strings = decimal_strings(0x51C, SAMPLES, 3, 1, 999);
    let tap_strings = decimal_strings(0x7A9, TAPS, 3, 1, 62);

    let mut group = c.benchmark_group("fir_filter");
    // Iterations here are long (BigDecimal ~150 µs) and stable; 30 samples
    // keeps the whole suite's wall time down without hurting the estimate.
    group.sample_size(30);
    group.throughput(Throughput::Elements((SAMPLES - TAPS) as u64));
    macro_rules! row {
        ($label:expr, $t:ty, $zero:expr) => {
            bench_fir!(
                group,
                $label,
                parse_all::<$t>(&signal_strings),
                parse_all::<$t>(&tap_strings),
                $zero
            )
        };
        (ref $label:expr, $t:ty, $zero:expr) => {
            bench_fir!(
                ref group,
                $label,
                parse_all::<$t>(&signal_strings),
                parse_all::<$t>(&tap_strings),
                $zero
            )
        };
    }
    row!("f32", f32, 0.0f32);
    row!("f64", f64, 0.0f64);
    row!("f16", f16, f16::ZERO);
    row!("bf16", bf16, bf16::ZERO);
    row!("i32f32", I32F32, I32F32::ZERO);
    row!("i64f64", I64F64, I64F64::ZERO);
    row!("rust_decimal", Decimal, Decimal::ZERO);
    row!(ref "bigdecimal", BigDecimal, BigDecimal::zero());
    row!("fastnum_d128", D128, D128::ZERO);
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
    targets = compound_interest, invoice_total, fir_filter
}
criterion_main!(benches);
