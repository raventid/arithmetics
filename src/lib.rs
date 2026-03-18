//! Microbenchmarks comparing Rust numeric types.
//!
//! This crate has no runtime API of its own; it exists for `cargo bench`
//! (see `benches/`) and `cargo test` (see `tests/`). The helpers below
//! generate deterministic inputs shared by both.

/// Minimal linear congruential generator (Knuth's MMIX constants) so input
/// data is deterministic without pulling in a `rand` dependency.
pub struct Lcg(u64);

impl Lcg {
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }

    /// Next pseudo-random value. Only the statistically stronger high 32
    /// bits of the state are returned.
    pub fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0 >> 32
    }
}

/// `n` decimal strings with `dp` fractional digits, drawn from
/// `[lo_units, hi_units]` expressed in units of `10^-dp`.
///
/// Strings with a fixed number of decimal places are exactly representable
/// in every decimal type under test; binary types (floats, fixed-point)
/// round once at construction, which is inherent to those types.
///
/// The benches use two ranges chosen so that every intermediate result
/// stays representable in ALL types, including `f16` (max 65 504):
/// * R1 `1.00..=100.00` (dp = 2) for scalar ops and conversions;
/// * R2 `0.500..=2.000` (dp = 3) for sums and dot products.
///
/// ```
/// let s = arithmetics::decimal_strings(1, 3, 2, 100, 10_000);
/// assert_eq!(s.len(), 3);
/// assert!(s.iter().all(|x| x.contains('.')));
/// ```
pub fn decimal_strings(seed: u64, n: usize, dp: u32, lo_units: u64, hi_units: u64) -> Vec<String> {
    assert!(lo_units <= hi_units);
    let span = hi_units - lo_units + 1;
    let scale = 10u64.pow(dp);
    let mut rng = Lcg::new(seed);
    (0..n)
        .map(|_| {
            let units = lo_units + rng.next() % span;
            format!("{}.{:0width$}", units / scale, units % scale, width = dp as usize)
        })
        .collect()
}

/// Parse every string into `T`, panicking on failure (setup-only helper).
pub fn parse_all<T: std::str::FromStr>(strings: &[String]) -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    strings.iter().map(|s| s.parse().unwrap()).collect()
}
