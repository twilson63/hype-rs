# Global Package Installation

Install hype-rs packages globally to create system-wide CLI commands from your Lua scripts.

## Overview

The global installation feature allows you to:

- **Expose executable commands** from packages via the `bin` field in `hype.json`
- **Install packages globally** to `~/.hype/packages/` with automatic wrapper generation
- **Run commands anywhere** without specifying the full script path
- **Manage installed packages** with install, uninstall, list, and which commands

Similar to npm's global installation, this feature makes it easy to create and distribute CLI tools written in Lua.

## Quick Start

### 1. Create a Package with Executables

Create `hype.json` with a `bin` field:

```json
{
  "name": "my-cli-tool",
  "version": "1.0.0",
  "description": "My awesome CLI tool",
  "bin": {
    "mytool": "bin/cli.lua"
  }
}
```

Create the executable script at `bin/cli.lua`:

```lua
local args = _G.args or {}

if #args < 1 then
    print("Usage: mytool <command>")
    os.exit(1)
end

print("Running command:", args[1])
```

### 2. Install Globally

```bash
cd my-cli-tool
hype install
```

Output:
```
Installing my-cli-tool@1.0.0...
✓ Copied package to /Users/you/.hype/packages/my-cli-tool@1.0.0
✓ Created executable: mytool
✓ Installation complete!

To use the commands, add ~/.hype/bin to your PATH:
  export PATH="$HOME/.hype/bin:$PATH"
```

### 3. Add to PATH

**Bash** (`~/.bashrc` or `~/.bash_profile`):
```bash
export PATH="$HOME/.hype/bin:$PATH"
```

**Zsh** (`~/.zshrc`):
```bash
export PATH="$HOME/.hype/bin:$PATH"
```

**Fish** (`~/.config/fish/config.fish`):
```fish
set -gx PATH $HOME/.hype/bin $PATH
```

**PowerShell** (`$PROFILE`):
```powershell
$env:PATH = "$env:USERPROFILE\.hype\bin;$env:PATH"
```

Reload your shell:
```bash
source ~/.bashrc  # or ~/.zshrc, etc.
```

### 4. Use Anywhere

```bash
mytool hello
```

Output:
```
Running command: hello
```

## Installation Workflow

### Install from Current Directory

```bash
cd my-package
hype install
```

### Install from Specific Path

```bash
hype install /path/to/my-package
```

### Force Reinstall

```bash
hype install --force
```

Overwrites existing installation without prompting.

### Verbose Output

```bash
hype install --verbose
```

Shows detailed progress during installation.

## CLI Command Reference

### `hype install [path]`

Install a package globally.

**Arguments:**
- `[path]` - Directory containing hype.json (default: current directory)

**Flags:**
- `--force, -f` - Overwrite existing installation
- `--verbose, -v` - Show detailed progress

**Exit Codes:**
- `0` - Success
- `1` - Manifest not found or invalid
- `2` - Command name conflict
- `3` - File copy error

**Example:**
```bash
hype install ./my-tool
```

---

### `hype uninstall <name>`

Remove a globally installed package.

**Arguments:**
- `<name>` - Package name to uninstall

**Flags:**
- `--verbose, -v` - Show detailed progress

**Exit Codes:**
- `0` - Success
- `1` - Package not found
- `2` - File deletion error

**Example:**
```bash
hype uninstall my-cli-tool
```

---

### `hype list`

List all globally installed packages.

**Flags:**
- `--json` - Output as JSON
- `--verbose, -v` - Show detailed information

**Example Output:**
```
Globally installed packages:

  http-fetcher@1.0.0
    Commands: fetch, http-get
    Location: /Users/you/.hype/packages/http-fetcher@1.0.0
    Installed: 2025-10-26

  my-cli-tool@1.0.0
    Commands: mytool
    Location: /Users/you/.hype/packages/my-cli-tool@1.0.0
    Installed: 2025-10-26

Total: 2 packages
```

---

### `hype which <command>`

Show which package provides a command.

**Arguments:**
- `<command>` - Command name to lookup

**Exit Codes:**
- `0` - Command found
- `1` - Command not found

**Example:**
```bash
$ hype which fetch
fetch is provided by http-fetcher@1.0.0
Location: /Users/you/.hype/packages/http-fetcher@1.0.0/bin/fetch.lua
```

## Manifest `bin` Field Specification

The `bin` field in `hype.json` maps command names to script paths.

### Basic Usage

```json
{
  "name": "my-package",
  "version": "1.0.0",
  "bin": {
    "mycommand": "bin/cli.lua"
  }
}
```

### Multiple Commands

```json
{
  "name": "http-tools",
  "version": "1.0.0",
  "bin": {
    "fetch": "bin/fetch.lua",
    "post": "bin/post.lua",
    "http-get": "bin/get.lua"
  }
}
```

### Rules and Constraints

**Command Names:**
- Alphanumeric characters, hyphens, and underscores only
- 1-64 characters in length
- Must be unique across installed packages

**Script Paths:**
- Relative paths within the package
- Must point to existing Lua files
- Cannot use `..` (parent directory traversal)
- Cannot be absolute paths

**Examples:**

✅ Valid:
```json
"bin": {
  "mytool": "bin/cli.lua",
  "my-tool": "scripts/main.lua",
  "tool_v2": "src/index.lua"
}
```

❌ Invalid:
```json
"bin": {
  "my tool": "cli.lua",           // spaces not allowed
  "tool": "../other/cli.lua",     // parent directory not allowed
  "cmd": "/usr/bin/script.lua"    // absolute path not allowed
}
```

## PATH Setup Instructions

After installing packages, you need to add `~/.hype/bin` to your PATH.

### Bash

Edit `~/.bashrc` or `~/.bash_profile`:

```bash
# Add hype-rs global bin directory
export PATH="$HOME/.hype/bin:$PATH"
```

Apply changes:
```bash
source ~/.bashrc
```

### Zsh

Edit `~/.zshrc`:

```bash
# Add hype-rs global bin directory
export PATH="$HOME/.hype/bin:$PATH"
```

Apply changes:
```bash
source ~/.zshrc
```

### Fish

Edit `~/.config/fish/config.fish`:

```fish
# Add hype-rs global bin directory
set -gx PATH $HOME/.hype/bin $PATH
```

Apply changes:
```fish
source ~/.config/fish/config.fish
```

### PowerShell

Edit your PowerShell profile (find location with `$PROFILE`):

```powershell
# Add hype-rs global bin directory
$env:PATH = "$env:USERPROFILE\.hype\bin;$env:PATH"
```

Apply changes:
```powershell
. $PROFILE
```

### Verify PATH Setup

```bash
echo $PATH | grep .hype
```

Should show `~/.hype/bin` (or `%USERPROFILE%\.hype\bin` on Windows).

## Directory Structure

After installation, packages are stored in `~/.hype/`:

```
~/.hype/
├── packages/               # Installed packages
│   ├── http-fetcher@1.0.0/
│   │   ├── hype.json
│   │   ├── bin/
│   │   │   └── fetch.lua
│   │   └── index.lua
│   └── my-tool@1.0.0/
│       └── ...
├── bin/                    # Executable wrappers
│   ├── fetch              # → ../packages/http-fetcher@1.0.0/bin/fetch.lua
│   ├── mycommand          # → ../packages/my-tool@1.0.0/bin/cli.lua
│   └── ...
└── registry.json          # Installed packages metadata
```

### Registry Format

`~/.hype/registry.json` tracks installed packages:

```json
{
  "packages": {
    "http-fetcher": {
      "version": "1.0.0",
      "install_date": "2025-10-26T12:00:00Z",
      "location": "/Users/you/.hype/packages/http-fetcher@1.0.0",
      "bin": {
        "fetch": "bin/fetch.lua"
      }
    }
  },
  "bin_commands": {
    "fetch": "http-fetcher@1.0.0"
  }
}
```

## Troubleshooting

### Command Not Found After Install

**Problem:**
```bash
$ mytool
command not found: mytool
```

**Solutions:**

1. **Verify PATH is configured:**
   ```bash
   echo $PATH | grep .hype
   ```
   If not present, add `~/.hype/bin` to your PATH (see [PATH Setup](#path-setup-instructions)).

2. **Reload shell:**
   ```bash
   source ~/.bashrc  # or ~/.zshrc, etc.
   ```

3. **Check wrapper exists:**
   ```bash
   ls -la ~/.hype/bin/mytool
   ```

4. **Verify installation:**
   ```bash
   hype list
   ```

### Command Name Conflict

**Problem:**
```bash
$ hype install
Error: Command name conflict: 'fetch' is already provided by http-tools@1.0.0
```

**Solutions:**

1. **Choose different command name** in `hype.json`:
   ```json
   "bin": {
     "myfetch": "bin/fetch.lua"
   }
   ```

2. **Uninstall conflicting package:**
   ```bash
   hype uninstall http-tools
   ```

3. **Force reinstall** (overwrites):
   ```bash
   hype install --force
   ```

### Permission Denied

**Problem:**
```bash
$ mytool
Permission denied
```

**Solution (Unix/macOS):**
```bash
chmod +x ~/.hype/bin/mytool
```

The installer should set this automatically. If not, it's a bug.

### hype Command Not Found in Wrapper

**Problem:**
```bash
$ mytool
Error: hype not found in PATH
```

**Solutions:**

1. **Verify hype is installed:**
   ```bash
   which hype
   ```

2. **Install hype-rs:**
   ```bash
   cargo install hype-rs
   ```

3. **Add hype to PATH** if installed but not found.

### Package Not Found After Install

**Problem:**
```bash
$ hype list
Total: 0 packages
```

But you just ran `hype install` successfully.

**Solutions:**

1. **Check registry file:**
   ```bash
   cat ~/.hype/registry.json
   ```

2. **Reinstall package:**
   ```bash
   hype install --force
   ```

3. **Check for errors during install:**
   ```bash
   hype install --verbose
   ```

## Examples

### Example 1: Simple HTTP Fetcher

**hype.json:**
```json
{
  "name": "simple-fetch",
  "version": "1.0.0",
  "description": "Simple HTTP GET tool",
  "bin": {
    "sfetch": "fetch.lua"
  }
}
```

**fetch.lua:**
```lua
local http = require('http')
local args = _G.args or {}

if #args < 1 then
    print("Usage: sfetch <url>")
    os.exit(1)
end

local response = http.get(args[1])
print(response:text())
```

**Install and use:**
```bash
hype install
sfetch https://httpbin.org/get
```

### Example 2: Multi-Command Tool

**hype.json:**
```json
{
  "name": "http-cli",
  "version": "2.0.0",
  "bin": {
    "hget": "bin/get.lua",
    "hpost": "bin/post.lua"
  }
}
```

**bin/get.lua:**
```lua
local http = require('http')
local args = _G.args or {}

if #args < 1 then
    print("Usage: hget <url>")
    os.exit(1)
end

local response = http.get(args[1])
print("Status:", response.status)
print("Body:", response:text())
```

**bin/post.lua:**
```lua
local http = require('http')
local args = _G.args or {}

if #args < 2 then
    print("Usage: hpost <url> <data>")
    os.exit(1)
end

local response = http.post(args[1], { body = args[2] })
print("Status:", response.status)
print("Body:", response:text())
```

**Install and use:**
```bash
hype install
hget https://httpbin.org/get
hpost https://httpbin.org/post "hello=world"
```

### Example 3: Tool with Shared Utilities

**hype.json:**
```json
{
  "name": "file-tools",
  "version": "1.0.0",
  "bin": {
    "fread": "bin/read.lua",
    "fwrite": "bin/write.lua"
  }
}
```

**lib/utils.lua:**
```lua
module.exports = {
    validate_file = function(path)
        if not path or path == "" then
            print("Error: No file specified")
            os.exit(1)
        end
    end
}
```

**bin/read.lua:**
```lua
local utils = require('../lib/utils')
local fs = require('fs')
local args = _G.args or {}

utils.validate_file(args[1])
local content = fs.readFileSync(args[1])
print(content)
```

**bin/write.lua:**
```lua
local utils = require('../lib/utils')
local fs = require('fs')
local args = _G.args or {}

utils.validate_file(args[1])
fs.writeFileSync(args[1], args[2] or "")
print("File written:", args[1])
```

## Advanced Usage

### Environment Variables

Override default installation directory:

```bash
export HYPE_HOME=/custom/path
hype install
# Installs to /custom/path/packages/
```

### List as JSON

```bash
hype list --json
```

Output:
```json
{
  "packages": {
    "http-fetcher": {
      "version": "1.0.0",
      "install_date": "2025-10-26T12:00:00Z",
      "location": "/Users/you/.hype/packages/http-fetcher@1.0.0",
      "bin": {
        "fetch": "bin/fetch.lua"
      }
    }
  },
  "total": 1
}
```

### Check Installed Version

```bash
hype which mytool
```

Shows package version and location.

## Best Practices

### 1. Use Descriptive Command Names

✅ Good:
```json
"bin": {
  "http-fetch": "fetch.lua",
  "json-format": "format.lua"
}
```

❌ Bad:
```json
"bin": {
  "f": "fetch.lua",
  "x": "format.lua"
}
```

### 2. Organize Scripts in `bin/` Directory

```
my-package/
├── hype.json
├── bin/
│   ├── command1.lua
│   └── command2.lua
└── lib/
    └── shared.lua
```

### 3. Provide Help Text

```lua
local args = _G.args or {}

if #args == 0 or args[1] == "--help" or args[1] == "-h" then
    print("Usage: mytool <command> [options]")
    print("Commands:")
    print("  fetch   Fetch data from URL")
    print("  post    Post data to URL")
    os.exit(0)
end
```

### 4. Handle Errors Gracefully

```lua
local http = require('http')
local response = http.get(url)

if not response:ok() then
    print("Error: HTTP " .. response.status .. " " .. response.statusText)
    os.exit(1)
end
```

### 5. Use Exit Codes

```lua
os.exit(0)  -- Success
os.exit(1)  -- General error
os.exit(2)  -- Invalid usage
```

## Related Documentation

- [Module System Guide](../modules/README.md) - Learn about `require()` and modules
- [Built-in Modules](../modules/builtin-modules.md) - Available built-in modules (fs, path, http, etc.)
- [require() API](../modules/require-api.md) - Full API documentation
- [Example CLI Tool](../../examples/cli-tool/) - Complete working example

## Future Features

Planned enhancements for global installation:

- **`hype setup`** - Auto-configure PATH for your shell
- **`hype upgrade <name>`** - Update installed packages
- **Remote registry** - Install from package registry (like npm)
- **`hype publish`** - Publish packages to registry
- **Version pinning** - `hype install pkg@1.2.3`
- **Dependency auto-install** - Install dependencies during package install

---

**Last Updated:** October 2025  
**Feature Status:** Phase 6 Complete
