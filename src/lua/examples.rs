//! Examples demonstrating the comprehensive Lua state management module

use crate::lua::*;
use std::time::Duration;
use std::path::PathBuf;

/// Example 1: Basic Lua state creation and execution
pub fn basic_execution_example() -> Result<()> {
    println!("=== Basic Execution Example ===");
    
    // Create default configuration
    let config = LuaStateConfig::default();
    
    // Create state manager
    let state_manager = LuaStateManager::new(config)?;
    
    // Execute simple code
    let result = state_manager.execute_code("return 'Hello, Lua!')")?;
    println!("Result: {:?}", result);
    
    // Get execution metrics
    let metrics = state_manager.get_metrics();
    println!("Execution time: {:?}", metrics.execution_time);
    println!("Instructions executed: {}", metrics.instructions_executed);
    println!("Memory usage: {} bytes", metrics.memory_usage);
    
    Ok(())
}

/// Example 2: Security sandboxing
pub fn security_sandboxing_example() -> Result<()> {
    println!("\n=== Security Sandboxing Example ===");
    
    // Create restrictive security policy
    let mut security_policy = SecurityPolicy::default();
    security_policy.allow_debug_operations = false;
    security_policy.allow_file_operations = false;
    security_policy.allow_os_operations = false;
    
    // Create state with security
    let config = LuaStateConfig::default();
    let state_manager = LuaStateManager::new(config)?;
    
    // Apply security sandbox
    {
        let lua = state_manager.lua.lock().unwrap();
        let security_manager = SecurityManager::new(security_policy);
        security_manager.apply_sandbox(&lua)?;
    }
    
    // Try to execute dangerous code (should fail)
    match state_manager.execute_code("os.execute('echo test')") {
        Ok(_) => println!("ERROR: Dangerous code was allowed to execute!"),
        Err(e) => println!("Good: Dangerous code was blocked: {}", e),
    }
    
    // Try to execute safe code (should succeed)
    match state_manager.execute_code("return math.sqrt(16)") {
        Ok(result) => println!("Safe code result: {:?}", result),
        Err(e) => println!("ERROR: Safe code was blocked: {}", e),
    }
    
    Ok(())
}

/// Example 3: Resource limits and monitoring
pub fn resource_limits_example() -> Result<()> {
    println!("\n=== Resource Limits Example ===");
    
    // Create configuration with strict limits
    let mut config = LuaStateConfig::default();
    config.instruction_limit = Some(10000); // 10K instructions
    config.time_limit = Some(Duration::from_millis(100)); // 100ms
    config.memory_limit = Some(1024 * 1024); // 1MB
    
    let state_manager = LuaStateManager::new(config)?;
    
    // Execute code within limits
    match state_manager.execute_code("for i = 1, 100 do math.sqrt(i) end return 'done'") {
        Ok(result) => println!("Code within limits: {:?}", result),
        Err(e) => println!("Code failed: {}", e),
    }
    
    // Try to exceed instruction limit
    match state_manager.execute_code("while true do end") {
        Ok(_) => println!("ERROR: Infinite loop was not stopped!"),
        Err(e) => println!("Good: Infinite loop was stopped: {}", e),
    }
    
    // Get final metrics
    let metrics = state_manager.get_metrics();
    println!("Final metrics: {:?}", metrics);
    
    Ok(())
}

/// Example 4: State lifecycle management
pub fn lifecycle_management_example() -> Result<()> {
    println!("\n=== Lifecycle Management Example ===");
    
    // Create lifecycle manager
    let lifecycle_manager = LuaStateLifecycleManager::new(
        Duration::from_millis(100), // cleanup interval
        Duration::from_secs(1),     // max idle time
    );
    
    // Create multiple states
    let config = LuaStateConfig::default();
    let state1_id = lifecycle_manager.create_state(config.clone(), false)?;
    let state2_id = lifecycle_manager.create_state(config.clone(), true)?; // persistent
    let state3_id = lifecycle_manager.create_state(config, false)?;
    
    println!("Created states: {}, {}, {}", state1_id, state2_id, state3_id);
    println!("Total states: {}", lifecycle_manager.get_total_states());
    println!("Persistent states: {}", lifecycle_manager.get_persistent_states());
    
    // Use state 1
    {
        let state = lifecycle_manager.get_state(state1_id)?;
        let manager = state.lock().unwrap();
        let result = manager.execute_code("return 'State 1 active'")?;
        println!("State 1 result: {:?}", result);
    }
    
    // Get state info
    let state1_info = lifecycle_manager.get_state_info(state1_id)?;
    println!("State 1 usage count: {}", state1_info.usage_count);
    println!("State 1 idle time: {:?}", state1_info.idle_time);
    
    // Cleanup idle states (state3 should be cleaned up)
    let cleaned = lifecycle_manager.cleanup_idle_states()?;
    println!("Cleaned up {} idle states", cleaned);
    println!("Remaining states: {}", lifecycle_manager.get_total_states());
    
    Ok(())
}

/// Example 5: State pooling for concurrent execution
pub fn state_pooling_example() -> Result<()> {
    println!("\n=== State Pooling Example ===");
    
    // Create lifecycle manager
    let lifecycle_manager = std::sync::Arc::new(LuaStateLifecycleManager::new(
        Duration::from_millis(100),
        Duration::from_secs(5),
    ));
    
    // Create state pool
    let config = LuaStateConfig::default();
    let pool = LuaStatePool::new(config, 3, lifecycle_manager);
    
    println!("Initial pool size: {}", pool.size());
    
    // Acquire states
    let state1 = pool.acquire()?;
    let state2 = pool.acquire()?;
    
    println!("After acquiring 2 states: {}", pool.size());
    
    // Use states
    {
        let state = pool.lifecycle_manager.get_state(state1)?;
        let manager = state.lock().unwrap();
        let result = manager.execute_code("return 'Pool state 1'")?;
        println!("Pool state 1 result: {:?}", result);
    }
    
    // Release states back to pool
    pool.release(state1)?;
    pool.release(state2)?;
    
    println!("After releasing states: {}", pool.size());
    
    Ok(())
}

/// Example 6: Error handling and recovery
pub fn error_handling_example() -> Result<()> {
    println!("\n=== Error Handling Example ===");
    
    let config = LuaStateConfig::default();
    let state_manager = LuaStateManager::new(config)?;
    
    // Create error handler
    let error_handler = LuaErrorHandler::new()
        .with_stack_trace(true)
        .with_max_stack_depth(10);
    
    // Test syntax error
    match state_manager.execute_code("for i = 1, 10 do print(i") {
        Ok(_) => println!("ERROR: Syntax error was not caught!"),
        Err(e) => println!("Syntax error caught: {}", e),
    }
    
    // Test runtime error
    match state_manager.execute_code("local x = nil; return x.method()") {
        Ok(_) => println!("ERROR: Runtime error was not caught!"),
        Err(e) => println!("Runtime error caught: {}", e),
    }
    
    // Test recovery after error
    match state_manager.execute_code("return 'Recovered successfully'") {
        Ok(result) => println!("Recovery result: {:?}", result),
        Err(e) => println!("Recovery failed: {}", e),
    }
    
    Ok(())
}

/// Example 7: Custom security policies
pub fn custom_security_example() -> Result<()> {
    println!("\n=== Custom Security Policy Example ===");
    
    // Create custom security policy
    let mut security_policy = SecurityPolicy::default();
    
    // Allow specific modules
    security_policy.allowed_modules.insert("string".to_string());
    security_policy.allowed_modules.insert("math".to_string());
    
    // Allow specific OS operations
    security_policy.allowed_os_operations.clock = true;
    security_policy.allowed_os_operations.date = true;
    
    // Allow limited file operations
    security_policy.allowed_file_operations.read = true;
    security_policy.allowed_file_operations.allowed_paths.push(PathBuf::from("./safe"));
    
    // Create state with custom security
    let config = LuaStateConfig::default();
    let state_manager = LuaStateManager::new(config)?;
    
    // Apply custom security
    {
        let lua = state_manager.lua.lock().unwrap();
        let security_manager = SecurityManager::new(security_policy);
        security_manager.apply_sandbox(&lua)?;
    }
    
    // Test allowed operations
    match state_manager.execute_code("return string.upper('hello')") {
        Ok(result) => println!("String operation result: {:?}", result),
        Err(e) => println!("String operation failed: {}", e),
    }
    
    match state_manager.execute_code("return os.date()") {
        Ok(result) => println!("Date operation result: {:?}", result),
        Err(e) => println!("Date operation failed: {}", e),
    }
    
    // Test denied operations
    match state_manager.execute_code("return io.open('test.txt', 'r')") {
        Ok(_) => println!("ERROR: File operation was allowed!"),
        Err(e) => println!("Good: File operation was denied: {}", e),
    }
    
    Ok(())
}

/// Run all examples
pub fn run_all_examples() -> Result<()> {
    println!("Running Lua State Management Examples\n");
    
    basic_execution_example()?;
    security_sandboxing_example()?;
    resource_limits_example()?;
    lifecycle_management_example()?;
    state_pooling_example()?;
    error_handling_example()?;
    custom_security_example()?;
    
    println!("\n=== All Examples Completed ===");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_execution() {
        assert!(basic_execution_example().is_ok());
    }

    #[test]
    fn test_security_sandboxing() {
        assert!(security_sandboxing_example().is_ok());
    }

    #[test]
    fn test_resource_limits() {
        assert!(resource_limits_example().is_ok());
    }

    #[test]
    fn test_lifecycle_management() {
        assert!(lifecycle_management_example().is_ok());
    }

    #[test]
    fn test_error_handling() {
        assert!(error_handling_example().is_ok());
    }

    #[test]
    fn test_custom_security() {
        assert!(custom_security_example().is_ok());
    }
}