use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    pub iterations: usize,
    pub warmup_iterations: usize,
    pub measurement_time_secs: u64,
    pub sample_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrecisionConfig {
    pub test_accumulation_errors: bool,
    pub test_small_numbers: bool,
    pub accumulation_iterations: usize,
    pub small_number_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    pub analyze_footprint: bool,
    pub analyze_allocation_patterns: bool,
    pub test_large_datasets: bool,
    pub large_dataset_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyConfig {
    pub test_overflow: bool,
    pub test_division_by_zero: bool,
    pub test_precision_loss: bool,
    pub overflow_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    pub cross_library_consistency: bool,
    pub edge_case_testing: bool,
    pub mathematical_properties: bool,
    pub conversion_accuracy: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilingConfig {
    pub basic_operations: bool,
    pub complex_operations: bool,
    pub memory_intensive: bool,
    pub performance_trends: bool,
    pub allocation_benchmarks: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorAnalysisConfig {
    pub floating_point_errors: bool,
    pub accumulation_errors: bool,
    pub catastrophic_cancellation: bool,
    pub associativity_violations: bool,
    pub library_accuracy_comparison: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    pub verbose: bool,
    pub save_results: bool,
    pub output_format: OutputFormat,
    pub output_directory: String,
    pub include_charts: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFormat {
    Text,
    Json,
    Csv,
    Html,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArithmeticConfig {
    pub benchmark: BenchmarkConfig,
    pub precision: PrecisionConfig,
    pub memory: MemoryConfig,
    pub safety: SafetyConfig,
    pub validation: ValidationConfig,
    pub profiling: ProfilingConfig,
    pub error_analysis: ErrorAnalysisConfig,
    pub output: OutputConfig,
    pub enabled_libraries: Vec<String>,
}

impl Default for ArithmeticConfig {
    fn default() -> Self {
        Self {
            benchmark: BenchmarkConfig {
                iterations: 10000,
                warmup_iterations: 100,
                measurement_time_secs: 5,
                sample_size: 100,
            },
            precision: PrecisionConfig {
                test_accumulation_errors: true,
                test_small_numbers: true,
                accumulation_iterations: 10000,
                small_number_threshold: 1e-10,
            },
            memory: MemoryConfig {
                analyze_footprint: true,
                analyze_allocation_patterns: true,
                test_large_datasets: true,
                large_dataset_size: 100000,
            },
            safety: SafetyConfig {
                test_overflow: true,
                test_division_by_zero: true,
                test_precision_loss: true,
                overflow_threshold: 1e15,
            },
            validation: ValidationConfig {
                cross_library_consistency: true,
                edge_case_testing: true,
                mathematical_properties: true,
                conversion_accuracy: true,
            },
            profiling: ProfilingConfig {
                basic_operations: true,
                complex_operations: true,
                memory_intensive: true,
                performance_trends: true,
                allocation_benchmarks: true,
            },
            error_analysis: ErrorAnalysisConfig {
                floating_point_errors: true,
                accumulation_errors: true,
                catastrophic_cancellation: true,
                associativity_violations: true,
                library_accuracy_comparison: true,
            },
            output: OutputConfig {
                verbose: false,
                save_results: true,
                output_format: OutputFormat::Text,
                output_directory: "results".to_string(),
                include_charts: false,
            },
            enabled_libraries: vec![
                "f64".to_string(),
                "rust_decimal".to_string(),
                "bigdecimal".to_string(),
                "d128".to_string(),
                "fixed".to_string(),
                "half".to_string(),
            ],
        }
    }
}

impl ArithmeticConfig {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: ArithmeticConfig = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn load_or_default<P: AsRef<Path>>(path: P) -> Self {
        Self::load_from_file(path).unwrap_or_else(|e| {
            eprintln!("Warning: Could not load config file: {}", e);
            eprintln!("Using default configuration");
            Self::default()
        })
    }

    pub fn is_library_enabled(&self, library: &str) -> bool {
        self.enabled_libraries.contains(&library.to_string())
    }

    pub fn create_default_config_file<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn std::error::Error>> {
        let default_config = Self::default();
        default_config.save_to_file(path)?;
        Ok(())
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.benchmark.iterations == 0 {
            return Err("Benchmark iterations must be greater than 0".to_string());
        }

        if self.precision.accumulation_iterations == 0 {
            return Err("Precision accumulation iterations must be greater than 0".to_string());
        }

        if self.memory.large_dataset_size == 0 {
            return Err("Large dataset size must be greater than 0".to_string());
        }

        if self.enabled_libraries.is_empty() {
            return Err("At least one library must be enabled".to_string());
        }

        if !Path::new(&self.output.output_directory).exists() {
            fs::create_dir_all(&self.output.output_directory).map_err(|e| {
                format!("Could not create output directory '{}': {}", self.output.output_directory, e)
            })?;
        }

        Ok(())
    }

    pub fn print_summary(&self) {
        println!("=== Configuration Summary ===");
        println!("Enabled libraries: {:?}", self.enabled_libraries);
        println!("Benchmark iterations: {}", self.benchmark.iterations);
        println!("Output format: {:?}", self.output.output_format);
        println!("Output directory: {}", self.output.output_directory);
        
        if self.output.verbose {
            println!("\n=== Detailed Configuration ===");
            println!("Benchmark config: {:?}", self.benchmark);
            println!("Precision config: {:?}", self.precision);
            println!("Memory config: {:?}", self.memory);
            println!("Safety config: {:?}", self.safety);
            println!("Validation config: {:?}", self.validation);
            println!("Profiling config: {:?}", self.profiling);
            println!("Error analysis config: {:?}", self.error_analysis);
            println!("Output config: {:?}", self.output);
        }
    }

    pub fn update_iterations(&mut self, iterations: usize) {
        self.benchmark.iterations = iterations;
        self.precision.accumulation_iterations = iterations;
    }

    pub fn enable_library(&mut self, library: &str) {
        let library = library.to_string();
        if !self.enabled_libraries.contains(&library) {
            self.enabled_libraries.push(library);
        }
    }

    pub fn disable_library(&mut self, library: &str) {
        self.enabled_libraries.retain(|lib| lib != library);
    }

    pub fn set_output_format(&mut self, format: OutputFormat) {
        self.output.output_format = format;
    }

    pub fn enable_all_analysis(&mut self) {
        self.precision.test_accumulation_errors = true;
        self.precision.test_small_numbers = true;
        self.memory.analyze_footprint = true;
        self.memory.analyze_allocation_patterns = true;
        self.safety.test_overflow = true;
        self.safety.test_division_by_zero = true;
        self.safety.test_precision_loss = true;
        self.validation.cross_library_consistency = true;
        self.validation.edge_case_testing = true;
        self.validation.mathematical_properties = true;
        self.profiling.basic_operations = true;
        self.profiling.complex_operations = true;
        self.profiling.memory_intensive = true;
        self.error_analysis.floating_point_errors = true;
        self.error_analysis.accumulation_errors = true;
        self.error_analysis.catastrophic_cancellation = true;
    }

    pub fn set_fast_mode(&mut self) {
        self.benchmark.iterations = 1000;
        self.benchmark.warmup_iterations = 10;
        self.benchmark.measurement_time_secs = 1;
        self.precision.accumulation_iterations = 1000;
        self.memory.large_dataset_size = 10000;
        self.memory.test_large_datasets = false;
        self.profiling.memory_intensive = false;
    }

    pub fn set_thorough_mode(&mut self) {
        self.benchmark.iterations = 100000;
        self.benchmark.warmup_iterations = 1000;
        self.benchmark.measurement_time_secs = 10;
        self.precision.accumulation_iterations = 100000;
        self.memory.large_dataset_size = 1000000;
        self.enable_all_analysis();
    }
}
