# Project Request Protocol: Module System & require() for Hype-RS

**Project Name**: Hype-RS Module System Implementation  
**Project ID**: HYPE-MVP-001  
**Priority**: ⭐⭐⭐⭐⭐ CRITICAL  
**Estimated Duration**: 2-3 weeks  
**Target Completion**: Week 6 of MVP  

---

## 1. Project Overview

### 1.1 Executive Summary

Implement a robust module system with `require()` function and `hype.json` manifest support, enabling Hype-RS to build a package ecosystem. This is the **foundational feature** upon which all other runtime features depend. Without module loading, the ecosystem cannot exist.

### 1.2 Business Case

| Aspect | Current State | After Implementation |
|--------|---------------|----------------------|
| Code Reusability | Ad-hoc file inclusion | Proper module system |
| Package Ecosystem | None | Foundation for 100+ packages |
| Community Adoption | Zero | Enables contribution |
| Project Structure | Unorganized | Standard hype.json format |
| Dependency Management | Manual | Automatic with version resolution |

### 1.3 Success Metrics

- ✅ `require()` successfully loads modules
- ✅ Circular dependencies handled gracefully
- ✅ Module caching prevents redundant loading
- ✅ `hype.json` properly parsed and validated
- ✅ Built-in modules accessible (5+ core modules)
- ✅ All unit tests passing
- ✅ Example app demonstrates module usage
- ✅ Documentation complete with examples

### 1.4 Scope

**In Scope**:
- Core `require()` implementation
- Module resolution algorithm
- Module caching system
- Circular dependency detection
- Built-in modules framework
- `hype.json` manifest format
- Basic fs, path, events built-in modules
- CLI integration

**Out of Scope**:
- Full npm-compatible registry (Phase 3)
- Version resolution algorithm (Phase 2)
- Lock file generation (Phase 2)
- Native module compilation (Phase 4)
- Package publishing tools (Phase 3)

---

## 2. Technical Requirements

### 2.1 Functional Requirements

#### F1: Module Loading
```
When: User calls require("module_name")
Then: Load module from proper location
And: Return module.exports object
And: Cache the result
```

#### F2: Module Resolution
```
Path resolution priority:
1. Check module cache
2. Check ./node_modules/module_name/hype.json
3. Check ../node_modules/module_name/hype.json
4. Check ../../node_modules/... (recursive up to project root)
5. Check ~/.hype/modules/module_name/hype.json
6. Check built-in modules
7. Throw error if not found
```

#### F3: Circular Dependency Handling
```
When: Module A requires Module B
And: Module B requires Module A
Then: Return partially-loaded module
And: Log warning
And: Allow execution to continue
```

#### F4: Module Manifest (hype.json)
```json
{
  "name": "module-name",
  "version": "1.0.0",
  "main": "index.lua",
  "description": "Module description",
  "dependencies": {
    "other-module": "^1.0.0"
  }
}
```

#### F5: Built-in Modules
```lua
local fs = require("fs")
local path = require("path")
local events = require("events")
local util = require("util")
local table_utils = require("table")
```

### 2.2 Non-Functional Requirements

#### NFR1: Performance
- Module loading: < 10ms per module (after cache)
- Circular dependency detection: < 1ms
- Module cache lookup: O(1) constant time

#### NFR2: Memory
- Module cache: < 1MB for 100 modules
- No memory leaks on reload
- Proper cleanup on exit

#### NFR3: Compatibility
- Compatible with standard Lua module patterns
- Compatible with Node.js module semantics
- Cross-platform path handling (Windows/Mac/Linux)

#### NFR4: Error Handling
- Clear error messages for missing modules
- Stack traces for require() errors
- Circular dependency warnings
- Version mismatch detection (Phase 2)

### 2.3 API Specification

#### require(module_id: string) -> table
```lua
local module = require("module-name")
-- Returns module.exports or error
```

#### module.exports = value
```lua
-- In module file
module.exports = {
    function1 = function() ... end,
    var1 = "value"
}
```

#### require.cache: table
```lua
-- Inspect loaded modules
for name, module in pairs(require.cache) do
    print(name, module)
end
```

#### require.resolve(module_id: string) -> string
```lua
-- Get full path to module
local path = require.resolve("module-name")
```

---

## 3. Solution Analysis

### 3.1 Solution A: Node.js-Compatible Module System

**Description**: Replicate Node.js module resolution exactly, including semantics.

**Architecture**:
```
User calls require("express")
    ↓
Check cache (require.cache)
    ↓
Resolve path (algorithm below)
    ↓
Load hype.json → get "main" field
    ↓
Load main file (Lua)
    ↓
Execute in isolated environment
    ↓
Return module.exports
    ↓
Cache result
```

**Module Resolution Algorithm**:
```
1. Check require.cache[module_id]
2. For each directory in search_paths:
     a. Check dir/node_modules/module_id/hype.json
     b. If found, return dir/node_modules/module_id
3. Check ~/.hype/modules/module_id/hype.json
4. Check built-in modules
5. Throw MODULE_NOT_FOUND
```

**Implementation Details**:
- Use `std::collections::HashMap<String, LuaValue>` for cache
- Use `Vec<PathBuf>` for search paths
- Implement circular dependency detection via loaded_stack
- Validate hype.json with serde_json
- Isolate module environment (sandboxed globals)

**Pros**:
- ✅ Familiar to Node.js developers
- ✅ Industry-standard semantics
- ✅ Well-tested pattern (npm uses it)
- ✅ Easy migration path for Node.js apps
- ✅ Large ecosystem compatibility

**Cons**:
- ❌ More complex to implement (3 weeks vs 2)
- ❌ Larger code footprint (~500 LOC)
- ❌ May not align with Lua conventions
- ❌ Requires full hype.json validation
- ❌ Version resolution complexity

**Effort**: 3 weeks  
**Risk**: Medium (well-documented pattern, but Lua-specific edge cases)  
**Code Complexity**: Medium

---

### 3.2 Solution B: Simple Lua-Style Module System

**Description**: Leverage Lua's native package.path mechanism with light modifications.

**Architecture**:
```
Extend Lua's built-in require() function
    ↓
Set LUA_PATH to include ./node_modules
    ↓
Load .lua files directly
    ↓
Cache in package.loaded
    ↓
Return module.exports (or last return value)
```

**Module Resolution Algorithm**:
```
Use Lua's native package.path:
- ./?.lua
- ./node_modules/?.lua
- ./node_modules/?/init.lua
- ~/.hype/modules/?.lua
- built-in modules
```

**Implementation Details**:
- Minimal Rust code (extend existing Lua runtime)
- Use Lua's native package.loaded caching
- Simple hype.json support (optional)
- No custom isolation (use Lua's _G)

**Pros**:
- ✅ Minimal code (2 weeks)
- ✅ Leverages Lua's built-in system
- ✅ Small memory footprint
- ✅ Natural to Lua developers
- ✅ Easy to understand and debug

**Cons**:
- ❌ Less structured than Node.js
- ❌ Circular dependencies not handled well
- ❌ No version resolution
- ❌ Hard to build ecosystem
- ❌ Less standardized
- ❌ Future migration costs

**Effort**: 2 weeks  
**Risk**: Low (builds on existing Lua features)  
**Code Complexity**: Low

---

### 3.3 Solution C: Hybrid Approach (Recommended)

**Description**: Combine Node.js semantics with Lua simplicity. Use Node.js resolution but keep Lua's execution model.

**Architecture**:
```
Parse hype.json for module metadata
    ↓
Use Node.js-compatible resolution algorithm
    ↓
Load .lua files using Lua's require
    ↓
Cache in Rust (faster, more control)
    ↓
Return module.exports or Lua table
```

**Module Resolution Algorithm**:
```
1. Check Rust cache
2. For each directory up from cwd:
     a. Check node_modules/module_id/hype.json
     b. If main specified, use it
     c. If not, try index.lua
3. Check built-in modules
4. Return error
```

**Implementation Details**:
- Node.js-compatible resolution (familiar)
- Simple hype.json parsing (only name, version, main)
- Rust-based caching (performance)
- Reuse Lua's module loading
- Optional circular dependency tracking

**Pros**:
- ✅ Best of both worlds
- ✅ Node.js compatibility (ecosystem benefit)
- ✅ Lua simplicity (low overhead)
- ✅ 2.5 weeks (fast enough)
- ✅ Scales to full npm-compatible later
- ✅ Better error messages than pure Lua
- ✅ Future-proof architecture

**Cons**:
- ⚠️ Moderate complexity (not trivial)
- ⚠️ Two systems to understand
- ⚠️ Some feature gaps vs pure Node.js (initially)

**Effort**: 2.5 weeks  
**Risk**: Low-Medium (combines proven patterns)  
**Code Complexity**: Medium

---

## 4. Recommended Solution

### 4.1 Decision: Solution C (Hybrid Approach)

**Rationale**:
1. **Ecosystem Viability**: Node.js-compatible resolution enables package ecosystem
2. **Implementation Speed**: Faster than full Node.js implementation (C)
3. **Future Flexibility**: Can evolve to full npm compatibility
4. **Community Appeal**: Familiar to JavaScript developers interested in Lua
5. **Maintainability**: Clear boundaries between Lua and Rust layers
6. **Technical Debt**: Minimal - can refactor later if needed

**Trade-offs Accepted**:
- Accept moderate code complexity for better ecosystem
- Accept 2.5 weeks timeline over faster 2-week option
- Accept Node.js-style API over pure Lua style

---

## 5. Implementation Steps

### 5.1 Phase 1: Foundation (Days 1-3)

#### Step 1.1: Data Structures & Module Registry
```rust
// src/modules/mod.rs
pub mod registry;
pub mod resolver;
pub mod loader;
pub mod builtins;

// src/modules/registry.rs
pub struct ModuleRegistry {
    cache: Arc<RwLock<HashMap<String, LuaValue>>>,
    search_paths: Vec<PathBuf>,
}

pub struct ModuleInfo {
    pub name: String,
    pub version: String,
    pub main: String,
    pub path: PathBuf,
}
```

**Deliverables**:
- [ ] Module registry struct with cache
- [ ] ModuleInfo struct with hype.json parsing
- [ ] Search paths configuration
- [ ] Unit tests for data structures

#### Step 1.2: hype.json Parser
```rust
// src/modules/manifest.rs
#[derive(Deserialize)]
pub struct HypeManifest {
    pub name: String,
    pub version: String,
    pub main: Option<String>,
    pub description: Option<String>,
    pub dependencies: Option<HashMap<String, String>>,
}

impl HypeManifest {
    pub fn load(path: &Path) -> Result<Self> { ... }
    pub fn validate(&self) -> Result<()> { ... }
}
```

**Deliverables**:
- [ ] HypeManifest struct with serde
- [ ] Parse function with error handling
- [ ] Validation logic (name, version format)
- [ ] Unit tests for parsing edge cases

### 5.2 Phase 2: Resolution Algorithm (Days 4-6)

#### Step 2.1: Module Resolver
```rust
// src/modules/resolver.rs
pub struct ModuleResolver {
    root_dir: PathBuf,
    search_paths: Vec<PathBuf>,
}

impl ModuleResolver {
    pub fn resolve(&self, module_id: &str) -> Result<PathBuf> {
        // Algorithm implementation
    }
    
    pub fn resolve_from(&self, from: &Path, module_id: &str) -> Result<PathBuf> {
        // Relative resolution
    }
}
```

**Algorithm**:
```
fn resolve(module_id: &str) -> Result<PathBuf> {
    // 1. Check built-ins first
    if let Some(path) = self.resolve_builtin(module_id) {
        return Ok(path);
    }
    
    // 2. Check each search path
    for search_path in &self.search_paths {
        let module_path = search_path.join("node_modules").join(module_id);
        if self.module_exists(&module_path) {
            return Ok(module_path);
        }
    }
    
    // 3. Check home directory
    let home_module = expand_tilde(&format!("~/.hype/modules/{}", module_id))?;
    if self.module_exists(&home_module) {
        return Ok(home_module);
    }
    
    // 4. Not found
    Err(HypeError::ModuleNotFound(module_id.to_string()))
}
```

**Deliverables**:
- [ ] ModuleResolver implementation
- [ ] Builtin module checking
- [ ] node_modules directory traversal
- [ ] Cross-platform path handling
- [ ] Integration tests for resolution

#### Step 2.2: Circular Dependency Detection
```rust
pub struct CircularDependencyDetector {
    loaded_stack: Vec<String>,
}

impl CircularDependencyDetector {
    pub fn check(&self, module_id: &str) -> Result<()> {
        if self.loaded_stack.contains(&module_id.to_string()) {
            return Err(HypeError::CircularDependency(
                format!("{} -> {}", 
                    self.loaded_stack.join(" -> "), 
                    module_id)
            ));
        }
        Ok(())
    }
}
```

**Deliverables**:
- [ ] Circular dependency detection
- [ ] Stack tracking during require()
- [ ] Clear error messages
- [ ] Tests for circular dependencies

### 5.3 Phase 3: Module Loader (Days 7-11)

#### Step 3.1: Core Loader Implementation
```rust
// src/modules/loader.rs
pub struct ModuleLoader {
    registry: Arc<ModuleRegistry>,
    resolver: ModuleResolver,
    detector: CircularDependencyDetector,
}

impl ModuleLoader {
    pub fn require(&mut self, module_id: &str) -> Result<LuaValue> {
        // 1. Check cache
        if let Some(cached) = self.registry.get(module_id) {
            return Ok(cached);
        }
        
        // 2. Check circular dependency
        self.detector.check(module_id)?;
        
        // 3. Resolve module path
        let module_path = self.resolver.resolve(module_id)?;
        
        // 4. Load module
        let module = self.load_module(&module_path)?;
        
        // 5. Cache and return
        self.registry.set(module_id.to_string(), module.clone());
        Ok(module)
    }
    
    fn load_module(&self, path: &Path) -> Result<LuaValue> {
        // Load hype.json or index.lua
        // Execute in module context
        // Return module.exports
    }
}
```

**Deliverables**:
- [ ] ModuleLoader struct
- [ ] Cache checking logic
- [ ] Module execution in isolated context
- [ ] module.exports extraction
- [ ] Error handling and recovery

#### Step 3.2: Built-in Modules Framework
```rust
// src/modules/builtins/mod.rs
pub mod fs;
pub mod path;
pub mod events;
pub mod util;
pub mod table_utils;

pub trait BuiltinModule {
    fn name(&self) -> &'static str;
    fn exports(&self, lua: &Lua) -> Result<LuaTable>;
}

pub fn load_builtin(name: &str, lua: &Lua) -> Result<LuaValue> {
    match name {
        "fs" => fs::FsModule::new().exports(lua),
        "path" => path::PathModule::new().exports(lua),
        "events" => events::EventsModule::new().exports(lua),
        // ...
    }
}
```

**Deliverables**:
- [ ] BuiltinModule trait
- [ ] fs module stub (basic functions)
- [ ] path module stub (path utilities)
- [ ] events module stub (event emitter)
- [ ] util module stub (utilities)
- [ ] table module stub (table operations)

### 5.4 Phase 4: Lua Integration (Days 12-14)

#### Step 4.1: Global require() Function
```rust
pub fn setup_require(lua: &Lua, loader: Arc<Mutex<ModuleLoader>>) -> Result<()> {
    let loader_clone = Arc::clone(&loader);
    let require_fn = lua.create_function(move |lua, module_id: String| {
        let mut loader = loader_clone.lock().unwrap();
        match loader.require(&module_id) {
            Ok(module) => Ok(module),
            Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
        }
    })?;
    
    lua.globals().set("require", require_fn)?;
    lua.globals().set("module", lua.create_table()?)?;
    
    Ok(())
}
```

**Deliverables**:
- [ ] Global require() function
- [ ] module.exports wrapping
- [ ] Error translation to Lua
- [ ] Integration with existing CLI

#### Step 4.2: Module Environment Setup
```rust
fn create_module_environment(lua: &Lua, module_path: &Path) -> Result<LuaTable> {
    let env = lua.create_table()?;
    
    // Standard globals
    let globals = lua.globals();
    for key in &["print", "tostring", "tonumber", "type", "pairs", "ipairs"] {
        env.set(*key, globals.get::<_, LuaValue>(*key)?)?;
    }
    
    // Module-specific
    env.set("__dirname", module_path.parent().unwrap().display().to_string())?;
    env.set("__filename", module_path.display().to_string())?;
    env.set("module", lua.create_table()?)?;
    
    Ok(env)
}
```

**Deliverables**:
- [ ] Module environment creation
- [ ] __dirname and __filename globals
- [ ] module.exports initialization
- [ ] Proper sandboxing

### 5.5 Phase 5: Testing & Validation (Days 15-17)

#### Step 5.1: Unit Tests
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_module_resolution() { ... }
    
    #[test]
    fn test_circular_dependency() { ... }
    
    #[test]
    fn test_builtin_modules() { ... }
    
    #[test]
    fn test_module_caching() { ... }
}
```

**Deliverables**:
- [ ] Unit tests for resolver
- [ ] Unit tests for loader
- [ ] Integration tests
- [ ] Edge case tests
- [ ] 90%+ code coverage

#### Step 5.2: Example Applications
```lua
-- examples/basic-module.lua
local math_utils = require("math_utils")
print(math_utils.add(5, 3))

-- examples/file-operations.lua
local fs = require("fs")
local content = fs.readFileSync("data.txt")
print(content)

-- examples/package-app/hype.json
{
  "name": "my-app",
  "version": "1.0.0",
  "main": "app.lua",
  "dependencies": {
    "helper-lib": "^1.0.0"
  }
}
```

**Deliverables**:
- [ ] 3+ example applications
- [ ] Documentation with code snippets
- [ ] Example hype.json files
- [ ] README for examples

### 5.6 Phase 6: Documentation & Polish (Days 18-19)

#### Step 6.1: API Documentation
```markdown
# require() API

## require(module_id: string)
Load and return a module.

### Parameters
- `module_id` (string): Name or path of module

### Returns
- (any): The module.exports value

### Examples
```lua
local fs = require("fs")
local myModule = require("./my-module")
```

### Errors
- `MODULE_NOT_FOUND`: Module not found in any search path
- `CIRCULAR_DEPENDENCY`: Circular dependency detected
```

**Deliverables**:
- [ ] require() API documentation
- [ ] hype.json format documentation
- [ ] Module creation guide
- [ ] Built-in modules documentation
- [ ] Architecture documentation

#### Step 6.2: Migration Guide
**Deliverables**:
- [ ] Guide for Lua developers
- [ ] Guide for Node.js developers
- [ ] Troubleshooting section
- [ ] FAQ

---

## 6. Success Criteria

### 6.1 Functional Criteria

- [ ] **C1.1**: require("fs") returns fs module with functions
- [ ] **C1.2**: require("path") returns path module with utilities
- [ ] **C1.3**: require("./local-module") loads relative modules
- [ ] **C1.4**: Circular dependencies are detected and reported
- [ ] **C1.5**: Modules are cached and reused
- [ ] **C1.6**: hype.json files are parsed and validated
- [ ] **C1.7**: Built-in modules work correctly
- [ ] **C1.8**: require.cache shows loaded modules
- [ ] **C1.9**: require.resolve() returns full path
- [ ] **C1.10**: Module environment has __dirname and __filename

### 6.2 Performance Criteria

- [ ] **C2.1**: First require() loads in < 50ms
- [ ] **C2.2**: Cached require() loads in < 1ms
- [ ] **C2.3**: Circular dependency detection < 1ms
- [ ] **C2.4**: Module cache < 1MB for 100 modules
- [ ] **C2.5**: No memory leaks on repeated reloads

### 6.3 Quality Criteria

- [ ] **C3.1**: 90%+ code coverage
- [ ] **C3.2**: All unit tests passing
- [ ] **C3.3**: All integration tests passing
- [ ] **C3.4**: No compiler warnings
- [ ] **C3.5**: rustfmt clean
- [ ] **C3.6**: clippy clean

### 6.4 Documentation Criteria

- [ ] **C4.1**: API documentation complete
- [ ] **C4.2**: 3+ working examples provided
- [ ] **C4.3**: Migration guides written
- [ ] **C4.4**: FAQ section complete
- [ ] **C4.5**: Architecture documentation written

### 6.5 Acceptance Criteria

**Feature Complete**:
```bash
hype examples/basic-module.lua
# Should output: Basic module loaded successfully

hype examples/file-operations.lua  
# Should read file and display content

cd examples/package-app && hype app.lua
# Should load dependencies and run
```

**No Regressions**:
- All existing tests still pass
- All existing CLI functionality works
- No performance degradation

---

## 7. Risk Assessment

### 7.1 Technical Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|-----------|
| Circular dependency performance | Medium | Medium | Pre-implement detection algorithm review |
| Lua FFI limitations | Low | High | Early spike testing in days 1-2 |
| Path resolution edge cases | Medium | Low | Comprehensive test suite |
| Module caching correctness | Low | High | Extensive testing with reload scenarios |

### 7.2 Schedule Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|-----------|
| Scope creep (version resolution) | High | Medium | Strict scope boundary in document |
| Testing takes longer | Medium | Medium | Plan tests in parallel with implementation |
| Integration complexities | Low | High | Daily integration tests |

---

## 8. Dependencies

### 8.1 External Dependencies

- ✅ **mlua** (0.9): Already in Cargo.toml
- ✅ **serde_json** (1.0): Already in Cargo.toml
- ✅ **serde** (1.0): Already in Cargo.toml
- ✅ **tempfile** (3.8): Already in Cargo.toml

No new external dependencies required.

### 8.2 Internal Dependencies

- Lua runtime (LuaStateManager)
- CLI argument parser
- Error handling system
- File I/O system

---

## 9. Rollout Plan

### 9.1 Development Timeline

```
Week 1 (5 days):
  Mon-Tue: Foundation (Step 1)
  Wed-Thu: Resolution algorithm (Step 2)
  Fri: Integration checkpoint

Week 2 (5 days):
  Mon-Tue: Module loader (Step 3)
  Wed-Thu: Lua integration (Step 4)
  Fri: Integration checkpoint

Week 3 (3 days):
  Mon-Tue: Testing & validation (Step 5)
  Wed: Documentation & polish (Step 6)
```

### 9.2 Release Plan

**v0.2.0-alpha** (After week 2):
- Functional module system
- Basic built-in modules
- Early feedback from community

**v0.2.0-beta** (After week 2.5):
- All tests passing
- Full documentation
- Example applications

**v0.2.0** (After week 3):
- Production ready
- Stable API
- Long-term support

---

## 10. Success Metrics & Reporting

### 10.1 Key Performance Indicators

1. **Code Quality**
   - Test coverage: Target 90%
   - No compiler warnings
   - Maintainability index > 80

2. **Performance**
   - First load: < 50ms
   - Cached load: < 1ms
   - Memory overhead: < 1MB per 100 modules

3. **Developer Experience**
   - Example apps runnable in < 5 minutes
   - Documentation comprehensible
   - Error messages actionable

### 10.2 Progress Tracking

Daily standup template:
```
✅ Completed today:
   - [Component]: [Task]

⏳ In progress:
   - [Component]: [Task]

🚧 Blockers:
   - [Issue]: [Impact]
```

### 10.3 Acceptance Signoff

**Technical Review**:
- [ ] Architecture review passed
- [ ] Code review passed (2+ reviewers)
- [ ] Performance testing passed

**Business Review**:
- [ ] All success criteria met
- [ ] No critical bugs
- [ ] Documentation complete
- [ ] Team sign-off

---

## 11. Appendices

### 11.1 Example: require() with hype.json

**Project Structure**:
```
my-app/
├── hype.json
├── app.lua
└── node_modules/
    └── math-lib/
        ├── hype.json
        ├── index.lua
        └── utils.lua
```

**hype.json**:
```json
{
  "name": "my-app",
  "version": "1.0.0",
  "main": "app.lua",
  "dependencies": {
    "math-lib": "^1.0.0"
  }
}
```

**math-lib/hype.json**:
```json
{
  "name": "math-lib",
  "version": "1.0.0",
  "main": "index.lua"
}
```

**math-lib/index.lua**:
```lua
module.exports = {
    add = function(a, b) return a + b end,
    multiply = function(a, b) return a * b end,
}
```

**app.lua**:
```lua
local math = require("math-lib")
print("5 + 3 =", math.add(5, 3))
print("5 * 3 =", math.multiply(5, 3))
```

**Execution**:
```bash
$ hype app.lua
5 + 3 = 8
5 * 3 = 15
```

### 11.2 Error Handling Examples

**Missing Module**:
```lua
local missing = require("non-existent")
-- Error: MODULE_NOT_FOUND
-- Details: Module 'non-existent' not found in:
--   ./node_modules/non-existent
--   ../node_modules/non-existent
--   ~/.hype/modules/non-existent
--   Built-in modules
```

**Circular Dependency**:
```lua
-- a.lua
local b = require("b")

-- b.lua
local a = require("a")

-- Error: CIRCULAR_DEPENDENCY
-- Details: require() cycle detected:
--   a -> b -> a
```

### 11.3 Built-in Module Stubs

**fs module (basic)**:
```lua
local fs = require("fs")

fs.readFileSync(path) -> string
fs.writeFileSync(path, content)
fs.existsSync(path) -> boolean
fs.statSync(path) -> table
```

**path module**:
```lua
local path = require("path")

path.join(...) -> string
path.dirname(path) -> string
path.basename(path) -> string
path.extname(path) -> string
path.resolve(...) -> string
```

**events module**:
```lua
local events = require("events")

events.EventEmitter:new() -> emitter
emitter:on(event, handler)
emitter:emit(event, ...)
emitter:off(event, handler)
```

---

## Sign-Off

**Prepared by**: AI Assistant  
**Date**: October 22, 2025  
**Status**: Ready for Implementation  

**Stakeholder Approvals**:

- [ ] Product Owner: _______________  Date: _______
- [ ] Technical Lead: _______________  Date: _______
- [ ] Architecture Review: _______________  Date: _______

---

**Document Version**: 1.0  
**Last Updated**: October 22, 2025  
**Next Review**: After implementation phase 1
