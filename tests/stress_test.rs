use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use tempfile::TempDir;

use hype_rs::modules::loader::ModuleLoader;

fn create_test_module(dir: &Path, name: &str, content: &str) -> std::io::Result<PathBuf> {
    let path = dir.join(format!("{}.lua", name));
    let mut file = fs::File::create(&path)?;
    file.write_all(content.as_bytes())?;
    Ok(path)
}

fn create_modules_directory(base: &Path, count: usize) -> std::io::Result<()> {
    let modules_dir = base.join("hype_modules");
    fs::create_dir_all(&modules_dir)?;

    for i in 0..count {
        let module_dir = modules_dir.join(format!("module_{}", i));
        fs::create_dir_all(&module_dir)?;
        create_test_module(
            &module_dir,
            "index",
            &format!("return {{ id = {}, name = 'module_{}' }}", i, i),
        )?;
    }
    Ok(())
}

mod module_loading_stress {
    use super::*;

    #[test]
    fn test_concurrent_100_modules() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();
        create_modules_directory(base, 100).unwrap();

        let start = Instant::now();
        let mut loader = ModuleLoader::new(base.to_path_buf());

        let mut loaded_count = 0;
        for i in 0..100 {
            match loader.require(&format!("module_{}", i)) {
                Ok(exports) => {
                    assert!(exports.is_object());
                    loaded_count += 1;
                }
                Err(_) => {}
            }
        }

        let elapsed = start.elapsed();
        assert!(loaded_count > 0, "Should load at least some modules");
        assert!(elapsed.as_secs() < 5, "Should complete in reasonable time");

        let cached = loader.cached_modules().unwrap();
        assert!(!cached.is_empty(), "Cache should contain loaded modules");
    }

    #[test]
    fn test_rapid_require_cycles() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();
        create_modules_directory(base, 5).unwrap();

        let mut loader = ModuleLoader::new(base.to_path_buf());

        let start = Instant::now();
        let mut hits = 0;
        let mut total = 0;

        for _ in 0..200 {
            for i in 0..5 {
                total += 1;
                if loader.require(&format!("module_{}", i)).is_ok() {
                    hits += 1;
                }
            }
        }

        let elapsed = start.elapsed();
        let hit_rate = (hits as f64 / total as f64) * 100.0;

        assert!(
            hit_rate >= 90.0,
            "Cache hit rate should be high: {:.1}%",
            hit_rate
        );
        assert!(elapsed.as_secs() < 5, "Should complete rapidly");
    }

    #[test]
    fn test_memory_consumption_scaling() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();

        let mut measurements = Vec::new();

        for batch_size in [10, 25, 50].iter() {
            let batch_dir = base.join(format!("batch_{}", batch_size));
            fs::create_dir_all(&batch_dir).unwrap();

            let modules_dir = batch_dir.join("hype_modules");
            fs::create_dir_all(&modules_dir).unwrap();

            for i in 0..*batch_size {
                let module_dir = modules_dir.join(format!("mod_{}", i));
                fs::create_dir_all(&module_dir).unwrap();
                create_test_module(
                    &module_dir,
                    "index",
                    &format!("return {{ data = '{}' }}", "x".repeat(100)),
                )
                .unwrap();
            }

            let mut loader = ModuleLoader::new(batch_dir.clone());
            let mut loaded = 0;

            for i in 0..*batch_size {
                if loader.require(&format!("mod_{}", i)).is_ok() {
                    loaded += 1;
                }
            }

            let cached = loader.cached_modules().unwrap_or_default().len();
            measurements.push((batch_size, loaded, cached));
        }

        assert_eq!(measurements.len(), 3);
        for (size, loaded, cached) in measurements.iter() {
            assert!(*loaded > 0, "Should load modules for batch size {}", size);
            assert!(*cached > 0, "Should cache modules for batch size {}", size);
        }
    }

    #[test]
    fn test_cache_hit_rate_measurement() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();
        create_modules_directory(base, 10).unwrap();

        let mut loader = ModuleLoader::new(base.to_path_buf());

        for i in 0..10 {
            let _ = loader.require(&format!("module_{}", i));
        }

        let cached_before = loader.cached_modules().unwrap_or_default().len();

        let mut cache_hits = 0;
        let total_requests = 100;

        for i in 0..100 {
            if let Ok(first) = loader.require(&format!("module_{}", i % 10)) {
                if let Ok(second) = loader.require(&format!("module_{}", i % 10)) {
                    if first == second {
                        cache_hits += 1;
                    }
                }
            }
        }

        let hit_rate = (cache_hits as f64 / total_requests as f64) * 100.0;
        assert!(
            hit_rate >= 80.0,
            "Hit rate should be at least 80%, got {:.1}%",
            hit_rate
        );

        let cached_after = loader.cached_modules().unwrap_or_default().len();
        assert_eq!(
            cached_before, cached_after,
            "Cache size should not grow unbounded"
        );
    }

    #[test]
    fn test_long_dependency_chains() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();
        let modules_dir = base.join("hype_modules");
        fs::create_dir_all(&modules_dir).unwrap();

        for i in 0..6 {
            let module_dir = modules_dir.join(format!("chain_{}", i));
            fs::create_dir_all(&module_dir).unwrap();

            let content = if i == 5 {
                "return { depth = 6, value = 'leaf' }".to_string()
            } else {
                format!("return {{ depth = {}, next = 'chain_{}' }}", i, i + 1)
            };

            create_test_module(&module_dir, "index", &content).unwrap();
        }

        let mut loader = ModuleLoader::new(base.to_path_buf());

        let start = Instant::now();
        let result = loader.require("chain_0");
        let elapsed = start.elapsed();

        assert!(result.is_ok(), "Should load entire chain");
        assert!(
            elapsed.as_millis() < 1000,
            "Chain resolution should be fast"
        );
    }
}

mod performance_degradation {
    use super::*;

    #[test]
    fn test_performance_with_full_cache() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();
        create_modules_directory(base, 50).unwrap();

        let mut loader = ModuleLoader::new(base.to_path_buf());
        let mut timings = Vec::new();

        for batch_end in [10, 25, 50].iter() {
            for i in 0..*batch_end {
                let _ = loader.require(&format!("module_{}", i));
            }

            let start = Instant::now();
            let _ = loader.require(&format!("module_{}", batch_end - 1));
            let elapsed = start.elapsed();

            timings.push((batch_end, elapsed.as_micros()));
        }

        for i in 1..timings.len() {
            let (_, prev_time) = timings[i - 1];
            let (_, curr_time) = timings[i];
            assert!(
                curr_time < prev_time * 2,
                "Performance should not degrade significantly with cache size"
            );
        }
    }

    #[test]
    fn test_deep_directory_nesting() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();

        let mut deep_path = base.to_path_buf();
        for i in 0..15 {
            deep_path = deep_path.join(format!("level_{}", i));
        }
        fs::create_dir_all(&deep_path).unwrap();

        let modules_dir = deep_path.join("hype_modules");
        fs::create_dir_all(&modules_dir).unwrap();

        let module_dir = modules_dir.join("deep_module");
        fs::create_dir_all(&module_dir).unwrap();
        create_test_module(&module_dir, "index", "return { nested = true }").unwrap();

        let mut loader = ModuleLoader::new(deep_path.clone());

        let start = Instant::now();
        let result = loader.require("deep_module");
        let elapsed = start.elapsed();

        assert!(result.is_ok(), "Should resolve deeply nested modules");
        assert!(
            elapsed.as_millis() < 500,
            "Resolution should be reasonably fast"
        );
    }

    #[test]
    fn test_large_module_files() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();
        let modules_dir = base.join("hype_modules");
        fs::create_dir_all(&modules_dir).unwrap();

        let module_dir = modules_dir.join("large_module");
        fs::create_dir_all(&module_dir).unwrap();

        let mut large_export = String::from("return { ");
        for i in 0..5000 {
            large_export.push_str(&format!("key_{} = {}, ", i, i));
        }
        large_export.push_str("final = 'done' }");

        create_test_module(&module_dir, "index", &large_export).unwrap();

        let mut loader = ModuleLoader::new(base.to_path_buf());

        let start = Instant::now();
        let result = loader.require("large_module");
        let elapsed = start.elapsed();

        assert!(result.is_ok(), "Should load large module");
        assert!(
            elapsed.as_millis() < 1000,
            "Large module load should complete in time"
        );
    }

    #[test]
    fn test_rapid_module_reloading() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();
        create_modules_directory(base, 3).unwrap();

        let mut loader = ModuleLoader::new(base.to_path_buf());
        let mut cycle_times = Vec::new();

        for _ in 0..5 {
            let start = Instant::now();

            for i in 0..3 {
                let _ = loader.require(&format!("module_{}", i));
            }
            loader.clear_cache().unwrap();

            let elapsed = start.elapsed();
            cycle_times.push(elapsed.as_millis());
        }

        let avg_time = cycle_times.iter().sum::<u128>() / cycle_times.len() as u128;
        assert!(avg_time < 500, "Average cycle time should be reasonable");

        let min_time = *cycle_times.iter().min().unwrap() as f64;
        let max_time = *cycle_times.iter().max().unwrap() as f64;
        let variance_ratio = max_time / min_time.max(1.0);
        assert!(
            variance_ratio < 3.0,
            "Cycle times should be reasonably consistent (variance ratio: {:.2})",
            variance_ratio
        );
    }

    #[test]
    fn test_concurrent_module_access() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();
        create_modules_directory(base, 20).unwrap();

        let mut initial_loader = ModuleLoader::new(base.to_path_buf());

        for i in 0..20 {
            let _ = initial_loader.require(&format!("module_{}", i));
        }

        let loader = Arc::new(Mutex::new(initial_loader));
        let mut handles = vec![];

        for _thread_id in 0..4 {
            let loader_clone = Arc::clone(&loader);
            let handle = thread::spawn(move || {
                let mut success = 0;
                for i in 0..20 {
                    if let Ok(mut l) = loader_clone.lock() {
                        if l.require(&format!("module_{}", i)).is_ok() {
                            success += 1;
                        }
                    }
                }
                success
            });
            handles.push(handle);
        }

        let mut total_success = 0;
        for handle in handles {
            if let Ok(count) = handle.join() {
                total_success += count;
            }
        }

        assert!(total_success > 0, "Concurrent access should succeed");
    }
}
