use std::fs;
use std::collections::HashMap;
use serde::Serialize;

/// Data visualization generator for benchmark results
pub struct VisualizationGenerator;

impl VisualizationGenerator {
    /// Generate HTML report with charts
    pub fn generate_html_report(
        benchmark_data: &HashMap<String, f64>,
        output_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let html_content = Self::create_html_template(benchmark_data);
        fs::write(output_path, html_content)?;
        Ok(())
    }

    fn create_html_template(data: &HashMap<String, f64>) -> String {
        let chart_data = Self::prepare_chart_data(data);
        
        format!(r#"
<!DOCTYPE html>
<html>
<head>
    <title>Arithmetic Libraries Performance Report</title>
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 20px; }}
        .chart-container {{ width: 800px; height: 400px; margin: 20px 0; }}
        .summary {{ background: #f5f5f5; padding: 15px; border-radius: 5px; }}
        table {{ border-collapse: collapse; width: 100%; }}
        th, td {{ border: 1px solid #ddd; padding: 8px; text-align: left; }}
        th {{ background-color: #f2f2f2; }}
    </style>
</head>
<body>
    <h1>Arithmetic Libraries Performance Analysis</h1>
    
    <div class="summary">
        <h2>Performance Summary</h2>
        <p>Generated on: {timestamp}</p>
        <p>Total benchmarks: {total_benchmarks}</p>
        <p>Best performing operation: {best_op}</p>
        <p>Slowest operation: {worst_op}</p>
    </div>

    <div class="chart-container">
        <canvas id="performanceChart"></canvas>
    </div>
    
    <div class="chart-container">
        <canvas id="comparisonChart"></canvas>
    </div>

    <h2>Detailed Results</h2>
    <table>
        <thead>
            <tr>
                <th>Operation</th>
                <th>Performance (ns)</th>
                <th>Relative Performance</th>
            </tr>
        </thead>
        <tbody>
            {table_rows}
        </tbody>
    </table>

    <script>
        // Performance bar chart
        const ctx1 = document.getElementById('performanceChart').getContext('2d');
        new Chart(ctx1, {{
            type: 'bar',
            data: {{
                labels: {labels},
                datasets: [{{
                    label: 'Performance (nanoseconds)',
                    data: {values},
                    backgroundColor: 'rgba(54, 162, 235, 0.8)',
                    borderColor: 'rgba(54, 162, 235, 1)',
                    borderWidth: 1
                }}]
            }},
            options: {{
                responsive: true,
                plugins: {{
                    title: {{
                        display: true,
                        text: 'Benchmark Performance Results'
                    }}
                }},
                scales: {{
                    y: {{
                        beginAtZero: true,
                        title: {{
                            display: true,
                            text: 'Time (nanoseconds)'
                        }}
                    }}
                }}
            }}
        }});

        // Library comparison pie chart
        const ctx2 = document.getElementById('comparisonChart').getContext('2d');
        new Chart(ctx2, {{
            type: 'doughnut',
            data: {{
                labels: {lib_labels},
                datasets: [{{
                    data: {lib_values},
                    backgroundColor: [
                        '#FF6384', '#36A2EB', '#FFCE56', '#4BC0C0', 
                        '#9966FF', '#FF9F40', '#FF6384', '#C9CBCF'
                    ]
                }}]
            }},
            options: {{
                responsive: true,
                plugins: {{
                    title: {{
                        display: true,
                        text: 'Performance Distribution by Library Type'
                    }},
                    legend: {{
                        position: 'bottom'
                    }}
                }}
            }}
        }});
    </script>
</body>
</html>
        "#,
        timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
        total_benchmarks = data.len(),
        best_op = Self::find_best_operation(data),
        worst_op = Self::find_worst_operation(data),
        table_rows = Self::generate_table_rows(data),
        labels = serde_json::to_string(&chart_data.labels).unwrap(),
        values = serde_json::to_string(&chart_data.values).unwrap(),
        lib_labels = serde_json::to_string(&chart_data.library_labels).unwrap(),
        lib_values = serde_json::to_string(&chart_data.library_values).unwrap(),
        )
    }

    fn prepare_chart_data(data: &HashMap<String, f64>) -> ChartData {
        let mut labels = Vec::new();
        let mut values = Vec::new();
        
        // Sort by performance for better visualization
        let mut sorted_data: Vec<_> = data.iter().collect();
        sorted_data.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());
        
        for (label, &value) in sorted_data {
            labels.push(label.clone());
            values.push(value);
        }

        // Group by library type for pie chart
        let (library_labels, library_values) = Self::group_by_library_type(data);

        ChartData {
            labels,
            values,
            library_labels,
            library_values,
        }
    }

    fn group_by_library_type(data: &HashMap<String, f64>) -> (Vec<String>, Vec<f64>) {
        let mut lib_groups: HashMap<String, f64> = HashMap::new();
        
        for (operation, &value) in data {
            let lib_type = if operation.contains("f64") {
                "f64 (native)"
            } else if operation.contains("decimal") {
                "rust_decimal"
            } else if operation.contains("bigdecimal") {
                "bigdecimal"
            } else if operation.contains("fixed") {
                "fixed-point"
            } else if operation.contains("half") {
                "half-precision"
            } else {
                "other"
            };
            
            *lib_groups.entry(lib_type.to_string()).or_insert(0.0) += value;
        }

        let labels: Vec<String> = lib_groups.keys().cloned().collect();
        let values: Vec<f64> = lib_groups.values().cloned().collect();
        
        (labels, values)
    }

    fn find_best_operation(data: &HashMap<String, f64>) -> String {
        data.iter()
            .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(k, _)| k.clone())
            .unwrap_or_else(|| "None".to_string())
    }

    fn find_worst_operation(data: &HashMap<String, f64>) -> String {
        data.iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(k, _)| k.clone())
            .unwrap_or_else(|| "None".to_string())
    }

    fn generate_table_rows(data: &HashMap<String, f64>) -> String {
        let min_value = data.values().cloned().fold(f64::INFINITY, f64::min);
        
        let mut sorted_data: Vec<_> = data.iter().collect();
        sorted_data.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());
        
        sorted_data
            .iter()
            .map(|(op, &value)| {
                let relative = value / min_value;
                format!(
                    "<tr><td>{}</td><td>{:.2}</td><td>{:.2}x</td></tr>",
                    op, value, relative
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Generate CSV export
    pub fn generate_csv_export(
        data: &HashMap<String, f64>,
        output_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut csv_content = String::from("Operation,Performance (ns),Library Type\n");
        
        for (operation, &value) in data {
            let lib_type = Self::determine_library_type(operation);
            csv_content.push_str(&format!("{},{},{}\n", operation, value, lib_type));
        }
        
        fs::write(output_path, csv_content)?;
        Ok(())
    }

    fn determine_library_type(operation: &str) -> &str {
        if operation.contains("f64") {
            "f64"
        } else if operation.contains("decimal") && !operation.contains("bigdecimal") {
            "rust_decimal"
        } else if operation.contains("bigdecimal") {
            "bigdecimal"
        } else if operation.contains("fixed") {
            "fixed"
        } else if operation.contains("half") {
            "half"
        } else {
            "other"
        }
    }

    /// Generate markdown report
    pub fn generate_markdown_report(
        data: &HashMap<String, f64>,
        output_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut md_content = String::from("# Arithmetic Libraries Performance Report\n\n");
        
        md_content.push_str(&format!("Generated on: {}\n\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
        
        md_content.push_str("## Performance Summary\n\n");
        md_content.push_str(&format!("- Total benchmarks: {}\n", data.len()));
        md_content.push_str(&format!("- Best performing: {}\n", Self::find_best_operation(data)));
        md_content.push_str(&format!("- Slowest operation: {}\n\n", Self::find_worst_operation(data)));
        
        md_content.push_str("## Detailed Results\n\n");
        md_content.push_str("| Operation | Performance (ns) | Library |\n");
        md_content.push_str("|-----------|------------------|----------|\n");
        
        let mut sorted_data: Vec<_> = data.iter().collect();
        sorted_data.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());
        
        for (operation, &value) in sorted_data {
            let lib_type = Self::determine_library_type(operation);
            md_content.push_str(&format!("| {} | {:.2} | {} |\n", operation, value, lib_type));
        }
        
        md_content.push_str("\n## Library Type Performance\n\n");
        let (lib_labels, lib_values) = Self::group_by_library_type(data);
        for (label, value) in lib_labels.iter().zip(lib_values.iter()) {
            md_content.push_str(&format!("- {}: {:.2} total ns\n", label, value));
        }
        
        fs::write(output_path, md_content)?;
        Ok(())
    }
}

#[derive(Debug)]
struct ChartData {
    labels: Vec<String>,
    values: Vec<f64>,
    library_labels: Vec<String>,
    library_values: Vec<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_html_generation() {
        let mut data = HashMap::new();
        data.insert("f64_add".to_string(), 1.5);
        data.insert("decimal_add".to_string(), 45.2);
        data.insert("bigdecimal_add".to_string(), 125.4);

        let output_path = "test_report.html";
        VisualizationGenerator::generate_html_report(&data, output_path).unwrap();
        
        assert!(std::path::Path::new(output_path).exists());
        let content = fs::read_to_string(output_path).unwrap();
        assert!(content.contains("Arithmetic Libraries Performance Report"));
        
        // Clean up
        let _ = fs::remove_file(output_path);
    }

    #[test]
    fn test_csv_generation() {
        let mut data = HashMap::new();
        data.insert("f64_mul".to_string(), 2.1);
        data.insert("decimal_mul".to_string(), 67.8);

        let output_path = "test_report.csv";
        VisualizationGenerator::generate_csv_export(&data, output_path).unwrap();
        
        assert!(std::path::Path::new(output_path).exists());
        let content = fs::read_to_string(output_path).unwrap();
        assert!(content.contains("Operation,Performance"));
        
        // Clean up
        let _ = fs::remove_file(output_path);
    }

    #[test]
    fn test_library_type_detection() {
        assert_eq!(VisualizationGenerator::determine_library_type("f64_addition"), "f64");
        assert_eq!(VisualizationGenerator::determine_library_type("decimal_multiplication"), "rust_decimal");
        assert_eq!(VisualizationGenerator::determine_library_type("bigdecimal_division"), "bigdecimal");
    }
}
