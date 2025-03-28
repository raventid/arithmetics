#!/bin/bash

# Comprehensive Arithmetic Libraries Benchmark Script
# This script runs all benchmarks and generates complete analysis reports

set -e

echo "🚀 Starting Comprehensive Arithmetic Libraries Analysis"
echo "======================================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Create output directory
OUTPUT_DIR="benchmark_results_$(date +%Y%m%d_%H%M%S)"
mkdir -p "$OUTPUT_DIR"

echo -e "${BLUE}📁 Created output directory: $OUTPUT_DIR${NC}"

# Function to run benchmark and capture output
run_benchmark() {
    local benchmark_name="$1"
    local output_file="$OUTPUT_DIR/${benchmark_name}_results.txt"
    
    echo -e "${YELLOW}🔄 Running $benchmark_name benchmark...${NC}"
    
    if cargo bench --bench "$benchmark_name" > "$output_file" 2>&1; then
        echo -e "${GREEN}✅ $benchmark_name completed successfully${NC}"
    else
        echo -e "${RED}❌ $benchmark_name failed${NC}"
        echo "Error output saved to: $output_file"
    fi
}

# Function to run tests
run_tests() {
    echo -e "${YELLOW}🧪 Running unit tests...${NC}"
    local test_output="$OUTPUT_DIR/test_results.txt"
    
    if cargo test > "$test_output" 2>&1; then
        echo -e "${GREEN}✅ All tests passed${NC}"
    else
        echo -e "${RED}❌ Some tests failed${NC}"
        echo "Test output saved to: $test_output"
    fi
}

# Main execution
main() {
    # Build the project first
    echo -e "${YELLOW}🔨 Building project...${NC}"
    if cargo build --release; then
        echo -e "${GREEN}✅ Build successful${NC}"
    else
        echo -e "${RED}❌ Build failed${NC}"
        exit 1
    fi
    
    # Run all benchmarks
    echo -e "\n${BLUE}📊 Running Benchmark Suite${NC}"
    echo "================================"
    
    run_benchmark "arithmetic_comparison"
    run_benchmark "complex_operations"
    run_benchmark "batch_operations"
    run_benchmark "fixed_point"
    run_benchmark "half_precision"
    run_benchmark "simd_operations"
    run_benchmark "multithreaded_operations"
    run_benchmark "real_world_applications"
    run_benchmark "profiling_benchmarks"
    run_benchmark "cross_platform_compatibility"
    run_benchmark "error_analysis_benchmarks"
    run_benchmark "integration_tests"
    
    # Run unit tests
    echo -e "\n${BLUE}🧪 Running Test Suite${NC}"
    echo "======================"
    run_tests
    
    # Generate summary report
    echo -e "\n${BLUE}📄 Generating Summary Report${NC}"
    echo "=============================="
    
    cat > "$OUTPUT_DIR/README.md" << EOF
# Arithmetic Libraries Benchmark Results

Generated on: $(date)
Platform: $(uname -a)
Rust Version: $(rustc --version)

## Benchmark Results

This directory contains comprehensive benchmark results for arithmetic libraries comparison.

### Files Overview

$(ls -la "$OUTPUT_DIR" | grep -v "^total" | grep -v "README.md" | awk '{print "- " $9 " (" $5 " bytes)"}')

### Quick Analysis

To analyze the results:

1. **Performance Overview**: Check \`*_results.txt\` files for detailed timing data
2. **Error Analysis**: Review error analysis benchmarks for precision insights
3. **Cross-Platform**: Examine platform compatibility results
4. **Integration**: Check integration test results for system health

### Key Metrics to Look For

- **Throughput**: Operations per second for each library
- **Latency**: Individual operation timing
- **Memory Usage**: Allocation patterns and peak usage
- **Precision**: Error rates and catastrophic cancellation detection
- **Scalability**: Multi-threading and SIMD performance gains

### Interpreting Results

- Lower timing values indicate better performance
- Higher throughput values indicate better performance
- Memory usage should be considered alongside performance
- Error rates should be minimized for precision-critical applications

EOF

    # Generate platform info
    echo -e "\n${BLUE}🖥️  Platform Information${NC}" >> "$OUTPUT_DIR/README.md"
    echo "=========================" >> "$OUTPUT_DIR/README.md"
    echo "" >> "$OUTPUT_DIR/README.md"
    echo "- OS: $(uname -s)" >> "$OUTPUT_DIR/README.md"
    echo "- Architecture: $(uname -m)" >> "$OUTPUT_DIR/README.md"
    echo "- CPU Info: $(sysctl -n machdep.cpu.brand_string 2>/dev/null || echo "N/A")" >> "$OUTPUT_DIR/README.md"
    echo "- Memory: $(system_profiler SPHardwareDataType 2>/dev/null | grep "Memory:" || echo "N/A")" >> "$OUTPUT_DIR/README.md"
    
    # Final summary
    echo -e "\n${GREEN}🎉 Benchmark Analysis Complete!${NC}"
    echo "=================================="
    echo -e "📂 Results saved to: ${YELLOW}$OUTPUT_DIR${NC}"
    echo -e "📄 Summary report: ${YELLOW}$OUTPUT_DIR/README.md${NC}"
    
    # Count successful benchmarks
    local success_count=$(ls "$OUTPUT_DIR"/*_results.txt 2>/dev/null | wc -l | tr -d ' ')
    echo -e "📊 Completed benchmarks: ${GREEN}$success_count${NC}"
    
    echo ""
    echo -e "${BLUE}Next Steps:${NC}"
    echo "1. Review individual benchmark results in $OUTPUT_DIR"
    echo "2. Compare performance across different arithmetic libraries"
    echo "3. Analyze precision vs performance trade-offs"
    echo "4. Use results to optimize your arithmetic-intensive applications"
    
    echo ""
    echo -e "${YELLOW}Happy benchmarking! 📈${NC}"
}

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Cargo not found. Please install Rust and Cargo.${NC}"
    exit 1
fi

# Run main function
main "$@"
