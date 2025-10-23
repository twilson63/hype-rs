# Module System Implementation Overview

## Quick Reference

**Project**: Hype-RS Module System (require() + hype.json)  
**ID**: HYPE-MVP-001  
**Priority**: ⭐⭐⭐⭐⭐  
**Duration**: 2-3 weeks  
**Solution**: Hybrid Approach (Node.js resolution + Lua execution)

---

## Visual Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Hype-RS Module System                     │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│ User Code: local module = require("module-name")             │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌──────────────────────────────────────────────────────────────┐
│ Global require() Function (Lua)                              │
└──────────────────────────────────────────────────────────────┘
                       │
                       ▼
┌──────────────────────────────────────────────────────────────┐
│ Module Loader (Rust)                                         │
│ ┌────────────────────────────────────────────────────────┐  │
│ │ 1. Check require.cache                                 │  │
│ │ 2. Check Circular Dependency Stack                     │  │
│ │ 3. Resolve Module Path                                 │  │
│ │ 4. Load Module (hype.json → main file)                 │  │
│ │ 5. Execute in Module Environment                       │  │
│ │ 6. Extract module.exports                              │  │
│ │ 7. Cache Result                                        │  │
│ │ 8. Return to User                                      │  │
│ └────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────┘
        │           │           │           │
        ▼           ▼           ▼           ▼
    ┌────────┐ ┌──────────┐ ┌─────────┐ ┌──────────┐
    │ Cache  │ │ Resolver │ │ Detector│ │ Built-in │
    │ System │ │ (Node.js)│ │ (Cycles)│ │ Modules  │
    └────────┘ └──────────┘ └─────────┘ └──────────┘
```

---

## Module Resolution Algorithm

```
require("module-name")
    ↓
    ├─→ Cached? → Return cached
    │
    ├─→ Circular? → Error
    │
    ├─→ Resolve path:
    │   1. ./node_modules/module-name/hype.json
    │   2. ../node_modules/module-name/hype.json
    │   3. ../../node_modules/module-name/hype.json
    │   4. ~/.hype/modules/module-name/hype.json
    │   5. Built-in modules
    │
    ├─→ Load module
    │   ├─→ Parse hype.json
    │   ├─→ Create environment (__dirname, __filename)
    │   ├─→ Execute Lua code
    │   └─→ Extract module.exports
    │
    ├─→ Cache result
    │
    └─→ Return module
```

---

## Implementation Timeline

```
        Week 1              Week 2              Week 3
   M T W T F            M T W T F            M T W
   
   ███ Foundation       ███ Loader         ███ Tests
   ▓▓▓ Resolution       ▓▓▓ Integration     ▓▓▓ Docs
   ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

Day 1-3: Foundation
   - Data structures
   - hype.json parser
   - Unit tests

Day 4-6: Resolution
   - Module resolver
   - Circular detection
   - Integration tests

Day 7-11: Loader
   - Module loader
   - Built-in modules
   - Lua integration

Day 12-14: Integration
   - Global require()
   - Module environment
   - CLI integration

Day 15-17: Testing
   - Unit tests (90%+)
   - Integration tests
   - Example apps

Day 18-19: Documentation
   - API docs
   - Migration guides
   - FAQ
```

---

## Key Components

### 1. ModuleRegistry (Rust)
```
- Module cache: HashMap<String, LuaValue>
- Search paths: Vec<PathBuf>
- Thread-safe: Arc<RwLock<>>
```

### 2. ModuleResolver (Rust)
```
- Implements Node.js resolution algorithm
- Handles relative paths
- Checks built-in modules
```

### 3. CircularDependencyDetector (Rust)
```
- Tracks loaded_stack
- Detects cycles
- Returns clear error messages
```

### 4. require() Function (Lua/Rust Bridge)
```
- Global function callable from Lua
- Bridges Lua to Rust loader
- Error translation to Lua exceptions
```

### 5. Built-in Modules (Rust)
```
- fs: File operations
- path: Path utilities
- events: Event emitter
- util: Utilities
- table: Table operations
```

---

## Code Structure

```
src/
├── modules/                    # NEW
│   ├── mod.rs                  # Module system exports
│   ├── registry.rs             # Module registry & cache
│   ├── resolver.rs             # Module path resolution
│   ├── loader.rs               # Module loading logic
│   ├── detector.rs             # Circular dependency detection
│   ├── manifest.rs             # hype.json parsing
│   └── builtins/               # Built-in modules
│       ├── mod.rs
│       ├── fs.rs               # File system module
│       ├── path.rs             # Path utilities module
│       ├── events.rs           # Event emitter module
│       ├── util.rs             # Utilities module
│       └── table.rs            # Table utilities module
│
├── error/                      # MODIFIED
│   └── mod.rs                  # Add ModuleError variants
│
├── lua/                        # MODIFIED
│   └── mod.rs                  # Register require() function
│
└── main.rs                     # UNCHANGED
```

---

## Example Usage

### Simple Module Usage
```lua
-- math-lib/index.lua
module.exports = {
    add = function(a, b) return a + b end,
    sub = function(a, b) return a - b end,
}

-- app.lua
local math = require("math-lib")
print(math.add(5, 3))  -- 8
```

### Multiple Dependencies
```lua
-- app.lua
local fs = require("fs")
local path = require("path")
local utils = require("utils")

local config = fs.readFileSync("config.json")
local dir = path.dirname(config)
local result = utils.process(dir)
```

### Circular Dependencies
```lua
-- a.lua
local b = require("b")

-- b.lua
local a = require("a")  -- Returns partially-loaded a
```

---

## Success Criteria Checklist

### Functional ✅
- [ ] require() loads modules
- [ ] Circular dependencies handled
- [ ] Modules are cached
- [ ] hype.json parsed correctly
- [ ] Built-in modules work
- [ ] require.cache visible
- [ ] require.resolve() works
- [ ] Module environment has __dirname, __filename

### Performance ✅
- [ ] First load < 50ms
- [ ] Cached load < 1ms
- [ ] Circular detect < 1ms
- [ ] Cache < 1MB per 100 modules
- [ ] No memory leaks

### Quality ✅
- [ ] 90%+ test coverage
- [ ] All tests passing
- [ ] No compiler warnings
- [ ] rustfmt clean
- [ ] clippy clean

### Documentation ✅
- [ ] API docs complete
- [ ] 3+ example apps
- [ ] Migration guides
- [ ] FAQ section
- [ ] Architecture docs

---

## Risk Mitigation

| Risk | Probability | Mitigation |
|------|-------------|-----------|
| Lua FFI limitations | Low | Early spike testing (Day 1) |
| Path resolution bugs | Medium | Comprehensive test suite |
| Performance issues | Low | Benchmarking during Phase 3 |
| Scope creep | High | Strict scope boundary enforcement |

---

## Dependencies

**New Dependencies**: None  
**Modified Dependencies**: None  

All required crates already in Cargo.toml:
- mlua (0.9)
- serde_json (1.0)
- serde (1.0)
- tempfile (3.8)

---

## Integration Points

1. **CLI**: Register require() in main execution
2. **Lua Runtime**: Setup module environment on startup
3. **Error System**: Add ModuleError variants
4. **File I/O**: Use existing file reading system

---

## Next Immediate Actions

1. **Approval** (This Document)
2. **Kickoff Meeting** (30 min)
3. **Architecture Review** (1 hour)
4. **Start Phase 1** (Day 1 implementation)
5. **Daily Standups** (15 min each)

---

## Document References

- **Full PRP**: `PRPs/module-system-prp.md` (1052 lines)
- **Roadmap**: `ROADMAP.md` (Phase 1.1)
- **Vision**: `VISION.md` (Year 1 goals)

---

**Last Updated**: October 22, 2025  
**Status**: Ready for Implementation  
**Approval Required**: ✋
