# Arithmetic Libraries Comparison

A comprehensive benchmarking and comparison project for Rust arithmetic libraries.

## Overview

This project compares performance, precision, and memory usage of various Rust arithmetic libraries:

- **f64**: Standard floating-point arithmetic (baseline)
- **rust_decimal**: High-precision decimal arithmetic 
- **bigdecimal**: Arbitrary precision decimal arithmetic
- **decimal**: IEEE 754 decimal128 standard implementation

## Getting Started

```bash
# Run the example
cargo run

# Run benchmarks
cargo bench

# Run benchmark script with summary
./benchmark.sh
```

## Current Implementation Status

✅ **Completed:**
- Basic project structure
- Addition and multiplication benchmarks
- rust_decimal and bigdecimal integration
- Performance analysis script

🚧 **In Progress:**
- Division operation benchmarks
- decimal128 integration
- Memory usage analysis
- Precision comparison tests

📋 **Roadmap:**
- Fixed-point arithmetic libraries
- Half-precision floating point
- Complex operations (sqrt, sin, power)
- Statistical analysis of results
- CI/CD integration

## Benchmark Results

Preliminary results show significant performance differences:
- **f64**: Baseline performance
- **rust_decimal**: ~15-20x slower than f64
- **bigdecimal**: ~300-400x slower than f64

## Libraries Tested

- **rust_decimal**: Fixed-precision decimal arithmetic for financial calculations
- **bigdecimal**: Variable-precision decimal arithmetic for maximum accuracy  
- **decimal**: IEEE 754 decimal128 standard for portable decimal arithmetic

## Usage

```rust
use rust_decimal::Decimal;
use bigdecimal::BigDecimal;
use decimal::d128;

// High-precision calculations
let decimal_result = Decimal::from_str("1.23")? + Decimal::from_str("4.56")?;
let big_result = BigDecimal::from_str("1.23")? + BigDecimal::from_str("4.56")?;
let d128_result = d128!(1.23) + d128!(4.56);
```
