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

Measured 2026-04-21 on an Apple M4 Pro, rustc 1.96.0, macOS — one machine,
one toolchain; rerun locally before drawing conclusions for your target.

Micro-benchmarks, **ns per element** (lower is better; mid estimate):

| Type | add | mul | div | sum | dot | parse | display | from_f64 | to_f64 |
|---|---|---|---|---|---|---|---|---|---|
| `f32` | 0.26 | 0.26 | 0.26 | 0.44 | 0.45 | — | — | — | — |
| `f64` | 0.26 | 0.26 | 0.26 | 0.44 | 0.47 | 5.75 | 44.0 | — | — |
| `f16` | 0.26 | 0.26 | 0.26 | 3.02 | 3.02 | 5.81 | 54.8 | 0.26 | 0.25 |
| `bf16` | 1.02 | 1.02 | 1.07 | 6.78 | 6.89 | 6.01 | 47.7 | 0.72 | 0.54 |
| `i32f32` | 0.27 | 0.33 | 4.24 | 0.05 | 0.29 | 25.0 | 40.6 | 1.48 | 0.82 |
| `i64f64` | 0.32 | 0.72 | 26.5 | 0.18 | 0.82 | — | — | — | — |
| `rust_decimal` | 1.01 | 1.26 | 15.6 | 1.25 | 1.75 | 3.84 | 26.5 | 142 | 6.85 |
| `bigdecimal` | 29.3 | 13.5 | 3,484 | 13.3 | 18.1 | 43.8 | 57.2 | 158 | 54.7 |
| `fastnum_d128` | 3.35 | 4.06 | 61.3 | 5.33 | 7.28 | 7.35 | 44.9 | 60.5 | 1.76 |
| `f16_f32acc` | — | — | — | 0.58 | — | — | — | — | — |
| `bf16_f32acc` | — | — | — | 0.47 | — | — | — | — | — |

Application kernels (lower is better; mid estimate):

| Type | compound_interest, ns/kernel | invoice_total, ns/item | fir_filter, ns/output |
|---|---|---|---|
| `f32` | 6.43 | 0.26 | 9.36 |
| `f64` | 6.43 | 0.33 | 9.38 |
| `f16` | 6.41 | 2.38 | 29.6 |
| `bf16` | 291 | 6.59 | 142 |
| `i32f32` | 30.5 | 0.35 | 10.3 |
| `i64f64` | 53.9 | 0.98 | 32.9 |
| `rust_decimal` | 921 | 2.81 | 101 |
| `bigdecimal` | 2,110 | 31.2 | 686 |
| `fastnum_d128` | 799 | 7.63 | 243 |

Reading the numbers:

- Between the fixed-width decimals, `rust_decimal` leads arithmetic and
  string handling by ~2–4× over `fastnum_d128`, which wins the f64
  round-trips; `bigdecimal` trails both by 10–100× everywhere.
- `div` spreads the field the most: 0.26 ns (floats) → 15.6 (`rust_decimal`,
  28 digits) → 61 (`fastnum`) → 3,484 (`BigDecimal`, 100 digits by default).
- `i32f32`'s 0.05 ns/element `sum` is real: an integer fold is associative,
  so LLVM vectorizes it — something it may not do for float folds.
- The `f16` rows equal `f32` on scalar ops because this CPU has hardware
  FP16 (see caveat below); `bf16` stays software and pays ~4×.

> **f16 portability caveat.** The `half` crate uses hardware float16
> instructions where the target has them (Apple Silicon does — hence the
> f32-equal rows above) and falls back to software conversion through f32
> elsewhere. On an x86-64 without AVX-512 FP16 expect the `f16` rows to
> look like the `bf16` rows, several times slower than `f32`. The
> precision results are portable; the f16 *speed* results are not.
- `sum/f16` vs `sum/f16_f32acc` (3.02 vs 0.58) is the serial dependency
  chain through a 16-bit accumulator versus the widen-then-fold pattern —
  and per `tests/precision.rs`, the 16-bit accumulator also stalls at 256.

## Precision findings

From `tests/precision.rs` (all asserted, not folklore):

- **0.1 added 10 000 times**: exactly 1000 for all three decimal types.
  f64 lands ~1.6e-8 off, f32 ~3e-2, `i32f32` ~2e-7 — and `f16` stops at
  **256**, because from there its spacing is 0.25 and adding 0.1 rounds to
  a no-op.
- **0.1 + 0.2 == 0.3** holds in every decimal type, famously fails in f64,
  and holds in `i32f32` only by a rounding coincidence — 0.1 is not
  representable there either, as `0.1 × 10 ≠ 1` shows.
- **(1/3) × 3**: f64 returns exactly 1.0 (the multiply's error cancels the
  divide's). `rust_decimal` gives 0.999…9 (28 digits) and `BigDecimal`
  0.999…9 (100 digits) — honest inexactness. `fastnum` D128 returns
  exactly 1 again: its 128-bit coefficient carries a guard digit past the
  nominal 38, so the multiply rounds back up.
- **30 periods of compound interest** agree with the `rust_decimal`
  reference to ~1e-12 relative for every 50-bit-plus type; f16/bf16 land
  within 15%/25% — usable as texture, not as money.

## License

MIT
