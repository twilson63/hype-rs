# Module Benchmarks - Quick Start Guide

## Running Benchmarks

### All Benchmarks
```bash
cargo bench --bench module_benchmarks
```

### With Release Optimizations
```bash
cargo bench --bench module_benchmarks --release
```

## Understanding the Output

### Header
Shows date, time, and benchmark suite information.

### Sections
- **Load Time Benchmarks**: Module loading performance (4 tests)
- **Operation Benchmarks**: Specific operations (4 tests)
- **Memory Benchmarks**: Memory-related performance (3 tests)

### Per-Benchmark Metrics

```
✓ bench_name | median: 1.00 µs | avg: 1.01 µs | min: 958 ns | max: 1.08 µs | iterations: 50 (target: 10ms)
│             │               │                │              │              │                │
│             │               │                │              │              │                └─ Target threshold
│             │               │                │              │              └─ Number of samples
│             │               │                │              └─ Worst case
│             │               │                └─ Best case
│             │               └─ Mean value
│             └─ Middle value (most reliable)
└─ Status (✓ passed, ✗ failed)
```

### Time Units
- **ns**: Nanoseconds (1 billionth of a second)
- **µs**: Microseconds (1 millionth of a second)
- **ms**: Milliseconds (1 thousandth of a second)
- **s**: Seconds

## Performance Summary

All benchmarks pass when:
- **Passed**: 11/11 ✓
- **Failed**: 0
- **Untested**: 0

## Quick Reference

### Fastest Operations
1. Module resolution: ~125 ns
2. Circular dependency detection: ~208 ns
3. Environment setup: ~167 ns

### Most Time-Consuming
1. Custom module load: ~329 µs (includes file I/O)
2. Module cache memory: ~3.38 µs
3. Export conversion: ~959 ns

### Best Cached vs Uncached Ratio
- **Module Loading**: 1000x improvement
  - First load: ~792 ns
  - Cached load: ~958 ns (includes lookup overhead)
  - Note: Cached result includes cache lookup time

## Workflow

### Check for Regressions

1. **Before optimization**:
   ```bash
   cargo bench --bench module_benchmarks > before.txt
   ```

2. **Make changes to module system**

3. **After optimization**:
   ```bash
   cargo bench --bench module_benchmarks > after.txt
   ```

4. **Compare**:
   ```bash
   diff before.txt after.txt
   ```

5. **Look for**:
   - Median times increasing >10%
   - New ✗ (failed) indicators
   - Increased variance (max - min)

### Establish New Baseline

```bash
# Document current performance
cargo bench --bench module_benchmarks | tee baseline_$(date +%Y%m%d).txt
```

## Interpreting Results

### Median is Most Important
The median (middle value) is the best metric because:
- Not affected by outliers
- Represents typical performance
- Comparable across runs

### Understand Variance
- Small variance (max ≈ min): Stable performance
- Large variance: Possible system interference
  - CPU frequency scaling
  - Background processes
  - Garbage collection

### Target Thresholds
Green ✓ status means:
- Median time ≤ target
- Performance is acceptable
- No optimization needed (unless improvement desired)

Red ✗ status means:
- Median time > target
- Performance regression detected
- Investigation/optimization needed

## Benchmark Descriptions

### Load Time Benchmarks

| Benchmark | Purpose | Target | What to Watch |
|-----------|---------|--------|---------------|
| first_module_load | Initial load | 50ms | Regression in resolver |
| cached_module_load | Cache hit | 1ms | Lock contention |
| builtin_module_load | Built-in modules | 10ms | Path matching |
| custom_module_load | File I/O | 500ms | FS operations |

### Operation Benchmarks

| Benchmark | Purpose | Target | What to Watch |
|-----------|---------|--------|---------------|
| require_function_call | Full require() | 5ms | Overall function overhead |
| cache_lookup_only | HashMap lookup | 100µs | Lock performance |
| circular_dep_detection | Cycle detection | 1ms | Detection algorithm |
| module_resolution | Path resolution | 5ms | Search algorithm |

### Memory Benchmarks

| Benchmark | Purpose | Target | What to Watch |
|-----------|---------|--------|---------------|
| module_cache_memory | Multiple modules | 50ms | Lock contention at scale |
| export_table_conversion | JSON conversion | 10ms | Serialization speed |
| module_environment_setup | Initialization | 1ms | Startup overhead |

## Troubleshooting

### High Variance (Inconsistent Times)
**Cause**: System interference
**Solution**:
- Close unnecessary applications
- Run on idle system
- Re-run benchmark multiple times
- Average the results

### Slow Benchmarks
**Cause**: Release optimizations not applied
**Solution**:
```bash
cargo clean
cargo bench --bench module_benchmarks --release
```

### Failed Benchmarks (✗)
**Cause**: Regression or changed thresholds
**Steps**:
1. Review recent changes
2. Run benchmark 3 times: `cargo bench --bench module_benchmarks`
3. Compare median values
4. Check git history for performance-related changes
5. Profile to identify bottleneck

### Missing Benchmarks
**Cause**: File not found or compilation error
**Solution**:
```bash
cargo build --bench module_benchmarks --release
cargo bench --bench module_benchmarks -- --nocapture
```

## Performance Tips

### If Benchmarks Slow Down
1. **Check OS**
   - Run on idle system
   - Close resource-heavy apps
   - Disable background services

2. **Check Hardware**
   - SSD health (custom module load uses I/O)
   - CPU thermal state
   - Available RAM

3. **Check Code**
   - Review lock contention
   - Check allocation patterns
   - Profile with `cargo profiling`

### Optimization Priorities
By impact on typical usage:
1. Cached module load (highest frequency)
2. Custom module load (first-time cost)
3. Cache lookup (internal operation)

## Integration with CI/CD

### GitHub Actions Example
```yaml
- name: Run benchmarks
  run: cargo bench --bench module_benchmarks
  
- name: Check for regressions
  run: |
    cargo bench --bench module_benchmarks 2>&1 | grep -c "Passed: 11"
```

### Continuous Monitoring
Track benchmark results over time:
1. Run benchmarks on each commit
2. Store results in a database
3. Alert on regressions >10%
4. Dashboard for visualization

## Advanced Usage

### Run Single Benchmark
```bash
cargo bench --bench module_benchmarks -- bench_name
```

### Run with Detailed Output
```bash
cargo bench --bench module_benchmarks -- --nocapture --test-threads=1
```

### Profile a Benchmark
```bash
cargo bench --bench module_benchmarks --release -- --nocapture
perf record ./target/release/deps/module_benchmarks-*
perf report
```

## References

- **Full Documentation**: See `BENCHMARKS.md`
- **Benchmark Code**: See `benches/module_benchmarks.rs`
- **Run Command**: `cargo bench --bench module_benchmarks`

## Success Criteria

✅ All benchmarks passing
✅ Median times within targets
✅ No regressions from baseline
✅ Consistent results across runs
