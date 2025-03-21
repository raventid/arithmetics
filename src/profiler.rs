use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::mem;

/// Custom allocator that tracks memory usage
pub struct TrackingAllocator {
    inner: System,
    allocated: AtomicU64,
    peak_allocated: AtomicU64,
    allocation_count: AtomicU64,
    deallocation_count: AtomicU64,
}

impl TrackingAllocator {
    pub const fn new() -> Self {
        Self {
            inner: System,
            allocated: AtomicU64::new(0),
            peak_allocated: AtomicU64::new(0),
            allocation_count: AtomicU64::new(0),
            deallocation_count: AtomicU64::new(0),
        }
    }

    pub fn current_usage(&self) -> u64 {
        self.allocated.load(Ordering::Relaxed)
    }

    pub fn peak_usage(&self) -> u64 {
        self.peak_allocated.load(Ordering::Relaxed)
    }

    pub fn allocation_count(&self) -> u64 {
        self.allocation_count.load(Ordering::Relaxed)
    }

    pub fn deallocation_count(&self) -> u64 {
        self.deallocation_count.load(Ordering::Relaxed)
    }

    pub fn reset_stats(&self) {
        self.allocated.store(0, Ordering::Relaxed);
        self.peak_allocated.store(0, Ordering::Relaxed);
        self.allocation_count.store(0, Ordering::Relaxed);
        self.deallocation_count.store(0, Ordering::Relaxed);
    }
}

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = self.inner.alloc(layout);
        if !ptr.is_null() {
            let size = layout.size() as u64;
            let old_allocated = self.allocated.fetch_add(size, Ordering::Relaxed);
            let new_allocated = old_allocated + size;
            
            // Update peak if necessary
            let mut peak = self.peak_allocated.load(Ordering::Relaxed);
            while new_allocated > peak {
                match self.peak_allocated.compare_exchange_weak(
                    peak,
                    new_allocated,
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => break,
                    Err(new_peak) => peak = new_peak,
                }
            }
            
            self.allocation_count.fetch_add(1, Ordering::Relaxed);
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.inner.dealloc(ptr, layout);
        let size = layout.size() as u64;
        self.allocated.fetch_sub(size, Ordering::Relaxed);
        self.deallocation_count.fetch_add(1, Ordering::Relaxed);
    }
}

/// Performance profiler for tracking operation timing
#[derive(Debug)]
pub struct PerformanceProfiler {
    measurements: HashMap<String, Vec<Duration>>,
    start_times: HashMap<String, Instant>,
}

impl PerformanceProfiler {
    pub fn new() -> Self {
        Self {
            measurements: HashMap::new(),
            start_times: HashMap::new(),
        }
    }

    pub fn start_measurement(&mut self, operation: &str) {
        self.start_times.insert(operation.to_string(), Instant::now());
    }

    pub fn end_measurement(&mut self, operation: &str) {
        if let Some(start_time) = self.start_times.remove(operation) {
            let duration = start_time.elapsed();
            self.measurements
                .entry(operation.to_string())
                .or_insert_with(Vec::new)
                .push(duration);
        }
    }

    pub fn measure_operation<F, T>(&mut self, operation: &str, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        self.start_measurement(operation);
        let result = f();
        self.end_measurement(operation);
        result
    }

    pub fn get_statistics(&self, operation: &str) -> Option<OperationStats> {
        self.measurements.get(operation).map(|durations| {
            let count = durations.len();
            let total: Duration = durations.iter().sum();
            let average = total / count as u32;
            
            let mut sorted = durations.clone();
            sorted.sort();
            
            let median = if sorted.is_empty() {
                Duration::ZERO
            } else if sorted.len() % 2 == 0 {
                (sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2]) / 2
            } else {
                sorted[sorted.len() / 2]
            };
            
            let min = sorted.first().copied().unwrap_or(Duration::ZERO);
            let max = sorted.last().copied().unwrap_or(Duration::ZERO);
            
            let p95_index = ((count as f64) * 0.95) as usize;
            let p95 = sorted.get(p95_index.min(count - 1))
                .copied()
                .unwrap_or(Duration::ZERO);
            
            let p99_index = ((count as f64) * 0.99) as usize;
            let p99 = sorted.get(p99_index.min(count - 1))
                .copied()
                .unwrap_or(Duration::ZERO);

            OperationStats {
                count,
                total,
                average,
                median,
                min,
                max,
                p95,
                p99,
            }
        })
    }

    pub fn print_summary(&self) {
        println!("Performance Profile Summary:");
        println!("{:<30} {:>10} {:>15} {:>15} {:>15} {:>15}", 
                 "Operation", "Count", "Average", "Median", "P95", "P99");
        println!("{}", "-".repeat(90));

        for (operation, _) in &self.measurements {
            if let Some(stats) = self.get_statistics(operation) {
                println!("{:<30} {:>10} {:>15.3?} {:>15.3?} {:>15.3?} {:>15.3?}",
                         operation,
                         stats.count,
                         stats.average,
                         stats.median,
                         stats.p95,
                         stats.p99);
            }
        }
    }

    pub fn clear(&mut self) {
        self.measurements.clear();
        self.start_times.clear();
    }
}

#[derive(Debug, Clone)]
pub struct OperationStats {
    pub count: usize,
    pub total: Duration,
    pub average: Duration,
    pub median: Duration,
    pub min: Duration,
    pub max: Duration,
    pub p95: Duration,
    pub p99: Duration,
}

/// Memory usage tracker for specific operations
#[derive(Debug)]
pub struct MemoryTracker {
    allocator: Arc<TrackingAllocator>,
    snapshots: Vec<MemorySnapshot>,
}

impl MemoryTracker {
    pub fn new(allocator: Arc<TrackingAllocator>) -> Self {
        Self {
            allocator,
            snapshots: Vec::new(),
        }
    }

    pub fn take_snapshot(&mut self, label: String) {
        let snapshot = MemorySnapshot {
            label,
            timestamp: Instant::now(),
            current_usage: self.allocator.current_usage(),
            peak_usage: self.allocator.peak_usage(),
            allocation_count: self.allocator.allocation_count(),
            deallocation_count: self.allocator.deallocation_count(),
        };
        self.snapshots.push(snapshot);
    }

    pub fn measure_memory_usage<F, T>(&mut self, operation: &str, f: F) -> (T, MemoryUsage)
    where
        F: FnOnce() -> T,
    {
        let before_usage = self.allocator.current_usage();
        let before_peak = self.allocator.peak_usage();
        let before_allocs = self.allocator.allocation_count();
        let before_deallocs = self.allocator.deallocation_count();
        
        self.take_snapshot(format!("{}_start", operation));
        let result = f();
        self.take_snapshot(format!("{}_end", operation));
        
        let after_usage = self.allocator.current_usage();
        let after_peak = self.allocator.peak_usage();
        let after_allocs = self.allocator.allocation_count();
        let after_deallocs = self.allocator.deallocation_count();
        
        let usage = MemoryUsage {
            net_allocation: (after_usage as i64) - (before_usage as i64),
            peak_during_operation: after_peak.saturating_sub(before_peak),
            allocations_made: after_allocs - before_allocs,
            deallocations_made: after_deallocs - before_deallocs,
        };
        
        (result, usage)
    }

    pub fn print_snapshots(&self) {
        println!("Memory Usage Snapshots:");
        println!("{:<30} {:>15} {:>15} {:>15} {:>15}", 
                 "Label", "Current (bytes)", "Peak (bytes)", "Allocs", "Deallocs");
        println!("{}", "-".repeat(90));

        for snapshot in &self.snapshots {
            println!("{:<30} {:>15} {:>15} {:>15} {:>15}",
                     snapshot.label,
                     snapshot.current_usage,
                     snapshot.peak_usage,
                     snapshot.allocation_count,
                     snapshot.deallocation_count);
        }
    }

    pub fn get_memory_deltas(&self) -> Vec<MemoryDelta> {
        let mut deltas = Vec::new();
        
        for i in 1..self.snapshots.len() {
            let prev = &self.snapshots[i - 1];
            let curr = &self.snapshots[i];
            
            let delta = MemoryDelta {
                from_label: prev.label.clone(),
                to_label: curr.label.clone(),
                duration: curr.timestamp.duration_since(prev.timestamp),
                usage_change: (curr.current_usage as i64) - (prev.current_usage as i64),
                peak_change: (curr.peak_usage as i64) - (prev.peak_usage as i64),
                allocation_change: curr.allocation_count - prev.allocation_count,
                deallocation_change: curr.deallocation_count - prev.deallocation_count,
            };
            
            deltas.push(delta);
        }
        
        deltas
    }
}

#[derive(Debug, Clone)]
pub struct MemorySnapshot {
    pub label: String,
    pub timestamp: Instant,
    pub current_usage: u64,
    pub peak_usage: u64,
    pub allocation_count: u64,
    pub deallocation_count: u64,
}

#[derive(Debug, Clone)]
pub struct MemoryUsage {
    pub net_allocation: i64,
    pub peak_during_operation: u64,
    pub allocations_made: u64,
    pub deallocations_made: u64,
}

#[derive(Debug, Clone)]
pub struct MemoryDelta {
    pub from_label: String,
    pub to_label: String,
    pub duration: Duration,
    pub usage_change: i64,
    pub peak_change: i64,
    pub allocation_change: u64,
    pub deallocation_change: u64,
}

/// Combined profiler that tracks both performance and memory
pub struct CombinedProfiler {
    performance: PerformanceProfiler,
    memory: MemoryTracker,
}

impl CombinedProfiler {
    pub fn new(allocator: Arc<TrackingAllocator>) -> Self {
        Self {
            performance: PerformanceProfiler::new(),
            memory: MemoryTracker::new(allocator),
        }
    }

    pub fn profile_operation<F, T>(&mut self, operation: &str, f: F) -> (T, OperationProfile)
    where
        F: FnOnce() -> T,
    {
        let start_time = Instant::now();
        let (result, memory_usage) = self.memory.measure_memory_usage(operation, || {
            self.performance.measure_operation(operation, f)
        });
        let duration = start_time.elapsed();

        let profile = OperationProfile {
            operation: operation.to_string(),
            duration,
            memory_usage,
        };

        (result, profile)
    }

    pub fn print_comprehensive_summary(&self) {
        println!("\n=== Comprehensive Performance and Memory Analysis ===");
        self.performance.print_summary();
        println!();
        self.memory.print_snapshots();
        
        println!("\nMemory Deltas:");
        let deltas = self.memory.get_memory_deltas();
        for delta in deltas {
            println!("{} -> {}: {:+} bytes in {:.3?}",
                     delta.from_label, delta.to_label, 
                     delta.usage_change, delta.duration);
        }
    }
}

#[derive(Debug, Clone)]
pub struct OperationProfile {
    pub operation: String,
    pub duration: Duration,
    pub memory_usage: MemoryUsage,
}

/// Cache performance analyzer
pub struct CacheAnalyzer {
    cache_lines_accessed: AtomicU64,
    cache_misses: AtomicU64,
    page_faults: AtomicU64,
}

impl CacheAnalyzer {
    pub fn new() -> Self {
        Self {
            cache_lines_accessed: AtomicU64::new(0),
            cache_misses: AtomicU64::new(0),
            page_faults: AtomicU64::new(0),
        }
    }

    /// Simulate cache behavior for array access patterns
    pub fn analyze_access_pattern<T>(&self, data: &[T], access_pattern: &[usize]) -> CacheStats {
        let cache_line_size = 64; // Common cache line size
        let type_size = mem::size_of::<T>();
        let elements_per_line = cache_line_size / type_size.max(1);
        
        let mut last_cache_line = None;
        let mut cache_lines_used = 0;
        let mut sequential_accesses = 0;
        let mut random_accesses = 0;
        
        for (i, &index) in access_pattern.iter().enumerate() {
            if index < data.len() {
                let cache_line = index / elements_per_line;
                
                if last_cache_line != Some(cache_line) {
                    cache_lines_used += 1;
                    last_cache_line = Some(cache_line);
                }
                
                if i > 0 {
                    let prev_index = access_pattern[i - 1];
                    if index == prev_index + 1 {
                        sequential_accesses += 1;
                    } else {
                        random_accesses += 1;
                    }
                }
            }
        }
        
        CacheStats {
            total_accesses: access_pattern.len(),
            cache_lines_used,
            sequential_accesses,
            random_accesses,
            estimated_cache_misses: cache_lines_used, // Simplified estimation
        }
    }
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub total_accesses: usize,
    pub cache_lines_used: usize,
    pub sequential_accesses: usize,
    pub random_accesses: usize,
    pub estimated_cache_misses: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_performance_profiler() {
        let mut profiler = PerformanceProfiler::new();
        
        // Measure a simple operation
        profiler.measure_operation("test_operation", || {
            thread::sleep(Duration::from_millis(10));
        });
        
        let stats = profiler.get_statistics("test_operation").unwrap();
        assert_eq!(stats.count, 1);
        assert!(stats.average >= Duration::from_millis(9)); // Allow some variance
    }

    #[test]
    fn test_memory_tracking() {
        let allocator = Arc::new(TrackingAllocator::new());
        let mut tracker = MemoryTracker::new(allocator.clone());
        
        allocator.reset_stats();
        tracker.take_snapshot("start".to_string());
        
        // Allocate some memory
        let _data: Vec<u64> = vec![1; 1000];
        
        tracker.take_snapshot("after_allocation".to_string());
        
        let deltas = tracker.get_memory_deltas();
        assert_eq!(deltas.len(), 1);
        assert!(deltas[0].usage_change > 0);
    }

    #[test]
    fn test_cache_analyzer() {
        let analyzer = CacheAnalyzer::new();
        let data: Vec<u64> = (0..1000).collect();
        
        // Sequential access pattern
        let sequential: Vec<usize> = (0..100).collect();
        let seq_stats = analyzer.analyze_access_pattern(&data, &sequential);
        
        // Random access pattern
        let random: Vec<usize> = (0..100).map(|i| (i * 17) % 1000).collect();
        let rand_stats = analyzer.analyze_access_pattern(&data, &random);
        
        assert!(seq_stats.sequential_accesses > rand_stats.sequential_accesses);
        assert!(rand_stats.random_accesses > seq_stats.random_accesses);
    }
}
