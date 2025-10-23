use crate::error::{HypeError, Result};
use mlua::{Function, Lua, Table, Value};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct SecurityPolicy {
    pub allowed_modules: HashSet<String>,
    pub denied_modules: HashSet<String>,
    pub allowed_functions: HashSet<String>,
    pub denied_functions: HashSet<String>,
    pub allowed_file_operations: FileOperationPolicy,
    pub allowed_os_operations: OsOperationPolicy,
    pub memory_limit: Option<usize>,
    pub execution_time_limit: Option<Duration>,
    pub instruction_limit: Option<u32>,
    pub allow_debug: bool,
    pub allow_package_loading: bool,
    pub allow_coroutine_creation: bool,
    pub allow_metatable_manipulation: bool,
    pub environment_policy: EnvironmentOperationPolicy,
}

#[derive(Debug, Clone)]
pub struct FileOperationPolicy {
    pub read: bool,
    pub write: bool,
    pub append: bool,
    pub allowed_paths: Vec<PathBuf>,
    pub denied_paths: Vec<PathBuf>,
    pub max_file_size: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct OsOperationPolicy {
    pub clock: bool,
    pub date: bool,
    pub time: bool,
    pub tmpname: bool,
    pub getenv: bool,
    pub setenv: bool,
}

#[derive(Debug, Clone)]
pub struct EnvironmentOperationPolicy {
    pub allow_read: bool,
    pub allow_write: bool,
    pub allow_sensitive_read: bool,
    pub allow_sensitive_write: bool,
    pub allowed_vars: HashSet<String>,
    pub denied_vars: HashSet<String>,
    pub case_sensitive: bool,
}

impl Default for SecurityPolicy {
    fn default() -> Self {
        Self {
            allowed_modules: HashSet::from_iter(vec![
                "string".to_string(),
                "table".to_string(),
                "math".to_string(),
                "bit32".to_string(),
                "utf8".to_string(),
            ]),
            denied_modules: HashSet::from_iter(vec![
                "io".to_string(),
                "os".to_string(),
                "debug".to_string(),
                "package".to_string(),
                "coroutine".to_string(),
            ]),
            allowed_functions: HashSet::new(),
            denied_functions: HashSet::from_iter(vec![
                "dofile".to_string(),
                "loadfile".to_string(),
                "load".to_string(),
                "require".to_string(),
                "collectgarbage".to_string(),
                "getfenv".to_string(),
                "setfenv".to_string(),
                "getmetatable".to_string(),
                "setmetatable".to_string(),
                "rawget".to_string(),
                "rawset".to_string(),
                "rawequal".to_string(),
                "newproxy".to_string(),
            ]),
            allowed_file_operations: FileOperationPolicy {
                read: false,
                write: false,
                append: false,
                allowed_paths: vec![],
                denied_paths: vec![],
                max_file_size: Some(1024 * 1024), // 1MB default
            },
            allowed_os_operations: OsOperationPolicy {
                clock: true,
                date: true,
                time: true,
                tmpname: false,
                getenv: false,
                setenv: false,
            },
            memory_limit: Some(64 * 1024 * 1024), // 64MB default
            execution_time_limit: Some(Duration::from_secs(30)),
            instruction_limit: Some(1000000), // 1M instructions
            allow_debug: false,
            allow_package_loading: false,
            allow_coroutine_creation: false,
            allow_metatable_manipulation: false,
            environment_policy: EnvironmentOperationPolicy {
                allow_read: true,
                allow_write: false,
                allow_sensitive_read: false,
                allow_sensitive_write: false,
                allowed_vars: HashSet::new(),
                denied_vars: HashSet::from_iter(vec![
                    "PASSWORD".to_string(),
                    "PASS".to_string(),
                    "TOKEN".to_string(),
                    "SECRET".to_string(),
                    "KEY".to_string(),
                    "API_KEY".to_string(),
                    "PRIVATE_KEY".to_string(),
                    "DATABASE_URL".to_string(),
                    "DB_PASSWORD".to_string(),
                    "AWS_SECRET_ACCESS_KEY".to_string(),
                    "AWS_ACCESS_KEY_ID".to_string(),
                    "GITHUB_TOKEN".to_string(),
                    "SLACK_TOKEN".to_string(),
                    "SSH_PRIVATE_KEY".to_string(),
                    "SSL_CERT".to_string(),
                    "SSL_KEY".to_string(),
                    "CREDENTIALS".to_string(),
                    "AUTH".to_string(),
                    "AUTH_TOKEN".to_string(),
                    "SESSION_KEY".to_string(),
                    "COOKIE_SECRET".to_string(),
                ]),
                case_sensitive: true,
            },
        }
    }
}

pub struct SecurityManager {
    policy: SecurityPolicy,
    execution_stats: Arc<RwLock<ExecutionStats>>,
    path_validator: PathValidator,
}

#[derive(Debug, Default, Clone)]
pub struct ExecutionStats {
    pub start_time: Option<Instant>,
    pub instructions_executed: u32,
    pub memory_usage: usize,
    pub files_accessed: Vec<PathBuf>,
    pub modules_loaded: Vec<String>,
    pub security_violations: Vec<SecurityViolation>,
}

#[derive(Debug, Clone)]
pub struct SecurityViolation {
    pub violation_type: ViolationType,
    pub operation: String,
    pub details: String,
    pub timestamp: Instant,
}

#[derive(Debug, Clone)]
pub enum ViolationType {
    ModuleAccessDenied,
    FunctionAccessDenied,
    FileAccessDenied,
    MemoryLimitExceeded,
    TimeLimitExceeded,
    InstructionLimitExceeded,
    DebugAccessDenied,
    PackageLoadingDenied,
    CoroutineCreationDenied,
    MetatableManipulationDenied,
}

impl SecurityManager {
    pub fn new(policy: SecurityPolicy) -> Self {
        Self {
            policy,
            execution_stats: Arc::new(RwLock::new(ExecutionStats::default())),
            path_validator: PathValidator::new(),
        }
    }

    pub fn apply_sandbox(&self, lua: &Lua) -> Result<()> {
        let globals = lua.globals();

        // Reset execution stats
        {
            let mut stats = self.execution_stats.write().unwrap();
            *stats = ExecutionStats::default();
            stats.start_time = Some(Instant::now());
        }

        // Apply module restrictions
        self.apply_module_restrictions(lua, &globals)?;

        // Apply function restrictions
        self.apply_function_restrictions(lua, &globals)?;

        // Set up safe replacements for dangerous functions
        self.setup_safe_replacements(lua, &globals)?;

        // Set up monitoring hooks
        self.setup_monitoring_hooks(lua)?;

        Ok(())
    }

    fn apply_module_restrictions(&self, lua: &Lua, globals: &Table) -> Result<()> {
        // Remove denied modules
        for module in &self.policy.denied_modules {
            globals.raw_remove(module.as_str())?;
        }

        // Keep only allowed modules
        let allowed_modules = &self.policy.allowed_modules;
        let mut modules_to_remove = Vec::new();

        // Get all global keys
        let globals_keys: Vec<String> = globals
            .clone()
            .pairs::<String, Value>()
            .filter_map(|pair| pair.ok().map(|(k, _)| k))
            .collect();

        for key in globals_keys {
            if !allowed_modules.contains(&key) && !self.policy.allowed_functions.contains(&key) {
                // Check if it's a module (table with functions)
                if let Ok(Value::Table(_)) = globals.get::<_, Value>(key.as_str()) {
                    modules_to_remove.push(key);
                }
            }
        }

        for module in modules_to_remove {
            globals.raw_remove(module.as_str())?;
        }

        Ok(())
    }

    fn apply_function_restrictions(&self, lua: &Lua, globals: &Table) -> Result<()> {
        // Remove denied functions
        for function in &self.policy.denied_functions {
            globals.raw_remove(function.as_str())?;
        }

        // Create safe function wrappers
        self.create_safe_function_wrappers(lua, globals)?;

        Ok(())
    }

    fn create_safe_function_wrappers(&self, lua: &Lua, globals: &Table) -> Result<()> {
        // Safe print function
        let safe_print = lua.create_function(|_lua, args: mlua::MultiValue| {
            for arg in args {
                print!("{}\t", crate::lua::error::format_lua_value(&arg));
            }
            println!();
            Ok(())
        })?;
        globals.set("print", safe_print)?;

        // Safe type function
        let safe_type = lua.create_function(|_lua, value: Value| {
            Ok(match value {
                Value::Nil => "nil",
                Value::Boolean(_) => "boolean",
                Value::Integer(_) => "number",
                Value::Number(_) => "number",
                Value::String(_) => "string",
                Value::Table(_) => "table",
                Value::Function(_) => "function",
                Value::LightUserData(_) => "userdata",
                Value::UserData(_) => "userdata",
                Value::Thread(_) => "thread",
                Value::Error(_) => "error",
            }
            .to_string())
        })?;
        globals.set("type", safe_type)?;

        // Safe tostring function
        let safe_tostring = lua.create_function(|_lua, value: Value| {
            Ok(crate::lua::error::format_lua_value(&value))
        })?;
        globals.set("tostring", safe_tostring)?;

        // Safe tonumber function
        let safe_tonumber = lua.create_function(|_lua, value: Value| match value {
            Value::Integer(i) => Ok(Some(Value::Number(i as f64))),
            Value::Number(n) => Ok(Some(Value::Number(n))),
            Value::String(s) => {
                let s = s.to_string_lossy();
                s.parse::<f64>()
                    .map(|n| Some(Value::Number(n)))
                    .or_else(|_| s.parse::<i64>().map(|i| Some(Value::Integer(i))))
                    .map_err(|_| mlua::Error::RuntimeError("cannot convert to number".to_string()))
            }
            _ => Ok(None),
        })?;
        globals.set("tonumber", safe_tonumber)?;

        Ok(())
    }

    fn setup_safe_replacements(&self, lua: &Lua, globals: &Table) -> Result<()> {
        // Safe os table if allowed
        if self.policy.allowed_os_operations.clock
            || self.policy.allowed_os_operations.date
            || self.policy.allowed_os_operations.time
        {
            let os_table = lua.create_table()?;

            if self.policy.allowed_os_operations.clock {
                if let Ok(os_func) = globals.get::<_, Table>("os") {
                    if let Ok(clock) = os_func.get::<_, Function>("clock") {
                        os_table.set("clock", clock)?;
                    }
                }
            }

            if self.policy.allowed_os_operations.date {
                if let Ok(os_func) = globals.get::<_, Table>("os") {
                    if let Ok(date) = os_func.get::<_, Function>("date") {
                        os_table.set("date", date)?;
                    }
                }
            }

            if self.policy.allowed_os_operations.time {
                if let Ok(os_func) = globals.get::<_, Table>("os") {
                    if let Ok(time) = os_func.get::<_, Function>("time") {
                        os_table.set("time", time)?;
                    }
                }
            }

            globals.set("os", os_table)?;
        }

        // Safe debug table if allowed
        if self.policy.allow_debug {
            let debug_table = lua.create_table()?;

            if let Ok(debug_func) = globals.get::<_, Table>("debug") {
                if let Ok(traceback) = debug_func.get::<_, Function>("traceback") {
                    debug_table.set("traceback", traceback)?;
                }
            }

            globals.set("debug", debug_table)?;
        }

        Ok(())
    }

    fn setup_monitoring_hooks(&self, lua: &Lua) -> Result<()> {
        let stats = self.execution_stats.clone();
        let memory_limit = self.policy.memory_limit;
        let time_limit = self.policy.execution_time_limit;
        let instruction_limit = self.policy.instruction_limit;

        // Note: Hook setup removed due to mlua API limitations
        // In a production implementation, you would need to use a different approach
        // for instruction counting and time limiting, possibly through a wrapper
        // that periodically checks execution state.

        Ok(())
    }

    pub fn check_file_access(&self, path: &Path, operation: &str) -> Result<()> {
        let path_buf = path.to_path_buf();

        // Check denied paths first
        for denied_path in &self.policy.allowed_file_operations.denied_paths {
            if path_buf.starts_with(denied_path) {
                let mut stats = self.execution_stats.write().unwrap();
                let violation = SecurityViolation {
                    violation_type: ViolationType::FileAccessDenied,
                    operation: operation.to_string(),
                    details: format!("Access to denied path: {}", path.display()),
                    timestamp: Instant::now(),
                };
                stats.security_violations.push(violation);
                return Err(HypeError::Execution(format!(
                    "File access denied: {}",
                    path.display()
                )));
            }
        }

        // Check allowed paths
        if !self.policy.allowed_file_operations.allowed_paths.is_empty() {
            let allowed = self
                .policy
                .allowed_file_operations
                .allowed_paths
                .iter()
                .any(|allowed_path| path_buf.starts_with(allowed_path));

            if !allowed {
                let mut stats = self.execution_stats.write().unwrap();
                let violation = SecurityViolation {
                    violation_type: ViolationType::FileAccessDenied,
                    operation: operation.to_string(),
                    details: format!("Access to non-allowed path: {}", path.display()),
                    timestamp: Instant::now(),
                };
                stats.security_violations.push(violation);
                return Err(HypeError::Execution(format!(
                    "File access not allowed: {}",
                    path.display()
                )));
            }
        }

        // Record file access
        {
            let mut stats = self.execution_stats.write().unwrap();
            stats.files_accessed.push(path_buf);
        }

        Ok(())
    }

    pub fn get_execution_stats(&self) -> ExecutionStats {
        self.execution_stats.read().unwrap().clone()
    }

    pub fn reset_stats(&self) {
        let mut stats = self.execution_stats.write().unwrap();
        *stats = ExecutionStats::default();
    }
}

pub struct PathValidator {
    // Add path validation logic here
}

impl PathValidator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn is_safe_path(&self, path: &Path) -> bool {
        // Basic path safety checks
        if path.is_absolute() {
            return false;
        }

        // Check for directory traversal
        if let Some(path_str) = path.to_str() {
            if path_str.contains("..") {
                return false;
            }
        }

        true
    }
}

impl Default for PathValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_policy_default() {
        let policy = SecurityPolicy::default();
        assert!(policy.denied_modules.contains("io"));
        assert!(policy.allowed_modules.contains("string"));
        assert!(!policy.allow_debug);
    }

    #[test]
    fn test_path_validator() {
        let validator = PathValidator::new();
        assert!(validator.is_safe_path(Path::new("safe.txt")));
        assert!(!validator.is_safe_path(Path::new("../dangerous.txt")));
        assert!(!validator.is_safe_path(Path::new("/absolute/path")));
    }

    #[test]
    fn test_security_manager_creation() {
        let policy = SecurityPolicy::default();
        let manager = SecurityManager::new(policy);
        let stats = manager.get_execution_stats();
        assert_eq!(stats.instructions_executed, 0);
    }
}
