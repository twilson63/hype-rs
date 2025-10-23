use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::error::{HypeError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionStats {
    pub script_name: String,
    pub script_path: String,
    pub start_time_secs: u64,
    pub end_time_secs: Option<u64>,
    pub execution_time: Duration,
    pub cpu_time: Duration,
    pub memory_usage: MemoryStats,
    pub lua_stats: LuaStats,
    pub io_stats: IoStats,
    pub error_stats: ErrorStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    pub initial_memory: usize,
    pub peak_memory: usize,
    pub final_memory: usize,
    pub memory_allocated: usize,
    pub memory_freed: usize,
    pub gc_collections: u32,
    pub gc_time: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LuaStats {
    pub instructions_executed: u32,
    pub function_calls: u32,
    pub table_operations: u32,
    pub string_operations: u32,
    pub upvalues_accessed: u32,
    pub lines_executed: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoStats {
    pub bytes_read: usize,
    pub bytes_written: usize,
    pub files_opened: u32,
    pub files_closed: u32,
    pub stdout_bytes: usize,
    pub stderr_bytes: usize,
    pub print_calls: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorStats {
    pub syntax_errors: u32,
    pub runtime_errors: u32,
    pub memory_errors: u32,
    pub timeout_errors: u32,
    pub security_violations: u32,
    pub total_errors: u32,
}

impl Default for ExecutionStats {
    fn default() -> Self {
        Self {
            script_name: String::new(),
            script_path: String::new(),
            start_time_secs: 0,
            end_time_secs: None,
            execution_time: Duration::ZERO,
            cpu_time: Duration::ZERO,
            memory_usage: MemoryStats::default(),
            lua_stats: LuaStats::default(),
            io_stats: IoStats::default(),
            error_stats: ErrorStats::default(),
        }
    }
}

impl Default for MemoryStats {
    fn default() -> Self {
        Self {
            initial_memory: 0,
            peak_memory: 0,
            final_memory: 0,
            memory_allocated: 0,
            memory_freed: 0,
            gc_collections: 0,
            gc_time: Duration::ZERO,
        }
    }
}

impl Default for LuaStats {
    fn default() -> Self {
        Self {
            instructions_executed: 0,
            function_calls: 0,
            table_operations: 0,
            string_operations: 0,
            upvalues_accessed: 0,
            lines_executed: 0,
        }
    }
}

impl Default for IoStats {
    fn default() -> Self {
        Self {
            bytes_read: 0,
            bytes_written: 0,
            files_opened: 0,
            files_closed: 0,
            stdout_bytes: 0,
            stderr_bytes: 0,
            print_calls: 0,
        }
    }
}

impl Default for ErrorStats {
    fn default() -> Self {
        Self {
            syntax_errors: 0,
            runtime_errors: 0,
            memory_errors: 0,
            timeout_errors: 0,
            security_violations: 0,
            total_errors: 0,
        }
    }
}

pub struct StatsCollector {
    enabled: bool,
    stats: Arc<Mutex<ExecutionStats>>,
    collection_start: Option<Instant>,
}

impl StatsCollector {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            stats: Arc::new(Mutex::new(ExecutionStats::default())),
            collection_start: None,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn start_collection(&mut self) {
        if !self.enabled {
            return;
        }

        self.collection_start = Some(Instant::now());

        let mut stats = self.stats.lock().unwrap();
        stats.start_time_secs = Instant::now()
            .duration_since(Instant::now() - Duration::from_secs(0))
            .as_secs();
        stats.end_time_secs = None;
        stats.execution_time = Duration::ZERO;

        // Reset all counters
        stats.memory_usage = MemoryStats::default();
        stats.lua_stats = LuaStats::default();
        stats.io_stats = IoStats::default();
        stats.error_stats = ErrorStats::default();
    }

    pub fn finalize_collection(&self) -> Result<ExecutionStats> {
        if !self.enabled {
            return Err(HypeError::Execution(
                "Statistics collection is not enabled".to_string(),
            ));
        }

        let mut stats = self.stats.lock().unwrap();
        stats.end_time_secs = Some(
            Instant::now()
                .duration_since(Instant::now() - Duration::from_secs(0))
                .as_secs(),
        );

        if let Some(start) = self.collection_start {
            stats.execution_time = start.elapsed();
        }

        Ok(stats.clone())
    }

    pub fn set_script_info(&self, name: &str, path: &str) {
        if !self.enabled {
            return;
        }

        let mut stats = self.stats.lock().unwrap();
        stats.script_name = name.to_string();
        stats.script_path = path.to_string();
    }

    pub fn record_instruction(&self) {
        if !self.enabled {
            return;
        }

        let mut stats = self.stats.lock().unwrap();
        stats.lua_stats.instructions_executed += 1;
    }

    pub fn record_function_call(&self) {
        if !self.enabled {
            return;
        }

        let mut stats = self.stats.lock().unwrap();
        stats.lua_stats.function_calls += 1;
    }

    pub fn record_table_operation(&self) {
        if !self.enabled {
            return;
        }

        let mut stats = self.stats.lock().unwrap();
        stats.lua_stats.table_operations += 1;
    }

    pub fn record_string_operation(&self) {
        if !self.enabled {
            return;
        }

        let mut stats = self.stats.lock().unwrap();
        stats.lua_stats.string_operations += 1;
    }

    pub fn record_line_executed(&self) {
        if !self.enabled {
            return;
        }

        let mut stats = self.stats.lock().unwrap();
        stats.lua_stats.lines_executed += 1;
    }

    pub fn record_memory_usage(&self, current: usize, peak: usize) {
        if !self.enabled {
            return;
        }

        let mut stats = self.stats.lock().unwrap();
        stats.memory_usage.final_memory = current;
        stats.memory_usage.peak_memory = peak;

        if stats.memory_usage.initial_memory == 0 {
            stats.memory_usage.initial_memory = current;
        }

        if current > stats.memory_usage.initial_memory {
            stats.memory_usage.memory_allocated += current - stats.memory_usage.initial_memory;
        }
    }

    pub fn record_gc_collection(&self, count: u32, gc_time: Duration) {
        if !self.enabled {
            return;
        }

        let mut stats = self.stats.lock().unwrap();
        stats.memory_usage.gc_collections += count;
        stats.memory_usage.gc_time += gc_time;
    }

    pub fn record_io_read(&self, bytes: usize) {
        if !self.enabled {
            return;
        }

        let mut stats = self.stats.lock().unwrap();
        stats.io_stats.bytes_read += bytes;
    }

    pub fn record_io_write(&self, bytes: usize) {
        if !self.enabled {
            return;
        }

        let mut stats = self.stats.lock().unwrap();
        stats.io_stats.bytes_written += bytes;
    }

    pub fn record_file_opened(&self) {
        if !self.enabled {
            return;
        }

        let mut stats = self.stats.lock().unwrap();
        stats.io_stats.files_opened += 1;
    }

    pub fn record_file_closed(&self) {
        if !self.enabled {
            return;
        }

        let mut stats = self.stats.lock().unwrap();
        stats.io_stats.files_closed += 1;
    }

    pub fn record_stdout(&self, bytes: usize) {
        if !self.enabled {
            return;
        }

        let mut stats = self.stats.lock().unwrap();
        stats.io_stats.stdout_bytes += bytes;
    }

    pub fn record_stderr(&self, bytes: usize) {
        if !self.enabled {
            return;
        }

        let mut stats = self.stats.lock().unwrap();
        stats.io_stats.stderr_bytes += bytes;
    }

    pub fn record_print_call(&self) {
        if !self.enabled {
            return;
        }

        let mut stats = self.stats.lock().unwrap();
        stats.io_stats.print_calls += 1;
    }

    pub fn record_syntax_error(&self) {
        if !self.enabled {
            return;
        }

        let mut stats = self.stats.lock().unwrap();
        stats.error_stats.syntax_errors += 1;
        stats.error_stats.total_errors += 1;
    }

    pub fn record_runtime_error(&self) {
        if !self.enabled {
            return;
        }

        let mut stats = self.stats.lock().unwrap();
        stats.error_stats.runtime_errors += 1;
        stats.error_stats.total_errors += 1;
    }

    pub fn record_memory_error(&self) {
        if !self.enabled {
            return;
        }

        let mut stats = self.stats.lock().unwrap();
        stats.error_stats.memory_errors += 1;
        stats.error_stats.total_errors += 1;
    }

    pub fn record_timeout_error(&self) {
        if !self.enabled {
            return;
        }

        let mut stats = self.stats.lock().unwrap();
        stats.error_stats.timeout_errors += 1;
        stats.error_stats.total_errors += 1;
    }

    pub fn record_security_violation(&self) {
        if !self.enabled {
            return;
        }

        let mut stats = self.stats.lock().unwrap();
        stats.error_stats.security_violations += 1;
        stats.error_stats.total_errors += 1;
    }

    pub fn get_current_stats(&self) -> ExecutionStats {
        self.stats.lock().unwrap().clone()
    }

    pub fn format_stats(&self, stats: &ExecutionStats) -> String {
        format!(
            r#"Execution Statistics:
  Script: {} ({})
  Execution Time: {:?}
  CPU Time: {:?}

Memory Usage:
  Initial: {} bytes
  Peak: {} bytes
  Final: {} bytes
  Allocated: {} bytes
  GC Collections: {}
  GC Time: {:?}

Lua Statistics:
  Instructions: {}
  Function Calls: {}
  Table Operations: {}
  String Operations: {}
  Lines Executed: {}

I/O Statistics:
  Bytes Read: {}
  Bytes Written: {}
  Files Opened: {}
  Files Closed: {}
  Stdout Bytes: {}
  Stderr Bytes: {}
  Print Calls: {}

Error Statistics:
  Syntax Errors: {}
  Runtime Errors: {}
  Memory Errors: {}
  Timeout Errors: {}
  Security Violations: {}
  Total Errors: {}
"#,
            stats.script_name,
            stats.script_path,
            stats.execution_time,
            stats.cpu_time,
            stats.memory_usage.initial_memory,
            stats.memory_usage.peak_memory,
            stats.memory_usage.final_memory,
            stats.memory_usage.memory_allocated,
            stats.memory_usage.gc_collections,
            stats.memory_usage.gc_time,
            stats.lua_stats.instructions_executed,
            stats.lua_stats.function_calls,
            stats.lua_stats.table_operations,
            stats.lua_stats.string_operations,
            stats.lua_stats.lines_executed,
            stats.io_stats.bytes_read,
            stats.io_stats.bytes_written,
            stats.io_stats.files_opened,
            stats.io_stats.files_closed,
            stats.io_stats.stdout_bytes,
            stats.io_stats.stderr_bytes,
            stats.io_stats.print_calls,
            stats.error_stats.syntax_errors,
            stats.error_stats.runtime_errors,
            stats.error_stats.memory_errors,
            stats.error_stats.timeout_errors,
            stats.error_stats.security_violations,
            stats.error_stats.total_errors
        )
    }

    pub fn export_json(&self, stats: &ExecutionStats) -> Result<String> {
        serde_json::to_string_pretty(stats)
            .map_err(|e| HypeError::Execution(format!("Failed to serialize stats to JSON: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_stats_collector_creation() {
        let collector = StatsCollector::new(true);
        assert!(collector.is_enabled());

        let collector = StatsCollector::new(false);
        assert!(!collector.is_enabled());
    }

    #[test]
    fn test_stats_collection() -> Result<()> {
        let mut collector = StatsCollector::new(true);

        collector.start_collection();
        collector.set_script_info("test.lua", "/path/to/test.lua");

        // Record some statistics
        collector.record_instruction();
        collector.record_instruction();
        collector.record_function_call();
        collector.record_table_operation();
        collector.record_string_operation();
        collector.record_line_executed();

        collector.record_io_read(100);
        collector.record_io_write(50);
        collector.record_stdout(25);
        collector.record_stderr(10);
        collector.record_print_call();

        // Wait a bit for execution time
        thread::sleep(Duration::from_millis(10));

        let stats = collector.finalize_collection()?;

        assert_eq!(stats.script_name, "test.lua");
        assert_eq!(stats.script_path, "/path/to/test.lua");
        assert!(stats.execution_time > Duration::ZERO);
        assert_eq!(stats.lua_stats.instructions_executed, 2);
        assert_eq!(stats.lua_stats.function_calls, 1);
        assert_eq!(stats.lua_stats.table_operations, 1);
        assert_eq!(stats.lua_stats.string_operations, 1);
        assert_eq!(stats.lua_stats.lines_executed, 1);
        assert_eq!(stats.io_stats.bytes_read, 100);
        assert_eq!(stats.io_stats.bytes_written, 50);
        assert_eq!(stats.io_stats.stdout_bytes, 25);
        assert_eq!(stats.io_stats.stderr_bytes, 10);
        assert_eq!(stats.io_stats.print_calls, 1);

        Ok(())
    }

    #[test]
    fn test_disabled_stats_collection() {
        let mut collector = StatsCollector::new(false);

        collector.start_collection();
        collector.record_instruction();
        collector.record_function_call();

        let stats = collector.get_current_stats();
        assert_eq!(stats.lua_stats.instructions_executed, 0);
        assert_eq!(stats.lua_stats.function_calls, 0);
    }

    #[test]
    fn test_error_recording() {
        let mut collector = StatsCollector::new(true);

        collector.start_collection();

        collector.record_syntax_error();
        collector.record_runtime_error();
        collector.record_memory_error();
        collector.record_timeout_error();
        collector.record_security_violation();

        let stats = collector.get_current_stats();
        assert_eq!(stats.error_stats.syntax_errors, 1);
        assert_eq!(stats.error_stats.runtime_errors, 1);
        assert_eq!(stats.error_stats.memory_errors, 1);
        assert_eq!(stats.error_stats.timeout_errors, 1);
        assert_eq!(stats.error_stats.security_violations, 1);
        assert_eq!(stats.error_stats.total_errors, 5);
    }

    #[test]
    fn test_stats_formatting() -> Result<()> {
        let mut collector = StatsCollector::new(true);

        collector.start_collection();
        collector.set_script_info("test.lua", "/path/to/test.lua");
        collector.record_instruction();

        let stats = collector.finalize_collection()?;
        let formatted = collector.format_stats(&stats);

        assert!(formatted.contains("test.lua"));
        assert!(formatted.contains("Instructions: 1"));
        assert!(formatted.contains("Execution Statistics"));

        Ok(())
    }

    #[test]
    fn test_json_export() -> Result<()> {
        let mut collector = StatsCollector::new(true);

        collector.start_collection();
        collector.set_script_info("test.lua", "/path/to/test.lua");

        let stats = collector.finalize_collection()?;
        let json = collector.export_json(&stats)?;

        assert!(json.contains("test.lua"));
        assert!(json.contains("script_name"));
        assert!(json.contains("execution_time"));

        // Verify it's valid JSON
        let _parsed: ExecutionStats = serde_json::from_str(&json).map_err(|e| {
            crate::error::HypeError::Execution(format!("JSON parsing error: {}", e))
        })?;

        Ok(())
    }
}
