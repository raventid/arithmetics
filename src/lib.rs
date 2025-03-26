pub mod precision;
pub mod safety;
pub mod validation;
pub mod profiling;
pub mod error_analysis;
pub mod cli;
pub mod config;
pub mod export;
pub mod gpu;
pub mod platform;
pub mod profiler;
pub mod regression;
pub mod visualization;

// Re-export commonly used types
pub use config::ArithmeticConfig;
pub use validation::ValidationSuite;
pub use profiling::PerformanceProfiler;
pub use error_analysis::AdvancedErrorAnalyzer;
