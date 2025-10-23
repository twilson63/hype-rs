# Agent Guidelines for hype-rs

## Build & Test Commands

**Build**: `cargo build`  
**Release Build**: `cargo build --release`  
**Run Tests**: `cargo test`  
**Run Single Test**: `cargo test <test_name> -- --nocapture`  
**Lint**: `cargo clippy -- -D warnings`  
**Format Check**: `cargo fmt -- --check`  
**Format Fix**: `cargo fmt`  

## Code Style Guidelines

### Imports
- Organize by: std library, external crates, internal modules (with `crate::`)
- Use `use` statements at the top of files
- Group related imports together

### Formatting & Types
- Run `cargo fmt` to auto-format code
- Explicit type annotations for function parameters and return types
- Use strong typing: enums for error types (`HypeError`, `FileError`, `ValidationError`)
- Custom `Result<T> = std::result::Result<T, HypeError>` type alias pattern

### Naming Conventions
- Functions: `snake_case`
- Types/Structs/Enums: `PascalCase`
- Constants: `SCREAMING_SNAKE_CASE`
- Module names: `snake_case`

### Error Handling
- Define custom error enums with `#[derive(Debug)]`
- Implement `fmt::Display` for user-friendly messages
- Use `?` operator for error propagation
- Return `HypeError` wrapped in `Result<T>` for all fallible operations

### Module Structure
- Organize by feature domain: `cli/`, `lua/`, `engine/`, `error/`, `file_io/`, `config/`
- Use `mod.rs` to expose public APIs
- Keep modules focused and single-responsibility

## Key Architecture Patterns

- Lua runtime: uses `mlua` crate with Lua 5.4 (vendored)
- CLI: uses `clap` with derive macros for argument parsing
- Error handling: comprehensive custom error types with context
- No comments unless absolutely necessary for non-obvious logic

## Dependencies
- **mlua** (0.9): Lua binding with Lua 5.4 vendored
- **clap** (4.4): CLI argument parsing with derive
- **anyhow**, **serde**, **tokio** (optional), **tempfile**, **regex**
