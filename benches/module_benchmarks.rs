use std::fs;
use std::path::PathBuf;
use std::time::Instant;
use tempfile::TempDir;

use hype_rs::modules::loader::ModuleLoader;
use hype_rs::modules::resolver::ModuleResolver;

const WARM_UP_ITERATIONS: usize = 3;

struct BenchmarkResult {
    name: String,
    median_ns: u128,
    min_ns: u128,
    max_ns: u128,
    avg_ns: u128,
    iterations: usize,
    target_ns: Option<u128>,
}

impl BenchmarkResult {
    fn new(name: &str, target_ns: Option<u128>) -> Self {
        Self {
            name: name.to_string(),
            median_ns: 0,
            min_ns: u128::MAX,
            max_ns: 0,
            avg_ns: 0,
            iterations: 0,
            target_ns,
        }
    }

    fn add_sample(&mut self, duration_ns: u128) {
        self.min_ns = self.min_ns.min(duration_ns);
        self.max_ns = self.max_ns.max(duration_ns);
        self.iterations += 1;
    }

    fn calculate(&mut self, samples: &[u128]) {
        if samples.is_empty() {
            return;
        }

        let total: u128 = samples.iter().sum();
        self.avg_ns = total / samples.len() as u128;

        let mut sorted = samples.to_vec();
        sorted.sort();
        self.median_ns = sorted[sorted.len() / 2];
    }

    fn status(&self) -> &str {
        if let Some(target) = self.target_ns {
            if self.median_ns <= target {
                "✓"
            } else {
                "✗"
            }
        } else {
            " "
        }
    }

    fn format_ns(ns: u128) -> String {
        if ns < 1000 {
            format!("{:.2} ns", ns)
        } else if ns < 1_000_000 {
            format!("{:.2} µs", ns as f64 / 1000.0)
        } else if ns < 1_000_000_000 {
            format!("{:.2} ms", ns as f64 / 1_000_000.0)
        } else {
            format!("{:.2} s", ns as f64 / 1_000_000_000.0)
        }
    }

    fn display(&self) {
        let target_str = if let Some(target) = self.target_ns {
            format!(" (target: {})", Self::format_ns(target))
        } else {
            String::new()
        };

        println!(
            "{} {} | median: {} | avg: {} | min: {} | max: {} | iterations: {}{}",
            self.status(),
            self.name,
            Self::format_ns(self.median_ns),
            Self::format_ns(self.avg_ns),
            Self::format_ns(self.min_ns),
            Self::format_ns(self.max_ns),
            self.iterations,
            target_str
        );
    }
}

fn bench_with_target<F>(
    name: &str,
    iterations: usize,
    target_ns: u128,
    mut f: F,
) -> BenchmarkResult
where
    F: FnMut() -> Result<(), Box<dyn std::error::Error>>,
{
    let mut result = BenchmarkResult::new(name, Some(target_ns));

    for _ in 0..WARM_UP_ITERATIONS {
        let _ = f();
    }

    let mut samples = Vec::new();
    for _ in 0..iterations {
        let start = Instant::now();
        let _ = f();
        let elapsed = start.elapsed().as_nanos();
        result.add_sample(elapsed);
        samples.push(elapsed);
    }

    result.calculate(&samples);
    result
}

// Load time benchmarks
fn bench_first_module_load() -> BenchmarkResult {
    let target = 50_000_000; // 50ms in nanoseconds

    bench_with_target(
        "bench_first_module_load",
        10,
        target,
        || {
            let mut loader = ModuleLoader::new(PathBuf::from("."));
            let _result = loader.require("fs")?;
            Ok(())
        },
    )
}

fn bench_cached_module_load() -> BenchmarkResult {
    let target = 1_000_000; // 1ms in nanoseconds

    bench_with_target(
        "bench_cached_module_load",
        100,
        target,
        || {
            let mut loader = ModuleLoader::new(PathBuf::from("."));
            loader.require("fs")?;
            loader.require("fs")?;
            Ok(())
        },
    )
}

fn bench_builtin_module_load() -> BenchmarkResult {
    let target = 10_000_000; // 10ms in nanoseconds

    bench_with_target(
        "bench_builtin_module_load",
        50,
        target,
        || {
            let mut loader = ModuleLoader::new(PathBuf::from("."));
            let _path = loader.require("path")?;
            Ok(())
        },
    )
}

fn bench_custom_module_load() -> BenchmarkResult {
    let target = 500_000_000; // 500ms in nanoseconds

    bench_with_target(
        "bench_custom_module_load",
        20,
        target,
        || {
            let temp_dir = TempDir::new()?;
            let temp_path = temp_dir.path();
            let node_modules = temp_path.join("node_modules");
            fs::create_dir_all(&node_modules)?;

            let custom_module_dir = node_modules.join("custom-module");
            fs::create_dir_all(&custom_module_dir)?;

            let test_file = custom_module_dir.join("index.lua");
            fs::write(&test_file, "return { test = 'value' }")?;

            let mut loader = ModuleLoader::new(temp_path.to_path_buf());
            let _result = loader.require("custom-module")?;

            Ok(())
        },
    )
}

// Operation benchmarks
fn bench_require_function_call() -> BenchmarkResult {
    let target = 5_000_000; // 5ms in nanoseconds

    bench_with_target(
        "bench_require_function_call",
        50,
        target,
        || {
            let mut loader = ModuleLoader::new(PathBuf::from("."));
            let _result = loader.require("fs")?;
            Ok(())
        },
    )
}

fn bench_cache_lookup_only() -> BenchmarkResult {
    let target = 100_000; // 100µs in nanoseconds

    bench_with_target(
        "bench_cache_lookup_only",
        1000,
        target,
        || {
            let mut loader = ModuleLoader::new(PathBuf::from("."));
            loader.require("fs")?;
            let _cached = loader.get_cached("fs")?;
            Ok(())
        },
    )
}

fn bench_circular_dep_detection() -> BenchmarkResult {
    let target = 1_000_000; // 1ms in nanoseconds

    bench_with_target(
        "bench_circular_dep_detection",
        100,
        target,
        || {
            let loader = ModuleLoader::new(PathBuf::from("."));

            let detector = loader.detector();
            let check_result = detector.check("module-a");
            if check_result.is_ok() {
                Ok(())
            } else {
                Err("Expected no circular dependency".into())
            }
        },
    )
}

fn bench_module_resolution() -> BenchmarkResult {
    let target = 5_000_000; // 5ms in nanoseconds

    bench_with_target(
        "bench_module_resolution",
        100,
        target,
        || {
            let resolver = ModuleResolver::new(PathBuf::from("."));
            let _path = resolver.resolve("fs")?;
            Ok(())
        },
    )
}

// Memory benchmarks
fn bench_module_cache_memory() -> BenchmarkResult {
    let target = 50_000_000; // 50ms in nanoseconds

    bench_with_target(
        "bench_module_cache_memory",
        5,
        target,
        || {
            let mut loader = ModuleLoader::new(PathBuf::from("."));

            for module in &["fs", "path", "events", "util", "table"] {
                loader.require(module)?;
            }

            let cached = loader.cached_modules()?;
            if cached.len() != 5 {
                return Err("Expected 5 cached modules".into());
            }

            Ok(())
        },
    )
}

fn bench_export_table_conversion() -> BenchmarkResult {
    let target = 10_000_000; // 10ms in nanoseconds

    bench_with_target(
        "bench_export_table_conversion",
        50,
        target,
        || {
            let mut loader = ModuleLoader::new(PathBuf::from("."));
            let _exports = loader.require("fs")?;

            let exports = loader.require("fs")?;
            if !exports.is_object() {
                return Err("Expected object exports".into());
            }

            Ok(())
        },
    )
}

fn bench_module_environment_setup() -> BenchmarkResult {
    let target = 1_000_000; // 1ms in nanoseconds

    bench_with_target(
        "bench_module_environment_setup",
        30,
        target,
        || {
            let loader = ModuleLoader::new(PathBuf::from("."));
            let _registry = loader.registry();
            let _resolver = loader.resolver();
            let _detector = loader.detector();

            Ok(())
        },
    )
}

fn print_header() {
    println!("\n╔════════════════════════════════════════════════════════════════════════╗");
    println!("║           HypeRS Module System Performance Benchmarks                  ║");
    println!("╠════════════════════════════════════════════════════════════════════════╣");
}

fn print_section(title: &str) {
    println!("║ {}{}║", title, " ".repeat(70 - title.len()));
    println!("╟────────────────────────────────────────────────────────────────────────╢");
}

fn print_footer() {
    println!("╚════════════════════════════════════════════════════════════════════════╝\n");
}

fn main() {
    print_header();

    print_section("LOAD TIME BENCHMARKS");
    let r1 = bench_first_module_load();
    r1.display();

    let r2 = bench_cached_module_load();
    r2.display();

    let r3 = bench_builtin_module_load();
    r3.display();

    let r4 = bench_custom_module_load();
    r4.display();

    print_section("OPERATION BENCHMARKS");
    let r5 = bench_require_function_call();
    r5.display();

    let r6 = bench_cache_lookup_only();
    r6.display();

    let r7 = bench_circular_dep_detection();
    r7.display();

    let r8 = bench_module_resolution();
    r8.display();

    print_section("MEMORY BENCHMARKS");
    let r9 = bench_module_cache_memory();
    r9.display();

    let r10 = bench_export_table_conversion();
    r10.display();

    let r11 = bench_module_environment_setup();
    r11.display();

    print_section("PERFORMANCE SUMMARY");

    let all_results = vec![r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11];

    let passed = all_results.iter().filter(|r| r.status() == "✓").count();
    let failed = all_results.iter().filter(|r| r.status() == "✗").count();
    let untested = all_results.iter().filter(|r| r.status() == " ").count();

    println!("║ Passed: {} | Failed: {} | Untested: {} {}║",
             passed,
             failed,
             untested,
             " ".repeat(70 - format!("Passed: {} | Failed: {} | Untested: {} ", passed, failed, untested).len())
    );

    print_footer();

    if failed > 0 {
        std::process::exit(1);
    }
}
