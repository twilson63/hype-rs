# PRP-013: Filesystem Module - Complete Implementation

**Status:** Proposed  
**Priority:** High  
**Created:** 2025-10-28  
**Author:** AI Assistant  
**Estimated Effort:** 16-24 hours

---

## Table of Contents

- [1. Project Overview](#1-project-overview)
- [2. Current State Analysis](#2-current-state-analysis)
- [3. Technical Requirements](#3-technical-requirements)
- [4. Proposed Solutions](#4-proposed-solutions)
- [5. Solution Comparison](#5-solution-comparison)
- [6. Recommended Solution](#6-recommended-solution)
- [7. Implementation Plan](#7-implementation-plan)
- [8. Success Criteria](#8-success-criteria)
- [9. Risk Assessment](#9-risk-assessment)
- [10. Future Enhancements](#10-future-enhancements)

---

## 1. Project Overview

### 1.1 Problem Statement

The `fs` module is currently **defined but not implemented**. The module exports function definitions but has no Lua bindings or actual functionality:

**Current State:**
- ✅ Module structure exists (`src/modules/builtins/fs.rs`)
- ✅ Function signatures declared (8 functions)
- ❌ **No Lua bindings** - Functions not callable from Lua
- ❌ **No implementation** - No actual file operations
- ❌ **No tests** - Zero test coverage

**Impact:**
- Users cannot perform basic file operations
- Examples in documentation don't work (`examples/file-operations.lua`)
- CLI tools requiring file I/O are blocked
- Competitive disadvantage vs Node.js, Python, Deno

### 1.2 Current Pain Points

#### Issue 1: No File Reading
```lua
local fs = require("fs")
-- Module loads but functions don't exist!
local content = fs.readFileSync("config.json")  -- ERROR: attempt to call nil value
```

#### Issue 2: No File Writing
```lua
-- Cannot save data
fs.writeFileSync("output.txt", data)  -- ERROR: attempt to call nil value
```

#### Issue 3: No Directory Operations
```lua
-- Cannot check existence, create dirs, list files
if fs.existsSync("data") then  -- ERROR
    local files = fs.readdirSync("data")  -- ERROR
end
```

#### Issue 4: Examples Don't Work
The `examples/file-operations.lua` file demonstrates patterns but none of them actually work because the fs module has no implementation.

### 1.3 Impact

**Severity:** High - Blocks core scripting use cases

**Affected Users:**
- 📝 **Script developers** - Cannot read/write config files (BLOCKER)
- 🔧 **CLI tool builders** - Cannot process files (BLOCKER)
- 📊 **Data processors** - Cannot read/write data files (BLOCKER)
- 🤖 **Automation scripts** - Cannot manipulate files (BLOCKER)

**Market Impact:**
- Cannot compete with Node.js, Python, Deno for scripting
- Basic file operations are table stakes for any scripting runtime
- Users expect `fs` module to work when it's advertised

### 1.4 Project Goals

1. **Primary Goal:** Implement complete filesystem functionality in Lua bindings
2. **API Compatibility:** Match Node.js-style API (familiar to developers)
3. **Cross-Platform:** Work consistently on Windows, macOS, Linux
4. **Safety:** Proper error handling, permissions checks, security boundaries
5. **Performance:** Efficient file operations with minimal overhead

---

## 2. Current State Analysis

### 2.1 Existing Module Structure

**File:** `src/modules/builtins/fs.rs` (134 lines)

**Current Implementation:**
```rust
pub struct FsModule;

impl BuiltinModule for FsModule {
    fn name(&self) -> &str {
        "fs"
    }

    fn exports(&self) -> Result<JsonValue, HypeError> {
        Ok(json!({
            "readFileSync": { "__fn": "readFileSync", "__desc": "Read file synchronously" },
            "writeFileSync": { "__fn": "writeFileSync", "__desc": "Write file synchronously" },
            "existsSync": { "__fn": "existsSync", "__desc": "Check if file exists" },
            "statSync": { "__fn": "statSync", "__desc": "Get file statistics" },
            "readdirSync": { "__fn": "readdirSync", "__desc": "Read directory contents" },
            "unlinkSync": { "__fn": "unlinkSync", "__desc": "Delete file" },
            "mkdirSync": { "__fn": "mkdirSync", "__desc": "Create directory" },
            "rmdirSync": { "__fn": "rmdirSync", "__desc": "Remove directory" }
        }))
    }
}
```

**What's Missing:**
- ❌ No `lua_bindings.rs` file
- ❌ No actual Rust functions implementing file operations
- ❌ No Lua function registration
- ❌ No error handling
- ❌ No security checks
- ❌ No tests

### 2.2 Declared Functions

| Function | Description | Status |
|----------|-------------|--------|
| `readFileSync(path)` | Read entire file as string | ❌ Not implemented |
| `writeFileSync(path, data)` | Write string/bytes to file | ❌ Not implemented |
| `existsSync(path)` | Check if path exists | ❌ Not implemented |
| `statSync(path)` | Get file metadata (size, type, times) | ❌ Not implemented |
| `readdirSync(path)` | List directory contents | ❌ Not implemented |
| `unlinkSync(path)` | Delete file | ❌ Not implemented |
| `mkdirSync(path)` | Create directory | ❌ Not implemented |
| `rmdirSync(path)` | Remove directory | ❌ Not implemented |

### 2.3 Dependencies

**Current:** Only `std::fs` and `std::path::Path` (unused imports)

**Needed:**
- `std::fs` - Core file operations
- `std::path::{Path, PathBuf}` - Path handling
- `std::io` - I/O operations and errors
- `mlua` - Lua bindings (already available)

**Optional:**
- `walkdir` - For recursive directory operations (future)
- `filetime` - For advanced timestamp manipulation (future)

### 2.4 Similar Implementations

**HTTP Module Pattern:**
- Has `lua_bindings.rs` with `create_http_module()`
- Registers functions with `lua.create_function()`
- Returns Lua tables with callable functions
- Comprehensive error handling

**We can follow the same pattern for `fs` module.**

---

## 3. Technical Requirements

### 3.1 Functional Requirements

#### FR-1: File Reading

**FR-1.1:** Read entire file as string
```lua
local content = fs.readFileSync("file.txt")
-- content: string
```

**FR-1.2:** Read binary files as bytes
```lua
local bytes = fs.readFileSync("image.png")
-- bytes: string (raw bytes)
```

**FR-1.3:** Handle encoding (UTF-8 default)
```lua
local content = fs.readFileSync("file.txt", {encoding = "utf8"})
```

**FR-1.4:** Error on non-existent files
```lua
local ok, err = pcall(fs.readFileSync, "missing.txt")
-- ok: false
-- err: "ENOENT: no such file or directory"
```

#### FR-2: File Writing

**FR-2.1:** Write string to file (create if missing)
```lua
fs.writeFileSync("output.txt", "Hello World")
```

**FR-2.2:** Write binary data
```lua
fs.writeFileSync("output.bin", binary_data)
```

**FR-2.3:** Create parent directories if needed (optional)
```lua
fs.writeFileSync("data/users/alice.json", data, {recursive = true})
```

**FR-2.4:** Atomic writes (temp file + rename pattern)

#### FR-3: Path Operations

**FR-3.1:** Check if path exists
```lua
if fs.existsSync("file.txt") then
    -- File exists
end
```

**FR-3.2:** Get file metadata
```lua
local stat = fs.statSync("file.txt")
-- stat.size: number (bytes)
-- stat.isFile: boolean
-- stat.isDirectory: boolean
-- stat.isSymlink: boolean
-- stat.mtime: number (modified timestamp)
-- stat.ctime: number (created timestamp)
-- stat.mode: number (permissions)
```

#### FR-4: Directory Operations

**FR-4.1:** List directory contents
```lua
local files = fs.readdirSync(".")
-- files: array of strings (filenames)
for _, name in ipairs(files) do
    print(name)
end
```

**FR-4.2:** Create directory
```lua
fs.mkdirSync("data")
```

**FR-4.3:** Create nested directories
```lua
fs.mkdirSync("data/users/profiles", {recursive = true})
```

**FR-4.4:** Remove empty directory
```lua
fs.rmdirSync("data")
```

**FR-4.5:** Remove directory recursively (optional)
```lua
fs.rmdirSync("data", {recursive = true})
```

#### FR-5: File Deletion

**FR-5.1:** Delete file
```lua
fs.unlinkSync("temp.txt")
```

**FR-5.2:** Error if file doesn't exist
```lua
local ok, err = pcall(fs.unlinkSync, "missing.txt")
-- ok: false
-- err: "ENOENT: no such file or directory"
```

### 3.2 Non-Functional Requirements

**NFR-1: Performance**
- File reads < 10ms for files < 1MB
- File writes < 20ms for files < 1MB
- Directory listings < 5ms for < 100 entries
- Stat operations < 1ms

**NFR-2: Security**
- Respect file permissions (don't bypass OS security)
- No path traversal vulnerabilities
- Proper error messages without leaking sensitive paths
- Optional sandbox mode (restrict to working directory)

**NFR-3: Cross-Platform**
- Works on Windows, macOS, Linux
- Handles platform-specific path separators
- Respects platform line endings
- Unicode filename support

**NFR-4: Error Handling**
- Clear, actionable error messages
- Proper error types (ENOENT, EACCES, EISDIR, etc.)
- No panics on user errors
- Graceful handling of permissions issues

**NFR-5: Code Quality**
- Unit test coverage > 80%
- Integration tests for all operations
- Clear, documented code
- Follows Rust best practices

### 3.3 API Design Principles

1. **Node.js Compatibility** - Familiar API for JS developers
2. **Synchronous First** - Simple, blocking operations (async later)
3. **Explicit Naming** - `readFileSync` makes sync nature clear
4. **Lua-Friendly** - Returns Lua types (tables, strings, numbers, booleans)
5. **Error Consistency** - Throw Lua errors with consistent format

---

## 4. Proposed Solutions

### Solution 1: Minimal Implementation (Core Functions Only)

**Description:**  
Implement only the 8 declared functions with basic functionality. No options, no advanced features, just core file operations.

**Architecture:**

```rust
// src/modules/builtins/fs/mod.rs
pub mod lua_bindings;
pub mod operations;

pub use operations::*;

// src/modules/builtins/fs/operations.rs
pub fn read_file_sync(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

pub fn write_file_sync(path: &str, data: &str) -> Result<(), std::io::Error> {
    std::fs::write(path, data)
}

pub fn exists_sync(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

// ... 5 more functions

// src/modules/builtins/fs/lua_bindings.rs
pub fn create_fs_module(lua: &Lua) -> mlua::Result<Table> {
    let fs_table = lua.create_table()?;
    
    // Register each function
    register_read_file_sync(lua, &fs_table)?;
    register_write_file_sync(lua, &fs_table)?;
    // ... 6 more registrations
    
    Ok(fs_table)
}

fn register_read_file_sync(lua: &Lua, table: &Table) -> mlua::Result<()> {
    let read_fn = lua.create_function(|_, path: String| {
        let content = read_file_sync(&path)
            .map_err(|e| mlua::Error::external(e))?;
        Ok(content)
    })?;
    table.set("readFileSync", read_fn)?;
    Ok(())
}
```

**Lua API:**

```lua
local fs = require("fs")

-- Read file (string only)
local content = fs.readFileSync("file.txt")

-- Write file (string only)
fs.writeFileSync("output.txt", "Hello")

-- Check existence
if fs.existsSync("file.txt") then end

-- Get metadata (basic)
local stat = fs.statSync("file.txt")
-- stat: {size, isFile, isDirectory, mtime}

-- List directory
local files = fs.readdirSync(".")

-- Create directory (non-recursive)
fs.mkdirSync("dir")

-- Delete file/directory
fs.unlinkSync("file.txt")
fs.rmdirSync("dir")
```

**Pros:**
- ✅ **Fast implementation** - 8-12 hours
- ✅ **Simple code** - Easy to understand and maintain
- ✅ **Low complexity** - Minimal error surface
- ✅ **Covers 80% of use cases**
- ✅ **Unblocks users immediately**

**Cons:**
- ❌ **No options** - No encoding, recursive, or advanced features
- ❌ **Limited stat()** - Only basic metadata
- ❌ **String-only** - No binary file support
- ❌ **No safety** - No path validation or security
- ❌ **No recursive mkdir** - Common use case missing

**Use Cases:**
- ✅ Reading config files (JSON, TOML, text)
- ✅ Writing output files
- ✅ Checking file existence
- ✅ Basic directory listings
- ❌ Binary files (images, videos)
- ❌ Nested directory creation
- ❌ Secure file operations

---

### Solution 2: Full-Featured Implementation (Node.js Parity)

**Description:**  
Complete implementation matching Node.js `fs` module API with options, binary support, permissions, and advanced features.

**Architecture:**

```rust
// src/modules/builtins/fs/mod.rs
pub mod error;
pub mod lua_bindings;
pub mod operations;
pub mod stat;
pub mod types;

pub use error::FsError;
pub use operations::*;
pub use stat::FileStat;
pub use types::*;

// src/modules/builtins/fs/types.rs
pub struct ReadOptions {
    pub encoding: Option<String>,  // "utf8", "binary"
    pub flag: Option<String>,       // "r", "r+"
}

pub struct WriteOptions {
    pub encoding: Option<String>,
    pub mode: Option<u32>,          // File permissions
    pub flag: Option<String>,       // "w", "a", "w+"
    pub recursive: bool,            // Create parent dirs
}

pub struct MkdirOptions {
    pub recursive: bool,
    pub mode: Option<u32>,
}

pub struct RmdirOptions {
    pub recursive: bool,
    pub force: bool,
}

// src/modules/builtins/fs/stat.rs
pub struct FileStat {
    pub size: u64,
    pub is_file: bool,
    pub is_directory: bool,
    pub is_symlink: bool,
    pub mtime: u64,
    pub ctime: u64,
    pub atime: u64,
    pub mode: u32,
    pub uid: Option<u32>,
    pub gid: Option<u32>,
}

// src/modules/builtins/fs/operations.rs
pub fn read_file_sync(path: &str, options: ReadOptions) -> Result<Vec<u8>, FsError> {
    // Validate path
    validate_path(path)?;
    
    // Read file
    let data = std::fs::read(path)?;
    
    // Handle encoding
    match options.encoding.as_deref() {
        Some("utf8") | None => {
            // Validate UTF-8
            String::from_utf8(data.clone())
                .map_err(|_| FsError::InvalidEncoding)?;
        }
        Some("binary") => {
            // Return raw bytes
        }
        _ => return Err(FsError::InvalidEncoding),
    }
    
    Ok(data)
}

pub fn write_file_sync(
    path: &str,
    data: &[u8],
    options: WriteOptions
) -> Result<(), FsError> {
    // Validate path
    validate_path(path)?;
    
    // Create parent directories if recursive
    if options.recursive {
        if let Some(parent) = Path::new(path).parent() {
            std::fs::create_dir_all(parent)?;
        }
    }
    
    // Write atomically (temp + rename)
    let temp_path = format!("{}.tmp.{}", path, random_suffix());
    std::fs::write(&temp_path, data)?;
    std::fs::rename(&temp_path, path)?;
    
    // Set permissions
    if let Some(mode) = options.mode {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let perms = std::fs::Permissions::from_mode(mode);
            std::fs::set_permissions(path, perms)?;
        }
    }
    
    Ok(())
}

pub fn mkdir_sync(path: &str, options: MkdirOptions) -> Result<(), FsError> {
    validate_path(path)?;
    
    if options.recursive {
        std::fs::create_dir_all(path)?;
    } else {
        std::fs::create_dir(path)?;
    }
    
    // Set permissions
    if let Some(mode) = options.mode {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let perms = std::fs::Permissions::from_mode(mode);
            std::fs::set_permissions(path, perms)?;
        }
    }
    
    Ok(())
}

pub fn stat_sync(path: &str) -> Result<FileStat, FsError> {
    let metadata = std::fs::metadata(path)?;
    
    Ok(FileStat {
        size: metadata.len(),
        is_file: metadata.is_file(),
        is_directory: metadata.is_dir(),
        is_symlink: metadata.is_symlink(),
        mtime: metadata.modified()?.duration_since(UNIX_EPOCH)?.as_secs(),
        ctime: metadata.created()?.duration_since(UNIX_EPOCH)?.as_secs(),
        atime: get_atime(&metadata),
        mode: get_mode(&metadata),
        uid: get_uid(&metadata),
        gid: get_gid(&metadata),
    })
}

fn validate_path(path: &str) -> Result<(), FsError> {
    // Check for null bytes
    if path.contains('\0') {
        return Err(FsError::InvalidPath("Path contains null byte".into()));
    }
    
    // Check for path traversal (optional security)
    if path.contains("..") {
        // Resolve and check if within allowed directory
    }
    
    Ok(())
}

// src/modules/builtins/fs/lua_bindings.rs
pub fn create_fs_module(lua: &Lua) -> mlua::Result<Table> {
    let fs_table = lua.create_table()?;
    
    register_read_file_sync(lua, &fs_table)?;
    register_write_file_sync(lua, &fs_table)?;
    register_exists_sync(lua, &fs_table)?;
    register_stat_sync(lua, &fs_table)?;
    register_readdir_sync(lua, &fs_table)?;
    register_unlink_sync(lua, &fs_table)?;
    register_mkdir_sync(lua, &fs_table)?;
    register_rmdir_sync(lua, &fs_table)?;
    
    Ok(fs_table)
}

fn register_read_file_sync(lua: &Lua, table: &Table) -> mlua::Result<()> {
    let read_fn = lua.create_function(
        |lua, (path, options): (String, Option<Table>)|
    {
        let opts = parse_read_options(options)?;
        let data = read_file_sync(&path, opts)
            .map_err(|e| mlua::Error::external(e))?;
        
        // Return as Lua string
        Ok(lua.create_string(&data)?)
    })?;
    table.set("readFileSync", read_fn)?;
    Ok(())
}

fn register_stat_sync(lua: &Lua, table: &Table) -> mlua::Result<()> {
    let stat_fn = lua.create_function(move |lua, path: String| {
        let stat = stat_sync(&path)
            .map_err(|e| mlua::Error::external(e))?;
        
        let table = lua.create_table()?;
        table.set("size", stat.size)?;
        table.set("isFile", stat.is_file)?;
        table.set("isDirectory", stat.is_directory)?;
        table.set("isSymlink", stat.is_symlink)?;
        table.set("mtime", stat.mtime)?;
        table.set("ctime", stat.ctime)?;
        table.set("atime", stat.atime)?;
        table.set("mode", stat.mode)?;
        
        Ok(table)
    })?;
    table.set("statSync", stat_fn)?;
    Ok(())
}
```

**Lua API:**

```lua
local fs = require("fs")

-- Read with encoding
local text = fs.readFileSync("file.txt", {encoding = "utf8"})
local binary = fs.readFileSync("image.png", {encoding = "binary"})

-- Write with options
fs.writeFileSync("output.txt", data, {
    encoding = "utf8",
    mode = 0o644,
    recursive = true  -- Create parent dirs
})

-- Advanced stat
local stat = fs.statSync("file.txt")
-- Full metadata: size, isFile, isDirectory, isSymlink,
-- mtime, ctime, atime, mode, uid, gid

-- Recursive mkdir
fs.mkdirSync("data/users/profiles", {recursive = true, mode = 0o755})

-- Recursive rmdir
fs.rmdirSync("data", {recursive = true, force = true})
```

**Pros:**
- ✅ **Complete feature set** - All Node.js features
- ✅ **Binary support** - Read/write any file type
- ✅ **Options support** - Encoding, permissions, recursive
- ✅ **Full metadata** - Complete file information
- ✅ **Security** - Path validation, atomic writes
- ✅ **Future-proof** - Easy to extend
- ✅ **Professional** - Production-ready quality

**Cons:**
- ❌ **High complexity** - ~400-500 lines of code
- ❌ **Longer implementation** - 20-28 hours
- ❌ **More testing needed** - Many edge cases
- ❌ **Platform-specific code** - Unix vs Windows handling
- ❌ **Maintenance burden** - More code to maintain

**Use Cases:**
- ✅ **All use cases from Solution 1**
- ✅ Binary files (images, videos, archives)
- ✅ Nested directory creation
- ✅ Secure file operations
- ✅ Advanced metadata access
- ✅ Permission management
- ✅ Atomic file writes

---

### Solution 3: Hybrid Approach (Essential + Progressive Enhancement)

**Description:**  
Start with essential functions (Solution 1) but design APIs to accept future options. Add advanced features incrementally based on user demand.

**Architecture:**

```rust
// Phase 1: Core implementation (like Solution 1)
// But with extensible signatures

pub fn read_file_sync(
    path: &str,
    options: Option<ReadOptions>
) -> Result<Vec<u8>, FsError> {
    let opts = options.unwrap_or_default();
    
    // Phase 1: Ignore options, just read as UTF-8
    let content = std::fs::read_to_string(path)?;
    Ok(content.into_bytes())
    
    // Phase 2: Add encoding support
    // Phase 3: Add flag support
}

pub fn write_file_sync(
    path: &str,
    data: &[u8],
    options: Option<WriteOptions>
) -> Result<(), FsError> {
    let opts = options.unwrap_or_default();
    
    // Phase 1: Simple write
    std::fs::write(path, data)?;
    
    // Phase 2: Add recursive
    // if opts.recursive { create_dir_all(parent)?; }
    
    // Phase 3: Add atomic writes
    // Phase 4: Add permissions
    
    Ok(())
}

pub fn mkdir_sync(
    path: &str,
    options: Option<MkdirOptions>
) -> Result<(), FsError> {
    let opts = options.unwrap_or_default();
    
    // Phase 1: Always recursive (safer default)
    std::fs::create_dir_all(path)?;
    
    // Phase 2: Honor recursive option
    // if opts.recursive { create_dir_all } else { create_dir }
    
    // Phase 3: Add permissions
    
    Ok(())
}
```

**Lua API (Phase 1):**

```lua
local fs = require("fs")

-- Phase 1: Simple API (options ignored)
local content = fs.readFileSync("file.txt")
local content = fs.readFileSync("file.txt", {encoding = "utf8"})  -- Works, ignores options

fs.writeFileSync("output.txt", data)
fs.writeFileSync("output.txt", data, {recursive = true})  -- Works, ignores options

-- mkdir is always recursive in Phase 1 (safer)
fs.mkdirSync("data/users/profiles")  -- Creates all parents
```

**Lua API (Phase 2 - Future):**

```lua
-- Phase 2: Options work
local binary = fs.readFileSync("image.png", {encoding = "binary"})
fs.writeFileSync("out.txt", data, {recursive = true})  -- Actually creates parents
fs.mkdirSync("dir", {recursive = false})  -- Only creates last dir

-- Phase 3: Advanced features
local stat = fs.statSync("file.txt")
-- Full metadata including uid, gid, atime
```

**Implementation Phases:**

**Phase 1 (v0.3.0) - 12-16 hours:**
- ✅ 8 core functions implemented
- ✅ UTF-8 text file support
- ✅ mkdir always recursive (safe default)
- ✅ Basic error handling
- ✅ Basic tests (>70% coverage)

**Phase 2 (v0.4.0) - 6-8 hours:**
- ✅ Binary file support
- ✅ Options parsing (encoding, recursive)
- ✅ Atomic writes for writeFileSync
- ✅ Enhanced error messages

**Phase 3 (v0.5.0) - 4-6 hours:**
- ✅ Full metadata in statSync
- ✅ Permission management
- ✅ Path validation and security
- ✅ Comprehensive tests (>90% coverage)

**Pros:**
- ✅ **Fast initial delivery** - 12-16 hours for Phase 1
- ✅ **Future-proof design** - APIs support future options
- ✅ **User feedback driven** - Add features based on demand
- ✅ **Balanced complexity** - Not too simple, not too complex
- ✅ **Safe defaults** - mkdir recursive by default prevents errors
- ✅ **Incremental testing** - Test each phase thoroughly
- ✅ **Clear roadmap** - Users know what's coming

**Cons:**
- ⚠️ **Options ignored initially** - May confuse users
- ⚠️ **Multi-phase delivery** - Some features delayed
- ⚠️ **API compatibility** - Must maintain backward compat
- ⚠️ **Documentation burden** - Must document what works when

**Use Cases:**
- ✅ **All Phase 1 use cases** - Text files, basic operations
- 🔄 **Binary files** - Phase 2
- 🔄 **Advanced metadata** - Phase 3
- 🔄 **Security features** - Phase 3

---

## 5. Solution Comparison

### 5.1 Feature Matrix

| Feature | Solution 1: Minimal | Solution 2: Full | Solution 3: Hybrid |
|---------|-------------------|------------------|-------------------|
| **Core Functions** |
| readFileSync (text) | ✅ Yes | ✅ Yes | ✅ Yes |
| readFileSync (binary) | ❌ No | ✅ Yes | 🔄 Phase 2 |
| writeFileSync (text) | ✅ Yes | ✅ Yes | ✅ Yes |
| writeFileSync (binary) | ❌ No | ✅ Yes | 🔄 Phase 2 |
| existsSync | ✅ Yes | ✅ Yes | ✅ Yes |
| statSync (basic) | ✅ Yes | ✅ Yes | ✅ Yes |
| statSync (full) | ❌ No | ✅ Yes | 🔄 Phase 3 |
| readdirSync | ✅ Yes | ✅ Yes | ✅ Yes |
| unlinkSync | ✅ Yes | ✅ Yes | ✅ Yes |
| mkdirSync | ⚠️ Non-recursive | ✅ Both | ✅ Recursive default |
| rmdirSync | ⚠️ Non-recursive | ✅ Both | ✅ Yes |
| **Options** |
| Encoding options | ❌ No | ✅ Yes | 🔄 Phase 2 |
| Recursive options | ❌ No | ✅ Yes | 🔄 Phase 2 |
| Permission options | ❌ No | ✅ Yes | 🔄 Phase 3 |
| **Safety** |
| Path validation | ❌ No | ✅ Yes | 🔄 Phase 3 |
| Atomic writes | ❌ No | ✅ Yes | 🔄 Phase 2 |
| Security checks | ❌ No | ✅ Yes | 🔄 Phase 3 |
| **Quality** |
| Error handling | ⚠️ Basic | ✅ Comprehensive | ✅ Progressive |
| Test coverage | ⚠️ 60-70% | ✅ 90%+ | ✅ 70-90% |
| Documentation | ⚠️ Basic | ✅ Complete | ✅ Progressive |

### 5.2 Implementation Effort

| Task | Solution 1 | Solution 2 | Solution 3 |
|------|-----------|-----------|-----------|
| Module structure | 1h | 2h | 1.5h |
| Core operations | 3-4h | 8-10h | 4-5h |
| Lua bindings | 2-3h | 5-6h | 3-4h |
| Error handling | 1h | 3-4h | 2h |
| Options parsing | 0h | 3-4h | 1h (Phase 1) |
| Security features | 0h | 3-4h | 0h (Phase 1) |
| Testing | 2-3h | 6-8h | 3-4h (Phase 1) |
| Documentation | 1h | 2-3h | 1-2h |
| **Total Phase 1** | **10-14h** | **32-40h** | **15-20h** |
| **Total All Phases** | **10-14h** | **32-40h** | **25-34h** |

### 5.3 Risk Assessment

**Solution 1: Minimal**
- 🟢 **Scope creep** - LOW (limited features)
- 🟢 **Technical risk** - LOW (simple code)
- 🟡 **User satisfaction** - MEDIUM (missing features)
- 🔴 **Future extensibility** - HIGH (hard to add features)
- 🟢 **Time risk** - LOW (quick delivery)

**Solution 2: Full**
- 🔴 **Scope creep** - HIGH (many features)
- 🟡 **Technical risk** - MEDIUM (platform-specific)
- 🟢 **User satisfaction** - HIGH (complete)
- 🟢 **Future extensibility** - LOW (already complete)
- 🔴 **Time risk** - HIGH (long implementation)

**Solution 3: Hybrid**
- 🟢 **Scope creep** - LOW (phased approach)
- 🟢 **Technical risk** - LOW (incremental)
- 🟢 **User satisfaction** - HIGH (now + future)
- 🟢 **Future extensibility** - HIGH (designed in)
- 🟢 **Time risk** - LOW (Phase 1 quick)

### 5.4 User Experience Comparison

**Scenario 1: Reading a config file**

```lua
-- Solution 1
local config = fs.readFileSync("config.json")
local data = json.decode(config)

-- Solution 2
local config = fs.readFileSync("config.json", {encoding = "utf8"})
local data = json.decode(config)

-- Solution 3 (Phase 1)
local config = fs.readFileSync("config.json")
local data = json.decode(config)
```

**Winner:** Tie (all work equally well)

**Scenario 2: Reading a binary file**

```lua
-- Solution 1
-- NOT POSSIBLE ❌

-- Solution 2
local bytes = fs.readFileSync("image.png", {encoding = "binary"})

-- Solution 3
-- Phase 1: NOT POSSIBLE ❌
-- Phase 2: Same as Solution 2 ✅
```

**Winner:** Solution 2 now, Solution 3 future

**Scenario 3: Creating nested directories**

```lua
-- Solution 1
fs.mkdirSync("data")  -- Only creates 'data'
-- ERROR: parent directory 'data' doesn't exist
fs.mkdirSync("data/users")  -- ❌ Fails!

-- Solution 2
fs.mkdirSync("data/users/profiles", {recursive = true})  -- ✅ Works

-- Solution 3 (Phase 1)
fs.mkdirSync("data/users/profiles")  -- ✅ Works! (recursive by default)
```

**Winner:** Solutions 2 & 3

**Scenario 4: Checking if file exists**

```lua
-- All solutions identical
if fs.existsSync("file.txt") then
    -- File exists
end
```

**Winner:** Tie

---

## 6. Recommended Solution

### 6.1 Decision: Solution 3 (Hybrid Approach)

**Rationale:**

1. **Balanced delivery timeline**
   - Phase 1 delivers in 15-20 hours (acceptable)
   - Unblocks users immediately with core functionality
   - Provides 80% of use cases in Phase 1

2. **Risk management**
   - Lower implementation risk than Solution 2
   - Defined scope prevents scope creep
   - User feedback informs Phase 2/3 priorities

3. **User experience**
   - Core operations work immediately
   - Safe defaults (mkdir recursive) prevent errors
   - Clear upgrade path to advanced features

4. **Future-proof**
   - API design allows natural extension
   - Can add features based on real user demand
   - Not over-engineering for hypothetical needs

5. **Best ROI**
   - 80% of value in 60% of the time
   - Remaining 20% of value added based on demand
   - Focuses resources on proven needs

**Trade-offs accepted:**
- ❌ Binary file support delayed (Phase 2)
- ❌ Advanced metadata delayed (Phase 3)
- ❌ Security features delayed (Phase 3)
- ✅ Core text file operations work now
- ✅ Safe defaults prevent common errors
- ✅ API remains simple and familiar

### 6.2 Delivery Phases

**Phase 1 (v0.3.0) - This PRP**

Core features (15-20 hours):
- ✅ readFileSync (UTF-8 text)
- ✅ writeFileSync (UTF-8 text)
- ✅ existsSync
- ✅ statSync (basic: size, type, mtime)
- ✅ readdirSync
- ✅ unlinkSync
- ✅ mkdirSync (recursive by default)
- ✅ rmdirSync
- ✅ Basic error handling
- ✅ 70%+ test coverage

**Phase 2 (v0.4.0) - Future PRP**

Enhanced features (6-8 hours):
- 🔄 Binary file support
- 🔄 Options parsing (encoding, recursive)
- 🔄 Atomic writes
- 🔄 Enhanced error messages
- 🔄 80%+ test coverage

**Phase 3 (v0.5.0) - Future PRP**

Advanced features (4-6 hours):
- 🔄 Full metadata (uid, gid, atime, permissions)
- 🔄 Permission management
- 🔄 Path validation and security
- 🔄 90%+ test coverage

---

## 7. Implementation Plan

### 7.1 Phase 1: Core Implementation (15-20 hours)

#### Step 1: Create Module Structure (1.5 hours)

**Files to create:**
- `src/modules/builtins/fs/operations.rs`
- `src/modules/builtins/fs/lua_bindings.rs`
- `src/modules/builtins/fs/error.rs`

**Update:**
- `src/modules/builtins/fs/mod.rs` - Export submodules

**File:** `src/modules/builtins/fs/error.rs`

```rust
use std::fmt;

#[derive(Debug)]
pub enum FsError {
    IoError(std::io::Error),
    InvalidPath(String),
    PermissionDenied(String),
    NotFound(String),
    AlreadyExists(String),
    InvalidOperation(String),
}

impl fmt::Display for FsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FsError::IoError(e) => write!(f, "IO error: {}", e),
            FsError::InvalidPath(msg) => write!(f, "Invalid path: {}", msg),
            FsError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            FsError::NotFound(msg) => write!(f, "Not found: {}", msg),
            FsError::AlreadyExists(msg) => write!(f, "Already exists: {}", msg),
            FsError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
        }
    }
}

impl std::error::Error for FsError {}

impl From<std::io::Error> for FsError {
    fn from(err: std::io::Error) -> Self {
        use std::io::ErrorKind;
        match err.kind() {
            ErrorKind::NotFound => FsError::NotFound(err.to_string()),
            ErrorKind::PermissionDenied => FsError::PermissionDenied(err.to_string()),
            ErrorKind::AlreadyExists => FsError::AlreadyExists(err.to_string()),
            _ => FsError::IoError(err),
        }
    }
}
```

#### Step 2: Implement Core Operations (4-5 hours)

**File:** `src/modules/builtins/fs/operations.rs`

```rust
use std::fs;
use std::path::Path;
use super::error::FsError;

pub type Result<T> = std::result::Result<T, FsError>;

pub fn read_file_sync(path: &str) -> Result<String> {
    fs::read_to_string(path).map_err(Into::into)
}

pub fn write_file_sync(path: &str, data: &str) -> Result<()> {
    fs::write(path, data).map_err(Into::into)
}

pub fn exists_sync(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn stat_sync(path: &str) -> Result<FileStat> {
    let metadata = fs::metadata(path)?;
    
    Ok(FileStat {
        size: metadata.len(),
        is_file: metadata.is_file(),
        is_directory: metadata.is_dir(),
        is_symlink: metadata.file_type().is_symlink(),
        mtime: metadata.modified()?
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0),
    })
}

pub fn readdir_sync(path: &str) -> Result<Vec<String>> {
    let entries = fs::read_dir(path)?;
    let mut names = Vec::new();
    
    for entry in entries {
        let entry = entry?;
        if let Some(name) = entry.file_name().to_str() {
            names.push(name.to_string());
        }
    }
    
    names.sort();
    Ok(names)
}

pub fn unlink_sync(path: &str) -> Result<()> {
    fs::remove_file(path).map_err(Into::into)
}

pub fn mkdir_sync(path: &str) -> Result<()> {
    fs::create_dir_all(path).map_err(Into::into)
}

pub fn rmdir_sync(path: &str) -> Result<()> {
    let metadata = fs::metadata(path)?;
    
    if !metadata.is_dir() {
        return Err(FsError::InvalidOperation(
            "Path is not a directory".to_string()
        ));
    }
    
    fs::remove_dir(path).map_err(Into::into)
}

pub struct FileStat {
    pub size: u64,
    pub is_file: bool,
    pub is_directory: bool,
    pub is_symlink: bool,
    pub mtime: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_read_write_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let file_path_str = file_path.to_str().unwrap();
        
        write_file_sync(file_path_str, "Hello World").unwrap();
        let content = read_file_sync(file_path_str).unwrap();
        assert_eq!(content, "Hello World");
    }

    #[test]
    fn test_exists() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        
        assert!(!exists_sync(file_path.to_str().unwrap()));
        
        fs::write(&file_path, "test").unwrap();
        assert!(exists_sync(file_path.to_str().unwrap()));
    }

    #[test]
    fn test_mkdir_rmdir() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path().join("test_dir");
        let dir_path_str = dir_path.to_str().unwrap();
        
        mkdir_sync(dir_path_str).unwrap();
        assert!(exists_sync(dir_path_str));
        
        rmdir_sync(dir_path_str).unwrap();
        assert!(!exists_sync(dir_path_str));
    }

    #[test]
    fn test_readdir() {
        let temp_dir = TempDir::new().unwrap();
        
        fs::write(temp_dir.path().join("file1.txt"), "").unwrap();
        fs::write(temp_dir.path().join("file2.txt"), "").unwrap();
        
        let files = readdir_sync(temp_dir.path().to_str().unwrap()).unwrap();
        assert_eq!(files.len(), 2);
        assert!(files.contains(&"file1.txt".to_string()));
        assert!(files.contains(&"file2.txt".to_string()));
    }

    #[test]
    fn test_stat() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "Hello").unwrap();
        
        let stat = stat_sync(file_path.to_str().unwrap()).unwrap();
        assert_eq!(stat.size, 5);
        assert!(stat.is_file);
        assert!(!stat.is_directory);
    }
}
```

#### Step 3: Create Lua Bindings (3-4 hours)

**File:** `src/modules/builtins/fs/lua_bindings.rs`

```rust
use mlua::{Lua, Table};
use super::operations::*;

pub fn create_fs_module(lua: &Lua) -> mlua::Result<Table> {
    let fs_table = lua.create_table()?;

    register_read_file_sync(lua, &fs_table)?;
    register_write_file_sync(lua, &fs_table)?;
    register_exists_sync(lua, &fs_table)?;
    register_stat_sync(lua, &fs_table)?;
    register_readdir_sync(lua, &fs_table)?;
    register_unlink_sync(lua, &fs_table)?;
    register_mkdir_sync(lua, &fs_table)?;
    register_rmdir_sync(lua, &fs_table)?;

    Ok(fs_table)
}

fn register_read_file_sync(lua: &Lua, table: &Table) -> mlua::Result<()> {
    let read_fn = lua.create_function(|_, path: String| {
        let content = read_file_sync(&path)
            .map_err(|e| mlua::Error::external(e))?;
        Ok(content)
    })?;
    table.set("readFileSync", read_fn)?;
    Ok(())
}

fn register_write_file_sync(lua: &Lua, table: &Table) -> mlua::Result<()> {
    let write_fn = lua.create_function(|_, (path, data): (String, String)| {
        write_file_sync(&path, &data)
            .map_err(|e| mlua::Error::external(e))?;
        Ok(())
    })?;
    table.set("writeFileSync", write_fn)?;
    Ok(())
}

fn register_exists_sync(lua: &Lua, table: &Table) -> mlua::Result<()> {
    let exists_fn = lua.create_function(|_, path: String| {
        Ok(exists_sync(&path))
    })?;
    table.set("existsSync", exists_fn)?;
    Ok(())
}

fn register_stat_sync(lua: &Lua, table: &Table) -> mlua::Result<()> {
    let stat_fn = lua.create_function(move |lua, path: String| {
        let stat = stat_sync(&path)
            .map_err(|e| mlua::Error::external(e))?;
        
        let table = lua.create_table()?;
        table.set("size", stat.size)?;
        table.set("isFile", stat.is_file)?;
        table.set("isDirectory", stat.is_directory)?;
        table.set("isSymlink", stat.is_symlink)?;
        table.set("mtime", stat.mtime)?;
        
        Ok(table)
    })?;
    table.set("statSync", stat_fn)?;
    Ok(())
}

fn register_readdir_sync(lua: &Lua, table: &Table) -> mlua::Result<()> {
    let readdir_fn = lua.create_function(move |lua, path: String| {
        let files = readdir_sync(&path)
            .map_err(|e| mlua::Error::external(e))?;
        
        let table = lua.create_table()?;
        for (i, name) in files.iter().enumerate() {
            table.set(i + 1, name.as_str())?;
        }
        
        Ok(table)
    })?;
    table.set("readdirSync", readdir_fn)?;
    Ok(())
}

fn register_unlink_sync(lua: &Lua, table: &Table) -> mlua::Result<()> {
    let unlink_fn = lua.create_function(|_, path: String| {
        unlink_sync(&path)
            .map_err(|e| mlua::Error::external(e))?;
        Ok(())
    })?;
    table.set("unlinkSync", unlink_fn)?;
    Ok(())
}

fn register_mkdir_sync(lua: &Lua, table: &Table) -> mlua::Result<()> {
    let mkdir_fn = lua.create_function(|_, path: String| {
        mkdir_sync(&path)
            .map_err(|e| mlua::Error::external(e))?;
        Ok(())
    })?;
    table.set("mkdirSync", mkdir_fn)?;
    Ok(())
}

fn register_rmdir_sync(lua: &Lua, table: &Table) -> mlua::Result<()> {
    let rmdir_fn = lua.create_function(|_, path: String| {
        rmdir_sync(&path)
            .map_err(|e| mlua::Error::external(e))?;
        Ok(())
    })?;
    table.set("rmdirSync", rmdir_fn)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_fs_module() {
        let lua = Lua::new();
        let result = create_fs_module(&lua);
        
        assert!(result.is_ok());
        let fs_table = result.unwrap();
        assert!(fs_table.contains_key("readFileSync").unwrap());
        assert!(fs_table.contains_key("writeFileSync").unwrap());
        assert!(fs_table.contains_key("existsSync").unwrap());
        assert!(fs_table.contains_key("statSync").unwrap());
        assert!(fs_table.contains_key("readdirSync").unwrap());
        assert!(fs_table.contains_key("unlinkSync").unwrap());
        assert!(fs_table.contains_key("mkdirSync").unwrap());
        assert!(fs_table.contains_key("rmdirSync").unwrap());
    }
}
```

#### Step 4: Wire Up Module (1 hour)

**File:** `src/modules/builtins/fs/mod.rs`

```rust
pub mod error;
pub mod lua_bindings;
pub mod operations;

use serde_json::{json, Value as JsonValue};

use super::BuiltinModule;
use crate::error::HypeError;

pub use error::FsError;
pub use lua_bindings::create_fs_module;
pub use operations::*;

pub struct FsModule;

impl FsModule {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FsModule {
    fn default() -> Self {
        Self::new()
    }
}

impl BuiltinModule for FsModule {
    fn name(&self) -> &str {
        "fs"
    }

    fn exports(&self) -> Result<JsonValue, HypeError> {
        Ok(json!({
            "__id": "fs",
            "__desc": "Filesystem operations module",
            "readFileSync": {
                "__fn": "readFileSync",
                "__desc": "Read file synchronously",
                "__signature": "readFileSync(path: string) -> string"
            },
            "writeFileSync": {
                "__fn": "writeFileSync",
                "__desc": "Write file synchronously",
                "__signature": "writeFileSync(path: string, data: string) -> nil"
            },
            "existsSync": {
                "__fn": "existsSync",
                "__desc": "Check if file exists",
                "__signature": "existsSync(path: string) -> boolean"
            },
            "statSync": {
                "__fn": "statSync",
                "__desc": "Get file statistics",
                "__signature": "statSync(path: string) -> {size, isFile, isDirectory, isSymlink, mtime}"
            },
            "readdirSync": {
                "__fn": "readdirSync",
                "__desc": "Read directory contents",
                "__signature": "readdirSync(path: string) -> string[]"
            },
            "unlinkSync": {
                "__fn": "unlinkSync",
                "__desc": "Delete file",
                "__signature": "unlinkSync(path: string) -> nil"
            },
            "mkdirSync": {
                "__fn": "mkdirSync",
                "__desc": "Create directory (recursive)",
                "__signature": "mkdirSync(path: string) -> nil"
            },
            "rmdirSync": {
                "__fn": "rmdirSync",
                "__desc": "Remove directory",
                "__signature": "rmdirSync(path: string) -> nil"
            }
        }))
    }
}
```

**Update:** `src/lua/require.rs` to register fs module

```rust
// In setup_require_fn function, add:
globals.set("require", require_fn)?;

// Register fs module
let fs_module = crate::modules::builtins::fs::create_fs_module(lua)?;
package_loaded.set("fs", fs_module)?;
```

#### Step 5: Testing (3-4 hours)

**File:** `tests/fs_operations_test.rs`

```rust
#[cfg(test)]
mod tests {
    use hype_rs::modules::builtins::fs::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_read_write_text_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let file_path_str = file_path.to_str().unwrap();
        
        write_file_sync(file_path_str, "Hello World").unwrap();
        let content = read_file_sync(file_path_str).unwrap();
        
        assert_eq!(content, "Hello World");
    }

    #[test]
    fn test_read_nonexistent_file() {
        let result = read_file_sync("/nonexistent/file.txt");
        assert!(result.is_err());
    }

    #[test]
    fn test_write_creates_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("new_file.txt");
        let file_path_str = file_path.to_str().unwrap();
        
        write_file_sync(file_path_str, "Content").unwrap();
        assert!(exists_sync(file_path_str));
    }

    #[test]
    fn test_exists_sync() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        
        assert!(!exists_sync(file_path.to_str().unwrap()));
        
        fs::write(&file_path, "test").unwrap();
        assert!(exists_sync(file_path.to_str().unwrap()));
    }

    #[test]
    fn test_stat_sync() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "12345").unwrap();
        
        let stat = stat_sync(file_path.to_str().unwrap()).unwrap();
        
        assert_eq!(stat.size, 5);
        assert!(stat.is_file);
        assert!(!stat.is_directory);
        assert!(!stat.is_symlink);
        assert!(stat.mtime > 0);
    }

    #[test]
    fn test_readdir_sync() {
        let temp_dir = TempDir::new().unwrap();
        
        fs::write(temp_dir.path().join("file1.txt"), "").unwrap();
        fs::write(temp_dir.path().join("file2.txt"), "").unwrap();
        fs::create_dir(temp_dir.path().join("subdir")).unwrap();
        
        let files = readdir_sync(temp_dir.path().to_str().unwrap()).unwrap();
        
        assert_eq!(files.len(), 3);
        assert!(files.contains(&"file1.txt".to_string()));
        assert!(files.contains(&"file2.txt".to_string()));
        assert!(files.contains(&"subdir".to_string()));
    }

    #[test]
    fn test_unlink_sync() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "test").unwrap();
        
        assert!(exists_sync(file_path.to_str().unwrap()));
        
        unlink_sync(file_path.to_str().unwrap()).unwrap();
        
        assert!(!exists_sync(file_path.to_str().unwrap()));
    }

    #[test]
    fn test_mkdir_sync_recursive() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path().join("a").join("b").join("c");
        
        mkdir_sync(dir_path.to_str().unwrap()).unwrap();
        
        assert!(exists_sync(dir_path.to_str().unwrap()));
    }

    #[test]
    fn test_rmdir_sync() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path().join("test_dir");
        fs::create_dir(&dir_path).unwrap();
        
        assert!(exists_sync(dir_path.to_str().unwrap()));
        
        rmdir_sync(dir_path.to_str().unwrap()).unwrap();
        
        assert!(!exists_sync(dir_path.to_str().unwrap()));
    }

    #[test]
    fn test_rmdir_nonempty_fails() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("file.txt"), "").unwrap();
        
        let result = rmdir_sync(temp_dir.path().to_str().unwrap());
        assert!(result.is_err());
    }

    #[test]
    fn test_utf8_content() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("utf8.txt");
        let file_path_str = file_path.to_str().unwrap();
        
        let content = "Hello 世界 🌍";
        write_file_sync(file_path_str, content).unwrap();
        
        let read_content = read_file_sync(file_path_str).unwrap();
        assert_eq!(read_content, content);
    }
}
```

**File:** `tests/lua_scripts/test_fs_operations.lua`

```lua
local fs = require("fs")

print("=== Testing Filesystem Module ===\n")

local temp_dir = "/tmp/hype_fs_test_" .. os.time()
local failed = false

print("1. Testing mkdirSync()...")
fs.mkdirSync(temp_dir .. "/data/users")
if fs.existsSync(temp_dir .. "/data/users") then
    print("✓ mkdirSync creates nested directories")
else
    print("✗ mkdirSync failed")
    failed = true
end

print("\n2. Testing writeFileSync()...")
fs.writeFileSync(temp_dir .. "/test.txt", "Hello World")
if fs.existsSync(temp_dir .. "/test.txt") then
    print("✓ writeFileSync creates file")
else
    print("✗ writeFileSync failed")
    failed = true
end

print("\n3. Testing readFileSync()...")
local content = fs.readFileSync(temp_dir .. "/test.txt")
if content == "Hello World" then
    print("✓ readFileSync reads content correctly")
else
    print("✗ readFileSync returned: " .. content)
    failed = true
end

print("\n4. Testing statSync()...")
local stat = fs.statSync(temp_dir .. "/test.txt")
if stat.size == 11 and stat.isFile and not stat.isDirectory then
    print("✓ statSync returns correct metadata")
else
    print("✗ statSync failed")
    failed = true
end

print("\n5. Testing readdirSync()...")
fs.writeFileSync(temp_dir .. "/file1.txt", "")
fs.writeFileSync(temp_dir .. "/file2.txt", "")
local files = fs.readdirSync(temp_dir)
if #files >= 3 then
    print("✓ readdirSync lists files")
else
    print("✗ readdirSync failed")
    failed = true
end

print("\n6. Testing unlinkSync()...")
fs.unlinkSync(temp_dir .. "/file1.txt")
if not fs.existsSync(temp_dir .. "/file1.txt") then
    print("✓ unlinkSync deletes file")
else
    print("✗ unlinkSync failed")
    failed = true
end

print("\n7. Testing UTF-8 content...")
fs.writeFileSync(temp_dir .. "/utf8.txt", "Hello 世界 🌍")
local utf8_content = fs.readFileSync(temp_dir .. "/utf8.txt")
if utf8_content == "Hello 世界 🌍" then
    print("✓ UTF-8 content preserved")
else
    print("✗ UTF-8 content corrupted")
    failed = true
end

print("\n8. Testing error handling...")
local ok, err = pcall(fs.readFileSync, temp_dir .. "/nonexistent.txt")
if not ok then
    print("✓ Error thrown for missing file")
else
    print("✗ Should have thrown error")
    failed = true
end

print("\n9. Cleanup...")
fs.unlinkSync(temp_dir .. "/test.txt")
fs.unlinkSync(temp_dir .. "/file2.txt")
fs.unlinkSync(temp_dir .. "/utf8.txt")
fs.rmdirSync(temp_dir .. "/data/users")
fs.rmdirSync(temp_dir .. "/data")
fs.rmdirSync(temp_dir)

if not fs.existsSync(temp_dir) then
    print("✓ Cleanup successful")
else
    print("? Cleanup incomplete")
end

if failed then
    print("\n=== Some Tests Failed ===")
    error("Tests failed")
else
    print("\n=== All Tests Passed ===")
end
```

#### Step 6: Documentation (1-2 hours)

Update `CHANGELOG.md`:

```markdown
## [0.3.0] - 2025-10-29

### Added
- Filesystem module (`fs`) now fully functional (PRP-013)
  - `fs.readFileSync(path)` - Read text files
  - `fs.writeFileSync(path, data)` - Write text files
  - `fs.existsSync(path)` - Check if path exists
  - `fs.statSync(path)` - Get file metadata (size, type, mtime)
  - `fs.readdirSync(path)` - List directory contents
  - `fs.unlinkSync(path)` - Delete files
  - `fs.mkdirSync(path)` - Create directories (recursive by default)
  - `fs.rmdirSync(path)` - Remove empty directories
  - UTF-8 text file support
  - Cross-platform path handling
  - Proper error handling with Lua-friendly error messages

### Changed
- `mkdirSync` now creates parent directories automatically (safer default)
- Version bumped: 0.2.0 → 0.3.0

### Technical Details
- All operations use Rust std::fs for reliability
- Comprehensive error handling with custom FsError types
- 80%+ test coverage with unit and integration tests
- Files sorted alphabetically in readdirSync for consistent ordering
```

### 7.2 Phase 2: Enhanced Features (6-8 hours - Future)

**To be implemented in v0.4.0:**
- Binary file support (encoding option)
- Options parsing for all functions
- Atomic writes for writeFileSync
- Enhanced error messages with error codes
- 90% test coverage

### 7.3 Phase 3: Advanced Features (4-6 hours - Future)

**To be implemented in v0.5.0:**
- Full metadata (uid, gid, atime, permissions)
- Permission management (chmod equivalent)
- Path validation and security checks
- Sandbox mode (restrict to working directory)
- 95% test coverage

---

## 8. Success Criteria

### 8.1 Functional Criteria

✅ **Must Have (Phase 1):**
1. All 8 functions implemented and callable from Lua
2. Text files (UTF-8) can be read and written
3. File and directory existence checks work
4. Directory listings return sorted filenames
5. Basic file metadata accessible (size, type, mtime)
6. Directories created recursively by default
7. Files and empty directories can be deleted
8. All example scripts work without errors

✅ **Should Have:**
9. Cross-platform compatibility (Windows, macOS, Linux)
10. UTF-8 filename and content support
11. Proper error handling with clear messages
12. No crashes or panics on user errors

### 8.2 Non-Functional Criteria

✅ **Performance:**
- readFileSync < 10ms for files < 1MB
- writeFileSync < 20ms for files < 1MB
- existsSync < 1ms
- statSync < 1ms
- readdirSync < 5ms for < 100 entries
- mkdirSync < 10ms for nested paths

✅ **Security:**
- Respects file permissions
- No path traversal vulnerabilities in Phase 1
- Clear error messages without leaking paths
- No buffer overflows or memory leaks

✅ **Quality:**
- Unit test coverage > 70%
- Integration test coverage > 80%
- All tests pass on all platforms
- No clippy warnings in new code
- Code formatted with cargo fmt

✅ **Documentation:**
- CHANGELOG updated for v0.3.0
- All functions have doc comments
- Examples updated to work with real fs module
- Clear migration guide from current state

### 8.3 Acceptance Tests

**Test 1: Read/Write Text File**
```lua
local fs = require("fs")
fs.writeFileSync("test.txt", "Hello World")
local content = fs.readFileSync("test.txt")
assert(content == "Hello World")
fs.unlinkSync("test.txt")
```
**Expected:** Pass ✅

**Test 2: Create Nested Directories**
```lua
fs.mkdirSync("a/b/c/d")
assert(fs.existsSync("a/b/c/d"))
fs.rmdirSync("a/b/c/d")
fs.rmdirSync("a/b/c")
fs.rmdirSync("a/b")
fs.rmdirSync("a")
```
**Expected:** Pass ✅

**Test 3: List Directory**
```lua
fs.mkdirSync("test_dir")
fs.writeFileSync("test_dir/file1.txt", "")
fs.writeFileSync("test_dir/file2.txt", "")
local files = fs.readdirSync("test_dir")
assert(#files == 2)
assert(files[1] == "file1.txt")
assert(files[2] == "file2.txt")
```
**Expected:** Pass ✅

**Test 4: Get File Stats**
```lua
fs.writeFileSync("test.txt", "12345")
local stat = fs.statSync("test.txt")
assert(stat.size == 5)
assert(stat.isFile == true)
assert(stat.isDirectory == false)
assert(stat.mtime > 0)
```
**Expected:** Pass ✅

**Test 5: Error Handling**
```lua
local ok, err = pcall(fs.readFileSync, "nonexistent.txt")
assert(ok == false)
assert(type(err) == "string")
```
**Expected:** Pass ✅

**Test 6: UTF-8 Support**
```lua
fs.writeFileSync("utf8.txt", "Hello 世界 🌍")
local content = fs.readFileSync("utf8.txt")
assert(content == "Hello 世界 🌍")
```
**Expected:** Pass ✅

**Test 7: Examples Work**
```bash
hype examples/file-operations.lua
```
**Expected:** Runs without errors ✅

---

## 9. Risk Assessment

### 9.1 Technical Risks

| Risk | Severity | Probability | Mitigation |
|------|----------|-------------|------------|
| Platform differences (Windows paths) | 🟡 Medium | 🟡 Medium | Use std::path for path handling, test on all platforms |
| UTF-8 encoding issues | 🟡 Medium | 🟡 Medium | Validate UTF-8, return clear errors for invalid encoding |
| Permission errors | 🟢 Low | 🟡 Medium | Clear error messages, respect OS permissions |
| Path traversal security | 🟡 Medium | 🟢 Low | Defer to Phase 3, document limitations |
| File locking conflicts | 🟢 Low | 🟢 Low | Use standard OS file operations |
| Memory leaks with large files | 🟡 Medium | 🟢 Low | Test with various file sizes, use streaming in Phase 2 |

### 9.2 User Impact Risks

| Risk | Severity | Mitigation |
|------|----------|------------|
| Breaking changes from current (non-working) state | 🟢 LOW | Current fs module doesn't work, so no breaking changes |
| Users expect Node.js exact compatibility | 🟡 MEDIUM | Document differences, focus on common use cases |
| mkdir recursive behavior surprise | 🟢 LOW | Document that recursive is default, safer for users |
| Error messages confusing | 🟡 MEDIUM | Use clear, actionable error messages |
| Cross-platform path issues | 🟡 MEDIUM | Extensive testing on all platforms |

### 9.3 Implementation Risks

| Risk | Severity | Mitigation |
|------|----------|------------|
| Scope creep (adding Phase 2/3 features) | 🟡 MEDIUM | Strict adherence to Phase 1 scope |
| Testing insufficient | 🟡 MEDIUM | Aim for >70% coverage, write tests first |
| Integration with require() breaks | 🟢 LOW | Follow http module pattern exactly |
| Performance issues with large files | 🟡 MEDIUM | Document file size limits, test with various sizes |

### 9.4 Mitigation Strategy

**Phase 1 Focus:**
- Implement only 8 declared functions
- Use std::fs for all operations (battle-tested)
- Follow http module integration pattern
- Comprehensive testing (unit + integration)
- Clear documentation of limitations

**Testing Strategy:**
- Unit tests for all operations
- Integration tests in Lua
- Test on all platforms (CI)
- Test with edge cases (empty files, large files, UTF-8)
- Test error conditions

**Error Handling:**
- Use custom FsError enum
- Convert io::Error to FsError with context
- Clear, actionable error messages
- No panics on user errors

---

## 10. Future Enhancements

### 10.1 Phase 2 (v0.4.0)

**Binary File Support** (3-4 hours)
```lua
local bytes = fs.readFileSync("image.png", {encoding = "binary"})
fs.writeFileSync("output.png", bytes, {encoding = "binary"})
```

**Options Parsing** (2-3 hours)
```lua
fs.writeFileSync("file.txt", data, {
    encoding = "utf8",
    recursive = true  -- Actually creates parent dirs
})

fs.mkdirSync("dir", {recursive = false})  -- Only creates last dir
```

**Atomic Writes** (1-2 hours)
```lua
-- Internally: write to temp file, then rename
fs.writeFileSync("critical.json", data)
-- Guaranteed atomic on POSIX systems
```

### 10.2 Phase 3 (v0.5.0)

**Full Metadata** (2-3 hours)
```lua
local stat = fs.statSync("file.txt")
-- stat.uid, stat.gid, stat.atime, stat.mode
-- Full file permissions and ownership
```

**Permission Management** (2-3 hours)
```lua
fs.chmodSync("file.txt", 0o644)
fs.chownSync("file.txt", uid, gid)  -- Unix only
```

**Path Validation** (1-2 hours)
```lua
-- Prevents path traversal
fs.readFileSync("../../etc/passwd")  -- Error: Path traversal detected
```

**Sandbox Mode** (1-2 hours)
```lua
-- Restrict fs operations to working directory
fs.setSandbox(true, process.cwd())
fs.readFileSync("/etc/passwd")  -- Error: Outside sandbox
```

### 10.3 Phase 4 (v0.6.0)

**Async Operations** (8-10 hours)
```lua
-- Non-blocking file operations
fs.readFile("large.txt", function(err, data)
    if err then
        print("Error:", err)
    else
        print("Read", #data, "bytes")
    end
end)
```

**Streaming API** (6-8 hours)
```lua
-- For large files
local stream = fs.createReadStream("large.log")
for line in stream:lines() do
    process(line)
end
stream:close()
```

**Watch API** (4-6 hours)
```lua
-- Monitor file changes
local watcher = fs.watch("config.json", function(event, filename)
    print("File changed:", filename)
    reload_config()
end)
```

**Advanced Directory Operations** (3-4 hours)
```lua
-- Recursive copy
fs.copySync("src", "dest", {recursive = true})

-- Recursive remove
fs.rmSync("node_modules", {recursive = true, force = true})

-- Walk directory tree
fs.walk("src", function(path, stat)
    if stat.isFile and path:match("%.lua$") then
        process_lua_file(path)
    end
end)
```

---

## Appendices

### Appendix A: API Reference

#### readFileSync(path)

Read entire file as UTF-8 string.

**Parameters:**
- `path` (string) - File path to read

**Returns:** (string) File contents

**Throws:** Error if file doesn't exist or can't be read

**Example:**
```lua
local config = fs.readFileSync("config.json")
local data = json.decode(config)
```

#### writeFileSync(path, data)

Write string to file (creates if missing, overwrites if exists).

**Parameters:**
- `path` (string) - File path to write
- `data` (string) - Content to write

**Returns:** nil

**Throws:** Error if write fails

**Example:**
```lua
fs.writeFileSync("output.txt", "Hello World")
```

#### existsSync(path)

Check if path exists (file or directory).

**Parameters:**
- `path` (string) - Path to check

**Returns:** (boolean) true if exists, false otherwise

**Example:**
```lua
if fs.existsSync("data.json") then
    local data = fs.readFileSync("data.json")
end
```

#### statSync(path)

Get file/directory metadata.

**Parameters:**
- `path` (string) - Path to stat

**Returns:** (table) File statistics:
- `size` (number) - Size in bytes
- `isFile` (boolean) - Is regular file
- `isDirectory` (boolean) - Is directory
- `isSymlink` (boolean) - Is symbolic link
- `mtime` (number) - Modified time (Unix timestamp)

**Throws:** Error if path doesn't exist

**Example:**
```lua
local stat = fs.statSync("file.txt")
print("Size:", stat.size, "bytes")
print("Modified:", os.date("%Y-%m-%d", stat.mtime))
```

#### readdirSync(path)

List directory contents (sorted alphabetically).

**Parameters:**
- `path` (string) - Directory path

**Returns:** (array) Array of filenames (strings)

**Throws:** Error if directory doesn't exist or can't be read

**Example:**
```lua
local files = fs.readdirSync(".")
for _, name in ipairs(files) do
    print("  -", name)
end
```

#### unlinkSync(path)

Delete file.

**Parameters:**
- `path` (string) - File path to delete

**Returns:** nil

**Throws:** Error if file doesn't exist or can't be deleted

**Example:**
```lua
fs.unlinkSync("temp.txt")
```

#### mkdirSync(path)

Create directory (creates parent directories if needed).

**Parameters:**
- `path` (string) - Directory path to create

**Returns:** nil

**Throws:** Error if creation fails

**Example:**
```lua
-- Creates a, a/b, and a/b/c
fs.mkdirSync("a/b/c")
```

#### rmdirSync(path)

Remove empty directory.

**Parameters:**
- `path` (string) - Directory path to remove

**Returns:** nil

**Throws:** Error if directory doesn't exist, isn't empty, or can't be removed

**Example:**
```lua
fs.rmdirSync("empty_dir")
```

### Appendix B: Migration Guide

**From:** Current state (non-functional fs module)  
**To:** v0.3.0 (fully functional fs module)

**Changes:**
- ✅ All functions now work (previously were stubs)
- ✅ `mkdirSync` is recursive by default (safer, more convenient)
- ✅ Error messages are now Lua-friendly
- ✅ File paths use platform-appropriate separators
- ✅ Examples in `examples/file-operations.lua` now work

**Breaking Changes:** None (module was non-functional before)

**New Features:**
- Complete filesystem API
- UTF-8 text file support
- Cross-platform compatibility
- Comprehensive error handling

### Appendix C: Testing Checklist

- [ ] Read text file
- [ ] Write text file
- [ ] Read non-existent file (error)
- [ ] Write to read-only location (error)
- [ ] Check existence (file exists)
- [ ] Check existence (file missing)
- [ ] Get file stats (regular file)
- [ ] Get directory stats
- [ ] List directory (multiple files)
- [ ] List directory (empty)
- [ ] List directory (non-existent)
- [ ] Delete file
- [ ] Delete non-existent file (error)
- [ ] Create directory (nested)
- [ ] Create directory (already exists - ok)
- [ ] Remove empty directory
- [ ] Remove non-empty directory (error)
- [ ] UTF-8 filenames
- [ ] UTF-8 file contents
- [ ] Large file (>1MB)
- [ ] Empty file (0 bytes)
- [ ] Special characters in filenames
- [ ] Paths with spaces
- [ ] Relative paths
- [ ] Absolute paths
- [ ] Cross-platform path separators
- [ ] Concurrent file access
- [ ] File permissions respected

---

**End of PRP-013**
