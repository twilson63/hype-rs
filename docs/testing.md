# Testing Strategy for Hype-RS

A comprehensive guide to testing Hype-RS modules, applications, and the runtime itself. This document covers testing approaches, organization, tools, and best practices.

## Table of Contents

- [Overview](#overview)
- [Testing Types](#testing-types)
- [Test Organization](#test-organization)
- [Running Tests](#running-tests)
- [Writing Tests](#writing-tests)
- [Coverage Reports](#coverage-reports)
- [Adding New Tests](#adding-new-tests)
- [CI/CD Integration](#cicd-integration)
- [Best Practices](#best-practices)

---

## Overview

The Hype-RS project uses a multi-layered testing strategy:

1. **Unit Tests**: Test individual functions and modules
2. **Integration Tests**: Test module interactions
3. **End-to-End Tests**: Test complete applications
4. **Stress Tests**: Test performance and limits
5. **Security Tests**: Verify sandbox behavior

### Testing Goals

✅ Catch bugs early  
✅ Ensure reliability  
✅ Document expected behavior  
✅ Enable confident refactoring  
✅ Prevent regressions  
✅ Maintain code quality  

### Test Coverage Targets

- Core modules: **95%+**
- Builtin modules: **90%+**
- Error handling: **100%**
- Edge cases: **85%+**
- Overall: **90%+**

---

## Testing Types

### 1. Unit Tests

Test individual functions in isolation.

**Characteristics:**
- Test one thing at a time
- No external dependencies
- Fast execution
- Deterministic results

**Example:**

```lua
-- test_math_utils.lua
local math_utils = require("math_utils")

assert(math_utils.add(2, 3) == 5, "add function failed")
assert(math_utils.multiply(4, 5) == 20, "multiply function failed")
print("✓ All unit tests passed")
```

### 2. Integration Tests

Test how modules work together.

**Characteristics:**
- Test module interactions
- Use multiple modules
- May access file system
- Realistic workflows

**Example:**

```lua
-- test_file_processing.lua
local fs = require("fs")
local path = require("path")
local string_utils = require("string_utils")

local content = fs.readFileSync("data.txt")
local lines = string_utils.split(content, "\n")
assert(#lines > 0, "file processing failed")
print("✓ Integration test passed")
```

### 3. End-to-End Tests

Test complete application workflows.

**Characteristics:**
- Test full user flows
- Include all system components
- Verify final output
- Check side effects (files created, etc.)

**Example:**

```lua
-- test_app_workflow.lua
local fs = require("fs")
local app = require("app")

app.run()

assert(fs.existsSync("output.txt"), "app did not create output")
local content = fs.readFileSync("output.txt")
assert(#content > 0, "output is empty")
print("✓ Application workflow test passed")
```

### 4. Stress Tests

Test performance, limits, and edge cases.

**Characteristics:**
- Large inputs
- Boundary conditions
- Performance benchmarks
- Resource usage

**Example:**

```lua
-- test_stress.lua
local math_utils = require("math_utils")

-- Test with large numbers
local start_time = os.time()
for i = 1, 10000 do
    math_utils.add(i, i)
end
local elapsed = os.time() - start_time
print("✓ 10000 additions completed in", elapsed, "seconds")
```

### 5. Security Tests

Verify sandbox restrictions work.

**Characteristics:**
- Test restricted operations
- Verify error handling
- Check resource limits
- Validate permissions

**Example:**

```lua
-- test_security.lua
local ok, err = pcall(function()
    os.execute("rm -rf /")  -- Should fail
end)
assert(not ok, "sandbox bypass detected")
print("✓ Security restrictions working")
```

---

## Test Organization

### Directory Structure

```
hype-rs/
├── tests/
│   ├── unit/
│   │   ├── test_fs_module.lua
│   │   ├── test_path_module.lua
│   │   ├── test_events_module.lua
│   │   └── test_custom_module.lua
│   ├── integration/
│   │   ├── test_fs_path_interaction.lua
│   │   ├── test_module_composition.lua
│   │   └── test_require_system.lua
│   ├── e2e/
│   │   ├── test_app_basic.lua
│   │   ├── test_app_file_ops.lua
│   │   └── test_app_complex.lua
│   ├── stress/
│   │   ├── test_large_files.lua
│   │   ├── test_many_modules.lua
│   │   └── test_performance.lua
│   ├── security/
│   │   ├── test_sandbox_file_access.lua
│   │   ├── test_sandbox_os_access.lua
│   │   └── test_resource_limits.lua
│   ├── fixtures/
│   │   ├── sample_files/
│   │   ├── test_data.json
│   │   └── test_config.lua
│   └── run_all_tests.lua
├── examples/
│   ├── basic-app.lua
│   ├── file-operations.lua
│   └── package-app/
│       ├── app.lua
│       └── modules/
└── benches/
    └── module_benchmarks.rs
```

### Test File Naming

- Unit tests: `test_<module_name>.lua`
- Integration tests: `test_<interaction>.lua`
- E2E tests: `test_app_<feature>.lua`
- Stress tests: `test_stress_<type>.lua`
- Security tests: `test_security_<aspect>.lua`

---

## Running Tests

### Run All Tests

```bash
cargo test
```

Runs all Rust tests and Lua test files.

### Run Specific Test Suite

```bash
cargo test lua_scripts
```

Tests only Lua script files.

### Run Integration Tests

```bash
cargo test integration
```

Runs integration tests.

### Run Tests with Output

```bash
cargo test -- --nocapture
```

Shows print statements during tests.

### Run Single Test

```bash
cargo test test_fs_module -- --nocapture
```

Run a specific test and show output.

### Continuous Testing

```bash
cargo watch -x test
```

Re-run tests whenever files change.

### Run Lua Tests Directly

```bash
hype tests/unit/test_fs_module.lua
hype tests/integration/test_require_system.lua
hype tests/e2e/test_app_basic.lua
```

---

## Writing Tests

### Test Framework

Hype-RS uses assertion-based testing with `assert()`:

```lua
-- Basic assertion
assert(value, "error message")

-- Testing equality
assert(result == expected, "values not equal")

-- Testing conditions
assert(result > 0, "result should be positive")
assert(not error_occurred, "error was raised")

-- Multiple assertions
function test_math_operations()
    assert(add(2, 3) == 5)
    assert(subtract(5, 2) == 3)
    assert(multiply(4, 5) == 20)
end
```

### Test Structure

Follow this pattern for each test file:

```lua
-- tests/unit/test_example.lua
-- Description of what this tests

local function setup()
    -- Initialize test data
    return {}
end

local function test_example_case_1()
    local data = setup()
    -- Arrange
    local input = 42
    
    -- Act
    local result = some_function(input)
    
    -- Assert
    assert(result == expected_value, "result mismatch")
end

local function test_example_case_2()
    local data = setup()
    -- Another test case
    assert(true, "test case 2")
end

-- Run all tests
local tests = {
    test_example_case_1,
    test_example_case_2,
}

print("Running tests...")
for _, test in ipairs(tests) do
    local ok, err = pcall(test)
    if ok then
        print("✓", test)
    else
        print("✗", test, ":", err)
    end
end
```

### Testing Error Cases

```lua
local function test_error_handling()
    local ok, err = pcall(function()
        require("nonexistent-module")
    end)
    
    assert(not ok, "should have raised an error")
    assert(err:find("Unknown built-in module"), "wrong error message")
end
```

### Testing File Operations

```lua
local function test_file_operations()
    local fs = require("fs")
    local path = require("path")
    
    local test_file = path.join("tests/fixtures", "test.txt")
    local content = "test content"
    
    -- Test write
    fs.writeFileSync(test_file, content)
    assert(fs.existsSync(test_file), "file not created")
    
    -- Test read
    local read_content = fs.readFileSync(test_file)
    assert(read_content == content, "content mismatch")
    
    -- Cleanup
    fs.unlinkSync(test_file)
    assert(not fs.existsSync(test_file), "file not deleted")
end
```

### Testing Module Loading

```lua
local function test_module_loading()
    -- First load
    local fs1 = require("fs")
    assert(fs1 ~= nil, "fs module not loaded")
    
    -- Second load (should be cached)
    local fs2 = require("fs")
    assert(fs1 == fs2, "module not cached")
    
    -- Custom module
    local custom = require("modules/math_utils")
    assert(custom.add, "function not exported")
end
```

---

## Coverage Reports

### Generating Coverage

With Rust tests:

```bash
cargo tarpaulin --out Html
```

Generates `tarpaulin-report.html` with coverage data.

### Coverage Targets by Component

**Core Modules (fs, path, events)**: 95%+
```
- All public functions
- All error paths
- Edge cases
- Permission errors
```

**Module System**: 90%+
```
- require() resolution
- Circular dependency detection
- Module caching
- __dirname and __filename
```

**Error Handling**: 100%
```
- All error types
- Error messages
- Stack traces
```

**Builtin Functions**: 85%+
```
- Normal usage
- Edge cases
- Invalid inputs
```

### Viewing Coverage

```bash
# Generate and view coverage
cargo tarpaulin --out Html && open tarpaulin-report.html

# Coverage summary
cargo tarpaulin --out Stdout
```

### Coverage by File

```
src/lua/require.rs: 94%
src/lua/env.rs: 91%
src/modules/builtins/fs.rs: 96%
src/modules/builtins/path.rs: 93%
src/modules/loader.rs: 87%
src/error/mod.rs: 100%
tests/: 100%
```

---

## Adding New Tests

### Step 1: Create Test File

Create `tests/unit/test_new_feature.lua`:

```lua
-- Test for new feature
-- Covers functionality of the new feature

local function test_new_feature_basic()
    -- Test basic functionality
    assert(true, "basic test")
end

local function test_new_feature_edge_case()
    -- Test edge cases
    assert(true, "edge case test")
end

-- Run tests
local tests = {
    test_new_feature_basic,
    test_new_feature_edge_case,
}

local passed = 0
local failed = 0

for _, test in ipairs(tests) do
    local ok, err = pcall(test)
    if ok then
        print("✓", test)
        passed = passed + 1
    else
        print("✗", test, ":", err)
        failed = failed + 1
    end
end

print("\nResults:", passed, "passed,", failed, "failed")
assert(failed == 0, "some tests failed")
```

### Step 2: Add to Test Suite

Update `tests/run_all_tests.lua`:

```lua
local tests_to_run = {
    "tests/unit/test_fs_module.lua",
    "tests/unit/test_path_module.lua",
    "tests/unit/test_new_feature.lua",  -- Add here
    -- ... more tests
}
```

### Step 3: Run and Verify

```bash
hype tests/unit/test_new_feature.lua
hype tests/run_all_tests.lua
cargo test
```

### Step 4: Add to CI/CD

If using GitHub Actions, update `.github/workflows/test.yml`:

```yaml
- name: Run new feature tests
  run: hype tests/unit/test_new_feature.lua
```

---

## CI/CD Integration

### GitHub Actions Example

`.github/workflows/test.yml`:

```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v2
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Run tests
        run: cargo test --verbose
      
      - name: Run integration tests
        run: hype tests/integration/test_module_composition.lua
      
      - name: Generate coverage
        run: cargo tarpaulin --out Xml
      
      - name: Upload coverage
        uses: codecov/codecov-action@v2
        with:
          files: ./cobertura.xml
```

### Pre-commit Hook

`.git/hooks/pre-commit`:

```bash
#!/bin/bash
echo "Running tests before commit..."
cargo test --lib || exit 1
hype tests/run_all_tests.lua || exit 1
echo "✓ All tests passed"
```

---

## Best Practices

### 1. Test One Thing Per Test

```lua
-- ✗ Bad: Testing multiple things
function test_string_utils()
    assert(uppercase("hello") == "HELLO")
    assert(lowercase("HELLO") == "hello")
    assert(trim("  hello  ") == "hello")
end

-- ✓ Good: Separate tests
function test_uppercase()
    assert(uppercase("hello") == "HELLO")
end

function test_lowercase()
    assert(lowercase("HELLO") == "hello")
end

function test_trim()
    assert(trim("  hello  ") == "hello")
end
```

### 2. Use Descriptive Names

```lua
-- ✗ Bad
function test1()
    assert(add(2, 3) == 5)
end

-- ✓ Good
function test_add_positive_numbers()
    assert(add(2, 3) == 5)
end

function test_add_negative_numbers()
    assert(add(-2, -3) == -5)
end
```

### 3. Clean Up After Tests

```lua
local function test_file_operations()
    local fs = require("fs")
    local test_file = "test_temp.txt"
    
    -- Create file
    fs.writeFileSync(test_file, "content")
    assert(fs.existsSync(test_file))
    
    -- Clean up
    fs.unlinkSync(test_file)
    assert(not fs.existsSync(test_file))
end
```

### 4. Test Error Messages

```lua
local function test_error_message_clarity()
    local ok, err = pcall(function()
        require("nonexistent")
    end)
    
    assert(not ok)
    assert(err:find("nonexistent"), "error should mention module name")
end
```

### 5. Use Setup and Teardown

```lua
local function setup()
    return {
        temp_dir = "tests/temp",
        test_file = "tests/temp/test.txt",
    }
end

local function teardown(context)
    local fs = require("fs")
    if fs.existsSync(context.test_file) then
        fs.unlinkSync(context.test_file)
    end
end

local function test_something(context)
    -- Use context.temp_dir
    assert(true)
    teardown(context)
end
```

### 6. Document Complex Tests

```lua
-- Test module circular dependency detection
-- Scenario: Module A requires Module B, Module B requires Module A
-- Expected: Error is raised with clear message
-- See: docs/module-system.md for more info
local function test_circular_dependency_detection()
    local ok, err = pcall(function()
        require("circular_a")
    end)
    
    assert(not ok, "should detect circular dependency")
    assert(err:find("Circular dependency"), "wrong error type")
end
```

### 7. Test Performance Baselines

```lua
local function test_performance_baseline()
    local math_utils = require("math_utils")
    
    local start = os.time()
    for i = 1, 100000 do
        math_utils.add(i, i)
    end
    local elapsed = (os.time() - start) * 1000  -- ms
    
    -- 100k additions should complete in < 500ms
    assert(elapsed < 500, "performance regression: " .. elapsed .. "ms")
end
```

---

## Debugging Tests

### Enable Verbose Output

```bash
hype tests/unit/test_example.lua
# or
cargo test -- --nocapture
```

### Print Debug Information

```lua
local function test_something()
    local result = some_function()
    print("DEBUG: result =", result)
    print("DEBUG: type =", type(result))
    assert(result == expected)
end
```

### Use debugger

```lua
-- Add breakpoint
debug.debug()

-- Show stack trace
debug.traceback()
```

### Test Individual Functions

```lua
-- Instead of running full test suite:
hype tests/unit/test_math_utils.lua

-- Run Lua directly with manual testing:
local math_utils = require("modules/math_utils")
print(math_utils.add(5, 3))  -- Manual test
```

---

## Troubleshooting

### Test Hangs

**Problem**: Test runs indefinitely  
**Solution**: Check for infinite loops, missing exit conditions

### File Not Found

**Problem**: `File not found` error in tests  
**Solution**: Use absolute paths or relative to test directory

```lua
local fs = require("fs")
local fixture_path = path.join(__dirname, "fixtures", "data.txt")
```

### Module Not Loaded

**Problem**: `Unknown built-in module` error  
**Solution**: Check module name spelling and availability

### Permission Denied

**Problem**: Can't write test files  
**Solution**: Check directory permissions, use temp directory

```lua
local temp_dir = "/tmp"  -- Or use os-specific temp
local test_file = path.join(temp_dir, "hype_test_" .. os.time() .. ".txt")
```

---

## Summary

Testing is crucial for maintaining code quality and preventing regressions. Follow these principles:

1. ✅ Write tests alongside code
2. ✅ Test happy paths and error cases
3. ✅ Keep tests fast and isolated
4. ✅ Organize tests logically
5. ✅ Maintain high coverage
6. ✅ Document test purposes
7. ✅ Run tests before committing

Happy testing! 🧪
