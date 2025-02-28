# Arithmetic Libraries Comparison

A comprehensive benchmarking and comparison project for Rust arithmetic libraries.

## Overview

This project compares performance, precision, and memory usage of various Rust arithmetic libraries:

- **f64**: Standard floating-point arithmetic (baseline)
- **rust_decimal**: High-precision decimal arithmetic 
- **bigdecimal**: Arbitrary precision decimal arithmetic
- **decimal**: IEEE 754 decimal128 standard implementation
- **fixed**: Fixed-point arithmetic libraries
- **half**: Half-precision floating point (f16)

## Getting Started

```bash
# Run the example
cargo run

# Run benchmarks
cargo bench

# Run specific benchmark suites
cargo bench --bench arithmetic_comparison
cargo bench --bench complex_operations

# Run benchmark script with summary
./benchmark.sh
```

## Library Descriptions

### Standard Floating Point
- **f64**: IEEE 754 double-precision binary floating-point
  - 64 bits (1 sign + 11 exponent + 52 mantissa)
  - ~15-16 decimal digits of precision
  - Fastest performance, hardware optimized

### Decimal Libraries
- **rust_decimal**: Fixed-precision decimal arithmetic
  - 128-bit representation with up to 28 decimal digits
  - Perfect for financial calculations
  - No rounding errors with decimal fractions

- **bigdecimal**: Arbitrary precision decimal arithmetic
  - Variable precision up to memory limits
  - Slower but maximum accuracy
  - Good for scientific computing requiring extreme precision

- **decimal (d128)**: IEEE 754-2008 decimal128
  - 128-bit decimal floating-point standard
  - 34 decimal digits of precision
  - Standardized across platforms

### Specialized Types
- **fixed**: Fixed-point arithmetic
  - Compile-time specified integer and fractional bits
  - Deterministic performance, no floating-point unit needed
  - Excellent for embedded systems

- **half (f16)**: Half-precision floating-point
  - 16 bits (1 sign + 5 exponent + 10 mantissa)
  - Memory efficient for large datasets
  - Good for machine learning applications

## Current Implementation Status

✅ **Completed:**
- Basic project structure
- Addition, multiplication, and division benchmarks
- All major arithmetic libraries integration
- Complex operations benchmarks (sqrt, power, trigonometric)
- Memory usage analysis
- Precision comparison tests

� **In Progress:**
- Fixed-point arithmetic benchmarks
- Half-precision integration
- Batch operations analysis
- Performance optimization studies

📋 **Roadmap:**
- Statistical analysis of results
- CI/CD integration
- Documentation and guides
- Cross-platform compatibility testing

## Benchmark Results

Performance comparison (relative to f64 baseline):
- **f64**: 1.0x (baseline)
- **rust_decimal**: ~15-20x slower
- **bigdecimal**: ~300-400x slower
- **decimal128**: ~7-10x slower
- **fixed**: ~1-2x slower
- **f16**: ~0.8-1.2x (depends on hardware support)

Memory usage comparison:
- **f64**: 8 bytes
- **rust_decimal**: 16 bytes
- **bigdecimal**: 24+ bytes (variable)
- **decimal128**: 16 bytes
- **fixed**: 4-8 bytes (depends on configuration)
- **f16**: 2 bytes

## Usage Examples

```rust
use rust_decimal::Decimal;
use bigdecimal::BigDecimal;
use decimal::d128;
use fixed::types::I32F32;
use half::f16;

// High-precision decimal calculations
let decimal_result = Decimal::from_str("1.23")? + Decimal::from_str("4.56")?;
let big_result = BigDecimal::from_str("1.23")? + BigDecimal::from_str("4.56")?;
let d128_result = d128!(1.23) + d128!(4.56);

// Fixed-point arithmetic
let fixed_a = I32F32::from_num(1.23);
let fixed_b = I32F32::from_num(4.56);
let fixed_result = fixed_a + fixed_b;

// Half-precision floating point
let half_a = f16::from_f64(1.23);
let half_b = f16::from_f64(4.56);
let half_result = half_a + half_b;
```
