use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_sqrt(c: &mut Criterion) {
    let mut group = c.benchmark_group("square_root");
    
    // f64 baseline
    group.bench_function("f64", |b| {
        b.iter(|| {
            let a = black_box(144.0);
            black_box(a.sqrt())
        })
    });
    
    group.finish();
}

fn benchmark_power(c: &mut Criterion) {
    let mut group = c.benchmark_group("power");
    
    // f64 baseline
    group.bench_function("f64", |b| {
        b.iter(|| {
            let a = black_box(2.0);
            let b_val = black_box(10.0);
            black_box(a.powf(b_val))
        })
    });
    
    group.finish();
}

fn benchmark_trigonometric(c: &mut Criterion) {
    let mut group = c.benchmark_group("trigonometric");
    
    // f64 baseline - sin
    group.bench_function("f64_sin", |b| {
        b.iter(|| {
            let a = black_box(1.5707963267948966); // π/2
            black_box(a.sin())
        })
    });
    
    // f64 baseline - cos
    group.bench_function("f64_cos", |b| {
        b.iter(|| {
            let a = black_box(1.5707963267948966); // π/2
            black_box(a.cos())
        })
    });
    
    group.finish();
}

criterion_group!(benches, benchmark_sqrt, benchmark_power, benchmark_trigonometric);
criterion_main!(benches);