use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::cli::args::{create_standard_arg_table, ParsedArguments};
use crate::engine::output::{OutputCapture, OutputFormat};
use crate::engine::stats::{ExecutionStats, StatsCollector};
use crate::engine::timeout::{TimeoutError, TimeoutManager};
use crate::error::{HypeError, Result};
use crate::file_io::read_lua_script;
use crate::lua::{
    create_cli_config, create_cli_security_policy, setup_require_fn, LuaStateConfig,
    LuaStateManager, SecurityPolicy,
};
use crate::modules::loader::ModuleLoader;

#[derive(Debug, Clone)]
pub struct ExecutionConfig {
    pub script_path: PathBuf,
    pub script_args: Vec<String>,
    pub parsed_args: Option<ParsedArguments>,
    pub verbose: bool,
    pub debug: bool,
    pub timeout: Option<Duration>,
    pub capture_output: bool,
    pub output_format: OutputFormat,
    pub enable_stats: bool,
    pub memory_limit: Option<usize>,
    pub instruction_limit: Option<u32>,
    pub allow_file_operations: bool,
    pub allow_os_operations: bool,
    pub allow_debug_operations: bool,
    pub allow_package_loading: bool,
}

impl Default for ExecutionConfig {
    fn default() -> Self {
        Self {
            script_path: PathBuf::new(),
            script_args: Vec::new(),
            parsed_args: None,
            verbose: false,
            debug: false,
            timeout: Some(Duration::from_secs(30)),
            capture_output: true,
            output_format: OutputFormat::Text,
            enable_stats: false,
            memory_limit: Some(64 * 1024 * 1024), // 64MB
            instruction_limit: Some(1_000_000),   // 1M instructions
            allow_file_operations: false,
            allow_os_operations: false,
            allow_debug_operations: false,
            allow_package_loading: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub success: bool,
    pub exit_code: i32,
    pub output: String,
    pub error_output: String,
    pub stats: Option<ExecutionStats>,
    pub execution_time: Duration,
    pub error: Option<String>,
}

impl Default for ExecutionResult {
    fn default() -> Self {
        Self {
            success: false,
            exit_code: 1,
            output: String::new(),
            error_output: String::new(),
            stats: None,
            execution_time: Duration::ZERO,
            error: None,
        }
    }
}

pub struct ExecutionEngine {
    config: ExecutionConfig,
    output_capture: Arc<Mutex<OutputCapture>>,
    timeout_manager: TimeoutManager,
    stats_collector: StatsCollector,
}

impl ExecutionEngine {
    pub fn new(config: ExecutionConfig) -> Result<Self> {
        let output_capture = Arc::new(Mutex::new(OutputCapture::new(config.capture_output)));
        let timeout_manager = TimeoutManager::new(config.timeout);
        let stats_collector = StatsCollector::new(config.enable_stats);

        Ok(Self {
            config,
            output_capture,
            timeout_manager,
            stats_collector,
        })
    }

    pub fn execute(&mut self) -> Result<ExecutionResult> {
        let start_time = Instant::now();
        let mut result = ExecutionResult::default();

        // Validate script path
        if !self.config.script_path.exists() {
            return Err(HypeError::File(crate::error::FileError::NotFound(
                self.config.script_path.clone(),
            )));
        }

        // Set up timeout handling
        let timeout_handle = if let Some(timeout) = self.config.timeout {
            Some(self.timeout_manager.start_timeout(timeout)?)
        } else {
            None
        };

        // Initialize statistics collection
        if self.config.enable_stats {
            self.stats_collector.start_collection();
        }

        // Execute the script
        let execution_result = self.execute_script_internal();

        // Stop timeout
        if let Some(handle) = timeout_handle {
            self.timeout_manager.stop_timeout(handle);
        }

        // Collect final statistics
        if self.config.enable_stats {
            result.stats = Some(self.stats_collector.finalize_collection()?);
        }

        // Calculate total execution time
        result.execution_time = start_time.elapsed();

        // Process execution result
        match execution_result {
            Ok(_) => {
                result.success = true;
                result.exit_code = 0;
            }
            Err(e) => {
                result.success = false;
                result.exit_code = 1;
                result.error = Some(e.to_string());

                // Check if it's a timeout error
                if matches!(e, HypeError::Execution(ref msg) if msg.contains("timeout")) {
                    result.error_output =
                        format!("Script execution timed out after {:?}", self.config.timeout);
                } else {
                    result.error_output = e.to_string();
                }
            }
        }

        // Capture output
        {
            let output = self.output_capture.lock().unwrap();
            result.output = output.get_stdout();
            result.error_output = format!("{}{}", result.error_output, output.get_stderr());
        }

        Ok(result)
    }

    fn execute_script_internal(&self) -> Result<()> {
        // Create Lua state configuration
        let mut lua_config =
            create_cli_config(self.config.verbose, self.config.debug, self.config.timeout);

        // Apply custom limits and permissions
        if let Some(memory_limit) = self.config.memory_limit {
            lua_config.memory_limit = Some(memory_limit);
        }
        if let Some(instruction_limit) = self.config.instruction_limit {
            lua_config.instruction_limit = Some(instruction_limit);
        }
        lua_config.allow_file_operations = self.config.allow_file_operations;
        lua_config.allow_os_operations = self.config.allow_os_operations;
        lua_config.allow_debug_operations = self.config.allow_debug_operations;
        lua_config.allow_package_loading = self.config.allow_package_loading;

        // Create security policy
        let security_policy = create_cli_security_policy(self.config.debug);

        // Create state manager
        let state_manager = LuaStateManager::new(lua_config)?;

        // Set up module system (require function)
        let lua = state_manager.lua.lock().unwrap();
        let cwd = std::env::current_dir().map_err(|e| HypeError::Io(e))?;
        let loader = Arc::new(Mutex::new(ModuleLoader::new(cwd)));
        setup_require_fn(&lua, loader)
            .map_err(|e| HypeError::Lua(format!("Failed to setup module system: {}", e)))?;
        drop(lua);

        // Set up output capture
        self.setup_output_capture(&state_manager)?;

        // Set up script arguments and environment
        self.setup_script_environment(&state_manager)?;

        // Load and execute the script
        let script_content = read_lua_script(&self.config.script_path)?;

        if self.config.debug {
            eprintln!("Script content length: {} bytes", script_content.len());
            eprintln!("Script path: {}", self.config.script_path.display());
        }

        if self.config.verbose {
            eprintln!(
                "Executing Lua script: {}",
                self.config.script_path.display()
            );
        }

        // Execute the script
        state_manager.execute_script(&self.config.script_path, &self.config.script_args)?;

        if self.config.verbose {
            let metrics = state_manager.get_metrics();
            eprintln!("Script executed successfully");
            eprintln!("Execution time: {:?}", metrics.execution_time);
            eprintln!("Instructions executed: {}", metrics.instructions_executed);
            eprintln!("Memory usage: {} bytes", metrics.memory_usage);
        }

        Ok(())
    }

    fn setup_output_capture(&self, state_manager: &LuaStateManager) -> Result<()> {
        if !self.config.capture_output {
            return Ok(());
        }

        let lua = state_manager.lua.lock().unwrap();
        let globals = lua.globals();

        // Override print function to capture output
        let output_capture = self.output_capture.clone();
        let print_override = lua.create_function(move |_, values: mlua::MultiValue| {
            let mut output = String::new();
            for (i, value) in values.into_iter().enumerate() {
                if i > 0 {
                    output.push('\t');
                }
                output.push_str(&crate::lua::error::format_lua_value(&value));
            }
            output.push('\n');

            if let Ok(mut capture) = output_capture.lock() {
                capture.capture_stdout(&output);
            }

            Ok(())
        })?;

        globals.set("print", print_override)?;

        // Override io.write and io.stdout for more comprehensive capture
        let output_capture_stdout = self.output_capture.clone();
        let io_write_override = lua.create_function(move |_, (text,): (String,)| {
            if let Ok(mut capture) = output_capture_stdout.lock() {
                capture.capture_stdout(&text);
            }
            Ok(())
        })?;

        let io_table = lua.create_table()?;
        io_table.set("write", io_write_override)?;

        // Set up stdout capture
        let stdout_table = lua.create_table()?;
        let output_capture_stdout_write = self.output_capture.clone();
        let stdout_write = lua.create_function(move |_, (text,): (String,)| {
            if let Ok(mut capture) = output_capture_stdout_write.lock() {
                capture.capture_stdout(&text);
            }
            Ok(())
        })?;
        stdout_table.set("write", stdout_write)?;
        io_table.set("stdout", stdout_table)?;

        globals.set("io", io_table)?;

        Ok(())
    }

    fn setup_script_environment(&self, state_manager: &LuaStateManager) -> Result<()> {
        let lua = state_manager.lua.lock().unwrap();
        let globals = lua.globals();

        // Set up comprehensive script arguments using the new argument parser
        if let Some(ref parsed_args) = self.config.parsed_args {
            create_standard_arg_table(&lua, parsed_args)
                .map_err(|e| HypeError::Lua(format!("Failed to create arg table: {}", e)))?;
        } else {
            // Fallback to simple args table for backward compatibility
            let args_table = lua.create_table()?;
            for (i, arg) in self.config.script_args.iter().enumerate() {
                args_table.set(i + 1, arg.clone())?;
            }
            globals.set("args", args_table)?;
        }

        // Set up execution flags
        globals.set("verbose", self.config.verbose)?;
        globals.set("debug", self.config.debug)?;

        // Set up script path information
        globals.set("SCRIPT_PATH", self.config.script_path.to_string_lossy())?;
        if let Some(parent) = self.config.script_path.parent() {
            globals.set("SCRIPT_DIR", parent.to_string_lossy())?;
        }
        if let Some(stem) = self.config.script_path.file_stem() {
            globals.set("SCRIPT_NAME", stem.to_string_lossy())?;
        }

        // Set up hype-specific globals
        let hype_table = lua.create_table()?;
        hype_table.set("version", env!("CARGO_PKG_VERSION"))?;
        hype_table.set("capture_output", self.config.capture_output)?;
        hype_table.set("enable_stats", self.config.enable_stats)?;

        if let Some(timeout) = self.config.timeout {
            hype_table.set("timeout_seconds", timeout.as_secs())?;
        }

        // Add argument parsing information to hype table
        if let Some(ref parsed_args) = self.config.parsed_args {
            let arg_info = lua.create_table()?;
            arg_info.set("indexed_count", parsed_args.indexed_args.len())?;
            arg_info.set("named_count", parsed_args.named_args.len())?;
            arg_info.set("flags_count", parsed_args.flags.len())?;
            arg_info.set("total_count", parsed_args.raw_args.len())?;
            hype_table.set("args", arg_info)?;
        }

        globals.set("hype", hype_table)?;

        Ok(())
    }

    pub fn get_config(&self) -> &ExecutionConfig {
        &self.config
    }

    pub fn update_config(&mut self, config: ExecutionConfig) {
        self.config = config;
    }

    pub fn interrupt(&self) -> Result<()> {
        // Handle script interruption gracefully
        self.timeout_manager.interrupt()?;
        Ok(())
    }

    pub fn force_stop(&self) -> Result<()> {
        // Force stop the execution
        self.timeout_manager.force_stop()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_execution_engine_creation() {
        let config = ExecutionConfig::default();
        let engine = ExecutionEngine::new(config);
        assert!(engine.is_ok());
    }

    #[test]
    fn test_simple_script_execution() -> Result<()> {
        let dir = tempdir()?;
        let script_path = dir.path().join("test.lua");
        fs::write(&script_path, "print(\"Hello, World!\")")?;

        let mut config = ExecutionConfig::default();
        config.script_path = script_path.clone();
        config.verbose = false;
        config.capture_output = true;

        let mut engine = ExecutionEngine::new(config)?;
        let result = engine.execute()?;

        assert!(result.success);
        assert_eq!(result.exit_code, 0);
        assert!(result.output.contains("Hello, World!"));
        assert!(result.error_output.is_empty());

        Ok(())
    }

    #[test]
    fn test_script_with_arguments() -> Result<()> {
        let dir = tempdir()?;
        let script_path = dir.path().join("args_test.lua");
        fs::write(
            &script_path,
            r#"
            for i, arg in ipairs(args) do
                print("arg" .. i .. ": " .. arg)
            end
        "#,
        )?;

        let mut config = ExecutionConfig::default();
        config.script_path = script_path.clone();
        config.script_args = vec!["hello".to_string(), "world".to_string()];
        config.capture_output = true;

        let mut engine = ExecutionEngine::new(config)?;
        let result = engine.execute()?;

        assert!(result.success);
        assert!(result.output.contains("arg1: hello"));
        assert!(result.output.contains("arg2: world"));

        Ok(())
    }

    #[test]
    fn test_script_error_handling() -> Result<()> {
        let dir = tempdir()?;
        let script_path = dir.path().join("error_test.lua");
        fs::write(&script_path, "error(\"This is a test error\")")?;

        let mut config = ExecutionConfig::default();
        config.script_path = script_path.clone();
        config.capture_output = true;

        let mut engine = ExecutionEngine::new(config)?;
        let result = engine.execute()?;

        assert!(!result.success);
        assert_eq!(result.exit_code, 1);
        assert!(!result.error_output.is_empty());

        Ok(())
    }
}
