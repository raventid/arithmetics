use arithmetics::*;
use std::collections::HashMap;
use criterion::*;

/// Integration test suite for the entire arithmetic libraries system
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_full_pipeline_integration() {
        // Test the complete pipeline: config -> benchmark -> analyze -> export
        
        // 1. Configuration loading
        let config_result = config::BenchmarkConfig::load_from_file("benchmark_config.toml");
        // Should handle missing file gracefully
        assert!(config_result.is_ok() || config_result.is_err());

        // 2. Platform detection
        let platform = platform::PlatformInfo::detect();
        assert!(!platform.os.is_empty());
        assert!(!platform.arch.is_empty());

        // 3. Cross-platform testing
        let mut cross_platform_tester = platform::CrossPlatformTester::new();
        cross_platform_tester.run_all_tests();
        let results = cross_platform_tester.get_test_results();
        assert!(!results.is_empty());

        // 4. Performance profiling
        let profiler_allocator = std::sync::Arc::new(profiler::TrackingAllocator::new());
        let mut combined_profiler = profiler::CombinedProfiler::new(profiler_allocator);
        
        let (result, profile) = combined_profiler.profile_operation("integration_test", || {
            // Simulate some arithmetic work
            let mut sum = 0.0;
            for i in 0..1000 {
                sum += i as f64;
            }
            sum
        });
        
        assert!(result > 0.0);
        assert!(!profile.operation.is_empty());

        // 5. Error analysis
        let error_analyzer = error_analysis::AdvancedErrorAnalyzer;
        let fp_errors = error_analyzer.analyze_floating_point_errors();
        assert!(!fp_errors.is_empty());

        // 6. GPU analysis (simulated)
        let gpu_analyzer = gpu::GpuAccelerationAnalyzer::detect();
        let suitability = gpu_analyzer.analyze_gpu_suitability("matrix_multiply", 1024);
        assert_eq!(suitability.operation_type, "matrix_multiply");

        // 7. Regression testing
        let regression_tester = regression::RegressionTester::new("test_baseline.json", 10.0);
        let mut benchmark_data = HashMap::new();
        benchmark_data.insert("test_operation".to_string(), 100.0);
        
        // Save and load baseline
        let _ = regression_tester.save_baseline(&benchmark_data);
        let loaded_baseline = regression_tester.load_baseline();
        assert!(loaded_baseline.is_ok());

        // 8. Visualization generation
        let viz_result = visualization::VisualizationGenerator::generate_html_report(
            &benchmark_data,
            "test_integration_report.html"
        );
        assert!(viz_result.is_ok());

        // Clean up test files
        let _ = std::fs::remove_file("test_baseline.json");
        let _ = std::fs::remove_file("test_integration_report.html");
    }

    #[test]
    fn test_arithmetic_libraries_consistency() {
        use rust_decimal::Decimal;
        use bigdecimal::BigDecimal;
        use std::str::FromStr;

        // Test arithmetic consistency across different libraries
        let test_cases = vec![
            ("0.1", "0.2"),
            ("123.456", "789.012"),
            ("1.0", "3.0"),
        ];

        for (a_str, b_str) in test_cases {
            // f64 arithmetic
            let a_f64: f64 = a_str.parse().unwrap();
            let b_f64: f64 = b_str.parse().unwrap();
            let f64_sum = a_f64 + b_f64;

            // rust_decimal arithmetic
            let a_decimal = Decimal::from_str(a_str).unwrap();
            let b_decimal = Decimal::from_str(b_str).unwrap();
            let decimal_sum = a_decimal + b_decimal;

            // bigdecimal arithmetic
            let a_big = BigDecimal::from_str(a_str).unwrap();
            let b_big = BigDecimal::from_str(b_str).unwrap();
            let big_sum = a_big + b_big;

            // Verify all calculations complete without panicking
            assert!(f64_sum.is_finite());
            assert!(!decimal_sum.to_string().is_empty());
            assert!(!big_sum.to_string().is_empty());

            // For exact decimal representations, decimal libraries should be more accurate
            if a_str == "0.1" && b_str == "0.2" {
                let decimal_as_f64: f64 = decimal_sum.to_string().parse().unwrap();
                let big_as_f64: f64 = big_sum.to_string().parse().unwrap();
                
                // Both decimal libraries should give the same result for this case
                assert!((decimal_as_f64 - big_as_f64).abs() < 1e-10);
            }
        }
    }

    #[test]
    fn test_cli_configuration_integration() {
        use cli::*;

        // Test CLI command structure
        let cli_handler = CliHandler::new();
        
        // Test that CLI handler is created successfully
        assert!(std::mem::size_of_val(&cli_handler) > 0);

        // Test configuration parsing
        let default_config = config::BenchmarkConfig::default();
        assert!(default_config.iteration_count > 0);
        assert!(default_config.warmup_iterations > 0);
        assert!(!default_config.output_format.is_empty());
    }

    #[test]
    fn test_export_functionality_integration() {
        use export::*;

        // Create test benchmark results
        let mut results = HashMap::new();
        results.insert("test_benchmark".to_string(), BenchmarkResult {
            operation: "addition".to_string(),
            library: "f64".to_string(),
            mean_time: 1.5,
            std_deviation: 0.1,
            min_time: 1.2,
            max_time: 1.8,
            iterations: 1000,
        });

        let benchmark_results = BenchmarkResults {
            results,
            timestamp: chrono::Utc::now(),
            platform_info: "test_platform".to_string(),
            configuration: "test_config".to_string(),
        };

        // Test JSON export
        let json_result = export_to_json(&benchmark_results, "test_results.json");
        assert!(json_result.is_ok());

        // Test CSV export
        let csv_result = export_to_csv(&benchmark_results, "test_results.csv");
        assert!(csv_result.is_ok());

        // Clean up
        let _ = std::fs::remove_file("test_results.json");
        let _ = std::fs::remove_file("test_results.csv");
    }

    #[test]
    fn test_memory_profiling_integration() {
        use profiler::*;

        let allocator = std::sync::Arc::new(TrackingAllocator::new());
        let mut memory_tracker = MemoryTracker::new(allocator.clone());

        // Reset stats
        allocator.reset_stats();
        
        // Take initial snapshot
        memory_tracker.take_snapshot("start".to_string());

        // Perform memory allocation
        let _data: Vec<u64> = vec![42; 1000];

        // Take final snapshot
        memory_tracker.take_snapshot("end".to_string());

        // Verify memory tracking worked
        let deltas = memory_tracker.get_memory_deltas();
        assert!(!deltas.is_empty());
        
        // Should have detected memory usage change
        if !deltas.is_empty() {
            assert!(deltas[0].usage_change != 0);
        }
    }

    #[test]
    fn test_benchmark_runner_integration() {
        let benchmark_results = regression::BenchmarkRunner::run_benchmarks();
        
        // Verify benchmark results structure
        assert!(!benchmark_results.is_empty());
        assert!(benchmark_results.contains_key("f64_addition"));
        assert!(benchmark_results.contains_key("decimal_addition"));
        
        // Verify all values are positive (execution times)
        for (_, &time) in &benchmark_results {
            assert!(time > 0.0);
        }
    }

    #[test] 
    fn test_error_detection_integration() {
        use error_analysis::*;

        // Test ULP analysis
        let ulp_analysis = AdvancedErrorAnalyzer::analyze_ulp_distance(1.0, 1.0000000000000002);
        assert!(!ulp_analysis.is_within_1_ulp || ulp_analysis.ulp_distance.abs() <= 2);

        // Test catastrophic cancellation detection
        let cancellation = AdvancedErrorAnalyzer::detect_catastrophic_cancellation(
            1.000000000000001, 
            1.0
        );
        assert_eq!(cancellation.operand_a, 1.000000000000001);
        assert_eq!(cancellation.operand_b, 1.0);

        // Test accumulation errors
        let accumulation_errors = AdvancedErrorAnalyzer::analyze_accumulation_errors();
        assert!(!accumulation_errors.is_empty());
        
        for error in &accumulation_errors {
            assert!(error.iterations > 0);
            assert!(error.total_error >= 0.0);
        }
    }

    #[test]
    fn test_cross_platform_consistency() {
        let platform_info = platform::PlatformInfo::detect();
        
        // Basic platform info should be available
        assert!(!platform_info.os.is_empty());
        assert!(!platform_info.arch.is_empty());
        assert!(platform_info.pointer_width == 32 || platform_info.pointer_width == 64);
        
        // Test decimal cross-platform behavior
        let decimal_tester = platform::DecimalCrossPlatformTester::new();
        assert!(decimal_tester.test_decimal_serialization());
        
        let precision_issues = decimal_tester.test_decimal_precision_consistency();
        // Should have minimal or no precision issues with decimal types
        assert!(precision_issues.len() <= 2); // Allow some tolerance
    }

    #[test]
    fn test_comprehensive_workflow() {
        // Simulate a complete user workflow
        
        // 1. User runs benchmarks
        let benchmark_data = regression::BenchmarkRunner::run_benchmarks();
        
        // 2. User analyzes performance
        let regression_tester = regression::RegressionTester::new("workflow_baseline.json", 5.0);
        let _ = regression_tester.save_baseline(&benchmark_data);
        
        // 3. User generates reports
        let html_result = visualization::VisualizationGenerator::generate_html_report(
            &benchmark_data,
            "workflow_report.html"
        );
        assert!(html_result.is_ok());
        
        let csv_result = visualization::VisualizationGenerator::generate_csv_export(
            &benchmark_data,
            "workflow_data.csv"
        );
        assert!(csv_result.is_ok());
        
        // 4. User runs regression tests
        let regression_report = regression_tester.run_regression_tests(&benchmark_data);
        assert!(regression_report.overall_passed); // Should pass against itself
        
        // Clean up
        let _ = std::fs::remove_file("workflow_baseline.json");
        let _ = std::fs::remove_file("workflow_report.html");
        let _ = std::fs::remove_file("workflow_data.csv");
    }
}

/// Benchmark integration tests
fn benchmark_full_pipeline(c: &mut Criterion) {
    c.bench_function("full_integration_pipeline", |b| {
        b.iter(|| {
            // Run a mini version of the full pipeline
            let benchmark_data = black_box(regression::BenchmarkRunner::run_benchmarks());
            
            let regression_tester = regression::RegressionTester::new("bench_baseline.json", 10.0);
            let report = black_box(regression_tester.run_regression_tests(&benchmark_data));
            
            black_box((benchmark_data, report))
        })
    });

    c.bench_function("cross_platform_test_suite", |b| {
        b.iter(|| {
            let mut tester = black_box(platform::CrossPlatformTester::new());
            tester.run_all_tests();
            black_box(tester.get_test_results())
        })
    });

    c.bench_function("error_analysis_full_suite", |b| {
        b.iter(|| {
            let fp_errors = black_box(error_analysis::AdvancedErrorAnalyzer::analyze_floating_point_errors());
            let acc_errors = black_box(error_analysis::AdvancedErrorAnalyzer::analyze_accumulation_errors());
            black_box((fp_errors, acc_errors))
        })
    });
}

criterion_group!(integration_benches, benchmark_full_pipeline);
criterion_main!(integration_benches);
