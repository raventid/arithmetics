pub mod precision;
pub mod validation;
pub mod profiling;
pub mod error_analysis;
pub mod gpu;
pub mod platform;
pub mod profiler;
pub mod regression;

// Re-export commonly used types
pub use validation::ValidationSuite;
pub use profiling::PerformanceProfiler;
pub use error_analysis::AdvancedErrorAnalyzer;
