pub mod debug;
pub mod env;
pub mod environment;
pub mod error;
pub mod lifecycle;
pub mod module_env;
pub mod path;
pub mod require;
pub mod security;
pub mod state;

use crate::error::Result;
use crate::file_io::read_lua_script;
use std::path::Path;
use std::time::Duration;

pub use environment::{
    EnvironmentAccess, EnvironmentConfig, EnvironmentManager, EnvironmentOperation,
};
pub use error::{LuaErrorHandler, LuaExecutionError};
pub use lifecycle::{
    LuaStateHandle, LuaStateLifecycleManager, LuaStatePool as LifecyclePool, StateInfo,
};
pub use module_env::{create_module_env, ModuleEnvironment};
pub use require::{setup_require_fn, RequireSetup};
pub use security::{FileOperationPolicy, OsOperationPolicy, SecurityManager, SecurityPolicy};
pub use state::{LuaStateConfig, LuaStateManager, LuaStateMetrics, LuaStatePool};

/// Execute a Lua script with the new comprehensive state management
pub fn execute_script(
    script_path: &Path,
    script_args: &[String],
    verbose: bool,
    debug: bool,
    timeout: Option<Duration>,
) -> Result<()> {
    // Create configuration based on parameters
    let mut config = LuaStateConfig::default();

    // Apply timeout if provided
    if let Some(timeout) = timeout {
        config.time_limit = Some(timeout);
    }

    // Enable debug operations if debug flag is set
    if debug {
        config.allow_debug_operations = true;
    }

    // Create security policy
    let security_policy = SecurityPolicy::default();

    // Create state manager
    let state_manager = LuaStateManager::new(config)?;

    // Apply security sandbox
    let security_manager = SecurityManager::new(security_policy);
    {
        let lua = state_manager.lua.lock().unwrap();
        security_manager.apply_sandbox(&lua)?;

        // Set up global variables
        let globals = lua.globals();

        // Create an args table with script arguments
        let args_table = lua.create_table()?;
        for (i, arg) in script_args.iter().enumerate() {
            args_table.set(i + 1, arg.clone())?;
        }
        globals.set("args", args_table)?;

        // Set up debug/verbose flags
        globals.set("verbose", verbose)?;
        globals.set("debug", debug)?;
    } // Lock is released here

    if verbose {
        eprintln!("Loading Lua script: {}", script_path.display());
    }

    // Load and execute the script
    let script_content = read_lua_script(script_path)?;

    if debug {
        eprintln!("Script content length: {} bytes", script_content.len());
    }

    // Execute the script using the state manager
    state_manager.execute_script(script_path, script_args)?;

    if verbose {
        let metrics = state_manager.get_metrics();
        eprintln!("Script executed successfully");
        eprintln!("Execution time: {:?}", metrics.execution_time);
        eprintln!("Instructions executed: {}", metrics.instructions_executed);
        eprintln!("Memory usage: {} bytes", metrics.memory_usage);
    }

    Ok(())
}

/// Execute Lua code directly with state management
pub fn execute_code(
    code: &str,
    config: Option<LuaStateConfig>,
    security_policy: Option<SecurityPolicy>,
) -> Result<String> {
    let config = config.unwrap_or_default();
    let security_policy = security_policy.unwrap_or_default();

    let state_manager = LuaStateManager::new(config)?;

    // Apply security sandbox
    {
        let lua = state_manager.lua.lock().unwrap();
        let security_manager = SecurityManager::new(security_policy);
        security_manager.apply_sandbox(&lua)?;
    }

    state_manager.execute_code(code)
}

/// Create a default Lua state configuration for CLI usage
pub fn create_cli_config(verbose: bool, debug: bool, timeout: Option<Duration>) -> LuaStateConfig {
    let mut config = LuaStateConfig::default();

    if let Some(timeout) = timeout {
        config.time_limit = Some(timeout);
    }

    if debug {
        config.allow_debug_operations = true;
        config.instruction_limit = None; // Remove instruction limit in debug mode
                                         // Allow environment variable writing in debug mode
        config.environment_config.allow_write = true;
        config.environment_config.allow_sensitive_read = true;
    }

    if verbose {
        config.allow_debug_operations = true;
    }

    config
}

/// Create a security policy for CLI usage
pub fn create_cli_security_policy(debug: bool) -> SecurityPolicy {
    let mut policy = SecurityPolicy::default();

    if debug {
        policy.allow_debug = true;
        policy.allow_package_loading = true;
        // Allow more environment access in debug mode
        policy.environment_policy.allow_write = true;
        policy.environment_policy.allow_sensitive_read = true;
    }

    policy
}

#[cfg(test)]
mod tests {
    use super::*;
    use mlua::Value;

    #[test]
    fn test_execute_code_simple() {
        let result = execute_code("return 2 + 2", None, None);
        assert!(result.is_ok());

        if let Ok(value) = result {
            assert_eq!(value, "4");
        }
    }

    #[test]
    fn test_cli_config_creation() {
        let config = create_cli_config(true, false, Some(Duration::from_secs(10)));
        assert!(config.allow_debug_operations);
        assert_eq!(config.time_limit, Some(Duration::from_secs(10)));
    }

    #[test]
    fn test_cli_security_policy() {
        let policy = create_cli_security_policy(true);
        assert!(policy.allow_debug);
        assert!(policy.allow_package_loading);
    }
}
