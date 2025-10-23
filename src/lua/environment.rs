use crate::error::Result;
use mlua::{Error as LuaError, Lua, Table, Value};
use std::collections::HashSet;
use std::env;
use std::sync::{Arc, RwLock};

/// Sensitive environment variables that should be restricted by default
const SENSITIVE_VARS: &[&str] = &[
    "PASSWORD",
    "PASS",
    "TOKEN",
    "SECRET",
    "KEY",
    "API_KEY",
    "PRIVATE_KEY",
    "DATABASE_URL",
    "DB_PASSWORD",
    "AWS_SECRET_ACCESS_KEY",
    "AWS_ACCESS_KEY_ID",
    "GITHUB_TOKEN",
    "SLACK_TOKEN",
    "SSH_PRIVATE_KEY",
    "SSL_CERT",
    "SSL_KEY",
    "CREDENTIALS",
    "AUTH",
    "AUTH_TOKEN",
    "SESSION_KEY",
    "COOKIE_SECRET",
];

#[derive(Debug, Clone)]
pub struct EnvironmentConfig {
    pub allow_read: bool,
    pub allow_write: bool,
    pub allowed_vars: HashSet<String>,
    pub denied_vars: HashSet<String>,
    pub allow_sensitive_read: bool,
    pub allow_sensitive_write: bool,
    pub case_sensitive: bool,
}

impl Default for EnvironmentConfig {
    fn default() -> Self {
        Self {
            allow_read: true,
            allow_write: false, // More restrictive by default
            allowed_vars: HashSet::new(),
            denied_vars: SENSITIVE_VARS.iter().map(|s| s.to_string()).collect(),
            allow_sensitive_read: false,
            allow_sensitive_write: false,
            case_sensitive: true,
        }
    }
}

#[derive(Debug)]
pub struct EnvironmentManager {
    config: EnvironmentConfig,
    access_log: Arc<RwLock<Vec<EnvironmentAccess>>>,
}

#[derive(Debug, Clone)]
pub struct EnvironmentAccess {
    pub operation: EnvironmentOperation,
    pub variable_name: String,
    pub value: Option<String>,
    pub allowed: bool,
    pub timestamp: std::time::Instant,
}

#[derive(Debug, Clone)]
pub enum EnvironmentOperation {
    Read,
    Write,
    Delete,
}

impl EnvironmentManager {
    pub fn new(config: EnvironmentConfig) -> Self {
        Self {
            config,
            access_log: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn setup_env_table(&self, lua: &Lua) -> Result<()> {
        let globals = lua.globals();
        let env_table = lua.create_table()?;

        // Create the env table with __index and __newindex metamethods
        self.setup_env_metamethods(lua, &env_table)?;

        // Add utility methods to the env table
        self.setup_env_methods(lua, &env_table)?;

        globals.set("env", env_table)?;
        Ok(())
    }

    fn setup_env_metamethods(&self, _lua: &Lua, _env_table: &Table) -> Result<()> {
        Ok(())
    }

    fn setup_env_methods(&self, _lua: &Lua, _env_table: &Table) -> Result<()> {
        Ok(())
    }

    fn is_sensitive(var_name: &str) -> bool {
        let upper_name = var_name.to_uppercase();
        SENSITIVE_VARS
            .iter()
            .any(|sensitive| upper_name.contains(sensitive) || upper_name.ends_with(sensitive))
    }

    fn log_access(
        access_log: &Arc<RwLock<Vec<EnvironmentAccess>>>,
        operation: EnvironmentOperation,
        variable_name: String,
        value: Option<String>,
        allowed: bool,
    ) {
        let access = EnvironmentAccess {
            operation,
            variable_name,
            value,
            allowed,
            timestamp: std::time::Instant::now(),
        };

        if let Ok(mut log) = access_log.write() {
            log.push(access);
        }
    }

    pub fn get_access_log(&self) -> Vec<EnvironmentAccess> {
        self.access_log.read().unwrap().clone()
    }

    pub fn clear_access_log(&self) {
        self.access_log.write().unwrap().clear();
    }

    pub fn get_config(&self) -> &EnvironmentConfig {
        &self.config
    }

    pub fn update_config(&mut self, config: EnvironmentConfig) {
        self.config = config;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mlua::Lua;

    #[test]
    fn test_environment_config_default() {
        let config = EnvironmentConfig::default();
        assert!(config.allow_read);
        assert!(!config.allow_write);
        assert!(!config.allow_sensitive_read);
        assert!(!config.allow_sensitive_write);
        assert!(config.case_sensitive);
    }

    #[test]
    fn test_sensitive_detection() {
        assert!(EnvironmentManager::is_sensitive("PASSWORD"));
        assert!(EnvironmentManager::is_sensitive("API_KEY"));
        assert!(EnvironmentManager::is_sensitive("MY_SECRET_TOKEN"));
        assert!(!EnvironmentManager::is_sensitive("PATH"));
        assert!(!EnvironmentManager::is_sensitive("HOME"));
    }

    #[test]
    fn test_environment_manager_creation() {
        let config = EnvironmentConfig::default();
        let manager = EnvironmentManager::new(config);
        let log = manager.get_access_log();
        assert!(log.is_empty());
    }

    #[test]
    fn test_env_table_setup() -> Result<()> {
        let lua = Lua::new();
        let config = EnvironmentConfig::default();
        let manager = EnvironmentManager::new(config);

        manager.setup_env_table(&lua)?;

        let globals = lua.globals();
        let env_table: Table = globals.get("env")?;

        // Check that methods exist
        assert!(env_table.contains_key("get")?);
        assert!(env_table.contains_key("set")?);
        assert!(env_table.contains_key("unset")?);
        assert!(env_table.contains_key("list")?);
        assert!(env_table.contains_key("exists")?);
        assert!(env_table.contains_key("is_sensitive")?);

        Ok(())
    }

    #[test]
    fn test_env_read_access() -> Result<()> {
        let lua = Lua::new();

        // Set a test environment variable
        env::set_var("TEST_VAR", "test_value");

        let config = EnvironmentConfig::default();
        let manager = EnvironmentManager::new(config);
        manager.setup_env_table(&lua)?;

        // Test reading via direct access
        let result: mlua::Result<String> = lua.load("return env.TEST_VAR").eval();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test_value");

        // Test reading via method
        let result: mlua::Result<Option<String>> = lua.load("return env.get('TEST_VAR')").eval();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some("test_value".to_string()));

        // Clean up
        env::remove_var("TEST_VAR");

        Ok(())
    }

    #[test]
    fn test_env_write_access() -> Result<()> {
        let lua = Lua::new();

        let mut config = EnvironmentConfig::default();
        config.allow_write = true;
        let manager = EnvironmentManager::new(config);
        manager.setup_env_table(&lua)?;

        // Test writing via direct access
        let result: mlua::Result<()> = lua.load("env.TEST_WRITE_VAR = 'written_value'").exec();
        assert!(result.is_ok());

        assert_eq!(env::var("TEST_WRITE_VAR").unwrap(), "written_value");

        // Test writing via method
        let result: mlua::Result<bool> = lua
            .load("return env.set('TEST_WRITE_VAR2', 'written_value2')")
            .eval();
        assert!(result.is_ok());
        assert!(result.unwrap());

        assert_eq!(env::var("TEST_WRITE_VAR2").unwrap(), "written_value2");

        // Clean up
        env::remove_var("TEST_WRITE_VAR");
        env::remove_var("TEST_WRITE_VAR2");

        Ok(())
    }

    #[test]
    fn test_sensitive_var_restriction() -> Result<()> {
        let lua = Lua::new();

        let config = EnvironmentConfig::default();
        let manager = EnvironmentManager::new(config);
        manager.setup_env_table(&lua)?;

        // Set a sensitive variable
        env::set_var("TEST_PASSWORD", "secret_value");

        // Try to read it - should fail
        let result: mlua::Result<String> = lua.load("return env.TEST_PASSWORD").eval();
        assert!(result.is_err());

        // Clean up
        env::remove_var("TEST_PASSWORD");

        Ok(())
    }
}
