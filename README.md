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

### Input ranges

Inputs are generated as canonical decimal strings (`src/lib.rs`, seeded
LCG — no `rand` dependency) and parsed per type during setup. A string
with a fixed number of decimal places is exactly representable in every
decimal type, so all types start from the same values; the binary types
round once at construction, which is inherent to using them at all.

Two ranges keep every intermediate result representable in **all** types —
f16 tops out at 65 504 and saturates to infinity beyond it:

- **R1** `1.00..=100.00` (2 dp), for `ops` and `convert`: sums ≤ 200,
  products ≤ 10 000, divisors ≥ 1.
- **R2** `0.500..=2.000` (3 dp), for `aggregate`: a 1000-element sum stays
  ≤ 2 000 and a dot product ≤ 4 000.

A single wider range (say 1–1000) looks harmless but silently overflows
f16 on both products and array sums, turning its rows into `inf` — which
benchmarks fast and means nothing.

### Division is not apples-to-apples

Each type computes a quotient to a different width, and that is the
honest out-of-the-box cost, so the `div` group makes no attempt to
normalize it:

- floats and fixed-point round to their fixed bit width;
- `rust_decimal` rounds to 28 significant digits;
- `fastnum` D128 rounds to its 128-bit coefficient (~38 digits);
- `BigDecimal` computes **100 significant digits** by default (a
  compile-time setting, `RUST_BIGDECIMAL_DEFAULT_PRECISION`) — a large
  part of why its `div` row is the slowest.

Related: exact decimal arithmetic grows digits instead of rounding. In the
compound-interest kernel BigDecimal's scale grows by 2 digits per period
(60 fractional digits after 30 periods); the fixed-width types round every
step instead. Same algorithm, structurally different work — that trade is
the point of the comparison. The scenario kernels therefore avoid division
inside loops, so digit growth stays bounded and comparable.

## Running

```sh
cargo bench                     # everything (~6 min)
cargo bench --bench ops         # one suite
cargo bench -- 'div/'           # one group across suites
cargo bench -- 'add/fastnum'    # one benchmark
cargo test                      # precision suite + doctests
cargo bench -- --test           # smoke-run every benchmark once, no timing
```

Criterion writes reports to `target/criterion/<group>/<bench>/`, including
an HTML `report/index.html` per benchmark and comparisons against the
previous run.

## Results

_Measured results land here._

## License

MIT
