use crate::error::HypeError;
use mlua::{Error as LuaError, Value};
use std::backtrace::Backtrace;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub enum LuaExecutionError {
    SyntaxError {
        message: String,
        line: Option<i32>,
        column: Option<i32>,
        source: Option<String>,
    },
    RuntimeError {
        message: String,
        stack_trace: Vec<String>,
        source: Option<String>,
    },
    MemoryError {
        message: String,
        limit: Option<usize>,
        usage: Option<usize>,
    },
    TimeoutError {
        message: String,
        timeout: std::time::Duration,
        elapsed: std::time::Duration,
    },
    InstructionLimitError {
        message: String,
        limit: u32,
        executed: u32,
    },
    SecurityError {
        message: String,
        operation: String,
        reason: String,
    },
    EnvironmentError {
        message: String,
        operation: String,
        variable_name: String,
        reason: String,
    },
    PanicError {
        message: String,
        backtrace: Backtrace,
    },
    UnknownError {
        message: String,
        original: String,
    },
}

impl LuaExecutionError {
    pub fn from_lua_error(error: LuaError) -> Self {
        match error {
            LuaError::SyntaxError {
                message,
                incomplete_input: _,
            } => {
                let (line, column) = Self::parse_syntax_position(&message);
                Self::SyntaxError {
                    message,
                    line,
                    column,
                    source: None,
                }
            }
            LuaError::RuntimeError(message) => {
                Self::RuntimeError {
                    message,
                    stack_trace: Vec::new(), // Will be populated separately
                    source: None,
                }
            }
            LuaError::MemoryError(message) => Self::MemoryError {
                message,
                limit: None,
                usage: None,
            },
            _ => Self::UnknownError {
                message: error.to_string(),
                original: format!("{:?}", error),
            },
        }
    }

    fn parse_syntax_position(message: &str) -> (Option<i32>, Option<i32>) {
        // Try to extract line and column from error message
        // Format varies by Lua version, but common patterns exist
        let re =
            regex::Regex::new(r#"(\[string "[^"]+"\]:(\d+):(\d+))|(\[string "[^"]+"\]:(\d+))"#)
                .ok();

        if let Some(caps) = re.and_then(|r| r.captures(message)) {
            let line = caps
                .get(2)
                .or_else(|| caps.get(5))
                .and_then(|m| m.as_str().parse().ok());
            let column = caps.get(3).and_then(|m| m.as_str().parse().ok());
            (line, column)
        } else {
            (None, None)
        }
    }

    pub fn with_stack_trace(mut self, stack_trace: Vec<String>) -> Self {
        if let Self::RuntimeError {
            stack_trace: ref mut st,
            ..
        } = self
        {
            *st = stack_trace;
        }
        self
    }

    pub fn with_source(mut self, source: String) -> Self {
        match &mut self {
            Self::SyntaxError {
                source: ref mut s, ..
            } => *s = Some(source),
            Self::RuntimeError {
                source: ref mut s, ..
            } => *s = Some(source),
            _ => {}
        }
        self
    }

    pub fn error_type(&self) -> &'static str {
        match self {
            Self::SyntaxError { .. } => "SyntaxError",
            Self::RuntimeError { .. } => "RuntimeError",
            Self::MemoryError { .. } => "MemoryError",
            Self::TimeoutError { .. } => "TimeoutError",
            Self::InstructionLimitError { .. } => "InstructionLimitError",
            Self::SecurityError { .. } => "SecurityError",
            Self::EnvironmentError { .. } => "EnvironmentError",
            Self::PanicError { .. } => "PanicError",
            Self::UnknownError { .. } => "UnknownError",
        }
    }

    pub fn user_friendly_message(&self) -> String {
        match self {
            Self::SyntaxError {
                message,
                line,
                column,
                ..
            } => {
                if let (Some(line), Some(column)) = (line, column) {
                    format!(
                        "Syntax error at line {}, column {}: {}",
                        line, column, message
                    )
                } else if let Some(line) = line {
                    format!("Syntax error at line {}: {}", line, message)
                } else {
                    format!("Syntax error: {}", message)
                }
            }
            Self::RuntimeError { message, .. } => {
                format!("Runtime error: {}", message)
            }
            Self::MemoryError {
                message,
                limit,
                usage,
            } => {
                if let (Some(limit), Some(usage)) = (limit, usage) {
                    format!(
                        "Memory error: {} (used: {} bytes, limit: {} bytes)",
                        message, usage, limit
                    )
                } else {
                    format!("Memory error: {}", message)
                }
            }
            Self::TimeoutError {
                timeout, elapsed, ..
            } => {
                format!(
                    "Script execution timed out after {:?} (limit: {:?})",
                    elapsed, timeout
                )
            }
            Self::InstructionLimitError {
                limit, executed, ..
            } => {
                format!("Instruction limit exceeded: {} > {}", executed, limit)
            }
            Self::SecurityError {
                operation, reason, ..
            } => {
                format!("Security violation in {}: {}", operation, reason)
            }
            Self::EnvironmentError {
                operation,
                variable_name,
                reason,
                ..
            } => {
                format!(
                    "Environment variable error in {} '{}': {}",
                    operation, variable_name, reason
                )
            }
            Self::PanicError { message, .. } => {
                format!("Lua panic: {}", message)
            }
            Self::UnknownError { message, .. } => {
                format!("Unknown Lua error: {}", message)
            }
        }
    }

    pub fn detailed_message(&self) -> String {
        let mut msg = self.user_friendly_message();

        match self {
            Self::RuntimeError { stack_trace, .. } => {
                if !stack_trace.is_empty() {
                    msg.push_str("\n\nStack trace:\n");
                    for (i, frame) in stack_trace.iter().enumerate() {
                        msg.push_str(&format!("  {}. {}\n", i + 1, frame));
                    }
                }
            }
            Self::SyntaxError { source, .. } => {
                if let Some(source) = source {
                    msg.push_str(&format!("\nSource: {}", source));
                }
            }
            _ => {}
        }

        msg
    }
}

impl fmt::Display for LuaExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.user_friendly_message())
    }
}

impl std::error::Error for LuaExecutionError {}

impl From<LuaExecutionError> for HypeError {
    fn from(error: LuaExecutionError) -> Self {
        HypeError::Lua(error.detailed_message())
    }
}

pub struct LuaErrorHandler {
    capture_stack_trace: bool,
    max_stack_depth: usize,
}

impl LuaErrorHandler {
    pub fn new() -> Self {
        Self {
            capture_stack_trace: true,
            max_stack_depth: 20,
        }
    }

    pub fn with_stack_trace(mut self, capture: bool) -> Self {
        self.capture_stack_trace = capture;
        self
    }

    pub fn with_max_stack_depth(mut self, depth: usize) -> Self {
        self.max_stack_depth = depth;
        self
    }

    pub fn handle_error(&self, error: LuaError, lua: &mlua::Lua) -> LuaExecutionError {
        let mut exec_error = LuaExecutionError::from_lua_error(error);

        if self.capture_stack_trace {
            if let Ok(stack_trace) = self.capture_stack_trace(lua) {
                exec_error = exec_error.with_stack_trace(stack_trace);
            }
        }

        exec_error
    }

    fn capture_stack_trace(&self, lua: &mlua::Lua) -> mlua::Result<Vec<String>> {
        let mut stack_trace = Vec::new();

        // Try to get stack trace using debug.traceback if available
        if let Ok(debug) = lua.globals().get::<_, mlua::Table>("debug") {
            if let Ok(traceback) = debug.get::<_, mlua::Function>("traceback") {
                if let Ok(trace) = traceback.call::<_, String>(()) {
                    for line in trace.lines().skip(1).take(self.max_stack_depth) {
                        stack_trace.push(line.trim().to_string());
                    }
                }
            }
        }

        // Fallback: try to get stack info manually - simplified due to API limitations
        if stack_trace.is_empty() {
            for level in 1..=self.max_stack_depth {
                if let Some(_info) = lua.inspect_stack(level) {
                    // Simplified stack trace due to mlua API limitations
                    stack_trace.push(format!("Level {}", level));
                } else {
                    break;
                }
            }
        }

        Ok(stack_trace)
    }

    pub fn create_panic_recovery_function<'a>(
        &self,
        lua: &'a mlua::Lua,
    ) -> mlua::Result<mlua::Function<'a>> {
        lua.create_function(move |_lua, ()| {
            // This function can be called to recover from panics
            Ok("panic recovered")
        })
    }
}

impl Default for LuaErrorHandler {
    fn default() -> Self {
        Self::new()
    }
}

// Utility functions for error handling
pub fn format_lua_value(value: &Value) -> String {
    match value {
        Value::Nil => "nil".to_string(),
        Value::Boolean(b) => b.to_string(),
        Value::Integer(i) => i.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => format!("\"{}\"", s.to_string_lossy()),
        Value::Table(_) => "<table>".to_string(),
        Value::Function(_) => "<function>".to_string(),
        Value::LightUserData(_) => "<lightuserdata>".to_string(),
        Value::UserData(_) => "<userdata>".to_string(),
        Value::Thread(_) => "<thread>".to_string(),
        Value::Error(e) => format!("<error: {}>", e),
    }
}

pub fn extract_error_context(error: &LuaExecutionError) -> HashMap<String, String> {
    let mut context = HashMap::new();

    context.insert("error_type".to_string(), error.error_type().to_string());
    context.insert("message".to_string(), error.user_friendly_message());

    match error {
        LuaExecutionError::SyntaxError {
            line,
            column,
            source,
            ..
        } => {
            if let Some(line) = line {
                context.insert("line".to_string(), line.to_string());
            }
            if let Some(column) = column {
                context.insert("column".to_string(), column.to_string());
            }
            if let Some(source) = source {
                context.insert("source".to_string(), source.clone());
            }
        }
        LuaExecutionError::RuntimeError { stack_trace, .. } => {
            context.insert("stack_depth".to_string(), stack_trace.len().to_string());
        }
        LuaExecutionError::MemoryError { limit, usage, .. } => {
            if let Some(limit) = limit {
                context.insert("memory_limit".to_string(), limit.to_string());
            }
            if let Some(usage) = usage {
                context.insert("memory_usage".to_string(), usage.to_string());
            }
        }
        LuaExecutionError::TimeoutError {
            timeout, elapsed, ..
        } => {
            context.insert("timeout_ms".to_string(), timeout.as_millis().to_string());
            context.insert("elapsed_ms".to_string(), elapsed.as_millis().to_string());
        }
        LuaExecutionError::InstructionLimitError {
            limit, executed, ..
        } => {
            context.insert("instruction_limit".to_string(), limit.to_string());
            context.insert("instructions_executed".to_string(), executed.to_string());
        }
        LuaExecutionError::SecurityError {
            operation, reason, ..
        } => {
            context.insert("operation".to_string(), operation.clone());
            context.insert("reason".to_string(), reason.clone());
        }
        LuaExecutionError::EnvironmentError {
            operation,
            variable_name,
            reason,
            ..
        } => {
            context.insert("operation".to_string(), operation.clone());
            context.insert("variable_name".to_string(), variable_name.clone());
            context.insert("reason".to_string(), reason.clone());
        }
        _ => {}
    }

    context
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_parsing() {
        let error = LuaError::SyntaxError {
            message: "syntax error near 'end'".to_string(),
            incomplete_input: false,
        };

        let exec_error = LuaExecutionError::from_lua_error(error);
        assert!(matches!(exec_error, LuaExecutionError::SyntaxError { .. }));
    }

    #[test]
    fn test_error_messages() {
        let error = LuaExecutionError::RuntimeError {
            message: "test error".to_string(),
            stack_trace: vec!["frame1".to_string(), "frame2".to_string()],
            source: None,
        };

        let friendly = error.user_friendly_message();
        assert!(friendly.contains("test error"));

        let detailed = error.detailed_message();
        assert!(detailed.contains("Stack trace"));
    }
}
