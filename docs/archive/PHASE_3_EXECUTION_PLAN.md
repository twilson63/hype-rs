# PHASE 3 EXECUTION PLAN: Module Loader & Built-in Modules

**Project**: HYPE-MVP-001  
**Phase**: 3 of 6  
**Duration**: 5 days (Days 7-11)  
**Objective**: Implement ModuleLoader core + 5 built-in modules  
**Status**: 🟢 READY FOR EXECUTION

---

## 📋 PHASE 3 ANALYSIS

### Requirements from PRD Section 5.3

**Core Module Loader**:
- ModuleLoader struct with registry, resolver, detector
- require() method with cache checking, circular dependency detection
- load_module() private method for module execution
- Module execution in isolated context
- module.exports extraction
- Error handling and recovery

**Built-in Modules Framework**:
- BuiltinModule trait definition
- load_builtin() function for dynamic module loading
- 5 core modules: fs, path, events, util, table

**API Specifications**:
- fs: readFileSync, writeFileSync, existsSync, statSync
- path: join, dirname, basename, extname, resolve, isAbsolute
- events: EventEmitter with on, emit, off, once
- util: inspect, promisify, inherits
- table: merge, clone, keys, values, filter, map

---

## 🎯 PHASE 3 DELIVERABLES

### Component 1: ModuleLoader Core
**File**: src/modules/loader.rs (~400 lines)

```rust
pub struct ModuleLoader {
    registry: Arc<ModuleRegistry>,
    resolver: ModuleResolver,
    detector: CircularDependencyDetector,
}

impl ModuleLoader {
    pub fn new(registry: Arc<ModuleRegistry>, 
               resolver: ModuleResolver,
               detector: CircularDependencyDetector) -> Self

    pub fn require(&mut self, module_id: &str) -> Result<LuaValue>
    
    pub fn require_from(&mut self, from: &Path, module_id: &str) -> Result<LuaValue>
    
    fn load_module(&self, path: &Path, lua: &Lua) -> Result<LuaValue>
    
    fn create_module_environment(&self, lua: &Lua, path: &Path) -> Result<LuaTable>
}
```

**Functionality**:
1. Check cache (return if found)
2. Check circular dependencies
3. Resolve module path
4. Load module file
5. Create isolated environment
6. Execute Lua code
7. Extract module.exports
8. Cache result

---

### Component 2: Built-in Module Framework
**File**: src/modules/builtins/mod.rs (~100 lines)

```rust
pub trait BuiltinModule: Send + Sync {
    fn name(&self) -> &'static str;
    fn exports(&self, lua: &Lua) -> Result<LuaTable>;
}

pub struct BuiltinRegistry {
    modules: HashMap<String, Arc<dyn BuiltinModule>>,
}

impl BuiltinRegistry {
    pub fn new() -> Self
    pub fn register(&mut self, module: Arc<dyn BuiltinModule>)
    pub fn load(&self, name: &str, lua: &Lua) -> Result<LuaValue>
    pub fn is_builtin(&self, name: &str) -> bool
}
```

---

### Component 3: fs Module
**File**: src/modules/builtins/fs.rs (~150 lines)

```lua
local fs = require("fs")

fs.readFileSync(path: string) -> string
fs.writeFileSync(path: string, content: string) -> void
fs.existsSync(path: string) -> boolean
fs.statSync(path: string) -> table {size, modified}
fs.mkdirSync(path: string) -> void
fs.rmdirSync(path: string) -> void
fs.unlinkSync(path: string) -> void
fs.renameSync(oldPath: string, newPath: string) -> void
```

---

### Component 4: path Module
**File**: src/modules/builtins/path.rs (~150 lines)

```lua
local path = require("path")

path.join(...: string) -> string
path.dirname(path: string) -> string
path.basename(path: string, ext?: string) -> string
path.extname(path: string) -> string
path.resolve(...: string) -> string
path.isAbsolute(path: string) -> boolean
path.normalize(path: string) -> string
path.relative(from: string, to: string) -> string
```

---

### Component 5: events Module
**File**: src/modules/builtins/events.rs (~180 lines)

```lua
local EventEmitter = require("events").EventEmitter

local emitter = EventEmitter:new()
emitter:on(event: string, handler: function) -> self
emitter:emit(event: string, ...: any) -> void
emitter:off(event: string, handler: function) -> self
emitter:once(event: string, handler: function) -> self
emitter:removeAllListeners(event?: string) -> self
emitter:listenerCount(event: string) -> number
emitter:getMaxListeners() -> number
emitter:setMaxListeners(n: number) -> self
```

---

### Component 6: util Module
**File**: src/modules/builtins/util.rs (~100 lines)

```lua
local util = require("util")

util.inspect(value: any, options?: table) -> string
util.format(format: string, ...: any) -> string
util.promisify(fn: function) -> function
util.inherits(constructor: function, super: function) -> void
util.deprecate(fn: function, message: string) -> function
```

---

### Component 7: table Module
**File**: src/modules/builtins/table.rs (~120 lines)

```lua
local tbl = require("table")

tbl.merge(t1: table, t2: table) -> table
tbl.clone(t: table) -> table
tbl.keys(t: table) -> table
tbl.values(t: table) -> table
tbl.filter(t: table, predicate: function) -> table
tbl.map(t: table, mapper: function) -> table
tbl.reduce(t: table, reducer: function, initial?: any) -> any
tbl.find(t: table, predicate: function) -> any
tbl.includes(t: table, value: any) -> boolean
```

---

## 📊 PHASE 3 TASK BREAKDOWN

### Task 1: ModuleLoader Core (Days 7-8)
- [ ] Implement ModuleLoader struct
- [ ] Implement require() method with caching
- [ ] Implement load_module() private method
- [ ] Create module execution environment
- [ ] Extract module.exports correctly
- [ ] Error handling and recovery
- [ ] Unit tests for loader
- [ ] Integration tests with resolver/detector

**LOC Target**: ~400 lines
**Tests Target**: 15+ tests
**Time**: 1.5 days

---

### Task 2: Built-in Framework & fs Module (Days 8-9)
- [ ] Implement BuiltinModule trait
- [ ] Implement BuiltinRegistry
- [ ] Implement fs module with 8 functions
- [ ] Error handling for file operations
- [ ] Unit tests for fs module
- [ ] Integration tests with ModuleLoader

**LOC Target**: ~250 lines
**Tests Target**: 20+ tests
**Time**: 1.5 days

---

### Task 3: path, events, util Modules (Days 9-10)
- [ ] Implement path module (8 functions)
- [ ] Implement events module with EventEmitter class
- [ ] Implement util module (5 functions)
- [ ] Unit tests for each module
- [ ] Integration tests
- [ ] Cross-platform compatibility

**LOC Target**: ~430 lines
**Tests Target**: 30+ tests
**Time**: 2 days

---

### Task 4: table Module & Integration (Days 10-11)
- [ ] Implement table module (10 functions)
- [ ] Register all built-in modules
- [ ] Integration testing
- [ ] Performance testing
- [ ] Edge case testing
- [ ] Documentation

**LOC Target**: ~120 lines
**Tests Target**: 15+ tests
**Time**: 1 day

---

## ✅ SUCCESS CRITERIA FOR PHASE 3

### Functional Criteria
- [ ] C1.1: require("fs") returns fs module ✅
- [ ] C1.2: require("path") returns path module ✅
- [ ] C1.5: Modules cached and reused ✅
- [ ] C1.7: Built-in modules work correctly ✅

### Performance Criteria
- [ ] C2.1: First require() < 50ms
- [ ] C2.2: Cached require() < 1ms

### Quality Criteria
- [ ] C3.1: 90%+ code coverage
- [ ] C3.2: All unit tests passing
- [ ] C3.3: All integration tests passing
- [ ] C3.4: No compiler warnings
- [ ] C3.5: rustfmt clean
- [ ] C3.6: clippy clean

---

## 📦 DELIVERABLES SUMMARY

### Code Files (1,100+ LOC)
1. src/modules/loader.rs (400 lines, 15 tests)
2. src/modules/builtins/mod.rs (100 lines, 5 tests)
3. src/modules/builtins/fs.rs (150 lines, 20 tests)
4. src/modules/builtins/path.rs (150 lines, 15 tests)
5. src/modules/builtins/events.rs (180 lines, 20 tests)
6. src/modules/builtins/util.rs (100 lines, 10 tests)
7. src/modules/builtins/table.rs (120 lines, 15 tests)

### Total Tests
- Expected: 100+ new tests
- Target Coverage: 90%+
- Pass Rate Target: 100%

### Documentation
- Inline code documentation for all public APIs
- Test examples demonstrating usage
- Integration test coverage

---

## 🏗️ ARCHITECTURE DECISIONS

### Module Execution
- Create isolated Lua environment per module
- Set __dirname and __filename globals
- Initialize module.exports table
- Load and execute module code
- Extract and return module.exports

### Error Handling
- Wrap Lua errors in HypeError
- Clear error messages with context
- Stack traces for debugging
- Recovery strategies for graceful degradation

### Built-in Module Design
- Trait-based plugin system
- Registry for dynamic loading
- Thread-safe Arc wrapping
- Lazy initialization where possible

---

## 🎬 IMPLEMENTATION ORDER

1. **Task 1 (Days 7-8)**: ModuleLoader core
   - Foundation for everything else
   - Must be solid before moving on

2. **Task 2 (Days 8-9)**: fs module + framework
   - Most commonly used module
   - Tests ModuleLoader integration

3. **Task 3 (Days 9-10)**: path, events, util
   - Complements fs module
   - Events is foundation for Phase 4

4. **Task 4 (Days 10-11)**: table module + testing
   - Polish and finalization
   - Comprehensive testing

---

## ⚠️ RISK MITIGATION

| Risk | Mitigation |
|------|-----------|
| Lua FFI issues | Early spike testing Day 7 |
| Module execution context | Comprehensive test suite |
| Path handling edge cases | Cross-platform testing |
| Performance regression | Benchmark vs Phase 2 |
| Memory leaks | Valgrind testing |

---

**Status**: 🟢 READY FOR PHASE 3 EXECUTION
**Next Action**: Delegate tasks to agents

