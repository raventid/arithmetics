use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Platform-specific configuration and behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformInfo {
    pub os: String,
    pub arch: String,
    pub endianness: String,
    pub pointer_width: usize,
    pub page_size: usize,
    pub cache_line_size: usize,
    pub cpu_features: Vec<String>,
    pub compiler: String,
    pub rust_version: String,
}

impl PlatformInfo {
    pub fn detect() -> Self {
        Self {
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            endianness: if cfg!(target_endian = "big") { "big".to_string() } else { "little".to_string() },
            pointer_width: std::mem::size_of::<usize>() * 8,
            page_size: Self::detect_page_size(),
            cache_line_size: Self::detect_cache_line_size(),
            cpu_features: Self::detect_cpu_features(),
            compiler: Self::detect_compiler(),
            rust_version: env!("RUSTC_VERSION").to_string(),
        }
    }

    fn detect_page_size() -> usize {
        #[cfg(unix)]
        {
            unsafe { libc::sysconf(libc::_SC_PAGESIZE) as usize }
        }
        #[cfg(windows)]
        {
            use winapi::um::sysinfoapi::{GetSystemInfo, SYSTEM_INFO};
            unsafe {
                let mut sys_info: SYSTEM_INFO = std::mem::zeroed();
                GetSystemInfo(&mut sys_info);
                sys_info.dwPageSize as usize
            }
        }
        #[cfg(not(any(unix, windows)))]
        {
            4096 // Default assumption
        }
    }

    fn detect_cache_line_size() -> usize {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            64 // Common cache line size for x86/x64
        }
        #[cfg(target_arch = "aarch64")]
        {
            64 // ARM64 typically uses 64-byte cache lines
        }
        #[cfg(target_arch = "arm")]
        {
            32 // ARM32 often uses 32-byte cache lines
        }
        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64", target_arch = "arm")))]
        {
            64 // Default assumption
        }
    }

    fn detect_cpu_features() -> Vec<String> {
        let mut features = Vec::new();
        
        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("sse") { features.push("sse".to_string()); }
            if is_x86_feature_detected!("sse2") { features.push("sse2".to_string()); }
            if is_x86_feature_detected!("sse3") { features.push("sse3".to_string()); }
            if is_x86_feature_detected!("ssse3") { features.push("ssse3".to_string()); }
            if is_x86_feature_detected!("sse4.1") { features.push("sse4.1".to_string()); }
            if is_x86_feature_detected!("sse4.2") { features.push("sse4.2".to_string()); }
            if is_x86_feature_detected!("avx") { features.push("avx".to_string()); }
            if is_x86_feature_detected!("avx2") { features.push("avx2".to_string()); }
            if is_x86_feature_detected!("fma") { features.push("fma".to_string()); }
        }
        
        #[cfg(target_arch = "aarch64")]
        {
            // ARM features would be detected here
            features.push("neon".to_string()); // Most ARM64 has NEON
        }
        
        features
    }

    fn detect_compiler() -> String {
        if cfg!(target_env = "msvc") {
            "msvc".to_string()
        } else if cfg!(target_env = "gnu") {
            "gnu".to_string()
        } else {
            "unknown".to_string()
        }
    }
}

/// Cross-platform arithmetic behavior tester
pub struct CrossPlatformTester {
    platform_info: PlatformInfo,
    test_results: HashMap<String, TestResult>,
}

impl CrossPlatformTester {
    pub fn new() -> Self {
        Self {
            platform_info: PlatformInfo::detect(),
            test_results: HashMap::new(),
        }
    }

    pub fn run_all_tests(&mut self) {
        self.test_float_behavior();
        self.test_integer_overflow();
        self.test_endianness_consistency();
        self.test_alignment_requirements();
        self.test_precision_limits();
        self.test_nan_inf_behavior();
        self.test_rounding_modes();
        self.test_subnormal_handling();
    }

    fn test_float_behavior(&mut self) {
        let mut issues = Vec::new();
        
        // Test basic floating point operations
        let a = 0.1f64;
        let b = 0.2f64;
        let c = 0.3f64;
        let sum = a + b;
        
        if (sum - c).abs() > 1e-15 {
            issues.push(format!("0.1 + 0.2 != 0.3, difference: {}", sum - c));
        }
        
        // Test denormal numbers
        let tiny = f64::MIN_POSITIVE / 2.0;
        if tiny != 0.0 && tiny.is_normal() {
            issues.push("Denormal number handling inconsistency".to_string());
        }
        
        // Test infinity arithmetic
        let inf = f64::INFINITY;
        if (inf + 1.0) != inf {
            issues.push("Infinity arithmetic inconsistency".to_string());
        }
        
        self.test_results.insert("float_behavior".to_string(), TestResult {
            passed: issues.is_empty(),
            issues,
            platform_specific: self.platform_info.clone(),
        });
    }

    fn test_integer_overflow(&mut self) {
        let mut issues = Vec::new();
        
        // Test wrapping behavior
        let max_i32 = i32::MAX;
        let wrapped = max_i32.wrapping_add(1);
        if wrapped != i32::MIN {
            issues.push("i32 wrapping overflow inconsistency".to_string());
        }
        
        // Test saturating behavior
        let saturated = max_i32.saturating_add(1);
        if saturated != max_i32 {
            issues.push("i32 saturating overflow inconsistency".to_string());
        }
        
        self.test_results.insert("integer_overflow".to_string(), TestResult {
            passed: issues.is_empty(),
            issues,
            platform_specific: self.platform_info.clone(),
        });
    }

    fn test_endianness_consistency(&mut self) {
        let mut issues = Vec::new();
        
        let value: u32 = 0x12345678;
        let bytes = value.to_ne_bytes();
        let reconstructed = u32::from_ne_bytes(bytes);
        
        if value != reconstructed {
            issues.push("Endianness conversion inconsistency".to_string());
        }
        
        // Test cross-endian compatibility
        let be_bytes = value.to_be_bytes();
        let le_bytes = value.to_le_bytes();
        
        if cfg!(target_endian = "big") && be_bytes != bytes {
            issues.push("Big-endian native conversion mismatch".to_string());
        }
        
        if cfg!(target_endian = "little") && le_bytes != bytes {
            issues.push("Little-endian native conversion mismatch".to_string());
        }
        
        self.test_results.insert("endianness".to_string(), TestResult {
            passed: issues.is_empty(),
            issues,
            platform_specific: self.platform_info.clone(),
        });
    }

    fn test_alignment_requirements(&mut self) {
        let mut issues = Vec::new();
        
        // Test structure alignment
        #[repr(C)]
        struct TestStruct {
            a: u8,
            b: u64,
            c: u8,
        }
        
        let expected_size = if cfg!(target_arch = "x86_64") || cfg!(target_arch = "aarch64") {
            24 // 8-byte alignment for u64
        } else {
            16 // Might be different on other platforms
        };
        
        if std::mem::size_of::<TestStruct>() < 10 {
            issues.push("Unexpected struct packing behavior".to_string());
        }
        
        // Test alignment of various types
        if std::mem::align_of::<u64>() != 8 && std::mem::align_of::<u64>() != 4 {
            issues.push("Unexpected u64 alignment".to_string());
        }
        
        self.test_results.insert("alignment".to_string(), TestResult {
            passed: issues.is_empty(),
            issues,
            platform_specific: self.platform_info.clone(),
        });
    }

    fn test_precision_limits(&mut self) {
        let mut issues = Vec::new();
        
        // Test f64 precision limits
        let large_int = 9007199254740992i64; // 2^53
        let as_float = large_int as f64;
        let back_to_int = as_float as i64;
        
        if large_int != back_to_int {
            issues.push("f64 integer precision limit exceeded".to_string());
        }
        
        // Test f32 precision
        let large_int_f32 = 16777216i32; // 2^24
        let as_f32 = large_int_f32 as f32;
        let back_to_i32 = as_f32 as i32;
        
        if large_int_f32 != back_to_i32 {
            issues.push("f32 integer precision limit exceeded".to_string());
        }
        
        self.test_results.insert("precision_limits".to_string(), TestResult {
            passed: issues.is_empty(),
            issues,
            platform_specific: self.platform_info.clone(),
        });
    }

    fn test_nan_inf_behavior(&mut self) {
        let mut issues = Vec::new();
        
        let nan = f64::NAN;
        let inf = f64::INFINITY;
        let neg_inf = f64::NEG_INFINITY;
        
        // Test NaN behavior
        if nan == nan {
            issues.push("NaN comparison should always be false".to_string());
        }
        
        if !nan.is_nan() {
            issues.push("NaN detection failed".to_string());
        }
        
        // Test infinity behavior
        if !inf.is_infinite() || !neg_inf.is_infinite() {
            issues.push("Infinity detection failed".to_string());
        }
        
        if inf <= 0.0 || neg_inf >= 0.0 {
            issues.push("Infinity comparison inconsistency".to_string());
        }
        
        // Test arithmetic with special values
        if (inf / inf).is_finite() {
            issues.push("inf/inf should be NaN".to_string());
        }
        
        self.test_results.insert("nan_inf".to_string(), TestResult {
            passed: issues.is_empty(),
            issues,
            platform_specific: self.platform_info.clone(),
        });
    }

    fn test_rounding_modes(&mut self) {
        let mut issues = Vec::new();
        
        // Test default rounding behavior
        let a = 1.5f64;
        let b = 2.5f64;
        
        // Rust uses "round half to even" by default
        if a.round() != 2.0 {
            issues.push("1.5 should round to 2.0".to_string());
        }
        
        if b.round() != 2.0 {
            issues.push("2.5 should round to 2.0 (round half to even)".to_string());
        }
        
        // Test truncation
        let c = 3.7f64;
        if c.trunc() != 3.0 {
            issues.push("3.7 should truncate to 3.0".to_string());
        }
        
        self.test_results.insert("rounding".to_string(), TestResult {
            passed: issues.is_empty(),
            issues,
            platform_specific: self.platform_info.clone(),
        });
    }

    fn test_subnormal_handling(&mut self) {
        let mut issues = Vec::new();
        
        // Create a subnormal number
        let mut x = f64::MIN_POSITIVE;
        for _ in 0..1000 {
            x /= 2.0;
            if x == 0.0 {
                break;
            }
        }
        
        // Test subnormal detection and handling
        if x != 0.0 && x.is_normal() {
            issues.push("Subnormal number incorrectly classified as normal".to_string());
        }
        
        // Test arithmetic with subnormals
        let tiny = f64::MIN_POSITIVE / 1e100;
        if tiny != 0.0 {
            let doubled = tiny * 2.0;
            if doubled == 0.0 && tiny != 0.0 {
                issues.push("Subnormal arithmetic inconsistency".to_string());
            }
        }
        
        self.test_results.insert("subnormal".to_string(), TestResult {
            passed: issues.is_empty(),
            issues,
            platform_specific: self.platform_info.clone(),
        });
    }

    pub fn get_platform_info(&self) -> &PlatformInfo {
        &self.platform_info
    }

    pub fn get_test_results(&self) -> &HashMap<String, TestResult> {
        &self.test_results
    }

    pub fn print_summary(&self) {
        println!("Cross-Platform Arithmetic Compatibility Report");
        println!("==============================================");
        println!("Platform: {} {}", self.platform_info.os, self.platform_info.arch);
        println!("Endianness: {}", self.platform_info.endianness);
        println!("Pointer width: {} bits", self.platform_info.pointer_width);
        println!("Page size: {} bytes", self.platform_info.page_size);
        println!("Cache line size: {} bytes", self.platform_info.cache_line_size);
        println!("CPU features: {:?}", self.platform_info.cpu_features);
        println!("Compiler: {}", self.platform_info.compiler);
        println!("Rust version: {}", self.platform_info.rust_version);
        println!();

        let mut passed = 0;
        let mut total = 0;

        for (test_name, result) in &self.test_results {
            total += 1;
            if result.passed {
                passed += 1;
                println!("✓ {}: PASSED", test_name);
            } else {
                println!("✗ {}: FAILED", test_name);
                for issue in &result.issues {
                    println!("  - {}", issue);
                }
            }
        }

        println!();
        println!("Summary: {}/{} tests passed", passed, total);
        
        if passed == total {
            println!("All tests passed! Platform behavior is consistent.");
        } else {
            println!("Some tests failed. Platform-specific behavior detected.");
        }
    }

    pub fn export_results_json(&self) -> serde_json::Result<String> {
        let report = CompatibilityReport {
            platform_info: self.platform_info.clone(),
            test_results: self.test_results.clone(),
            timestamp: chrono::Utc::now(),
        };
        serde_json::to_string_pretty(&report)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub passed: bool,
    pub issues: Vec<String>,
    pub platform_specific: PlatformInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityReport {
    pub platform_info: PlatformInfo,
    pub test_results: HashMap<String, TestResult>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Decimal type cross-platform behavior tester
pub struct DecimalCrossPlatformTester {
    platform_info: PlatformInfo,
}

impl DecimalCrossPlatformTester {
    pub fn new() -> Self {
        Self {
            platform_info: PlatformInfo::detect(),
        }
    }

    pub fn test_decimal_serialization(&self) -> bool {
        use rust_decimal::Decimal;
        use std::str::FromStr;
        
        let original = Decimal::from_str("123.456789123456789").unwrap();
        let serialized = original.to_string();
        let deserialized = Decimal::from_str(&serialized).unwrap();
        
        original == deserialized
    }

    pub fn test_decimal_precision_consistency(&self) -> Vec<String> {
        use rust_decimal::Decimal;
        let mut issues = Vec::new();
        
        // Test maximum precision
        let max_precision = Decimal::from_str("1.234567890123456789012345678").unwrap();
        let serialized = max_precision.to_string();
        
        if serialized.len() < 20 {
            issues.push("Decimal precision loss detected".to_string());
        }
        
        // Test arithmetic consistency
        let a = Decimal::from_str("0.1").unwrap();
        let b = Decimal::from_str("0.2").unwrap();
        let c = Decimal::from_str("0.3").unwrap();
        let sum = a + b;
        
        if sum != c {
            issues.push(format!("Decimal arithmetic inconsistency: 0.1 + 0.2 = {}", sum));
        }
        
        issues
    }

    pub fn test_bigdecimal_cross_platform(&self) -> Vec<String> {
        use bigdecimal::BigDecimal;
        use std::str::FromStr;
        let mut issues = Vec::new();
        
        // Test very large numbers
        let large = BigDecimal::from_str("123456789012345678901234567890.123456789012345678901234567890").unwrap();
        let serialized = large.to_string();
        let deserialized = BigDecimal::from_str(&serialized).unwrap();
        
        if large != deserialized {
            issues.push("BigDecimal serialization/deserialization inconsistency".to_string());
        }
        
        // Test arithmetic with large numbers
        let a = BigDecimal::from_str("999999999999999999999999999999").unwrap();
        let b = BigDecimal::from(1);
        let sum = &a + &b;
        
        if sum.to_string() != "1000000000000000000000000000000" {
            issues.push("BigDecimal large number arithmetic inconsistency".to_string());
        }
        
        issues
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_detection() {
        let info = PlatformInfo::detect();
        assert!(!info.os.is_empty());
        assert!(!info.arch.is_empty());
        assert!(info.pointer_width == 32 || info.pointer_width == 64);
    }

    #[test]
    fn test_cross_platform_tester() {
        let mut tester = CrossPlatformTester::new();
        tester.run_all_tests();
        
        let results = tester.get_test_results();
        assert!(!results.is_empty());
        
        // Most basic tests should pass
        assert!(results.get("integer_overflow").unwrap().passed);
        assert!(results.get("endianness").unwrap().passed);
    }

    #[test]
    fn test_decimal_cross_platform() {
        let tester = DecimalCrossPlatformTester::new();
        assert!(tester.test_decimal_serialization());
        
        let precision_issues = tester.test_decimal_precision_consistency();
        assert!(precision_issues.is_empty());
    }
}
