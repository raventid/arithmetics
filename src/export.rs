use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crate::config::OutputFormat;
use crate::profiling::PerformanceMetrics;
use crate::validation::ValidationResult;
use crate::error_analysis::{ErrorAnalysis, AccumulationError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResults {
    pub timestamp: String,
    pub configuration: BenchmarkConfiguration,
    pub performance_metrics: Vec<PerformanceMetrics>,
    pub validation_results: Vec<ValidationResult>,
    pub error_analyses: Vec<ErrorAnalysis>,
    pub accumulation_errors: Vec<AccumulationError>,
    pub library_comparison: HashMap<String, LibraryMetrics>,
    pub summary: ResultSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkConfiguration {
    pub enabled_libraries: Vec<String>,
    pub iterations: usize,
    pub test_types: Vec<String>,
    pub platform_info: PlatformInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformInfo {
    pub os: String,
    pub architecture: String,
    pub rust_version: String,
    pub cpu_cores: usize,
    pub total_memory_gb: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryMetrics {
    pub library_name: String,
    pub avg_performance_score: f64,
    pub memory_efficiency_score: f64,
    pub precision_score: f64,
    pub safety_score: f64,
    pub overall_score: f64,
    pub strengths: Vec<String>,
    pub weaknesses: Vec<String>,
    pub recommended_use_cases: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultSummary {
    pub total_tests_run: usize,
    pub tests_passed: usize,
    pub tests_failed: usize,
    pub fastest_library: String,
    pub most_precise_library: String,
    pub most_memory_efficient: String,
    pub safest_library: String,
    pub overall_recommendation: String,
    pub key_findings: Vec<String>,
}

pub struct ResultExporter;

impl ResultExporter {
    pub fn export_results(
        results: &BenchmarkResults,
        output_dir: &str,
        format: &OutputFormat,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Ensure output directory exists
        fs::create_dir_all(output_dir)?;
        
        match format {
            OutputFormat::Json => Self::export_json(results, output_dir),
            OutputFormat::Csv => Self::export_csv(results, output_dir),
            OutputFormat::Html => Self::export_html(results, output_dir),
            OutputFormat::Text => Self::export_text(results, output_dir),
        }
    }

    fn export_json(
        results: &BenchmarkResults,
        output_dir: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let timestamp = &results.timestamp;
        let filename = format!("{}/benchmark_results_{}.json", output_dir, timestamp);
        
        let json_content = serde_json::to_string_pretty(results)?;
        fs::write(&filename, json_content)?;
        
        Ok(filename)
    }

    fn export_csv(
        results: &BenchmarkResults,
        output_dir: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let timestamp = &results.timestamp;
        
        // Export performance metrics
        let perf_filename = format!("{}/performance_metrics_{}.csv", output_dir, timestamp);
        let mut perf_csv = String::new();
        perf_csv.push_str("Library,Operation,Duration_ns,Iterations,Avg_ns_per_op,Ops_per_second\n");
        
        for metric in &results.performance_metrics {
            perf_csv.push_str(&format!(
                "{},{},{},{},{:.2},{:.2e}\n",
                metric.library,
                metric.operation,
                metric.duration.as_nanos(),
                metric.iterations,
                metric.avg_ns_per_op,
                metric.ops_per_second
            ));
        }
        fs::write(&perf_filename, perf_csv)?;

        // Export validation results
        let val_filename = format!("{}/validation_results_{}.csv", output_dir, timestamp);
        let mut val_csv = String::new();
        val_csv.push_str("Test_Name,Expected,Actual,Passed,Error_Margin\n");
        
        for result in &results.validation_results {
            val_csv.push_str(&format!(
                "{},{},{},{},{}\n",
                result.test_name,
                result.expected,
                result.actual,
                result.passed,
                result.error_margin.map_or("N/A".to_string(), |e| format!("{:.3e}", e))
            ));
        }
        fs::write(&val_filename, val_csv)?;

        // Export library comparison
        let comp_filename = format!("{}/library_comparison_{}.csv", output_dir, timestamp);
        let mut comp_csv = String::new();
        comp_csv.push_str("Library,Performance_Score,Memory_Score,Precision_Score,Safety_Score,Overall_Score\n");
        
        for (_, metrics) in &results.library_comparison {
            comp_csv.push_str(&format!(
                "{},{:.2},{:.2},{:.2},{:.2},{:.2}\n",
                metrics.library_name,
                metrics.avg_performance_score,
                metrics.memory_efficiency_score,
                metrics.precision_score,
                metrics.safety_score,
                metrics.overall_score
            ));
        }
        fs::write(&comp_filename, comp_csv)?;

        Ok(format!("Exported CSV files: {}, {}, {}", perf_filename, val_filename, comp_filename))
    }

    fn export_html(
        results: &BenchmarkResults,
        output_dir: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let timestamp = &results.timestamp;
        let filename = format!("{}/benchmark_report_{}.html", output_dir, timestamp);
        
        let html_content = Self::generate_html_report(results);
        fs::write(&filename, html_content)?;
        
        Ok(filename)
    }

    fn export_text(
        results: &BenchmarkResults,
        output_dir: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let timestamp = &results.timestamp;
        let filename = format!("{}/benchmark_report_{}.txt", output_dir, timestamp);
        
        let text_content = Self::generate_text_report(results);
        fs::write(&filename, text_content)?;
        
        Ok(filename)
    }

    fn generate_html_report(results: &BenchmarkResults) -> String {
        format!(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Arithmetic Libraries Benchmark Report</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; line-height: 1.6; }}
        .header {{ background-color: #f4f4f4; padding: 20px; border-radius: 8px; }}
        .section {{ margin: 30px 0; }}
        .metric-table {{ width: 100%; border-collapse: collapse; margin: 20px 0; }}
        .metric-table th, .metric-table td {{ 
            border: 1px solid #ddd; 
            padding: 12px; 
            text-align: left; 
        }}
        .metric-table th {{ background-color: #f2f2f2; }}
        .summary-box {{ 
            background-color: #e8f5e8; 
            border: 1px solid #4caf50; 
            padding: 15px; 
            border-radius: 5px; 
            margin: 20px 0;
        }}
        .finding {{ margin: 10px 0; padding: 10px; background-color: #f9f9f9; border-left: 4px solid #2196f3; }}
        .score {{ font-weight: bold; }}
        .high-score {{ color: #4caf50; }}
        .medium-score {{ color: #ff9800; }}
        .low-score {{ color: #f44336; }}
    </style>
</head>
<body>
    <div class="header">
        <h1>Arithmetic Libraries Benchmark Report</h1>
        <p><strong>Generated:</strong> {}</p>
        <p><strong>Platform:</strong> {} on {}</p>
        <p><strong>Libraries Tested:</strong> {}</p>
    </div>

    <div class="section">
        <h2>Executive Summary</h2>
        <div class="summary-box">
            <p><strong>Tests Run:</strong> {} | <strong>Passed:</strong> {} | <strong>Failed:</strong> {}</p>
            <p><strong>Fastest Library:</strong> {}</p>
            <p><strong>Most Precise:</strong> {}</p>
            <p><strong>Most Memory Efficient:</strong> {}</p>
            <p><strong>Overall Recommendation:</strong> {}</p>
        </div>
    </div>

    <div class="section">
        <h2>Library Comparison</h2>
        <table class="metric-table">
            <thead>
                <tr>
                    <th>Library</th>
                    <th>Performance</th>
                    <th>Memory Efficiency</th>
                    <th>Precision</th>
                    <th>Safety</th>
                    <th>Overall Score</th>
                </tr>
            </thead>
            <tbody>
                {}
            </tbody>
        </table>
    </div>

    <div class="section">
        <h2>Key Findings</h2>
        {}
    </div>

    <div class="section">
        <h2>Performance Metrics</h2>
        <p>Detailed performance data shows operations per second for each library across different arithmetic operations.</p>
        <table class="metric-table">
            <thead>
                <tr>
                    <th>Library</th>
                    <th>Operation</th>
                    <th>Ops/Second</th>
                    <th>Avg ns/Op</th>
                </tr>
            </thead>
            <tbody>
                {}
            </tbody>
        </table>
    </div>

    <div class="section">
        <h2>Platform Information</h2>
        <p><strong>Operating System:</strong> {}</p>
        <p><strong>Architecture:</strong> {}</p>
        <p><strong>Rust Version:</strong> {}</p>
        <p><strong>CPU Cores:</strong> {}</p>
        <p><strong>Total Memory:</strong> {:.1} GB</p>
    </div>
</body>
</html>
        "#,
        results.timestamp,
        results.configuration.platform_info.os,
        results.configuration.platform_info.architecture,
        results.configuration.enabled_libraries.join(", "),
        results.summary.total_tests_run,
        results.summary.tests_passed,
        results.summary.tests_failed,
        results.summary.fastest_library,
        results.summary.most_precise_library,
        results.summary.most_memory_efficient,
        results.summary.overall_recommendation,
        Self::generate_library_comparison_html(&results.library_comparison),
        Self::generate_findings_html(&results.summary.key_findings),
        Self::generate_performance_table_html(&results.performance_metrics),
        results.configuration.platform_info.os,
        results.configuration.platform_info.architecture,
        results.configuration.platform_info.rust_version,
        results.configuration.platform_info.cpu_cores,
        results.configuration.platform_info.total_memory_gb
        )
    }

    fn generate_library_comparison_html(comparison: &HashMap<String, LibraryMetrics>) -> String {
        let mut html = String::new();
        for (_, metrics) in comparison {
            html.push_str(&format!(
                r#"<tr>
                    <td>{}</td>
                    <td class="score {}">{:.1}</td>
                    <td class="score {}">{:.1}</td>
                    <td class="score {}">{:.1}</td>
                    <td class="score {}">{:.1}</td>
                    <td class="score {}">{:.1}</td>
                </tr>"#,
                metrics.library_name,
                Self::score_class(metrics.avg_performance_score), metrics.avg_performance_score,
                Self::score_class(metrics.memory_efficiency_score), metrics.memory_efficiency_score,
                Self::score_class(metrics.precision_score), metrics.precision_score,
                Self::score_class(metrics.safety_score), metrics.safety_score,
                Self::score_class(metrics.overall_score), metrics.overall_score
            ));
        }
        html
    }

    fn generate_findings_html(findings: &[String]) -> String {
        findings.iter()
            .map(|finding| format!("<div class=\"finding\">{}</div>", finding))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn generate_performance_table_html(metrics: &[PerformanceMetrics]) -> String {
        let mut html = String::new();
        for metric in metrics.iter().take(20) { // Limit to first 20 for readability
            html.push_str(&format!(
                "<tr><td>{}</td><td>{}</td><td>{:.2e}</td><td>{:.2}</td></tr>",
                metric.library,
                metric.operation,
                metric.ops_per_second,
                metric.avg_ns_per_op
            ));
        }
        html
    }

    fn score_class(score: f64) -> &'static str {
        if score >= 8.0 { "high-score" }
        else if score >= 6.0 { "medium-score" }
        else { "low-score" }
    }

    fn generate_text_report(results: &BenchmarkResults) -> String {
        format!(r#"
ARITHMETIC LIBRARIES BENCHMARK REPORT
=====================================

Generated: {}
Platform: {} on {}
Libraries Tested: {}

EXECUTIVE SUMMARY
-----------------
Tests Run: {} | Passed: {} | Failed: {}
Fastest Library: {}
Most Precise: {}
Most Memory Efficient: {}
Overall Recommendation: {}

LIBRARY SCORES
--------------
{}

KEY FINDINGS
------------
{}

PERFORMANCE HIGHLIGHTS
---------------------
{}

PLATFORM INFORMATION
--------------------
Operating System: {}
Architecture: {}
Rust Version: {}
CPU Cores: {}
Total Memory: {:.1} GB

        "#,
        results.timestamp,
        results.configuration.platform_info.os,
        results.configuration.platform_info.architecture,
        results.configuration.enabled_libraries.join(", "),
        results.summary.total_tests_run,
        results.summary.tests_passed,
        results.summary.tests_failed,
        results.summary.fastest_library,
        results.summary.most_precise_library,
        results.summary.most_memory_efficient,
        results.summary.overall_recommendation,
        Self::generate_library_scores_text(&results.library_comparison),
        results.summary.key_findings.join("\n- "),
        Self::generate_performance_highlights_text(&results.performance_metrics),
        results.configuration.platform_info.os,
        results.configuration.platform_info.architecture,
        results.configuration.platform_info.rust_version,
        results.configuration.platform_info.cpu_cores,
        results.configuration.platform_info.total_memory_gb
        )
    }

    fn generate_library_scores_text(comparison: &HashMap<String, LibraryMetrics>) -> String {
        let mut text = String::new();
        for (_, metrics) in comparison {
            text.push_str(&format!(
                "{}: Overall {:.1} (Perf: {:.1}, Memory: {:.1}, Precision: {:.1}, Safety: {:.1})\n",
                metrics.library_name,
                metrics.overall_score,
                metrics.avg_performance_score,
                metrics.memory_efficiency_score,
                metrics.precision_score,
                metrics.safety_score
            ));
        }
        text
    }

    fn generate_performance_highlights_text(metrics: &[PerformanceMetrics]) -> String {
        let mut text = String::new();
        for metric in metrics.iter().take(10) {
            text.push_str(&format!(
                "{} {}: {:.2e} ops/sec\n",
                metric.library,
                metric.operation,
                metric.ops_per_second
            ));
        }
        text
    }

    pub fn create_benchmark_results(
        performance_metrics: Vec<PerformanceMetrics>,
        validation_results: Vec<ValidationResult>,
        error_analyses: Vec<ErrorAnalysis>,
        accumulation_errors: Vec<AccumulationError>,
        config: &crate::config::ArithmeticConfig,
    ) -> BenchmarkResults {
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S").to_string();
        
        let library_comparison = Self::calculate_library_metrics(&performance_metrics, &validation_results);
        let summary = Self::generate_summary(&performance_metrics, &validation_results, &library_comparison);
        
        BenchmarkResults {
            timestamp,
            configuration: BenchmarkConfiguration {
                enabled_libraries: config.enabled_libraries.clone(),
                iterations: config.benchmark.iterations,
                test_types: vec!["Performance".to_string(), "Validation".to_string(), "Error Analysis".to_string()],
                platform_info: Self::gather_platform_info(),
            },
            performance_metrics,
            validation_results,
            error_analyses,
            accumulation_errors,
            library_comparison,
            summary,
        }
    }

    fn calculate_library_metrics(
        performance_metrics: &[PerformanceMetrics],
        validation_results: &[ValidationResult],
    ) -> HashMap<String, LibraryMetrics> {
        let mut metrics = HashMap::new();
        
        // Group performance by library
        let mut perf_by_library: HashMap<String, Vec<f64>> = HashMap::new();
        for metric in performance_metrics {
            perf_by_library
                .entry(metric.library.clone())
                .or_insert_with(Vec::new)
                .push(metric.ops_per_second);
        }
        
        // Calculate metrics for each library
        for (library, perfs) in perf_by_library {
            let avg_perf = perfs.iter().sum::<f64>() / perfs.len() as f64;
            let validation_score = Self::calculate_validation_score(&library, validation_results);
            
            let lib_metrics = LibraryMetrics {
                library_name: library.clone(),
                avg_performance_score: Self::normalize_performance_score(avg_perf),
                memory_efficiency_score: Self::estimate_memory_score(&library),
                precision_score: validation_score,
                safety_score: Self::estimate_safety_score(&library),
                overall_score: 0.0, // Will be calculated below
                strengths: Self::identify_strengths(&library),
                weaknesses: Self::identify_weaknesses(&library),
                recommended_use_cases: Self::recommend_use_cases(&library),
            };
            
            // Calculate overall score
            let overall = (lib_metrics.avg_performance_score + 
                          lib_metrics.memory_efficiency_score + 
                          lib_metrics.precision_score + 
                          lib_metrics.safety_score) / 4.0;
            
            let mut final_metrics = lib_metrics;
            final_metrics.overall_score = overall;
            
            metrics.insert(library, final_metrics);
        }
        
        metrics
    }

    fn normalize_performance_score(ops_per_sec: f64) -> f64 {
        // Normalize to 0-10 scale (log scale for wide range)
        let score = (ops_per_sec.log10() - 3.0) * 2.0; // Adjust scaling as needed
        score.max(0.0).min(10.0)
    }

    fn calculate_validation_score(library: &str, validation_results: &[ValidationResult]) -> f64 {
        let library_results: Vec<_> = validation_results
            .iter()
            .filter(|r| r.test_name.contains(library))
            .collect();
        
        if library_results.is_empty() {
            return 5.0; // Default score
        }
        
        let passed = library_results.iter().filter(|r| r.passed).count();
        (passed as f64 / library_results.len() as f64) * 10.0
    }

    fn estimate_memory_score(library: &str) -> f64 {
        match library {
            "f64" => 9.0,
            "fixed" => 8.5,
            "rust_decimal" => 7.0,
            "d128" => 7.0,
            "half" => 10.0,
            "bigdecimal" => 4.0,
            _ => 5.0,
        }
    }

    fn estimate_safety_score(library: &str) -> f64 {
        match library {
            "rust_decimal" => 9.5,
            "bigdecimal" => 9.0,
            "d128" => 8.5,
            "fixed" => 8.0,
            "f64" => 6.0,
            "half" => 6.5,
            _ => 5.0,
        }
    }

    fn identify_strengths(library: &str) -> Vec<String> {
        match library {
            "f64" => vec!["Fastest performance".to_string(), "Hardware optimized".to_string()],
            "rust_decimal" => vec!["Exact decimal arithmetic".to_string(), "Good performance".to_string()],
            "bigdecimal" => vec!["Arbitrary precision".to_string(), "Maximum accuracy".to_string()],
            "d128" => vec!["IEEE standard".to_string(), "High precision".to_string()],
            "fixed" => vec!["Deterministic performance".to_string(), "Memory efficient".to_string()],
            "half" => vec!["Minimal memory usage".to_string(), "Good for ML".to_string()],
            _ => vec!["General purpose".to_string()],
        }
    }

    fn identify_weaknesses(library: &str) -> Vec<String> {
        match library {
            "f64" => vec!["Floating-point precision issues".to_string(), "Rounding errors".to_string()],
            "rust_decimal" => vec!["Limited to 28 digits".to_string(), "Slower than f64".to_string()],
            "bigdecimal" => vec!["Slowest performance".to_string(), "High memory usage".to_string()],
            "d128" => vec!["Moderate performance".to_string(), "Limited adoption".to_string()],
            "fixed" => vec!["Limited precision range".to_string(), "Configuration complexity".to_string()],
            "half" => vec!["Very limited precision".to_string(), "Not suitable for precision work".to_string()],
            _ => vec!["Unspecified limitations".to_string()],
        }
    }

    fn recommend_use_cases(library: &str) -> Vec<String> {
        match library {
            "f64" => vec!["General computing".to_string(), "Scientific calculations".to_string()],
            "rust_decimal" => vec!["Financial calculations".to_string(), "Exact decimal arithmetic".to_string()],
            "bigdecimal" => vec!["High-precision mathematics".to_string(), "Critical calculations".to_string()],
            "d128" => vec!["Scientific computing".to_string(), "Standards compliance".to_string()],
            "fixed" => vec!["Embedded systems".to_string(), "Real-time applications".to_string()],
            "half" => vec!["Machine learning".to_string(), "Memory-constrained applications".to_string()],
            _ => vec!["General purpose".to_string()],
        }
    }

    fn generate_summary(
        performance_metrics: &[PerformanceMetrics],
        validation_results: &[ValidationResult],
        library_comparison: &HashMap<String, LibraryMetrics>,
    ) -> ResultSummary {
        let total_tests = validation_results.len();
        let tests_passed = validation_results.iter().filter(|r| r.passed).count();
        let tests_failed = total_tests - tests_passed;

        let fastest_library = performance_metrics
            .iter()
            .max_by(|a, b| a.ops_per_second.partial_cmp(&b.ops_per_second).unwrap())
            .map(|m| m.library.clone())
            .unwrap_or_else(|| "Unknown".to_string());

        let most_precise_library = library_comparison
            .values()
            .max_by(|a, b| a.precision_score.partial_cmp(&b.precision_score).unwrap())
            .map(|m| m.library_name.clone())
            .unwrap_or_else(|| "Unknown".to_string());

        let most_memory_efficient = library_comparison
            .values()
            .max_by(|a, b| a.memory_efficiency_score.partial_cmp(&b.memory_efficiency_score).unwrap())
            .map(|m| m.library_name.clone())
            .unwrap_or_else(|| "Unknown".to_string());

        let safest_library = library_comparison
            .values()
            .max_by(|a, b| a.safety_score.partial_cmp(&b.safety_score).unwrap())
            .map(|m| m.library_name.clone())
            .unwrap_or_else(|| "Unknown".to_string());

        let overall_recommendation = library_comparison
            .values()
            .max_by(|a, b| a.overall_score.partial_cmp(&b.overall_score).unwrap())
            .map(|m| m.library_name.clone())
            .unwrap_or_else(|| "Unknown".to_string());

        ResultSummary {
            total_tests_run: total_tests,
            tests_passed,
            tests_failed,
            fastest_library,
            most_precise_library,
            most_memory_efficient,
            safest_library,
            overall_recommendation,
            key_findings: vec![
                format!("Performance varies significantly across libraries, with {} leading", fastest_library),
                format!("{} provides the best precision for exact calculations", most_precise_library),
                format!("{} offers the best memory efficiency", most_memory_efficient),
                "Choice of library should depend on specific use case requirements".to_string(),
            ],
        }
    }

    fn gather_platform_info() -> PlatformInfo {
        PlatformInfo {
            os: std::env::consts::OS.to_string(),
            architecture: std::env::consts::ARCH.to_string(),
            rust_version: env!("RUSTC_VERSION").to_string(),
            cpu_cores: num_cpus::get(),
            total_memory_gb: Self::get_total_memory_gb(),
        }
    }

    fn get_total_memory_gb() -> f64 {
        // Simplified memory detection - in a real implementation,
        // you might use a system information crate
        8.0 // Default assumption
    }
}
