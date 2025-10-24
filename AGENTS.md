# Agent Guidelines for hype-rs

## Build & Test Commands

**Build**: `cargo build`  
**Release Build**: `cargo build --release`  
**Run Tests**: `cargo test`  
**Run Single Test**: `cargo test test_name -- --nocapture`  
**Run Test Subset**: `cargo test module_name`  
**Lint**: `cargo clippy -- -D warnings`  
**Format Check**: `cargo fmt -- --check`  
**Format Fix**: `cargo fmt`  
**Run Benchmarks**: `cargo bench`

## Code Style Guidelines

### Imports
- Organize by: std library, external crates, internal modules (with `crate::`)
- Use `use` statements at the top of files, group related imports
- Example: `use crate::error::{HypeError, Result};`

### Formatting & Types
- ALWAYS run `cargo fmt` to auto-format code
- Explicit type annotations for function parameters and return types
- Use strong typing: enums for error types (`HypeError`, `FileError`, `ValidationError`)
- Custom `Result<T> = std::result::Result<T, HypeError>` type alias pattern
- Implement `fmt::Display` for user-facing error messages

### Naming Conventions
- Functions: `snake_case` (e.g., `execute_script`, `create_cli_config`)
- Types/Structs/Enums: `PascalCase` (e.g., `LuaStateManager`, `HypeError`)
- Constants: `SCREAMING_SNAKE_CASE`
- Module names: `snake_case`

### Error Handling
- Define custom error enums with `#[derive(Debug)]`
- Implement `std::error::Error` trait for custom errors
- Use `?` operator for error propagation
- Return `Result<T>` for all fallible operations (use crate's `Result` type alias)
- Provide context with detailed error messages

### Module Structure
- Organize by feature domain: `cli/`, `lua/`, `engine/`, `error/`, `file_io/`, `config/`, `modules/`
- Use `mod.rs` to expose public APIs with `pub use` re-exports
- Keep modules focused and single-responsibility
- Submodules: declare in parent `mod.rs` with `pub mod submodule;`

### Comments
- NO COMMENTS unless absolutely necessary for non-obvious logic
- Use doc comments (`///`) for public APIs only
- Prefer self-documenting code with clear naming

## Key Architecture Patterns

- **Lua runtime**: uses `mlua` crate with Lua 5.4 (vendored)
- **CLI**: uses `clap` with derive macros for argument parsing
- **Error handling**: comprehensive custom error types with context propagation
- **State management**: `LuaStateManager` with security policies and configs
- **Module system**: custom `require()` implementation with caching and resolution

## Dependencies
- **mlua** (0.9): Lua binding with Lua 5.4 vendored
- **clap** (4.4): CLI argument parsing with derive
- **serde/serde_json**: Config and manifest serialization
- **anyhow**, **regex**, **tempfile**
- **tokio** (optional feature "async")
