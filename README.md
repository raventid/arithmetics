# Arithmetic Libraries Comparison

A comprehensive benchmarking and comparison project for Rust arithmetic libraries.

## Overview

This project compares performance, precision, and memory usage of various Rust arithmetic libraries:

- **f64**: Standard floating-point arithmetic
- **rust_decimal**: High-precision decimal arithmetic
- **bigdecimal**: Arbitrary precision decimal arithmetic

## Getting Started

```bash
# Run the example
cargo run

# Run benchmarks
cargo bench
```

## Benchmark Results

Preliminary results show significant performance differences between libraries.
Full analysis coming soon.

## Libraries Tested

- **rust_decimal**: Fixed-precision decimal arithmetic for financial calculations
- **bigdecimal**: Variable-precision decimal arithmetic for maximum accuracy

## Usage

```rust
use rust_decimal::Decimal;
use bigdecimal::BigDecimal;

// High-precision calculations
let decimal_result = Decimal::from_str("1.23")? + Decimal::from_str("4.56")?;
let big_result = BigDecimal::from_str("1.23")? + BigDecimal::from_str("4.56")?;
```
