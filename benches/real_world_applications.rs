use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use rust_decimal::Decimal;
use bigdecimal::BigDecimal;
use decimal::d128;
use std::str::FromStr;
use std::collections::HashMap;

/// Financial calculations benchmark
fn benchmark_financial_calculations(c: &mut Criterion) {
    // Stock portfolio calculation
    let stocks = vec![
        ("AAPL", 150.25, 100),
        ("GOOGL", 2800.50, 50),
        ("MSFT", 300.75, 75),
        ("AMZN", 3200.00, 25),
        ("TSLA", 800.125, 60),
    ];

    c.bench_function("portfolio_value_f64", |b| {
        b.iter(|| {
            let total_value: f64 = black_box(&stocks)
                .iter()
                .map(|(_, price, shares)| price * (*shares as f64))
                .sum();
            black_box(total_value)
        })
    });

    c.bench_function("portfolio_value_decimal", |b| {
        b.iter(|| {
            let total_value = black_box(&stocks)
                .iter()
                .map(|(_, price, shares)| {
                    Decimal::from_str(&price.to_string()).unwrap() * Decimal::from(*shares)
                })
                .fold(Decimal::ZERO, |acc, val| acc + val);
            black_box(total_value)
        })
    });

    // Compound interest calculation
    let principal = 10000.0;
    let rate = 0.05; // 5% annual rate
    let years = 30;

    c.bench_function("compound_interest_f64", |b| {
        b.iter(|| {
            let amount = black_box(principal) * (1.0 + black_box(rate)).powi(black_box(years));
            black_box(amount)
        })
    });

    c.bench_function("compound_interest_decimal", |b| {
        b.iter(|| {
            let principal_dec = Decimal::from_str("10000.00").unwrap();
            let rate_dec = Decimal::from_str("0.05").unwrap();
            let one = Decimal::ONE;
            
            let mut amount = principal_dec;
            for _ in 0..black_box(years) {
                amount = amount * (one + rate_dec);
            }
            black_box(amount)
        })
    });

    // Currency conversion with fees
    let amounts = vec![100.0, 250.50, 1000.25, 75.75, 500.0];
    let exchange_rate = 1.2345;
    let fee_percentage = 0.015; // 1.5% fee

    c.bench_function("currency_conversion_f64", |b| {
        b.iter(|| {
            let converted: Vec<f64> = black_box(&amounts)
                .iter()
                .map(|&amount| {
                    let converted = amount * black_box(exchange_rate);
                    converted * (1.0 - black_box(fee_percentage))
                })
                .collect();
            black_box(converted)
        })
    });

    c.bench_function("currency_conversion_decimal", |b| {
        b.iter(|| {
            let rate_dec = Decimal::from_str("1.2345").unwrap();
            let fee_dec = Decimal::from_str("0.015").unwrap();
            let one = Decimal::ONE;
            
            let converted: Vec<Decimal> = black_box(&amounts)
                .iter()
                .map(|&amount| {
                    let amount_dec = Decimal::from_str(&amount.to_string()).unwrap();
                    let converted = amount_dec * rate_dec;
                    converted * (one - fee_dec)
                })
                .collect();
            black_box(converted)
        })
    });
}

/// Scientific computing benchmarks
fn benchmark_scientific_computing(c: &mut Criterion) {
    // Monte Carlo π estimation
    let samples = 100_000;
    
    c.bench_function("monte_carlo_pi_f64", |b| {
        b.iter(|| {
            use std::f64::consts::PI;
            let mut inside_circle = 0;
            
            for i in 0..black_box(samples) {
                let x = (i as f64 / samples as f64 * 2.0) - 1.0;
                let y = ((i * 17 + 7) as f64 / samples as f64 * 2.0) - 1.0; // Simple pseudo-random
                
                if x * x + y * y <= 1.0 {
                    inside_circle += 1;
                }
            }
            
            let pi_estimate = 4.0 * inside_circle as f64 / samples as f64;
            black_box(pi_estimate)
        })
    });

    // Numerical integration (trapezoidal rule)
    let a = 0.0;
    let b = 1.0;
    let n = 10_000;
    
    c.bench_function("numerical_integration_f64", |b| {
        b.iter(|| {
            let h = (black_box(b) - black_box(a)) / black_box(n) as f64;
            let mut sum = 0.0;
            
            for i in 0..=n {
                let x = a + i as f64 * h;
                let y = x * x; // Integrating x^2
                
                if i == 0 || i == n {
                    sum += y;
                } else {
                    sum += 2.0 * y;
                }
            }
            
            let integral = sum * h / 2.0;
            black_box(integral)
        })
    });

    c.bench_function("numerical_integration_decimal", |b| {
        b.iter(|| {
            let a_dec = Decimal::ZERO;
            let b_dec = Decimal::ONE;
            let n_dec = Decimal::from(10000);
            let h = (b_dec - a_dec) / n_dec;
            let mut sum = Decimal::ZERO;
            let two = Decimal::from(2);
            
            for i in 0..=10000 {
                let x = a_dec + Decimal::from(i) * h;
                let y = x * x; // x^2
                
                if i == 0 || i == 10000 {
                    sum += y;
                } else {
                    sum += two * y;
                }
            }
            
            let integral = sum * h / two;
            black_box(integral)
        })
    });

    // Matrix determinant calculation (3x3)
    let matrix = [
        [2.0, 1.0, 3.0],
        [1.0, 4.0, 2.0],
        [3.0, 2.0, 1.0],
    ];
    
    c.bench_function("matrix_determinant_3x3_f64", |b| {
        b.iter(|| {
            let m = black_box(&matrix);
            let det = m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
                    - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
                    + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0]);
            black_box(det)
        })
    });
}

/// Gaming and graphics benchmarks
fn benchmark_gaming_graphics(c: &mut Criterion) {
    // 3D vector operations
    struct Vec3 { x: f64, y: f64, z: f64 }
    
    impl Vec3 {
        fn dot(&self, other: &Vec3) -> f64 {
            self.x * other.x + self.y * other.y + self.z * other.z
        }
        
        fn cross(&self, other: &Vec3) -> Vec3 {
            Vec3 {
                x: self.y * other.z - self.z * other.y,
                y: self.z * other.x - self.x * other.z,
                z: self.x * other.y - self.y * other.x,
            }
        }
        
        fn magnitude(&self) -> f64 {
            (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
        }
        
        fn normalize(&self) -> Vec3 {
            let mag = self.magnitude();
            Vec3 {
                x: self.x / mag,
                y: self.y / mag,
                z: self.z / mag,
            }
        }
    }
    
    let vectors: Vec<Vec3> = (0..1000)
        .map(|i| Vec3 {
            x: (i as f64 * 0.1).sin(),
            y: (i as f64 * 0.1).cos(),
            z: i as f64 * 0.001,
        })
        .collect();
    
    c.bench_function("vector_operations_batch", |b| {
        b.iter(|| {
            let mut results = Vec::new();
            let vecs = black_box(&vectors);
            
            for i in 0..vecs.len() - 1 {
                let dot = vecs[i].dot(&vecs[i + 1]);
                let cross = vecs[i].cross(&vecs[i + 1]);
                let normalized = cross.normalize();
                results.push((dot, normalized.magnitude()));
            }
            
            black_box(results)
        })
    });

    // Physics simulation - simple particle system
    struct Particle {
        x: f64, y: f64, z: f64,
        vx: f64, vy: f64, vz: f64,
        mass: f64,
    }
    
    let mut particles: Vec<Particle> = (0..100)
        .map(|i| Particle {
            x: i as f64 * 0.1,
            y: (i as f64 * 0.1).sin(),
            z: (i as f64 * 0.1).cos(),
            vx: (i as f64 * 0.01).sin(),
            vy: (i as f64 * 0.01).cos(),
            vz: i as f64 * 0.001,
            mass: 1.0 + i as f64 * 0.01,
        })
        .collect();
    
    c.bench_function("particle_physics_simulation", |b| {
        b.iter(|| {
            let dt = 0.016; // 60 FPS
            
            for particle in black_box(&mut particles) {
                // Simple gravity
                particle.vy -= 9.81 * dt;
                
                // Update position
                particle.x += particle.vx * dt;
                particle.y += particle.vy * dt;
                particle.z += particle.vz * dt;
                
                // Simple collision with ground
                if particle.y < 0.0 {
                    particle.y = 0.0;
                    particle.vy = -particle.vy * 0.8; // Damping
                }
            }
            
            black_box(&particles)
        })
    });

    // Ray-sphere intersection
    let spheres = vec![
        (0.0, 0.0, 5.0, 1.0), // (x, y, z, radius)
        (2.0, 1.0, 6.0, 0.5),
        (-1.0, -1.0, 4.0, 1.5),
    ];
    
    let rays = (0..1000)
        .map(|i| {
            let angle = i as f64 * 0.01;
            (angle.sin(), angle.cos(), 1.0) // (dx, dy, dz) direction
        })
        .collect::<Vec<_>>();
    
    c.bench_function("ray_sphere_intersection", |b| {
        b.iter(|| {
            let mut intersections = 0;
            
            for &(dx, dy, dz) in black_box(&rays) {
                for &(sx, sy, sz, radius) in black_box(&spheres) {
                    // Ray from origin (0,0,0) with direction (dx, dy, dz)
                    let a = dx * dx + dy * dy + dz * dz;
                    let b = 2.0 * (-sx * dx - sy * dy - sz * dz);
                    let c = sx * sx + sy * sy + sz * sz - radius * radius;
                    
                    let discriminant = b * b - 4.0 * a * c;
                    if discriminant >= 0.0 {
                        intersections += 1;
                    }
                }
            }
            
            black_box(intersections)
        })
    });
}

/// Machine learning benchmarks
fn benchmark_machine_learning(c: &mut Criterion) {
    // Linear regression gradient descent
    let training_data: Vec<(f64, f64)> = (0..1000)
        .map(|i| {
            let x = i as f64 * 0.01;
            let y = 2.0 * x + 1.0 + (i as f64 * 0.1).sin() * 0.1; // y = 2x + 1 + noise
            (x, y)
        })
        .collect();
    
    c.bench_function("linear_regression_gradient_descent", |b| {
        b.iter(|| {
            let mut w = 0.0; // weight
            let mut b = 0.0; // bias
            let learning_rate = 0.01;
            
            for _ in 0..100 { // 100 iterations
                let mut dw = 0.0;
                let mut db = 0.0;
                let n = black_box(&training_data).len() as f64;
                
                for &(x, y) in black_box(&training_data) {
                    let prediction = w * x + b;
                    let error = prediction - y;
                    
                    dw += error * x;
                    db += error;
                }
                
                w -= learning_rate * dw / n;
                b -= learning_rate * db / n;
            }
            
            black_box((w, b))
        })
    });

    // Neural network forward pass (simple 2-layer network)
    let weights1 = vec![vec![0.1, 0.2, 0.3], vec![0.4, 0.5, 0.6]]; // 3x2
    let weights2 = vec![vec![0.7, 0.8]]; // 2x1
    let inputs: Vec<Vec<f64>> = (0..100)
        .map(|i| vec![(i as f64 * 0.01).sin(), (i as f64 * 0.01).cos(), i as f64 * 0.001])
        .collect();
    
    c.bench_function("neural_network_forward_pass", |b| {
        b.iter(|| {
            let mut outputs = Vec::new();
            
            for input in black_box(&inputs) {
                // First layer
                let mut hidden = vec![0.0; 2];
                for i in 0..2 {
                    for j in 0..3 {
                        hidden[i] += input[j] * weights1[i][j];
                    }
                    hidden[i] = hidden[i].tanh(); // Activation function
                }
                
                // Second layer
                let mut output = 0.0;
                for i in 0..2 {
                    output += hidden[i] * weights2[0][i];
                }
                output = 1.0 / (1.0 + (-output).exp()); // Sigmoid
                
                outputs.push(output);
            }
            
            black_box(outputs)
        })
    });

    // K-means clustering (simplified)
    let points: Vec<(f64, f64)> = (0..500)
        .map(|i| {
            let angle = i as f64 * 0.01;
            let radius = 1.0 + (i as f64 * 0.05).sin() * 0.3;
            (radius * angle.cos(), radius * angle.sin())
        })
        .collect();
    
    c.bench_function("kmeans_clustering_iteration", |b| {
        b.iter(|| {
            let k = 3;
            let mut centroids = vec![(0.0, 0.0), (1.0, 1.0), (-1.0, -1.0)];
            
            // One iteration of k-means
            let mut clusters = vec![Vec::new(); k];
            
            // Assign points to clusters
            for &point in black_box(&points) {
                let mut min_dist = f64::INFINITY;
                let mut cluster_id = 0;
                
                for (i, &centroid) in centroids.iter().enumerate() {
                    let dist = (point.0 - centroid.0).powi(2) + (point.1 - centroid.1).powi(2);
                    if dist < min_dist {
                        min_dist = dist;
                        cluster_id = i;
                    }
                }
                
                clusters[cluster_id].push(point);
            }
            
            // Update centroids
            for (i, cluster) in clusters.iter().enumerate() {
                if !cluster.is_empty() {
                    let sum_x: f64 = cluster.iter().map(|p| p.0).sum();
                    let sum_y: f64 = cluster.iter().map(|p| p.1).sum();
                    centroids[i] = (sum_x / cluster.len() as f64, sum_y / cluster.len() as f64);
                }
            }
            
            black_box(centroids)
        })
    });
}

/// Embedded systems simulation benchmarks
fn benchmark_embedded_systems(c: &mut Criterion) {
    use fixed::types::I16F16;
    
    // PID controller simulation
    c.bench_function("pid_controller_f64", |b| {
        b.iter(|| {
            let mut integral = 0.0;
            let mut previous_error = 0.0;
            let setpoint = 100.0;
            let kp = 0.5;
            let ki = 0.1;
            let kd = 0.01;
            let dt = 0.01;
            
            let mut current_value = 0.0;
            let mut outputs = Vec::new();
            
            for _ in 0..1000 {
                let error = setpoint - current_value;
                integral += error * dt;
                let derivative = (error - previous_error) / dt;
                
                let output = kp * error + ki * integral + kd * derivative;
                outputs.push(output);
                
                // Simple system response
                current_value += output * 0.1;
                previous_error = error;
            }
            
            black_box(outputs)
        })
    });

    c.bench_function("pid_controller_fixed_point", |b| {
        b.iter(|| {
            let mut integral = I16F16::from_num(0.0);
            let mut previous_error = I16F16::from_num(0.0);
            let setpoint = I16F16::from_num(100.0);
            let kp = I16F16::from_num(0.5);
            let ki = I16F16::from_num(0.1);
            let kd = I16F16::from_num(0.01);
            let dt = I16F16::from_num(0.01);
            
            let mut current_value = I16F16::from_num(0.0);
            let mut outputs = Vec::new();
            
            for _ in 0..1000 {
                let error = setpoint - current_value;
                integral += error * dt;
                let derivative = (error - previous_error) / dt;
                
                let output = kp * error + ki * integral + kd * derivative;
                outputs.push(output);
                
                // Simple system response
                current_value += output * I16F16::from_num(0.1);
                previous_error = error;
            }
            
            black_box(outputs)
        })
    });

    // Digital signal processing - simple FIR filter
    let signal: Vec<f64> = (0..1000)
        .map(|i| {
            (i as f64 * 0.1).sin() + 0.5 * (i as f64 * 0.3).sin() + 0.1 * (i as f64 * 0.7).sin()
        })
        .collect();
    
    let filter_coeffs = vec![0.1, 0.2, 0.4, 0.2, 0.1]; // Simple low-pass filter
    
    c.bench_function("fir_filter_f64", |b| {
        b.iter(|| {
            let mut output = Vec::new();
            let signal = black_box(&signal);
            let coeffs = black_box(&filter_coeffs);
            
            for i in 0..signal.len() {
                let mut sum = 0.0;
                for (j, &coeff) in coeffs.iter().enumerate() {
                    if i >= j {
                        sum += coeff * signal[i - j];
                    }
                }
                output.push(sum);
            }
            
            black_box(output)
        })
    });
}

criterion_group!(
    real_world_benches,
    benchmark_financial_calculations,
    benchmark_scientific_computing,
    benchmark_gaming_graphics,
    benchmark_machine_learning,
    benchmark_embedded_systems
);
criterion_main!(real_world_benches);
