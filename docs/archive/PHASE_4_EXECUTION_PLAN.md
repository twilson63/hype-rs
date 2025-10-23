# Phase 4 Execution Plan: Lua Integration

**Status**: Ready for Implementation  
**Estimated Duration**: 3 days  
**Target Completion**: This session  

---

## 1. Phase 4 Overview

### Objective
Integrate the ModuleLoader (from Phase 3) with the Lua runtime, exposing the `require()` function and module system to Lua scripts.

### Key Deliverables
1. Global `require()` function for Lua
2. `__dirname` and `__filename` globals
3. `require.cache` and `require.resolve()` API
4. Module environment setup with proper sandboxing
5. CLI integration for module loading
6. Integration tests and examples

---

## 2. Phase 4 Requirements Breakdown

### 2.1 Functional Requirements

#### F4.1: Global require() Function
- **What**: Make `require()` callable from Lua
- **How**: Create Lua function wrapping ModuleLoader
- **Input**: module_id (string)
- **Output**: module.exports (table/any)
- **Error Handling**: Convert Rust errors to Lua errors
- **File**: `src/lua/require.rs` (NEW)

#### F4.2: Module Environment Variables
- **__dirname**: Full path to module directory
- **__filename**: Full path to module file
- **module**: Module metadata table
- **File**: `src/lua/module_env.rs` (NEW)

#### F4.3: require.cache Object
- **What**: Table of all loaded modules by path
- **How**: Mirror ModuleRegistry cache to Lua
- **Access Pattern**: `require.cache[path] = exports`
- **File**: `src/lua/require.rs` (modification)

#### F4.4: require.resolve() Function
- **What**: Resolve module ID to full path
- **How**: Call ModuleResolver.resolve()
- **Input**: module_id (string)
- **Output**: absolute path (string)
- **File**: `src/lua/require.rs` (modification)

#### F4.5: CLI Integration
- **Flag**: `--module` / `-m` to load module files
- **Behavior**: Set up module system before execution
- **File**: `src/cli/commands.rs` (modification)

### 2.2 Non-Functional Requirements

#### Performance
- require() lookup: < 1ms (cached)
- Environment setup: < 5ms per module
- No blocking operations

#### Memory
- Module cache: Shared between Rust and Lua
- __dirname/__filename: Stack-allocated strings
- No duplicate data structures

#### Code Quality
- 90%+ test coverage
- Proper error handling
- Thread-safe access to ModuleLoader
- Clear rustdoc comments

---

## 3. Implementation Roadmap

### Task 1: require() Function (6 hours)
**File**: `src/lua/require.rs` (NEW, ~250 LOC)

**Subtasks**:
1. Create `RequireSetup` struct
2. Implement `setup_require_fn(lua, loader) -> Result<()>`
3. Create Lua function closure for require()
4. Handle module_id parsing
5. Call ModuleLoader.require()
6. Wrap exports in Lua table
7. Error translation (Rust → Lua)
8. Create 8+ tests

**Expected Output**:
```rust
pub struct RequireSetup {
    loader: Arc<Mutex<ModuleLoader>>,
}

impl RequireSetup {
    pub fn new(loader: Arc<Mutex<ModuleLoader>>) -> Self { ... }
    pub fn setup(&self, lua: &Lua) -> Result<()> { ... }
}
```

---

### Task 2: Module Environment Setup (4 hours)
**File**: `src/lua/module_env.rs` (NEW, ~180 LOC)

**Subtasks**:
1. Create `ModuleEnvironment` struct
2. Implement `create_env(lua, module_path) -> Result<LuaTable>`
3. Add standard globals (print, type, pairs, etc.)
4. Set __dirname from module path
5. Set __filename from module path
6. Initialize module.exports table
7. Create sandboxing function
8. Create 6+ tests

**Expected Output**:
```rust
pub struct ModuleEnvironment;

impl ModuleEnvironment {
    pub fn create(
        lua: &Lua,
        module_path: &Path,
    ) -> Result<LuaTable> { ... }
}
```

---

### Task 3: require.cache & require.resolve() (5 hours)
**File**: `src/lua/require.rs` (modification, +100 LOC)

**Subtasks**:
1. Create `require` object (Lua table)
2. Add `cache` property → mirror of ModuleRegistry
3. Add `resolve(module_id) -> path` function
4. Handle cache updates on module load
5. Synchronize Rust cache with Lua table
6. Create 5+ tests

**Expected Output**:
```lua
-- In Lua
local fs = require("fs")
local path = require.resolve("fs")  -- "/path/to/fs"
for key, module in pairs(require.cache) do
    print(key, module)
end
```

---

### Task 4: CLI Integration (3 hours)
**File**: `src/cli/commands.rs` (modification, +50 LOC)

**Subtasks**:
1. Add `--module` / `-m` argument to parser
2. Create ModuleLoader on startup
3. Initialize require() in Lua before script
4. Handle module-specific error messages
5. Create 4+ tests

**Expected Output**:
```bash
hype --module ./my-module.lua
hype -m app.lua
```

---

### Task 5: Integration Tests (4 hours)
**File**: `tests/module_integration_test.rs` (NEW, ~200 LOC)

**Test Cases**:
1. Basic require() of built-in module
2. require() of custom module
3. Module caching verification
4. __dirname and __filename in module
5. require.resolve() accuracy
6. Circular dependency handling
7. Error handling and messages
8. CLI --module flag
9. Multiple modules interaction
10. Cache clearing

---

### Task 6: Examples (3 hours)
**Files**: `examples/module-*.lua` (NEW, ~150 LOC)

**Examples**:
1. `examples/basic-require.lua` - Load fs module
2. `examples/path-operations.lua` - Use path module
3. `examples/custom-module.lua` - Define and load custom module
4. `examples/module-with-dependencies.lua` - Module requiring other modules

---

### Task 7: Documentation (3 hours)
**Files**: `docs/modules.md`, `docs/require-api.md` (NEW, ~300 LOC)

**Content**:
1. require() API documentation
2. Module environment variables
3. require.cache explanation
4. require.resolve() documentation
5. CLI integration guide
6. Module creation guide
7. Examples and troubleshooting

---

## 4. Implementation Order

**Day 1** (5 hours):
- Task 1: require() Function
- Task 2: Module Environment Setup

**Day 2** (5 hours):
- Task 3: require.cache & require.resolve()
- Task 4: CLI Integration

**Day 3** (5 hours):
- Task 5: Integration Tests (2 hours)
- Task 6: Examples (1.5 hours)
- Task 7: Documentation (1.5 hours)

---

## 5. Success Criteria

### Functional
✅ require("fs") returns fs module exports  
✅ require("path") returns path module exports  
✅ require("custom-module") loads custom modules  
✅ __dirname points to module directory  
✅ __filename points to module file  
✅ require.cache contains loaded modules  
✅ require.resolve("fs") returns path to fs  
✅ Circular dependencies detected and reported  
✅ CLI --module flag works  

### Quality
✅ 90%+ code coverage  
✅ All tests passing (120+ total)  
✅ 0 compiler errors  
✅ rustfmt & clippy clean  
✅ Documentation complete  

### Performance
✅ require() lookup < 1ms (cached)  
✅ Module load < 50ms (first time)  
✅ Environment setup < 5ms  

---

## 6. Risk Assessment

### Risk: Thread Safety of ModuleLoader
**Mitigation**: Wrap in Arc<Mutex<>> for Lua access

### Risk: Lua Environment Pollution
**Mitigation**: Create isolated environment per module

### Risk: Circular Dependency in Lua
**Mitigation**: Use existing detector from Phase 3

### Risk: File Path Handling (Windows/Mac/Linux)
**Mitigation**: Use std::path::Path throughout

---

## 7. Dependencies

**Phase 3 Deliverables Required**:
- ModuleLoader ✅
- ModuleResolver ✅
- ModuleRegistry ✅
- CircularDependencyDetector ✅
- BuiltinRegistry ✅
- 5 Built-in modules (fs, path, events, util, table) ✅

**External Dependencies**:
- mlua >= 0.9 (already in Cargo.toml) ✅
- serde_json >= 1.0 (already in Cargo.toml) ✅

---

## 8. Deliverables Checklist

### Code (7 new files, 2 modified)
- [ ] src/lua/require.rs (NEW)
- [ ] src/lua/module_env.rs (NEW)
- [ ] tests/module_integration_test.rs (NEW)
- [ ] examples/basic-require.lua (NEW)
- [ ] examples/path-operations.lua (NEW)
- [ ] examples/custom-module.lua (NEW)
- [ ] examples/module-with-dependencies.lua (NEW)
- [ ] src/cli/commands.rs (MODIFIED)
- [ ] src/lua/mod.rs (MODIFIED - add exports)

### Documentation (2 new files)
- [ ] docs/modules.md
- [ ] docs/require-api.md

### Tests
- [ ] 30+ new integration tests
- [ ] 90%+ total coverage target

### Build & Artifacts
- [ ] cargo build --release succeeds
- [ ] All tests pass
- [ ] No compiler warnings (module code)

---

## 9. Next Steps After Phase 4

### Phase 5: Testing & Validation
- Advanced test scenarios
- Performance benchmarking
- Example applications

### Phase 6: Documentation & Polish
- API documentation
- Migration guides
- FAQ & troubleshooting

### Phase 7: Release
- Version 0.1.0 of module system
- Public documentation
- Community announcement

---

## 10. Metrics & Goals

**Lines of Code**: ~750 LOC for Phase 4
**Tests Added**: 30+ new tests
**Documentation**: 300+ LOC
**Total Deliverables**: 10+ files
**Estimated Effort**: 3 days (15 hours)
**Target Test Coverage**: 90%+ (total modules)

