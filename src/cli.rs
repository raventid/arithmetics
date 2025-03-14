use clap::{Parser, Subcommand, ValueEnum};
use crate::config::{ArithmeticConfig, OutputFormat};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "arithmetics")]
#[command(about = "A comprehensive arithmetic libraries comparison tool for Rust")]
#[command(version = "0.1.0")]
#[command(author = "raventid")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Configuration file path
    #[arg(short, long, default_value = "config.toml")]
    pub config: PathBuf,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Number of benchmark iterations
    #[arg(short, long)]
    pub iterations: Option<usize>,

    /// Output directory for results
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Output format
    #[arg(long, value_enum)]
    pub format: Option<CliOutputFormat>,

    /// Libraries to enable (comma-separated)
    #[arg(long)]
    pub libraries: Option<String>,

    /// Enable fast mode (fewer iterations)
    #[arg(long)]
    pub fast: bool,

    /// Enable thorough mode (more iterations and all analysis)
    #[arg(long)]
    pub thorough: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run the full benchmark suite
    Benchmark {
        /// Specific benchmark to run
        #[arg(short, long)]
        suite: Option<String>,
        
        /// Save results to file
        #[arg(short, long)]
        save: bool,
    },
    
    /// Run validation tests only
    Validate {
        /// Enable cross-library consistency checks
        #[arg(long)]
        cross_library: bool,
        
        /// Enable edge case testing
        #[arg(long)]
        edge_cases: bool,
    },
    
    /// Run analysis modules
    Analyze {
        /// Analysis type to run
        #[arg(value_enum)]
        analysis_type: AnalysisType,
    },
    
    /// Generate configuration file
    Config {
        /// Create default configuration
        #[arg(long)]
        create_default: bool,
        
        /// Validate existing configuration
        #[arg(long)]
        validate: bool,
        
        /// Print configuration summary
        #[arg(long)]
        show: bool,
    },
    
    /// Run performance profiling
    Profile {
        /// Enable detailed profiling
        #[arg(long)]
        detailed: bool,
        
        /// Include memory analysis
        #[arg(long)]
        memory: bool,
    },
    
    /// Compare specific arithmetic operations
    Compare {
        /// Operation to compare
        #[arg(value_enum)]
        operation: Operation,
        
        /// First operand
        #[arg(short = 'a')]
        operand_a: f64,
        
        /// Second operand  
        #[arg(short = 'b')]
        operand_b: Option<f64>,
    },
    
    /// Generate reports
    Report {
        /// Report type to generate
        #[arg(value_enum)]
        report_type: ReportType,
        
        /// Input directory containing results
        #[arg(short, long)]
        input: Option<PathBuf>,
    },
}

#[derive(Clone, ValueEnum)]
pub enum CliOutputFormat {
    Text,
    Json,
    Csv,
    Html,
}

#[derive(Clone, ValueEnum)]
pub enum AnalysisType {
    Precision,
    Memory,
    Safety,
    Error,
    All,
}

#[derive(Clone, ValueEnum)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Sqrt,
    Power,
}

#[derive(Clone, ValueEnum)]
pub enum ReportType {
    Summary,
    Detailed,
    Comparison,
    Charts,
}

impl From<CliOutputFormat> for OutputFormat {
    fn from(cli_format: CliOutputFormat) -> Self {
        match cli_format {
            CliOutputFormat::Text => OutputFormat::Text,
            CliOutputFormat::Json => OutputFormat::Json,
            CliOutputFormat::Csv => OutputFormat::Csv,
            CliOutputFormat::Html => OutputFormat::Html,
        }
    }
}

pub struct CliHandler;

impl CliHandler {
    pub fn handle_cli() -> Result<(), Box<dyn std::error::Error>> {
        let cli = Cli::parse();
        
        // Load or create configuration
        let mut config = if cli.config.exists() {
            ArithmeticConfig::load_from_file(&cli.config)?
        } else {
            println!("Configuration file not found, using defaults");
            ArithmeticConfig::default()
        };

        // Apply CLI overrides
        Self::apply_cli_overrides(&mut config, &cli);

        // Validate configuration
        config.validate().map_err(|e| format!("Configuration error: {}", e))?;

        // Handle commands
        match cli.command {
            Some(Commands::Benchmark { suite, save }) => {
                Self::handle_benchmark(&config, suite, save)?;
            },
            Some(Commands::Validate { cross_library, edge_cases }) => {
                Self::handle_validate(&config, cross_library, edge_cases)?;
            },
            Some(Commands::Analyze { analysis_type }) => {
                Self::handle_analyze(&config, analysis_type)?;
            },
            Some(Commands::Config { create_default, validate, show }) => {
                Self::handle_config(&cli.config, create_default, validate, show)?;
            },
            Some(Commands::Profile { detailed, memory }) => {
                Self::handle_profile(&config, detailed, memory)?;
            },
            Some(Commands::Compare { operation, operand_a, operand_b }) => {
                Self::handle_compare(&config, operation, operand_a, operand_b)?;
            },
            Some(Commands::Report { report_type, input }) => {
                Self::handle_report(&config, report_type, input)?;
            },
            None => {
                // Default: run full analysis
                Self::run_full_analysis(&config)?;
            },
        }

        Ok(())
    }

    fn apply_cli_overrides(config: &mut ArithmeticConfig, cli: &Cli) {
        if cli.verbose {
            config.output.verbose = true;
        }

        if let Some(iterations) = cli.iterations {
            config.update_iterations(iterations);
        }

        if let Some(output) = &cli.output {
            config.output.output_directory = output.to_string_lossy().to_string();
        }

        if let Some(format) = &cli.format {
            config.output.output_format = format.clone().into();
        }

        if let Some(libraries) = &cli.libraries {
            config.enabled_libraries = libraries
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
        }

        if cli.fast {
            config.set_fast_mode();
        }

        if cli.thorough {
            config.set_thorough_mode();
        }
    }

    fn handle_benchmark(
        config: &ArithmeticConfig,
        suite: Option<String>,
        save: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Running benchmark suite...");
        
        if let Some(suite_name) = suite {
            println!("Running specific benchmark: {}", suite_name);
            // Implementation would run specific benchmark
        } else {
            println!("Running all benchmarks");
            // Implementation would run all benchmarks
        }

        if save {
            println!("Results will be saved to: {}", config.output.output_directory);
        }

        Ok(())
    }

    fn handle_validate(
        config: &ArithmeticConfig,
        cross_library: bool,
        edge_cases: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Running validation tests...");
        
        if cross_library {
            println!("Including cross-library consistency checks");
        }
        
        if edge_cases {
            println!("Including edge case testing");
        }

        // Implementation would run validation tests
        Ok(())
    }

    fn handle_analyze(
        config: &ArithmeticConfig,
        analysis_type: AnalysisType,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match analysis_type {
            AnalysisType::Precision => {
                println!("Running precision analysis...");
                // Implementation would run precision analysis
            },
            AnalysisType::Memory => {
                println!("Running memory analysis...");
                // Implementation would run memory analysis
            },
            AnalysisType::Safety => {
                println!("Running safety analysis...");
                // Implementation would run safety analysis
            },
            AnalysisType::Error => {
                println!("Running error analysis...");
                // Implementation would run error analysis
            },
            AnalysisType::All => {
                println!("Running all analyses...");
                // Implementation would run all analyses
            },
        }

        Ok(())
    }

    fn handle_config(
        config_path: &PathBuf,
        create_default: bool,
        validate: bool,
        show: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if create_default {
            ArithmeticConfig::create_default_config_file(config_path)?;
            println!("Created default configuration file: {}", config_path.display());
        }

        if validate {
            let config = ArithmeticConfig::load_from_file(config_path)?;
            config.validate()?;
            println!("Configuration is valid");
        }

        if show {
            let config = ArithmeticConfig::load_from_file(config_path)?;
            config.print_summary();
        }

        Ok(())
    }

    fn handle_profile(
        config: &ArithmeticConfig,
        detailed: bool,
        memory: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Running performance profiling...");
        
        if detailed {
            println!("Including detailed profiling");
        }
        
        if memory {
            println!("Including memory profiling");
        }

        // Implementation would run profiling
        Ok(())
    }

    fn handle_compare(
        config: &ArithmeticConfig,
        operation: Operation,
        operand_a: f64,
        operand_b: Option<f64>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Comparing operation: {:?}", operation);
        println!("Operand A: {}", operand_a);
        
        if let Some(b) = operand_b {
            println!("Operand B: {}", b);
        }

        // Implementation would compare operation across libraries
        Ok(())
    }

    fn handle_report(
        config: &ArithmeticConfig,
        report_type: ReportType,
        input: Option<PathBuf>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Generating report: {:?}", report_type);
        
        let input_dir = input
            .as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| config.output.output_directory.clone());
            
        println!("Using input directory: {}", input_dir);

        // Implementation would generate reports
        Ok(())
    }

    fn run_full_analysis(config: &ArithmeticConfig) -> Result<(), Box<dyn std::error::Error>> {
        println!("Running full arithmetic libraries analysis...");
        config.print_summary();
        
        // Implementation would run all enabled analysis modules
        // This would call the existing main.rs functionality
        
        Ok(())
    }
}
