# Direct File/Directory Fallback Feature

## Overview

The module resolution system in hype-rs now supports a fallback mechanism that allows `require()` to load Lua files and directories directly from the root directory, without requiring them to be in the `hype_modules/` directory.

## Resolution Order

The module resolver now follows this priority order:

1. **Built-in modules** (fs, path, events, util, table, http)
2. **Relative paths** (./ or ../)
3. **Absolute paths** (if allowed)
4. **hype_modules directories** (walk up from current directory)
5. **~/.hype/modules directories** (global modules)
6. **Direct file/directory fallback** (NEW)
7. Error if not found

## Direct File Fallback Behavior

When a module is not found in `hype_modules/` or `~/.hype/modules/`, the resolver will try to load it directly from the root directory using this search pattern:

```
root_dir/{module_id}.lua
root_dir/{module_id}/index.lua
root_dir/{module_id}/init.lua
root_dir/{module_id}
```

### Examples

**Example 1: Direct .lua file**
```
.
├── hello.lua
├── foo.lua

-- In hello.lua:
local foo = require("foo")  -- Loads foo.lua via fallback
```

**Example 2: Directory with index.lua**
```
.
├── hello.lua
├── utils/
│   └── index.lua

-- In hello.lua:
local utils = require("utils")  -- Loads utils/index.lua via fallback
```

**Example 3: Directory with init.lua**
```
.
├── hello.lua
├── helpers/
│   └── init.lua

-- In hello.lua:
local helpers = require("helpers")  -- Loads helpers/init.lua via fallback
```

**Example 4: Empty directory**
```
.
├── hello.lua
├── mylib/

-- In hello.lua:
local mylib = require("mylib")  -- Loads mylib directory via fallback
```

## Priority: hype_modules takes precedence

If a module exists in both `hype_modules/` and as a direct file in the root directory, `hype_modules/` takes priority:

```
.
├── hello.lua
├── utils.lua
├── hype_modules/
│   └── utils/
│       └── index.lua

-- In hello.lua:
local utils = require("utils")
-- This loads hype_modules/utils/index.lua, NOT utils.lua
```

## Implementation Details

### Code Changes

The feature was implemented by:

1. Adding a new `try_direct_file_fallback()` method in `ModuleResolver`
2. Modifying the `resolve()` method to call the fallback when `resolve_module_paths()` fails
3. Adding comprehensive tests to verify the behavior

### Files Modified

- `src/modules/resolver.rs`: Core implementation and tests

### Tests Added

Six new tests verify the functionality:

- `test_direct_file_fallback_lua_file`: Direct .lua file loading
- `test_direct_file_fallback_directory_with_index`: Directory with index.lua
- `test_direct_file_fallback_directory_with_init`: Directory with init.lua
- `test_direct_file_fallback_directory_only`: Empty directory fallback
- `test_direct_file_fallback_hype_modules_priority`: Verifies hype_modules takes priority
- `test_direct_file_fallback_nonexistent`: Error handling for missing modules

## Backward Compatibility

This feature is **fully backward compatible**:

- All existing code using `hype_modules/` continues to work unchanged
- `hype_modules/` still has priority over direct files
- No breaking changes to the API or module resolution order

## Use Cases

This feature enables:

- **Simpler project structures** for small scripts without a `hype_modules/` directory
- **Flexible local module organization** without directory hierarchy constraints
- **Easier module sharing** between related files in the same directory
- **Gradual migration** from monolithic scripts to modular code

## Performance Impact

The fallback mechanism:
- Only activates if the module is not found in standard locations
- Adds minimal overhead (checking a few file paths)
- Does not affect performance for modules in `hype_modules/`
- Is cached like all other module loads

## Example Script

Here's a complete example demonstrating the feature:

```lua
-- main.lua
local utils = require("utils")
local helpers = require("helpers")

print(utils.greeting)
print(helpers.add(5, 3))
```

```lua
-- utils.lua
return {
  greeting = "Hello from utils.lua"
}
```

```lua
-- helpers/init.lua
return {
  add = function(a, b) return a + b end
}
```

With the direct file fallback, you can run this structure without requiring `hype_modules/`:

```
.
├── main.lua
├── utils.lua
├── helpers/
│   └── init.lua
```

## Testing

All tests pass with this feature:

```
cargo test --lib modules::resolver::tests::test_direct_file_fallback
```

Results: **6 passed, 0 failed**

All resolver tests: **33 passed, 0 failed**

Full test suite: **479+ passed**
