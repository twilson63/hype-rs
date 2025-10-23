use mlua::prelude::*;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct DebugInfo {
    pub start_time: Instant,
    pub enabled: bool,
    pub verbose: bool,
}

impl DebugInfo {
    pub fn new(enabled: bool, verbose: bool) -> Self {
        Self {
            start_time: Instant::now(),
            enabled,
            verbose,
        }
    }

    pub fn print(&self, msg: &str) {
        if self.enabled {
            eprintln!("[DEBUG] {}", msg);
        }
    }

    pub fn print_verbose(&self, msg: &str) {
        if self.verbose {
            eprintln!("[VERBOSE] {}", msg);
        }
    }

    pub fn elapsed(&self) -> std::time::Duration {
        self.start_time.elapsed()
    }
}

pub struct DebugModule;

impl DebugModule {
    pub fn register(lua: &Lua, debug_info: &DebugInfo) -> LuaResult<()> {
        let globals = lua.globals();

        let debug_table = lua.create_table()?;

        debug_table.set("enabled", debug_info.enabled)?;
        debug_table.set("verbose", debug_info.verbose)?;

        let print_fn = lua.create_function(|_, msg: String| {
            eprintln!("[DEBUG] {}", msg);
            Ok(())
        })?;

        debug_table.set("print", print_fn)?;

        let info_fn = lua.create_function(|_, msg: String| {
            eprintln!("[INFO] {}", msg);
            Ok(())
        })?;

        debug_table.set("info", info_fn)?;

        globals.set("debug", debug_table)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_info_creation() {
        let debug = DebugInfo::new(true, false);
        assert!(debug.enabled);
        assert!(!debug.verbose);
    }

    #[test]
    fn test_debug_info_elapsed() {
        let debug = DebugInfo::new(true, false);
        std::thread::sleep(std::time::Duration::from_millis(10));
        let elapsed = debug.elapsed();
        assert!(elapsed.as_millis() >= 10);
    }
}
