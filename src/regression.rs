use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Performance regression testing framework
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionTest {
    pub name: String,
    pub baseline_performance: f64,
    pub current_performance: f64,
    pub threshold_percent: f64,
    pub timestamp: DateTime<Utc>,
    pub passed: bool,
    pub regression_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionReport {
    pub tests: Vec<RegressionTest>,
    pub overall_passed: bool,
    pub timestamp: DateTime<Utc>,
    pub summary: RegressionSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionSummary {
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub average_regression: f64,
    pub worst_regression: f64,
}

pub struct RegressionTester {
    baseline_path: String,
    threshold_percent: f64,
}

impl RegressionTester {
    pub fn new(baseline_path: &str, threshold_percent: f64) -> Self {
        Self {
            baseline_path: baseline_path.to_string(),
            threshold_percent,
        }
    }

    /// Save current performance as baseline
    pub fn save_baseline(&self, performance_data: &HashMap<String, f64>) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(performance_data)?;
        fs::write(&self.baseline_path, json)?;
        Ok(())
    }

    /// Load baseline performance data
    pub fn load_baseline(&self) -> Result<HashMap<String, f64>, Box<dyn std::error::Error>> {
        if !Path::new(&self.baseline_path).exists() {
            return Ok(HashMap::new());
        }
        
        let content = fs::read_to_string(&self.baseline_path)?;
        let baseline: HashMap<String, f64> = serde_json::from_str(&content)?;
        Ok(baseline)
    }

    /// Run regression tests
    pub fn run_regression_tests(&self, current_performance: &HashMap<String, f64>) -> RegressionReport {
        let baseline = self.load_baseline().unwrap_or_default();
        let mut tests = Vec::new();
        let mut total_regression = 0.0;
        let mut worst_regression = 0.0;

        for (test_name, &current_perf) in current_performance {
            if let Some(&baseline_perf) = baseline.get(test_name) {
                let regression_percent = if baseline_perf > 0.0 {
                    ((current_perf - baseline_perf) / baseline_perf) * 100.0
                } else {
                    0.0
                };

                let passed = regression_percent <= self.threshold_percent;
                
                tests.push(RegressionTest {
                    name: test_name.clone(),
                    baseline_performance: baseline_perf,
                    current_performance: current_perf,
                    threshold_percent: self.threshold_percent,
                    timestamp: Utc::now(),
                    passed,
                    regression_percent,
                });

                total_regression += regression_percent;
                if regression_percent > worst_regression {
                    worst_regression = regression_percent;
                }
            }
        }

        let passed_count = tests.iter().filter(|t| t.passed).count();
        let average_regression = if !tests.is_empty() {
            total_regression / tests.len() as f64
        } else {
            0.0
        };

        let summary = RegressionSummary {
            total_tests: tests.len(),
            passed_tests: passed_count,
            failed_tests: tests.len() - passed_count,
            average_regression,
            worst_regression,
        };

        RegressionReport {
            tests,
            overall_passed: passed_count == tests.len(),
            timestamp: Utc::now(),
            summary,
        }
    }

    /// Print regression report
    pub fn print_report(&self, report: &RegressionReport) {
        println!("=== Performance Regression Report ===");
        println!("Timestamp: {}", report.timestamp);
        println!("Overall Status: {}", if report.overall_passed { "PASSED" } else { "FAILED" });
        println!();

        println!("Summary:");
        println!("  Total Tests: {}", report.summary.total_tests);
        println!("  Passed: {}", report.summary.passed_tests);
        println!("  Failed: {}", report.summary.failed_tests);
        println!("  Average Regression: {:.2}%", report.summary.average_regression);
        println!("  Worst Regression: {:.2}%", report.summary.worst_regression);
        println!();

        println!("Individual Test Results:");
        println!("{:<40} {:<15} {:<15} {:<15} {:<10}", 
                 "Test Name", "Baseline", "Current", "Regression %", "Status");
        println!("{}", "-".repeat(95));

        for test in &report.tests {
            let status = if test.passed { "PASS" } else { "FAIL" };
            println!("{:<40} {:<15.6} {:<15.6} {:<15.2} {:<10}",
                     test.name,
                     test.baseline_performance,
                     test.current_performance,
                     test.regression_percent,
                     status);
        }

        if !report.overall_passed {
            println!();
            println!("Failed Tests:");
            for test in report.tests.iter().filter(|t| !t.passed) {
                println!("  {} - {:.2}% regression (threshold: {:.2}%)",
                         test.name, test.regression_percent, test.threshold_percent);
            }
        }
    }

    /// Save regression report
    pub fn save_report(&self, report: &RegressionReport, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(report)?;
        fs::write(path, json)?;
        Ok(())
    }
}

/// Simple benchmark runner for regression testing
pub struct BenchmarkRunner;

impl BenchmarkRunner {
    /// Simulate benchmark runs and return performance metrics
    pub fn run_benchmarks() -> HashMap<String, f64> {
        let mut results = HashMap::new();
        
        // Simulate various benchmark results (execution time in nanoseconds)
        results.insert("f64_addition".to_string(), 1.2);
        results.insert("f64_multiplication".to_string(), 1.5);
        results.insert("f64_division".to_string(), 8.3);
        results.insert("decimal_addition".to_string(), 45.2);
        results.insert("decimal_multiplication".to_string(), 67.8);
        results.insert("bigdecimal_addition".to_string(), 125.4);
        results.insert("vector_operations".to_string(), 234.7);
        results.insert("matrix_multiplication".to_string(), 1567.2);
        results.insert("statistical_operations".to_string(), 456.8);
        results.insert("precision_tests".to_string(), 89.3);

        // Add some random variation to simulate real performance fluctuations
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        chrono::Utc::now().timestamp_nanos().hash(&mut hasher);
        let seed = hasher.finish();
        
        for (_, value) in results.iter_mut() {
            let variation = ((seed % 100) as f64 - 50.0) / 1000.0; // ±5% variation
            *value *= 1.0 + variation;
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_regression_tester() {
        let test_baseline_path = "test_baseline.json";
        let tester = RegressionTester::new(test_baseline_path, 10.0);

        // Create test baseline
        let mut baseline = HashMap::new();
        baseline.insert("test_op".to_string(), 100.0);
        tester.save_baseline(&baseline).unwrap();

        // Test with current performance
        let mut current = HashMap::new();
        current.insert("test_op".to_string(), 105.0); // 5% regression

        let report = tester.run_regression_tests(&current);
        assert_eq!(report.tests.len(), 1);
        assert!(report.tests[0].passed); // 5% is within 10% threshold

        // Clean up
        let _ = fs::remove_file(test_baseline_path);
    }

    #[test]
    fn test_benchmark_runner() {
        let results = BenchmarkRunner::run_benchmarks();
        assert!(!results.is_empty());
        assert!(results.contains_key("f64_addition"));
        assert!(results.contains_key("decimal_addition"));
    }

    #[test]
    fn test_regression_detection() {
        let test_baseline_path = "test_regression.json";
        let tester = RegressionTester::new(test_baseline_path, 5.0);

        let mut baseline = HashMap::new();
        baseline.insert("slow_op".to_string(), 100.0);
        tester.save_baseline(&baseline).unwrap();

        let mut current = HashMap::new();
        current.insert("slow_op".to_string(), 120.0); // 20% regression

        let report = tester.run_regression_tests(&current);
        assert!(!report.tests[0].passed); // Should fail with 20% regression
        assert!(!report.overall_passed);

        // Clean up
        let _ = fs::remove_file(test_baseline_path);
    }
}
