# HTTP CLI Tools - Example Package

A complete example demonstrating hype-rs global package installation with executable CLI tools.

## Overview

This package provides two HTTP CLI tools:

- **hfetch** - Make HTTP GET requests from the command line
- **hpost** - Make HTTP POST requests from the command line

This example demonstrates:

- Using the `bin` field in `hype.json` to expose executable commands
- Creating multiple commands from a single package
- Organizing code with shared utilities in `lib/`
- Using built-in modules (`http`, `fs`)
- Handling command-line arguments (`_G.args`)
- Error handling with meaningful exit codes
- Providing help text and usage information

## Installation

### 1. Install Globally

From this directory:

```bash
hype install
```

Output:
```
Installing http-cli-tools@1.0.0...
✓ Copied package to /Users/you/.hype/packages/http-cli-tools@1.0.0
✓ Created executable: hfetch
✓ Created executable: hpost
✓ Installation complete!

To use the commands, add ~/.hype/bin to your PATH:
  export PATH="$HOME/.hype/bin:$PATH"
```

### 2. Add to PATH

**Bash/Zsh** (`~/.bashrc` or `~/.zshrc`):
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
source ~/.bashrc  # or ~/.zshrc
```

## Usage

### hfetch - HTTP GET Tool

**Basic usage:**
```bash
hfetch <url> [options]
```

**Options:**
- `--json` - Parse and pretty-print JSON response
- `--headers` - Show response headers
- `--verbose` - Show detailed request information
- `-h, --help` - Show help message

**Examples:**

Fetch a URL:
```bash
hfetch https://httpbin.org/get
```

Fetch and parse JSON:
```bash
hfetch https://api.github.com/users/octocat --json
```

Show response headers:
```bash
hfetch https://httpbin.org/get --headers
```

### hpost - HTTP POST Tool

**Basic usage:**
```bash
hpost <url> <data> [options]
```

**Arguments:**
- `url` - URL to post to
- `data` - Data to send (or `@filename` to read from file)

**Options:**
- `--json` - Set Content-Type to application/json
- `--headers` - Show response headers
- `--verbose` - Show detailed request information
- `-h, --help` - Show help message

**Examples:**

Post form data:
```bash
hpost https://httpbin.org/post "name=test&value=123"
```

Post JSON:
```bash
hpost https://httpbin.org/post '{"name":"test","value":123}' --json
```

Post from file:
```bash
hpost https://httpbin.org/post @data.json --json
```

## Package Structure

```
cli-tool/
├── hype.json           # Package manifest with bin field
├── README.md           # This file
├── bin/                # Executable scripts
│   ├── fetch.lua       # HTTP GET command
│   └── post.lua        # HTTP POST command
└── lib/                # Shared utilities
    └── utils.lua       # JSON formatting, validation, etc.
```

## How It Works

### 1. Manifest with bin Field

`hype.json` declares executable commands:

```json
{
  "name": "http-cli-tools",
  "version": "1.0.0",
  "bin": {
    "hfetch": "bin/fetch.lua",
    "hpost": "bin/post.lua"
  }
}
```

### 2. Executable Scripts

Each script in `bin/` is a complete CLI tool:

```lua
local http = require('http')
local args = _G.args or {}

if #args < 1 then
    print("Usage: hfetch <url>")
    os.exit(1)
end

local response = http.get(args[1])
print(response:text())
```

### 3. Global Installation

`hype install` creates wrapper scripts in `~/.hype/bin/`:

```bash
#!/usr/bin/env bash
# Auto-generated wrapper for hfetch
PACKAGE_DIR="$HOME/.hype/packages/http-cli-tools@1.0.0"
exec hype "$PACKAGE_DIR/bin/fetch.lua" "$@"
```

### 4. PATH Integration

Adding `~/.hype/bin` to PATH makes commands available system-wide:

```bash
$ hfetch https://httpbin.org/get
{
  "url": "https://httpbin.org/get",
  ...
}
```

## Using Built-in Modules

### HTTP Module

Make HTTP requests:

```lua
local http = require('http')

local response = http.get('https://api.example.com')
if response:ok() then
    print(response:text())
end

local post_response = http.post('https://api.example.com', {
    body = '{"key":"value"}',
    headers = { ["Content-Type"] = "application/json" }
})
```

### File System Module

Read/write files:

```lua
local fs = require('fs')

local content = fs.readFileSync('data.json')
fs.writeFileSync('output.txt', 'Hello, World!')
```

## Command-Line Arguments

Scripts receive arguments via `_G.args`:

```lua
local args = _G.args or {}

print("Command:", args[0])        -- Command name
print("First arg:", args[1])      -- First argument
print("Total args:", #args)       -- Argument count

for i, arg in ipairs(args) do
    print(i, arg)
end
```

## Error Handling

Use exit codes to indicate success/failure:

```lua
if not valid then
    print("Error: Invalid input")
    os.exit(1)  -- Non-zero exit code for errors
end

print("Success!")
os.exit(0)  -- Zero exit code for success
```

Use pcall for safe error handling:

```lua
local success, result = pcall(function()
    return http.get(url)
end)

if not success then
    print("Error:", result)
    os.exit(1)
end
```

## Shared Utilities

Organize common code in `lib/`:

**lib/utils.lua:**
```lua
module.exports = {
    pretty_json = function(data)
        -- Format JSON for display
    end,
    
    validate_url = function(url)
        -- Validate URL format
    end
}
```

**bin/fetch.lua:**
```lua
local utils = require('../lib/utils')

local data = response:json()
print(utils.pretty_json(data))
```

## Testing the Package

Before installing globally, test scripts directly:

```bash
hype bin/fetch.lua https://httpbin.org/get
hype bin/post.lua https://httpbin.org/post "test=data"
```

## Uninstalling

Remove the package:

```bash
hype uninstall http-cli-tools
```

This removes:
- Package directory: `~/.hype/packages/http-cli-tools@1.0.0/`
- Wrapper scripts: `~/.hype/bin/hfetch`, `~/.hype/bin/hpost`
- Registry entry

## Customization

### Add More Commands

1. Create new script in `bin/`:
   ```bash
   touch bin/delete.lua
   ```

2. Add to `bin` field in `hype.json`:
   ```json
   "bin": {
     "hfetch": "bin/fetch.lua",
     "hpost": "bin/post.lua",
     "hdelete": "bin/delete.lua"
   }
   ```

3. Reinstall:
   ```bash
   hype install --force
   ```

### Add Dependencies

Update `hype.json`:
```json
{
  "dependencies": {
    "some-module": "^1.0.0"
  }
}
```

## Learn More

- [Global Installation Guide](../../docs/features/global-install.md) - Complete documentation
- [Module System](../../docs/modules/README.md) - Using `require()` and modules
- [Built-in Modules](../../docs/modules/builtin-modules.md) - Available modules
- [HTTP Module API](../../docs/modules/http-api.md) - HTTP module reference

## License

MIT
