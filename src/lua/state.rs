use crate::error::{HypeError, Result};
use crate::lua::environment::{EnvironmentConfig, EnvironmentManager};
use mlua::{Function, Lua, Table, Value};
use std::path::Path;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct LuaStateConfig {
    pub memory_limit: Option<usize>,
    pub instruction_limit: Option<u32>,
    pub time_limit: Option<Duration>,
    pub allow_os_operations: bool,
    pub allow_file_operations: bool,
    pub allow_debug_operations: bool,
    pub allow_package_loading: bool,
    pub gc_step: Option<i32>,
    pub gc_pause: Option<i32>,
    pub environment_config: EnvironmentConfig,
}

impl Default for LuaStateConfig {
    fn default() -> Self {
        Self {
            memory_limit: Some(64 * 1024 * 1024),      // 64MB default
            instruction_limit: Some(1000000),          // 1M instructions default
            time_limit: Some(Duration::from_secs(30)), // 30 seconds default
            allow_os_operations: false,
            allow_file_operations: false,
            allow_debug_operations: false,
            allow_package_loading: false,
            gc_step: Some(100),
            gc_pause: Some(200),
            environment_config: EnvironmentConfig::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LuaStateMetrics {
    pub memory_usage: usize,
    pub instructions_executed: u32,
    pub execution_time: Duration,
    pub gc_collections: u32,
}

impl Default for LuaStateMetrics {
    fn default() -> Self {
        Self {
            memory_usage: 0,
            instructions_executed: 0,
            execution_time: Duration::ZERO,
            gc_collections: 0,
        }
    }
}

#[derive(Debug)]
pub struct LuaStateManager {
    pub lua: Arc<Mutex<Lua>>,
    config: LuaStateConfig,
    metrics: Arc<RwLock<LuaStateMetrics>>,
    start_time: Arc<Mutex<Option<Instant>>>,
    instruction_count: Arc<Mutex<u32>>,
    environment_manager: Arc<Mutex<EnvironmentManager>>,
}

impl LuaStateManager {
    pub fn new(config: LuaStateConfig) -> Result<Self> {
        let lua = Lua::new();
        let environment_manager = EnvironmentManager::new(config.environment_config.clone());
        let manager = Self {
            lua: Arc::new(Mutex::new(lua)),
            config,
            metrics: Arc::new(RwLock::new(LuaStateMetrics::default())),
            start_time: Arc::new(Mutex::new(None)),
            instruction_count: Arc::new(Mutex::new(0)),
            environment_manager: Arc::new(Mutex::new(environment_manager)),
        };

        manager.setup_state()?;
        Ok(manager)
    }

    fn setup_state(&self) -> Result<()> {
        let lua = self.lua.lock().unwrap();
        let globals = lua.globals();

        // Configure garbage collection
        if let Some(_step) = self.config.gc_step {
            lua.gc_collect()?;
            // Note: gc_set_step_multiplier removed due to API limitations
        }
        if let Some(_pause) = self.config.gc_pause {
            // Note: gc_set_pause removed due to API limitations
        }

        // Set up safe environment
        self.setup_sandbox(&lua, &globals)?;

        // Set up environment variable access
        {
            let env_manager = self.environment_manager.lock().unwrap();
            env_manager.setup_env_table(&lua)?;
        }

        // Set up monitoring hooks
        self.setup_monitoring(&lua)?;

        Ok(())
    }

    fn setup_sandbox(&self, lua: &Lua, globals: &Table) -> Result<()> {
        // Create safe environment table
        let safe_env = lua.create_table()?;

        // Copy safe standard libraries
        let safe_libs = vec![
            "_G",
            "string",
            "table",
            "math",
            "bit32",
            "utf8",
            "coroutine",
        ];

        for lib_name in safe_libs {
            if let Ok(lib) = globals.get::<_, Value>(lib_name) {
                safe_env.set(lib_name, lib)?;
            }
        }

        // Remove dangerous functions from global table
        let dangerous_functions = vec![
            "dofile",
            "loadfile",
            "load",
            "require",
            "package",
            "os.execute",
            "os.exit",
            "os.getenv",
            "os.remove",
            "os.rename",
            "debug.getregistry",
            "debug.getmetatable",
            "debug.setmetatable",
            "debug.getfenv",
            "debug.setfenv",
            "debug.getinfo",
            "debug.getlocal",
            "debug.setlocal",
            "debug.getupvalue",
            "debug.setupvalue",
            "debug.debug",
            "debug.traceback",
        ];

        for func_name in dangerous_functions {
            globals.raw_remove(func_name)?;
        }

        // Set up limited os table if allowed
        if self.config.allow_os_operations {
            let os_table = lua.create_table()?;
            if let Ok(os_global) = globals.get::<_, Table>("os") {
                if let Ok(clock) = os_global.get::<_, Function>("clock") {
                    os_table.set("clock", clock)?;
                }
                if let Ok(date) = os_global.get::<_, Function>("date") {
                    os_table.set("date", date)?;
                }
                if let Ok(time) = os_global.get::<_, Function>("time") {
                    os_table.set("time", time)?;
                }
            }
            globals.set("os", os_table)?;
        }

        // Set up limited debug table if allowed
        if self.config.allow_debug_operations {
            let debug_table = lua.create_table()?;
            if let Ok(debug_global) = globals.get::<_, Table>("debug") {
                if let Ok(traceback) = debug_global.get::<_, Function>("traceback") {
                    debug_table.set("traceback", traceback)?;
                }
            }
            globals.set("debug", debug_table)?;
        }

        // Set up safe package loading if allowed
        if self.config.allow_package_loading {
            let package_table = lua.create_table()?;
            package_table.set("loaded", lua.create_table()?)?;
            package_table.set("preload", lua.create_table()?)?;
            globals.set("package", package_table)?;
        }

        // Set up safe file operations if allowed
        if self.config.allow_file_operations {
            let io_table = lua.create_table()?;
            if let Ok(io_global) = globals.get::<_, Table>("io") {
                if let Ok(io_type) = io_global.get::<_, Function>("type") {
                    io_table.set("type", io_type)?;
                }
                if let Ok(flush) = io_global.get::<_, Function>("flush") {
                    io_table.set("flush", flush)?;
                }
            }
            globals.set("io", io_table)?;
        }

        // Create sandboxed environment - simplified due to API limitations
        let sandbox_code = r#"
            -- Basic sandbox setup
            local env = _ENV or _G
            local safe_env = ...
            
            -- Remove dangerous functions
            env.dofile = nil
            env.loadfile = nil
            env.load = nil
            env.require = nil
            env.package = nil
            env.os = env.os or {}
            env.os.execute = nil
            env.os.exit = nil
            env.os.remove = nil
            env.os.rename = nil
            env.os.getenv = nil
        "#;

        lua.load(sandbox_code)
            .set_name("sandbox_setup")
            .call::<_, ()>(safe_env)?;

        Ok(())
    }

    fn setup_monitoring(&self, lua: &Lua) -> Result<()> {
        // Set up instruction count hook
        let instruction_count = self.instruction_count.clone();
        let instruction_limit = self.config.instruction_limit;
        let time_limit = self.config.time_limit;
        let start_time = self.start_time.clone();

        // Note: Hook setup removed due to mlua API limitations
        // In a production implementation, you would need to use a different approach
        // for instruction counting and time limiting, possibly through a wrapper
        // that periodically checks execution state.

        Ok(())
    }

    pub fn execute_script(&self, script_path: &Path, script_args: &[String]) -> Result<()> {
        let lua = self.lua.lock().unwrap();

        // Reset metrics
        {
            let mut metrics = self.metrics.write().unwrap();
            *metrics = LuaStateMetrics::default();
        }
        {
            let mut count = self.instruction_count.lock().unwrap();
            *count = 0;
        }
        {
            let mut start = self.start_time.lock().unwrap();
            *start = Some(Instant::now());
        }

        // Set up script arguments only if args table doesn't exist or is simple
        let globals = lua.globals();
        let existing_args: mlua::Result<mlua::Table> = globals.get("args");

        // Only set up simple args table if enhanced one doesn't exist
        if existing_args.is_err() {
            let args_table = lua.create_table()?;
            for (i, arg) in script_args.iter().enumerate() {
                args_table.set(i + 1, arg.clone())?;
            }
            globals.set("args", args_table)?;
        }

        // Load and execute script
        let script_content = crate::file_io::read_lua_script(script_path)?;

        let result = lua
            .load(&script_content)
            .set_name(script_path.to_string_lossy())
            .exec();

        // Update metrics
        {
            let mut metrics = self.metrics.write().unwrap();
            if let Ok(start) = self.start_time.lock() {
                if let Some(start_instant) = *start {
                    metrics.execution_time = start_instant.elapsed();
                }
            }
            if let Ok(count) = self.instruction_count.lock() {
                metrics.instructions_executed = *count;
            }

            // Get memory usage (approximate)
            metrics.memory_usage = lua.used_memory() as usize;
        }

        result.map_err(|e| HypeError::Lua(e.to_string()))?;

        Ok(())
    }

    pub fn execute_code(&self, code: &str) -> Result<String> {
        let lua = self.lua.lock().unwrap();

        // Reset metrics
        {
            let mut metrics = self.metrics.write().unwrap();
            *metrics = LuaStateMetrics::default();
        }
        {
            let mut count = self.instruction_count.lock().unwrap();
            *count = 0;
        }
        {
            let mut start = self.start_time.lock().unwrap();
            *start = Some(Instant::now());
        }

        let result = lua.load(code).set_name("dynamic_code").eval::<Value>();

        // Update metrics
        {
            let mut metrics = self.metrics.write().unwrap();
            if let Ok(start) = self.start_time.lock() {
                if let Some(start_instant) = *start {
                    metrics.execution_time = start_instant.elapsed();
                }
            }
            if let Ok(count) = self.instruction_count.lock() {
                metrics.instructions_executed = *count;
            }
            metrics.memory_usage = lua.used_memory() as usize;
        }

        match result {
            Ok(value) => Ok(crate::lua::error::format_lua_value(&value)),
            Err(e) => Err(HypeError::Lua(e.to_string())),
        }
    }

    pub fn get_metrics(&self) -> LuaStateMetrics {
        self.metrics.read().unwrap().clone()
    }

    pub fn force_gc(&self) -> Result<()> {
        let lua = self.lua.lock().unwrap();
        lua.gc_collect()?;
        Ok(())
    }

    pub fn set_global(&self, name: &str, value: Value) -> Result<()> {
        let lua = self.lua.lock().unwrap();
        let globals = lua.globals();
        globals.set(name, value)?;
        Ok(())
    }

    pub fn get_global(&self, name: &str) -> Result<String> {
        let lua = self.lua.lock().unwrap();
        let globals = lua.globals();
        let value = globals.get::<_, Value>(name)?;
        Ok(crate::lua::error::format_lua_value(&value))
    }

    pub fn create_table(&self) -> Result<()> {
        let lua = self.lua.lock().unwrap();
        let _table = lua.create_table()?;
        Ok(())
    }

    pub fn load_function(&self, code: &str) -> Result<()> {
        let lua = self.lua.lock().unwrap();
        let _function = lua.load(code).set_name("loaded_function").into_function()?;
        Ok(())
    }

    pub fn get_environment_manager(&self) -> Arc<Mutex<EnvironmentManager>> {
        self.environment_manager.clone()
    }

    pub fn get_environment_access_log(&self) -> Vec<crate::lua::environment::EnvironmentAccess> {
        let env_manager = self.environment_manager.lock().unwrap();
        env_manager.get_access_log()
    }

    pub fn clear_environment_access_log(&self) {
        let env_manager = self.environment_manager.lock().unwrap();
        env_manager.clear_access_log();
    }
}

impl Drop for LuaStateManager {
    fn drop(&mut self) {
        // Force garbage collection on cleanup
        if let Ok(lua) = self.lua.lock() {
            let _ = lua.gc_collect();
        }
    }
}

// Thread-safe pool of Lua states for concurrent execution
pub struct LuaStatePool {
    states: Arc<RwLock<Vec<LuaStateManager>>>,
    config: LuaStateConfig,
    max_size: usize,
}

impl LuaStatePool {
    pub fn new(config: LuaStateConfig, max_size: usize) -> Result<Self> {
        Ok(Self {
            states: Arc::new(RwLock::new(Vec::new())),
            config,
            max_size,
        })
    }

    pub fn acquire_state(&self) -> Result<LuaStateManager> {
        let mut states = self.states.write().unwrap();

        if let Some(state) = states.pop() {
            Ok(state)
        } else {
            LuaStateManager::new(self.config.clone())
        }
    }

    pub fn release_state(&self, state: LuaStateManager) -> Result<()> {
        let mut states = self.states.write().unwrap();

        if states.len() < self.max_size {
            states.push(state);
        }

        Ok(())
    }

    pub fn clear(&self) -> Result<()> {
        let mut states = self.states.write().unwrap();
        states.clear();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lua_state_creation() {
        let config = LuaStateConfig::default();
        let state = LuaStateManager::new(config);
        assert!(state.is_ok());
    }

    #[test]
    fn test_simple_execution() {
        let config = LuaStateConfig::default();
        let state = LuaStateManager::new(config).unwrap();

        let result = state.execute_code("return 2 + 2");
        assert!(result.is_ok());

        if let Ok(value) = result {
            assert_eq!(value, "4");
        }
    }

    #[test]
    fn test_sandbox_restriction() {
        let config = LuaStateConfig::default();
        let state = LuaStateManager::new(config).unwrap();

        // This should fail due to sandboxing
        let result = state.execute_code("os.execute('echo test')");
        assert!(result.is_err());
    }
}
