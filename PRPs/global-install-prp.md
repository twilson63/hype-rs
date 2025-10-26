# Project Request Protocol (PRP): Global Package Installation with Binary Executables

**Project ID**: PRP-007  
**Status**: 📋 Proposed  
**Priority**: High  
**Created**: 2025-10-26  
**Author**: Claude (AI Assistant)  
**Estimated Effort**: 8-13 hours (1-2 days)  

---

## Executive Summary

Implement npm-like global package installation functionality, allowing hype-rs packages to expose executable commands through a `bin` field in `hype.json`. When installed globally, packages create wrapper scripts that make Lua scripts executable as system commands.

**Current State**: Packages can only be run directly with `hype script.lua` from their directory.  
**Desired State**: Packages declare executables in `hype.json` and install globally to create system-wide commands.  
**Gap**: No global installation mechanism, no `bin` field support, no executable wrapper generation.

**Example Use Case**:
```bash
# Package defines: "bin": { "mycli": "cli.lua" }
cd my-package
hype install

# Now use anywhere:
mycli fetch https://api.github.com
```

---

## Table of Contents

- [1. Project Overview](#1-project-overview)
- [2. Current State Analysis](#2-current-state-analysis)
- [3. Technical Requirements](#3-technical-requirements)
- [4. Proposed Architecture](#4-proposed-architecture)
- [5. Implementation Plan](#5-implementation-plan)
- [6. API Specification](#6-api-specification)
- [7. Success Criteria](#7-success-criteria)
- [8. Risk Assessment](#8-risk-assessment)
- [9. Future Enhancements](#9-future-enhancements)
- [10. References](#10-references)

---

## 1. Project Overview

### 1.1 Background

**Node.js package.json `bin` field**:
```json
{
  "name": "my-tool",
  "version": "1.0.0",
  "bin": {
    "mytool": "cli.js"
  }
}
```

After `npm install -g my-tool`, users can run `mytool` from any directory. The system creates a symlink/wrapper that executes `node cli.js` with the global package path.

**hype-rs Current Limitation**: Packages must be executed manually:
```bash
cd /path/to/package
hype main.lua arg1 arg2
```

No way to install once and use globally as a command.

### 1.2 Project Goals

1. **Manifest Extension**: Add `bin` field to `hype.json` manifest
2. **Global Registry**: Create system to track globally installed packages
3. **Installation Command**: Implement `hype install` command
4. **Wrapper Generation**: Create platform-specific executable wrappers
5. **PATH Integration**: Guide users to add `~/.hype/bin` to PATH
6. **Package Management**: Support install, uninstall, list operations

### 1.3 Target Workflow

**Package Creation**:
```json
// my-cli-tool/hype.json
{
  "name": "http-fetcher",
  "version": "1.0.0",
  "description": "CLI tool for HTTP requests",
  "main": "index.lua",
  "bin": {
    "fetch": "bin/fetch.lua",
    "http-get": "bin/get.lua"
  }
}
```

```lua
-- my-cli-tool/bin/fetch.lua
local http = require('http')
local args = _G.args or {}

if #args < 1 then
    print("Usage: fetch <url>")
    os.exit(1)
end

local response = http.get(args[1])
print(response:text())
```

**Installation & Usage**:
```bash
# Install globally
cd my-cli-tool
hype install
# → Installs to ~/.hype/packages/http-fetcher@1.0.0/
# → Creates ~/.hype/bin/fetch
# → Creates ~/.hype/bin/http-get

# Use anywhere
fetch https://httpbin.org/get
http-get https://api.github.com/users/octocat

# List installed packages
hype list
# → http-fetcher@1.0.0
#   Commands: fetch, http-get

# Uninstall
hype uninstall http-fetcher
```

---

## 2. Current State Analysis

### 2.1 Existing Manifest Structure

**File**: `src/modules/manifest.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HypeManifest {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub main: Option<String>,
    pub dependencies: Option<Vec<String>>,
    // Missing: pub bin: Option<HashMap<String, String>>,
}
```

**Capabilities**:
- ✅ Load/save from `hype.json`
- ✅ Validation (name, version, dependencies)
- ✅ Semver version support
- ✅ Builder pattern for construction

**Missing**:
- ❌ No `bin` field for executables
- ❌ No global installation support
- ❌ No package registry

### 2.2 CLI Command Structure

**File**: `src/cli/commands.rs`

**Existing Commands**:
- `run_script()` - Execute Lua scripts
- `run_module()` - Execute modules with manifest

**Missing Commands**:
- ❌ `install_package()` - Install globally
- ❌ `uninstall_package()` - Remove global package
- ❌ `list_packages()` - List installed packages

### 2.3 Module System

**Module Loader** (`src/modules/loader.rs`):
- Loads modules from `hype_modules/` directories
- Resolves dependencies
- Caches loaded modules

**Gap**: No concept of global package storage or system-wide executable registration.

---

## 3. Technical Requirements

### 3.1 Functional Requirements

**FR-1: Manifest `bin` Field**
- Support `bin` field as HashMap<String, String> (command → script path)
- Validate bin entries (script files must exist)
- Support multiple commands per package
- Serialize/deserialize to JSON

**FR-2: Global Installation**
- Install packages to user-specific directory (not system-wide)
- Create versioned package directories
- Copy all package files (preserve directory structure)
- Support installing from current directory or path

**FR-3: Executable Wrapper Generation**
- Generate platform-specific wrapper scripts
- Unix/macOS: Bash scripts with shebang
- Windows: Batch (.bat) or PowerShell (.ps1) scripts
- Pass all arguments to underlying Lua script
- Handle relative/absolute paths correctly

**FR-4: Package Registry**
- Track installed packages and versions
- Store metadata (install date, commands, location)
- Support multiple versions of same package
- Detect and prevent command name conflicts

**FR-5: CLI Commands**
- `hype install [path]` - Install package globally
- `hype uninstall <name>` - Remove package
- `hype list` - Show installed packages
- `hype which <command>` - Show which package provides command

### 3.2 Non-Functional Requirements

**NFR-1: Cross-Platform Compatibility**
- Work on Unix, macOS, and Windows
- Handle path separators correctly
- Use platform-appropriate wrapper scripts

**NFR-2: User Experience**
- Clear error messages
- Progress indication for installation
- Post-install instructions (PATH setup)
- Prevent accidental overwrites

**NFR-3: Security**
- Don't require sudo/admin privileges
- User-local installation only
- Validate manifest before installation
- Prevent directory traversal in bin paths

**NFR-4: Performance**
- Fast installation (just file copy + wrapper creation)
- Minimal startup overhead for wrappers
- No network calls (local installation only for now)

---

## 4. Proposed Architecture

### 4.1 Directory Structure

**Global Package Location**:
```
~/.hype/
├── packages/               # Installed packages
│   ├── http-fetcher@1.0.0/
│   │   ├── hype.json
│   │   ├── bin/
│   │   │   └── fetch.lua
│   │   └── index.lua
│   └── another-tool@2.1.0/
│       └── ...
├── bin/                    # Executable wrappers
│   ├── fetch              # → ../packages/http-fetcher@1.0.0/bin/fetch.lua
│   └── another-cmd
└── registry.json          # Installed packages metadata
```

**Registry Format** (`~/.hype/registry.json`):
```json
{
  "packages": {
    "http-fetcher": {
      "version": "1.0.0",
      "install_date": "2025-10-26T12:00:00Z",
      "location": "/Users/you/.hype/packages/http-fetcher@1.0.0",
      "bin": {
        "fetch": "bin/fetch.lua",
        "http-get": "bin/get.lua"
      }
    }
  },
  "bin_commands": {
    "fetch": "http-fetcher@1.0.0",
    "http-get": "http-fetcher@1.0.0"
  }
}
```

### 4.2 Component Design

**1. Extended Manifest** (`src/modules/manifest.rs`):
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HypeManifest {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub main: Option<String>,
    pub dependencies: Option<Vec<String>>,
    #[serde(default)]
    pub bin: Option<HashMap<String, String>>, // NEW
}

impl HypeManifest {
    pub fn with_bin(mut self, bin: HashMap<String, String>) -> Self;
    pub fn validate_bin(&self, package_dir: &Path) -> Result<()>;
}
```

**2. Global Package Registry** (`src/modules/registry_global.rs` - NEW):
```rust
pub struct GlobalPackageRegistry {
    root_dir: PathBuf,        // ~/.hype
    packages_dir: PathBuf,    // ~/.hype/packages
    bin_dir: PathBuf,         // ~/.hype/bin
    registry_file: PathBuf,   // ~/.hype/registry.json
}

pub struct InstalledPackage {
    pub name: String,
    pub version: String,
    pub install_date: DateTime<Utc>,
    pub location: PathBuf,
    pub bin: HashMap<String, String>,
}

impl GlobalPackageRegistry {
    pub fn new() -> Result<Self>;
    pub fn load() -> Result<Self>;
    
    // Package operations
    pub fn install(&mut self, manifest: &HypeManifest, source: &Path) -> Result<InstallResult>;
    pub fn uninstall(&mut self, name: &str) -> Result<UninstallResult>;
    pub fn list(&self) -> Result<Vec<InstalledPackage>>;
    pub fn get(&self, name: &str) -> Result<Option<InstalledPackage>>;
    
    // Binary operations
    pub fn create_bin_wrapper(&self, cmd: &str, script_path: &Path) -> Result<PathBuf>;
    pub fn remove_bin_wrapper(&self, cmd: &str) -> Result<()>;
    pub fn which_command(&self, cmd: &str) -> Result<Option<String>>;
    
    // Internal
    fn copy_package(&self, source: &Path, dest: &Path) -> Result<()>;
    fn save_registry(&self) -> Result<()>;
    fn check_bin_conflict(&self, bin_map: &HashMap<String, String>) -> Result<()>;
}
```

**3. Wrapper Generator** (`src/modules/bin_wrapper.rs` - NEW):
```rust
pub struct BinWrapper;

impl BinWrapper {
    // Generate Unix/macOS bash wrapper
    pub fn create_unix_wrapper(
        bin_path: &Path,
        hype_executable: &Path,
        script_path: &Path,
    ) -> Result<()>;
    
    // Generate Windows batch wrapper
    pub fn create_windows_wrapper(
        bin_path: &Path,
        hype_executable: &Path,
        script_path: &Path,
    ) -> Result<()>;
    
    // Platform-agnostic interface
    pub fn create_wrapper(
        bin_path: &Path,
        hype_executable: &Path,
        script_path: &Path,
    ) -> Result<()> {
        #[cfg(unix)]
        Self::create_unix_wrapper(bin_path, hype_executable, script_path)?;
        
        #[cfg(windows)]
        Self::create_windows_wrapper(bin_path, hype_executable, script_path)?;
        
        Ok(())
    }
}
```

**4. Install Command** (`src/cli/install.rs` - NEW):
```rust
pub struct InstallArgs {
    pub path: Option<PathBuf>,  // Install from this path (default: ".")
    pub global: bool,            // --global flag (default: true for now)
    pub force: bool,             // --force to overwrite
}

pub fn install_package(args: InstallArgs) -> Result<()> {
    // 1. Find and load hype.json
    // 2. Validate manifest (including bin field)
    // 3. Load global registry
    // 4. Check for conflicts
    // 5. Copy package to global location
    // 6. Create bin wrappers
    // 7. Update registry
    // 8. Print success message with PATH instructions
}

pub fn uninstall_package(name: String) -> Result<()>;
pub fn list_packages() -> Result<()>;
pub fn which_command(cmd: String) -> Result<()>;
```

### 4.3 Wrapper Script Templates

**Unix/macOS Wrapper** (`~/.hype/bin/mycmd`):
```bash
#!/usr/bin/env bash
# Auto-generated by hype-rs v1.0.0
# Package: http-fetcher@1.0.0
# Script: bin/fetch.lua

# Detect hype location
HYPE_BIN="$(command -v hype)"
if [ -z "$HYPE_BIN" ]; then
    echo "Error: hype not found in PATH" >&2
    exit 1
fi

# Package location
PACKAGE_DIR="$HOME/.hype/packages/http-fetcher@1.0.0"
SCRIPT_PATH="$PACKAGE_DIR/bin/fetch.lua"

# Execute with all arguments
exec "$HYPE_BIN" "$SCRIPT_PATH" "$@"
```

**Windows Batch Wrapper** (`~/.hype/bin/mycmd.bat`):
```batch
@echo off
REM Auto-generated by hype-rs v1.0.0
REM Package: http-fetcher@1.0.0
REM Script: bin\fetch.lua

REM Detect hype location
where hype >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo Error: hype not found in PATH >&2
    exit /b 1
)

REM Package location
set PACKAGE_DIR=%USERPROFILE%\.hype\packages\http-fetcher@1.0.0
set SCRIPT_PATH=%PACKAGE_DIR%\bin\fetch.lua

REM Execute with all arguments
hype "%SCRIPT_PATH%" %*
```

---

## 5. Implementation Plan

### Phase 1: Manifest Extensions (2-3 hours)

**Objective**: Add `bin` field support to HypeManifest

**Tasks**:
1. Modify `src/modules/manifest.rs`:
   - Add `bin: Option<HashMap<String, String>>` field
   - Add `with_bin()` builder method
   - Add `validate_bin()` validation
   - Update tests

**Files Modified**:
- `src/modules/manifest.rs`

**Tests**:
- Deserialize manifest with bin field
- Validate bin paths exist
- Reject invalid bin entries (absolute paths, directory traversal)

**Deliverables**:
- ✅ `bin` field in manifest
- ✅ Validation logic
- ✅ Tests passing

---

### Phase 2: Global Registry Implementation (3-4 hours)

**Objective**: Create global package registry system

**Tasks**:
1. Create `src/modules/registry_global.rs`:
   - Implement `GlobalPackageRegistry` struct
   - Implement `InstalledPackage` struct
   - Add registry JSON serialization
   - Add file I/O operations

2. Add directory initialization:
   - Create `~/.hype/` on first use
   - Create `packages/` and `bin/` subdirectories
   - Initialize `registry.json` if missing

3. Implement core operations:
   - `load()` - Load existing registry
   - `save_registry()` - Persist to disk
   - `check_bin_conflict()` - Detect command conflicts

**Files Created**:
- `src/modules/registry_global.rs`

**Files Modified**:
- `src/modules/mod.rs` - Export new module

**Tests**:
- Create registry from scratch
- Load existing registry
- Detect command name conflicts
- Handle missing registry file

**Deliverables**:
- ✅ Registry structure
- ✅ Persistence layer
- ✅ Conflict detection

---

### Phase 3: Wrapper Script Generation (2-3 hours)

**Objective**: Generate platform-specific executable wrappers

**Tasks**:
1. Create `src/modules/bin_wrapper.rs`:
   - Implement Unix wrapper generation (bash)
   - Implement Windows wrapper generation (batch)
   - Add permission setting (chmod +x on Unix)

2. Add template-based generation:
   - Embed wrapper templates
   - Substitute package paths
   - Handle edge cases (spaces in paths, special characters)

**Files Created**:
- `src/modules/bin_wrapper.rs`

**Files Modified**:
- `src/modules/mod.rs`

**Tests**:
- Generate Unix wrapper
- Generate Windows wrapper
- Verify executable permissions
- Test with spaces in paths

**Deliverables**:
- ✅ Cross-platform wrapper generation
- ✅ Executable permissions
- ✅ Path handling

---

### Phase 4: Install Command (2-3 hours)

**Objective**: Implement `hype install` command

**Tasks**:
1. Create `src/cli/install.rs`:
   - Implement `install_package()`
   - Implement `uninstall_package()`
   - Implement `list_packages()`
   - Implement `which_command()`

2. Add package copying:
   - Copy entire package directory
   - Preserve directory structure
   - Handle symlinks correctly

3. Add user feedback:
   - Progress messages
   - Success confirmation
   - PATH setup instructions

**Files Created**:
- `src/cli/install.rs`

**Files Modified**:
- `src/cli/mod.rs`
- `src/cli/commands.rs`
- `src/cli/parser.rs` (add subcommands)
- `src/main.rs` (route to new commands)

**Tests**:
- Install package from current directory
- Install package from path
- Uninstall package
- List installed packages
- Detect missing manifest

**Deliverables**:
- ✅ Working `hype install`
- ✅ Working `hype uninstall`
- ✅ Working `hype list`

---

### Phase 5: CLI Integration (1-2 hours)

**Objective**: Wire up new commands to main CLI

**Tasks**:
1. Update CLI parser:
   - Add `install` subcommand
   - Add `uninstall` subcommand
   - Add `list` subcommand
   - Add `which` subcommand

2. Add help text:
   - Command descriptions
   - Usage examples
   - Flag documentation

**Files Modified**:
- `src/cli/parser.rs` or main CLI
- `src/main.rs`

**Tests**:
- Parse `hype install`
- Parse `hype install --force`
- Parse `hype uninstall <name>`
- Help text displays correctly

**Deliverables**:
- ✅ CLI accepts new commands
- ✅ Help text complete

---

### Phase 6: Documentation & Examples (1-2 hours)

**Objective**: Document feature and create examples

**Tasks**:
1. Create documentation:
   - `docs/features/global-install.md`
   - Update README.md
   - Add usage examples

2. Create example CLI tool:
   - `examples/cli-tool/hype.json`
   - `examples/cli-tool/bin/tool.lua`
   - Add README with usage

3. Add to main docs:
   - Update module system docs
   - Add to getting started guide

**Files Created**:
- `docs/features/global-install.md`
- `examples/cli-tool/` (directory)

**Files Modified**:
- `README.md`
- `docs/modules/README.md`

**Deliverables**:
- ✅ Complete documentation
- ✅ Working example
- ✅ Updated README

---

### Phase 7: Testing & Polish (1-2 hours)

**Objective**: Comprehensive testing and bug fixes

**Tasks**:
1. Integration tests:
   - End-to-end install/uninstall
   - Wrapper execution
   - Conflict handling

2. Cross-platform testing:
   - Test on macOS
   - Test on Linux (if available)
   - Test on Windows (if available)

3. Edge cases:
   - Package without bin field
   - Invalid bin paths
   - Command name conflicts
   - Uninstall non-existent package
   - Multiple versions

**Tests**:
- Full integration test suite
- Platform-specific tests
- Error handling tests

**Deliverables**:
- ✅ All tests passing
- ✅ Cross-platform verified
- ✅ Edge cases handled

---

## 6. API Specification

### 6.1 Manifest Format

```json
{
  "name": "my-cli-tool",
  "version": "1.2.3",
  "description": "An awesome CLI tool",
  "main": "index.lua",
  "bin": {
    "mytool": "bin/cli.lua",
    "mt": "bin/cli.lua",
    "tool-helper": "scripts/helper.lua"
  },
  "dependencies": []
}
```

**Constraints**:
- `bin` keys (command names): alphanumeric, hyphens, underscores, 1-64 chars
- `bin` values (script paths): relative paths within package, must exist
- Command names must not conflict with existing global commands
- Script paths must be readable Lua files

### 6.2 CLI Commands

#### `hype install [path]`

Install a package globally.

**Arguments**:
- `[path]` - Directory containing hype.json (default: current directory)

**Flags**:
- `--force, -f` - Overwrite existing installation
- `--verbose, -v` - Show detailed progress

**Examples**:
```bash
# Install from current directory
cd my-package && hype install

# Install from specific path
hype install ./packages/my-tool

# Force reinstall
hype install --force
```

**Exit Codes**:
- 0: Success
- 1: Manifest not found or invalid
- 2: Command name conflict
- 3: File copy error

---

#### `hype uninstall <name>`

Remove a globally installed package.

**Arguments**:
- `<name>` - Package name to uninstall

**Flags**:
- `--verbose, -v` - Show detailed progress

**Examples**:
```bash
hype uninstall my-cli-tool
```

**Exit Codes**:
- 0: Success
- 1: Package not found
- 2: File deletion error

---

#### `hype list`

List all globally installed packages.

**Flags**:
- `--json` - Output as JSON
- `--verbose, -v` - Show detailed information

**Output**:
```
Globally installed packages:

  http-fetcher@1.0.0
    Commands: fetch, http-get
    Location: /Users/you/.hype/packages/http-fetcher@1.0.0
    Installed: 2025-10-26

  another-tool@2.1.0
    Commands: anothercmd
    Location: /Users/you/.hype/packages/another-tool@2.1.0
    Installed: 2025-10-25

Total: 2 packages
```

---

#### `hype which <command>`

Show which package provides a command.

**Arguments**:
- `<command>` - Command name to lookup

**Examples**:
```bash
$ hype which fetch
fetch is provided by http-fetcher@1.0.0
Location: /Users/you/.hype/packages/http-fetcher@1.0.0/bin/fetch.lua
```

**Exit Codes**:
- 0: Command found
- 1: Command not found

---

### 6.3 Environment Variables

**`HYPE_HOME`** (optional):
Override default installation directory (default: `~/.hype`)

```bash
export HYPE_HOME=/custom/path
hype install  # Installs to /custom/path/packages/
```

---

## 7. Success Criteria

### 7.1 Functional Tests

✅ **Manifest Support**:
- [ ] Load manifest with `bin` field
- [ ] Validate bin entries
- [ ] Reject invalid bin configurations
- [ ] Serialize/deserialize correctly

✅ **Installation**:
- [ ] Install package from current directory
- [ ] Install package from specified path
- [ ] Copy all package files correctly
- [ ] Create all bin wrappers
- [ ] Update registry
- [ ] Detect and prevent command conflicts

✅ **Uninstallation**:
- [ ] Remove package directory
- [ ] Remove all bin wrappers
- [ ] Update registry
- [ ] Handle non-existent packages gracefully

✅ **Execution**:
- [ ] Wrapper executes Lua script
- [ ] Arguments pass through correctly
- [ ] Environment variables accessible
- [ ] Exit codes propagate

✅ **Registry**:
- [ ] Persist across operations
- [ ] Handle concurrent access safely
- [ ] Recover from corrupted registry

### 7.2 Non-Functional Tests

✅ **Cross-Platform**:
- [ ] Works on macOS
- [ ] Works on Linux
- [ ] Works on Windows
- [ ] Handles path separators correctly

✅ **User Experience**:
- [ ] Clear error messages
- [ ] Progress indication
- [ ] PATH setup instructions shown
- [ ] Help text comprehensive

✅ **Security**:
- [ ] No sudo required
- [ ] User-only installations
- [ ] Path traversal prevented
- [ ] Symlink attacks mitigated

### 7.3 Documentation

✅ **Complete Documentation**:
- [ ] Feature documentation written
- [ ] API reference complete
- [ ] Examples provided
- [ ] README updated
- [ ] Migration guide (if needed)

---

## 8. Risk Assessment

### 8.1 Technical Risks

**Risk 1: Command Name Conflicts**  
**Severity**: Medium  
**Mitigation**:
- Check for conflicts before installation
- Require `--force` flag to overwrite
- Show clear error with conflicting package name
- Implement `hype which <cmd>` to debug conflicts

**Risk 2: PATH Configuration**  
**Severity**: Medium  
**Mitigation**:
- Provide clear instructions to add `~/.hype/bin` to PATH
- Implement `hype setup` command to auto-configure (future)
- Document manual setup for all shells (bash, zsh, fish, PowerShell)
- Check PATH and warn if not configured

**Risk 3: Platform-Specific Issues**  
**Severity**: Medium  
**Mitigation**:
- Test on all major platforms
- Use Rust's cross-platform abstractions (std::fs, Path)
- Conditional compilation for platform-specific code
- Document known limitations

**Risk 4: Registry Corruption**  
**Severity**: Low  
**Mitigation**:
- Atomic writes (write to temp file, rename)
- Validate JSON on load
- Implement repair command (`hype repair`)
- Keep backup of registry on modification

### 8.2 User Experience Risks

**Risk 1: Confusion with npm**  
**Severity**: Low  
**Mitigation**:
- Be explicit about hype-specific behavior
- Document differences from npm
- Use similar but not identical commands

**Risk 2: Disk Space**  
**Severity**: Low  
**Mitigation**:
- Show package size before install
- Implement `hype cache clean` (future)
- Document where packages are stored

---

## 9. Future Enhancements

### 9.1 Short-term (Next Release)

- **PATH Auto-Setup**: `hype setup` command to modify shell config
- **Symlink Support**: Use symlinks instead of copying for local packages
- **Version Management**: `hype install pkg@1.2.3` for specific versions
- **Upgrade Command**: `hype upgrade <name>` to update packages

### 9.2 Medium-term (Future Releases)

- **Package Registry**: Remote package registry (like npm registry)
- **hype publish**: Publish packages to registry
- **hype search**: Search for packages
- **Dependency Auto-Install**: Install dependencies during package install
- **Scripts**: Support `scripts` field in manifest (preinstall, postinstall)

### 9.3 Long-term (Future Vision)

- **Binary Distribution**: Compile Lua to bytecode for distribution
- **Code Signing**: Sign packages for security
- **License Checking**: Validate package licenses
- **Audit**: Security audit for installed packages
- **Workspaces**: Monorepo support

---

## 10. References

### 10.1 Similar Implementations

- **npm**: https://docs.npmjs.com/cli/v9/configuring-npm/package-json#bin
- **cargo install**: https://doc.rust-lang.org/cargo/commands/cargo-install.html
- **pip**: https://pip.pypa.io/en/stable/reference/pip_install/
- **gem**: https://guides.rubygems.org/make-your-own-gem/

### 10.2 Internal Documentation

- `docs/modules/README.md` - Module system overview
- `src/modules/manifest.rs` - Current manifest implementation
- `PRPs/module-system-prp.md` - Original module system PRP

### 10.3 External Resources

- Semantic Versioning: https://semver.org/
- XDG Base Directory Specification (Linux): https://specifications.freedesktop.org/basedir-spec/latest/
- Platform conventions:
  - macOS: `~/Library/Application Support/hype/`
  - Windows: `%APPDATA%\hype\`
  - Linux: `~/.local/share/hype/` or `~/.hype/`

---

## Appendix A: Example Package

**Directory Structure**:
```
my-http-tool/
├── hype.json
├── bin/
│   ├── fetch.lua
│   └── post.lua
├── lib/
│   └── utils.lua
└── README.md
```

**hype.json**:
```json
{
  "name": "my-http-tool",
  "version": "1.0.0",
  "description": "HTTP CLI utilities",
  "main": "index.lua",
  "bin": {
    "myfetch": "bin/fetch.lua",
    "mypost": "bin/post.lua"
  },
  "dependencies": []
}
```

**bin/fetch.lua**:
```lua
#!/usr/bin/env hype
local http = require('http')

-- Get command-line arguments
local args = _G.args or {}

if #args < 1 then
    print("Usage: myfetch <url> [options]")
    print("Options:")
    print("  --json    Format output as JSON")
    os.exit(1)
end

local url = args[1]
local format_json = false

for i = 2, #args do
    if args[i] == "--json" then
        format_json = true
    end
end

-- Make HTTP request
local response = http.get(url)

if not response:ok() then
    print("Error: HTTP " .. response.status .. " " .. response.statusText)
    os.exit(1)
end

-- Output response
if format_json then
    local data = response:json()
    print(require('json').encode(data))
else
    print(response:text())
end
```

**Usage After Install**:
```bash
$ cd my-http-tool
$ hype install
Installing my-http-tool@1.0.0...
✓ Copied package to /Users/you/.hype/packages/my-http-tool@1.0.0
✓ Created executable: myfetch
✓ Created executable: mypost
✓ Installation complete!

To use the commands, add ~/.hype/bin to your PATH:
  export PATH="$HOME/.hype/bin:$PATH"

$ export PATH="$HOME/.hype/bin:$PATH"
$ myfetch https://httpbin.org/get
{
  "url": "https://httpbin.org/get",
  ...
}

$ myfetch https://api.github.com/users/octocat --json
{"login":"octocat",...}
```

---

## Appendix B: Implementation Checklist

### Phase 1: Manifest Extensions
- [ ] Add `bin` field to `HypeManifest`
- [ ] Add `with_bin()` method
- [ ] Add `validate_bin()` validation
- [ ] Write tests for bin field
- [ ] Update documentation

### Phase 2: Global Registry
- [ ] Create `registry_global.rs`
- [ ] Implement `GlobalPackageRegistry` struct
- [ ] Implement `InstalledPackage` struct
- [ ] Add JSON persistence
- [ ] Write registry tests

### Phase 3: Wrapper Generation
- [ ] Create `bin_wrapper.rs`
- [ ] Implement Unix wrapper generation
- [ ] Implement Windows wrapper generation
- [ ] Add executable permissions
- [ ] Write wrapper tests

### Phase 4: Install Command
- [ ] Create `install.rs`
- [ ] Implement `install_package()`
- [ ] Implement `uninstall_package()`
- [ ] Implement `list_packages()`
- [ ] Implement `which_command()`
- [ ] Write command tests

### Phase 5: CLI Integration
- [ ] Add `install` subcommand to parser
- [ ] Add `uninstall` subcommand
- [ ] Add `list` subcommand
- [ ] Add `which` subcommand
- [ ] Update help text

### Phase 6: Documentation
- [ ] Write `docs/features/global-install.md`
- [ ] Update README.md
- [ ] Create example CLI tool
- [ ] Add usage examples
- [ ] Update module docs

### Phase 7: Testing
- [ ] End-to-end integration tests
- [ ] Cross-platform testing
- [ ] Edge case handling
- [ ] Performance testing
- [ ] Security review

---

**End of PRP-007**
