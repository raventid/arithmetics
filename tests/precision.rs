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

/// The textbook example: 0.1 + 0.2 is exactly 0.3 in decimal radix.
#[test]
fn zero_one_plus_zero_two_decimal_radix() {
    let sum = Decimal::from_str("0.1").unwrap() + Decimal::from_str("0.2").unwrap();
    assert_eq!(sum, Decimal::from_str("0.3").unwrap());

    let sum = BigDecimal::from_str("0.1").unwrap() + BigDecimal::from_str("0.2").unwrap();
    assert_eq!(sum, BigDecimal::from_str("0.3").unwrap());

    let sum = "0.1".parse::<D128>().unwrap() + "0.2".parse::<D128>().unwrap();
    assert_eq!(sum, "0.3".parse::<D128>().unwrap());
}

/// ...and famously not 0.3 in f64, where 0.1 + 0.2 is
/// 0.30000000000000004440892098500626.
#[test]
fn zero_one_plus_zero_two_f64() {
    assert_ne!(0.1_f64 + 0.2_f64, 0.3_f64);
}

/// Binary fixed-point cannot represent 0.1 / 0.2 / 0.3 either, but with
/// 32 fractional bits the three rounding errors happen to cancel, so the
/// comparison holds by coincidence. That 0.1 itself is inexact shows as
/// soon as you multiply: 0.1 × 10 ≠ 1.
#[test]
fn zero_one_plus_zero_two_i32f32() {
    let a = I32F32::from_str("0.1").unwrap();
    let b = I32F32::from_str("0.2").unwrap();
    let c = I32F32::from_str("0.3").unwrap();
    assert_eq!(a + b, c);
    assert_ne!(a * I32F32::from_num(10), I32F32::ONE);
}
