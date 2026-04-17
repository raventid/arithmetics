# arithmetics

Microbenchmarks comparing Rust numeric types — native floats, decimals,
binary fixed-point and half-precision floats — for speed, per-value memory
cost, and (in the test suite) where each representation is exact and where
it drifts.

## Types under test

| Label | Type | Crate | Stack size | Representation |
|---|---|---|---|---|
| `f32` / `f64` | built-in | — | 4 / 8 B | IEEE 754 binary floating point |
| `f16` | `half::f16` | half 2.7.1 | 2 B | IEEE 754 binary16 |
| `bf16` | `half::bf16` | half 2.7.1 | 2 B | bfloat16 (truncated f32) |
| `i32f32` | `fixed::types::I32F32` | fixed 1.31.0 | 8 B | binary fixed-point, 32 int + 32 frac bits |
| `i64f64` | `fixed::types::I64F64` | fixed 1.31.0 | 16 B | binary fixed-point, 64 int + 64 frac bits |
| `rust_decimal` | `rust_decimal::Decimal` | rust_decimal 1.42.1 | 16 B | decimal, 96-bit mantissa, 28 significant digits |
| `bigdecimal` | `bigdecimal::BigDecimal` | bigdecimal 0.4.10 | handle + heap | decimal, arbitrary precision, allocates per value |
| `fastnum_d128` | `fastnum::D128` | fastnum 0.7.5 | 24 B | decimal, 128-bit coefficient, ~38 significant digits |

The `decimal` (d128) crate this repository used to include was dropped in
the rewrite: unmaintained since 2018, its C build no longer compiles on
current stable Rust.

## What is measured

| Bench file | Groups | Workload |
|---|---|---|
| `benches/ops.rs` | `add`, `mul`, `div` | one scalar operation over 1000 hoisted element pairs |
| `benches/aggregate.rs` | `sum`, `dot` | array reductions with each type's own accumulator (plus f32-accumulator variants for f16/bf16) |
| `benches/convert.rs` | `parse`, `display`, `from_f64`, `to_f64` | boundary crossings: strings and f64 round-trips |
| `benches/real_world.rs` | `compound_interest`, `invoice_total`, `fir_filter` | small matched-algorithm application kernels |

`tests/precision.rs` covers the accuracy side: accumulation drift,
0.1 + 0.2 representability, (1/3) × 3 round-trips, cross-type agreement on
the compound-interest kernel, and per-value memory sizes.

## Methodology

- Setup (parsing, array construction) happens outside the timed closure;
  only the operation under test runs inside `b.iter()`.
- No single-op timing: a lone float add is sub-nanosecond, below the
  harness's own overhead. Every group iterates over 1000-element hoisted
  arrays and declares `Throughput::Elements`, so criterion reports a
  per-element rate.
- Inputs and results pass through `std::hint::black_box`, so the compiler
  can neither constant-fold the arithmetic nor discard it.
- Every comparison group runs the identical algorithm on identical inputs.
  The compound-interest kernel is an explicit loop for the floats too — no
  closed-form `powi` shortcut that would change the op count.
- Benchmark state is re-initialized inside the timed closure; nothing
  accumulates across iterations.
- BigDecimal's per-operation heap allocation is deliberately left inside
  the timed loop: allocation *is* part of that type's cost.
- Single-threaded throughout. The previous multithreaded suite mostly
  measured thread-pool construction, not arithmetic.
- `[profile.bench]` pins `lto = true, codegen-units = 1` and `Cargo.lock`
  is committed, so numbers are reproducible for a given machine and
  toolchain.

## Results

_Measured results land here._

## License

MIT
