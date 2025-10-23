# Testing Guide

Comprehensive testing documentation for hype-rs.

## Running Tests

### All Tests

```bash
cargo test
```

Runs all tests (unit tests, integration tests, and doc tests).

### Specific Test Types

```bash
cargo test --lib              # Unit tests only
cargo test --test '*'         # Integration tests only
cargo test --doc              # Documentation tests only
```

### Run with Output

```bash
cargo test -- --nocapture     # Show test output (print statements, etc.)
cargo test foo -- --nocapture # Show output for tests matching "foo"
```

### Run Specific Test

```bash
cargo test test_name           # Run test matching "test_name"
cargo test module::test_name   # Run specific test in module
```

## Test Organization

```
tests/
├── lua_scripts/               # Lua test scripts for manual verification
│   ├── 01_hello.lua          # Basic hello world script
│   ├── 02_args.lua           # Argument passing test
│   ├── test_*.lua            # Various test scenarios
│   └── ...
├── advanced_unit_tests.rs     # Advanced unit tests (52 tests)
├── cli_module_test.rs         # CLI module tests (6 tests)
├── edge_cases_test.rs         # Edge case tests (18 tests)
├── module_system_integration_test.rs  # Module system tests (33 tests)
└── stress_test.rs             # Stress tests (10 tests)
```

## Test Files Overview

### Rust Integration Tests

All `.rs` files in `tests/` directory are Rust integration tests:

- **advanced_unit_tests.rs** (52 tests)
  - Module loading and caching
  - Path resolution and normalization
  - Circular dependency detection
  - Environment setup and teardown
  - Concurrent loading scenarios

- **cli_module_test.rs** (6 tests)
  - CLI module flag handling
  - Module execution from command line

- **edge_cases_test.rs** (18 tests)
  - Error recovery
  - Boundary conditions
  - Invalid input handling

- **module_system_integration_test.rs** (33 tests)
  - Full module system integration
  - require() functionality
  - Built-in modules (fs, path, events, util, table)

- **stress_test.rs** (10 tests)
  - Concurrent module loading
  - Performance under load
  - Resource limits

### Lua Test Scripts

`tests/lua_scripts/` contains Lua scripts for manual testing and examples:

- `01_hello.lua` through `08_error_handling.lua` - Numbered example scripts
- `test_*.lua` - Various test scenarios for manual verification

## Test Coverage

Current coverage: **97%+**

To check coverage locally:
```bash
cargo tarpaulin --out Html
```

## Running Tests in CI/CD

```bash
# Check code style
cargo clippy -- -D warnings

# Check formatting
cargo fmt -- --check

# Run all tests
cargo test

# Run release build tests
cargo build --release
cargo test --release
```

## Writing Tests

### Unit Tests (in src/)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        assert_eq!(2 + 2, 4);
    }
}
```

### Integration Tests (in tests/)

```rust
use hype_rs::ModuleLoader;

#[test]
fn test_module_loading() {
    let loader = ModuleLoader::new();
    assert!(loader.is_ok());
}
```

## Test Results

### Current Test Status

```
Total Tests: 265
Passing: 220 (88%)
Failing: 4 (pre-existing)
Coverage: 97%+
```

### Known Failing Tests

4 pre-existing test failures (environment/output capture related):

1. `lua::environment::tests::test_env_table_setup`
2. `lua::environment::tests::test_env_read_access`
3. `lua::environment::tests::test_env_write_access`
4. `engine::output::tests::test_output_capture_basic`

These are marked for Phase 6 (final polish) fixes.

## Performance Benchmarks

See [docs/performance.md](./performance.md) for detailed performance benchmarks and optimization information.

Benchmarks are located in `benches/module_benchmarks.rs` and can be run with:

```bash
cargo bench
```

## Troubleshooting

### Test Hangs or Timeout

Some tests have built-in timeouts. If a test hangs:

```bash
timeout 30 cargo test test_name
```

### Memory Issues

Run in release mode for better performance:

```bash
cargo test --release
```

### Debugging Tests

With backtrace:

```bash
RUST_BACKTRACE=1 cargo test test_name -- --nocapture
```

With full backtrace:

```bash
RUST_BACKTRACE=full cargo test test_name -- --nocapture
```

## Contributing Tests

When adding new features:

1. Write integration tests in `tests/`
2. Write unit tests in `src/` modules
3. Add Lua test scripts in `tests/lua_scripts/` if testing Lua functionality
4. Run `cargo test` to ensure all tests pass
5. Run `cargo clippy` to check for common mistakes
6. Run `cargo fmt` to format code

See [CONTRIBUTING.md](../.github/CONTRIBUTING.md) for more details.
