use criterion::{black_box, criterion_group, criterion_main, Criterion};
use arithmetics::error_analysis::{AdvancedErrorAnalyzer, UlpAnalysis};

fn benchmark_error_detection(c: &mut Criterion) {
    c.bench_function("ulp_distance_calculation", |b| {
        b.iter(|| {
            let expected = 1.0;
            let actual = 1.0000000000000002;
            let analysis = black_box(AdvancedErrorAnalyzer::analyze_ulp_distance(expected, actual));
            black_box(analysis)
        })
    });

    c.bench_function("catastrophic_cancellation_detection", |b| {
        b.iter(|| {
            let a = 1.000000000000001;
            let b = 1.0;
            let result = black_box(AdvancedErrorAnalyzer::detect_catastrophic_cancellation(a, b));
            black_box(result)
        })
    });

    c.bench_function("floating_point_error_analysis", |b| {
        b.iter(|| {
            let errors = black_box(AdvancedErrorAnalyzer::analyze_floating_point_errors());
            black_box(errors)
        })
    });

    c.bench_function("accumulation_error_analysis", |b| {
        b.iter(|| {
            let errors = black_box(AdvancedErrorAnalyzer::analyze_accumulation_errors());
            black_box(errors)
        })
    });
}

criterion_group!(error_benches, benchmark_error_detection);
criterion_main!(error_benches);
