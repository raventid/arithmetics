//! Precision behaviour the speed benchmarks cannot show: where each
//! representation is exact, where it drifts, and by how much.

use std::str::FromStr;

use bigdecimal::BigDecimal;
use fastnum::D128;
use fixed::types::I32F32;
use half::f16;
use rust_decimal::Decimal;

const STEPS: usize = 10_000;

/// 0.1 added 10 000 times is exactly 1000 in every decimal-radix type.
#[test]
fn accumulation_decimals_are_exact() {
    let step = Decimal::from_str("0.1").unwrap();
    let mut sum = Decimal::ZERO;
    for _ in 0..STEPS {
        sum = sum + step;
    }
    assert_eq!(sum, Decimal::from(1000));

    let step = BigDecimal::from_str("0.1").unwrap();
    let mut sum = BigDecimal::from(0);
    for _ in 0..STEPS {
        sum = sum + &step;
    }
    assert_eq!(sum, BigDecimal::from(1000));

    let step: D128 = "0.1".parse().unwrap();
    let mut sum = D128::ZERO;
    for _ in 0..STEPS {
        sum = sum + step;
    }
    assert_eq!(sum, "1000".parse::<D128>().unwrap());
}

/// f64 cannot represent 0.1, so the sum drifts — but only in the 8th
/// decimal place or so after 10 000 additions.
#[test]
fn accumulation_f64_drifts() {
    let mut sum = 0.0_f64;
    for _ in 0..STEPS {
        sum += 0.1;
    }
    let err = (sum - 1000.0).abs();
    assert!(err > 0.0 && err < 1e-6, "f64 accumulated error: {err:e}");
}

/// f32's 24-bit mantissa drifts far sooner.
#[test]
fn accumulation_f32_drifts_more() {
    let mut sum = 0.0_f32;
    for _ in 0..STEPS {
        sum += 0.1;
    }
    let err = (sum - 1000.0).abs();
    assert!(err > 1e-4 && err < 0.5, "f32 accumulated error: {err:e}");
}

/// Binary fixed-point cannot represent 0.1 either; with 32 fractional bits
/// the per-step error is ~2.3e-11, so 10 000 steps land within 1e-6.
#[test]
fn accumulation_i32f32_drifts_slightly() {
    let step = I32F32::from_str("0.1").unwrap();
    let mut sum = I32F32::ZERO;
    for _ in 0..STEPS {
        sum = sum + step;
    }
    let err = (sum.to_num::<f64>() - 1000.0).abs();
    assert!(err > 0.0 && err < 1e-6, "i32f32 accumulated error: {err:e}");
}

/// f16 stalls: once the sum reaches 256, the spacing between consecutive
/// f16 values is 0.25, so adding 0.1 rounds back to the same value and the
/// sum never moves again — it ends at 256, not 1000.
#[test]
fn accumulation_f16_stalls() {
    let step = f16::from_f32(0.1);
    let mut sum = f16::ZERO;
    for _ in 0..STEPS {
        sum = sum + step;
    }
    assert_eq!(sum, f16::from_f32(256.0));
}
