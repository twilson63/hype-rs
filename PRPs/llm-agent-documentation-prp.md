# Project Request Protocol (PRP): LLM Agent Documentation Flag

**Project ID**: PRP-009  
**Status**: 📋 Proposed  
**Priority**: Medium  
**Created**: 2025-10-26  
**Author**: AI Assistant  
**Estimated Effort**: 3-5 hours (4-6 hours)  

---

## Executive Summary

Implement an `--agent` flag for the hype-rs CLI that outputs machine-readable documentation specifically designed for LLMs (Large Language Models) and AI coding assistants. This will enable AI agents to better understand how to use hype-rs in their code generation and assistance tasks.

**Current State**: LLMs must parse human-readable help text and README files to understand hype-rs capabilities.  
**Desired State**: Single `--agent` command outputs structured, LLM-optimized documentation.  
**Gap**: No machine-readable, LLM-specific documentation format.

**Example Use Case**:
```bash
hype --agent
# Outputs structured documentation optimized for LLM consumption
# Includes capabilities, examples, best practices, and constraints
```

---

## Table of Contents

- [1. Project Overview](#1-project-overview)
- [2. Current State Analysis](#2-current-state-analysis)
- [3. Technical Requirements](#3-technical-requirements)
- [4. Proposed Solutions](#4-proposed-solutions)
  - [Solution 1: JSON Schema Format](#solution-1-json-schema-format)
  - [Solution 2: Markdown with YAML Frontmatter](#solution-2-markdown-with-yaml-frontmatter)
  - [Solution 3: Custom DSL Format](#solution-3-custom-dsl-format)
- [5. Solution Comparison](#5-solution-comparison)
- [6. Recommended Solution](#6-recommended-solution)
- [7. Implementation Plan](#7-implementation-plan)
- [8. Success Criteria](#8-success-criteria)
- [9. Future Enhancements](#9-future-enhancements)

---

## 1. Project Overview

### 1.1 Background

**Problem**: AI coding assistants (Claude, GPT-4, Copilot) need to understand how to use hype-rs effectively but currently rely on:
- Human-readable help text
- README documentation
- Example files
- Trial and error

**Opportunity**: Modern LLMs can process structured documentation much more effectively than unstructured text. A dedicated `--agent` flag could provide:
- Canonical usage patterns
- Capability descriptions
- Constraint information
- Best practices
- Common pitfalls

### 1.2 Goals

1. **Primary Goal**: Enable LLMs to accurately use hype-rs without accessing external documentation
2. **Secondary Goal**: Provide complete API reference for all built-in modules with signatures, examples, and return types
3. **Tertiary Goal**: Enable "tool use" LLMs to generate correct hype-rs code without hallucinating APIs
4. **Quaternary Goal**: Establish pattern for Rust CLI tools to provide LLM-consumable documentation

### 1.3 Key Requirement: Self-Contained Documentation

**CRITICAL**: The `--agent` output MUST be completely self-contained. LLMs should NOT need to:
- Access the project README
- Read external documentation sites
- Query additional endpoints
- Reference example files

**Everything needed must be in the JSON output**:
- Complete API signatures for all modules
- Parameter types and descriptions
- Return types and values
- Error conditions
- Usage examples with expected output
- Common patterns and best practices

### 1.4 Non-Goals

- Not replacing human documentation (README, docs/)
- Not an interactive query system (static output only)
- Not a tutorial system (focused on reference)

### 1.4 Use Cases

**Use Case 1: AI Coding Assistant**
```
User: "Create a Lua script that processes JSON and run it with hype"
Assistant: *runs `hype --agent` to understand capabilities*
Assistant: *generates appropriate Lua script with hype invocation*
```

**Use Case 2: IDE Integration**
```
VSCode extension runs `hype --agent` on startup
Provides autocomplete and inline documentation
Shows capability warnings (e.g., "sandboxing enabled")
```

**Use Case 3: Tool Discovery**
```
Developer: hype --agent | jq '.modules'
# Quickly see all available built-in modules
```

---

## 2. Current State Analysis

### 2.1 Current Documentation

**Available to LLMs**:
- `hype --help` - Human-formatted help text
- `README.md` - User-focused documentation
- `docs/` - Detailed guides
- Examples in `examples/`

**Problems**:
1. **Format Inconsistency**: Each source has different structure
2. **Ambiguity**: Natural language can be misinterpreted
3. **Completeness**: No single source has all information
4. **Versioning**: No clear version-to-capability mapping

### 2.2 What LLMs Need for Tool Use

Based on research on LLM tool use patterns and best practices:

1. **Structured Format**: JSON/YAML for reliable parsing
2. **Complete API Reference**: Full function signatures with types
3. **Parameter Documentation**: Each parameter with type, description, and constraints
4. **Return Value Documentation**: What each function returns, with type information
5. **Complete Capabilities**: What CAN the tool do
6. **Constraints**: What CAN'T the tool do
7. **Error Documentation**: All possible errors with causes and solutions
8. **Examples**: Concrete usage patterns with expected output
9. **Version Info**: Ensure compatibility
10. **Module Relationships**: How modules interact

**Critical for Tool Use LLMs**:
- Function signatures must include parameter names and types
- Return types must be explicit (not just "returns response")
- Optional parameters clearly marked
- Error conditions documented (what errors can occur)
- Examples must show actual usage, not pseudocode

### 2.3 Gaps

| Need | Current State | Gap |
|------|---------------|-----|
| Machine-readable format | Only human text | ❌ No structured output |
| Capability enumeration | Scattered in docs | ❌ No canonical list |
| Constraint documentation | Mentioned in security section | ⚠️ Not comprehensive |
| Version mapping | Only in `--version` | ⚠️ Not linked to features |
| Examples database | In README/examples/ | ❌ Not queryable |

---

## 3. Technical Requirements

### 3.1 Functional Requirements

**FR-1: Agent Flag**
- Add `--agent` flag to CLI
- Mutually exclusive with other flags (runs alone)
- Outputs to stdout for piping
- Exit code 0 on success

**FR-2: Structured Output**
- Machine-readable format (JSON or YAML)
- Versioned schema
- Complete and self-contained
- Deterministic output (same version = same output)

**FR-3: Content Requirements**
Must include (self-contained, no external docs needed):
- Tool identity (name, version, description)
- Capabilities (commands, modules, features)
- **Complete API reference for all built-in modules**:
  - Function signatures with parameter names and types
  - Parameter descriptions with constraints
  - Return type and value documentation
  - Possible errors and error conditions
  - Usage example for each function
- **Response/Return format documentation**:
  - Structure of returned values (e.g., HTTP response format)
  - Field descriptions and types
  - Success/failure indicators
- Constraints (security, limitations, restrictions)
- Usage examples (complete, runnable code)
- Error patterns (common mistakes with solutions)
- Best practices (recommended approaches)

**FR-4: Discoverability**
- Mentioned in `--help` output
- Documented in README
- Examples of usage in docs

### 3.2 Non-Functional Requirements

**NFR-1: Performance**
- Execute in < 100ms
- No network calls
- No file system reads (embedded)

**NFR-2: Maintainability**
- Easy to update when features change
- Validated at compile time (if possible)
- Separate from help text (different audience)

**NFR-3: Stability**
- Schema versioning for compatibility
- Backward compatible changes only
- Clear deprecation process

**NFR-4: Usability**
- Human-readable (pretty-printed JSON/YAML)
- Self-documenting structure
- Minimal cognitive load

---

## 3.3 API Documentation Requirements (Self-Contained)

For `--agent` to be truly useful for tool-use LLMs, every built-in module must have complete API documentation:

### Required for Each Module

**Module-Level**:
- Name and purpose
- When to use this module
- Overall usage example
- Relationship to other modules

**Function-Level** (for EVERY function):
1. **Signature**: `functionName(param1: type, param2?: type): returnType`
   - Parameter names (not just types)
   - Parameter types (string, number, table, any, etc.)
   - Optional parameters marked with `?`
   - Return type explicitly stated

2. **Description**: One-line summary of what function does

3. **Parameters Section**: For each parameter:
   - Name and type
   - Description of purpose
   - Constraints (e.g., "must be valid URL", "max 1000 chars")
   - Default value if optional

4. **Returns Section**:
   - Type of return value
   - Structure if complex (e.g., table with specific fields)
   - Success vs error return differences

5. **Errors Section**:
   - List of possible errors this function can raise
   - Conditions that cause each error
   - How to handle each error

6. **Example**:
   - Complete, runnable code snippet
   - Shows typical usage
   - Includes error handling if relevant

### Required for Complex Return Types

If a function returns a structured type (like HTTP Response), document the structure:

```json
"response_format": {
  "status": {
    "type": "number",
    "description": "HTTP status code (200, 404, 500, etc.)"
  },
  "body": {
    "type": "any",
    "description": "Response body, auto-parsed from JSON if content-type is application/json"
  },
  "headers": {
    "type": "table",
    "description": "Response headers as key-value pairs"
  },
  "ok": {
    "type": "boolean",
    "description": "true if status is 200-299, false otherwise"
  }
}
```

### Why This Level of Detail Matters

**Without complete signatures**: LLM must guess parameter order and types → hallucination  
**With complete signatures**: LLM uses exact API → correct code

**Without return type docs**: LLM doesn't know how to use the result  
**With return type docs**: LLM accesses correct fields

**Without error docs**: LLM can't write defensive code  
**With error docs**: LLM adds proper error handling

---

## 4. Proposed Solutions

### Solution 1: JSON Schema Format

**Overview**: Output a JSON document with a formal JSON Schema definition.

**Example Output**:
```json
{
  "schema_version": "1.0.0",
  "tool": {
    "name": "hype-rs",
    "version": "0.1.0",
    "description": "A fast, lightweight Lua runtime with package management",
    "repository": "https://github.com/twilson63/hype-rs"
  },
  "capabilities": {
    "commands": [
      {
        "name": "run",
        "description": "Execute a Lua script",
        "usage": "hype [script] [args...]",
        "examples": [
          "hype script.lua arg1 arg2",
          "hype run script.lua --verbose"
        ]
      },
      {
        "name": "install",
        "description": "Install a package globally",
        "usage": "hype install [path]",
        "examples": ["hype install", "hype install ./my-pkg"]
      }
    ],
    "modules": [
      {
        "name": "fs",
        "description": "File system operations (read/write files, directories)",
        "api": {
          "readFile": {
            "signature": "fs.readFile(path: string): string",
            "description": "Read entire file as string",
            "example": "local content = fs.readFile('data.txt')",
            "returns": "File content as string",
            "errors": ["FileNotFound", "PermissionDenied"]
          },
          "writeFile": {
            "signature": "fs.writeFile(path: string, content: string): boolean",
            "description": "Write string to file, creating if needed",
            "example": "fs.writeFile('out.txt', 'Hello')",
            "returns": "true on success",
            "errors": ["PermissionDenied", "DiskFull"]
          },
          "exists": {
            "signature": "fs.exists(path: string): boolean",
            "description": "Check if file or directory exists",
            "example": "if fs.exists('config.json') then ... end",
            "returns": "true if exists, false otherwise"
          },
          "mkdir": {
            "signature": "fs.mkdir(path: string): boolean",
            "description": "Create directory (including parent dirs)",
            "example": "fs.mkdir('data/output')",
            "returns": "true on success"
          },
          "readDir": {
            "signature": "fs.readDir(path: string): table",
            "description": "List directory contents",
            "example": "local files = fs.readDir('.')",
            "returns": "Array of filenames"
          }
        },
        "usage_example": "local fs = require('fs')\nlocal data = fs.readFile('input.txt')\nfs.writeFile('output.txt', data)"
      },
      {
        "name": "http",
        "description": "HTTP client for API requests with JSON support",
        "api": {
          "get": {
            "signature": "http.get(url: string, options?: table): Response",
            "description": "Perform HTTP GET request",
            "example": "local res = http.get('https://api.github.com/users/octocat')",
            "params": {
              "url": "Full URL including protocol",
              "options": "Optional table with headers, timeout"
            },
            "returns": "Response table with status, body, headers"
          },
          "post": {
            "signature": "http.post(url: string, body: any, options?: table): Response",
            "description": "Perform HTTP POST request with JSON body",
            "example": "local res = http.post('https://api.example.com/data', {name='test'})",
            "params": {
              "url": "Full URL including protocol",
              "body": "Table to be JSON encoded, or string",
              "options": "Optional headers, timeout"
            },
            "returns": "Response table"
          },
          "put": {
            "signature": "http.put(url: string, body: any, options?: table): Response",
            "description": "Perform HTTP PUT request",
            "example": "http.put('https://api.example.com/item/1', {status='done'})"
          },
          "delete": {
            "signature": "http.delete(url: string, options?: table): Response",
            "description": "Perform HTTP DELETE request",
            "example": "http.delete('https://api.example.com/item/1')"
          }
        },
        "response_format": {
          "status": "HTTP status code (number)",
          "body": "Response body (string, auto-parsed if JSON)",
          "headers": "Response headers (table)",
          "ok": "true if status 200-299 (boolean)"
        },
        "usage_example": "local http = require('http')\nlocal res = http.get('https://api.github.com/users/octocat')\nif res.ok then\n  print('Name:', res.body.name)\nend"
      },
      {
        "name": "path",
        "description": "Path manipulation utilities (cross-platform)",
        "api": {
          "join": {
            "signature": "path.join(...parts: string): string",
            "description": "Join path segments using OS separator",
            "example": "path.join('data', 'files', 'test.txt') -- data/files/test.txt",
            "returns": "Combined path string"
          },
          "dirname": {
            "signature": "path.dirname(path: string): string",
            "description": "Get directory part of path",
            "example": "path.dirname('/usr/local/bin/hype') -- /usr/local/bin"
          },
          "basename": {
            "signature": "path.basename(path: string): string",
            "description": "Get filename part of path",
            "example": "path.basename('/usr/local/bin/hype') -- hype"
          },
          "resolve": {
            "signature": "path.resolve(path: string): string",
            "description": "Convert to absolute path",
            "example": "path.resolve('./data') -- /home/user/project/data"
          },
          "extension": {
            "signature": "path.extension(path: string): string",
            "description": "Get file extension",
            "example": "path.extension('file.lua') -- .lua"
          }
        },
        "usage_example": "local path = require('path')\nlocal file = path.join(path.dirname(arg[0]), 'data.txt')"
      },
      {
        "name": "events",
        "description": "Event emitter for pub/sub patterns",
        "api": {
          "new": {
            "signature": "events.new(): EventEmitter",
            "description": "Create new event emitter",
            "example": "local emitter = events.new()"
          },
          "on": {
            "signature": "emitter:on(event: string, handler: function)",
            "description": "Register event listener",
            "example": "emitter:on('data', function(val) print(val) end)"
          },
          "emit": {
            "signature": "emitter:emit(event: string, ...args)",
            "description": "Trigger event with arguments",
            "example": "emitter:emit('data', 'hello', 123)"
          },
          "off": {
            "signature": "emitter:off(event: string, handler?: function)",
            "description": "Remove event listener(s)",
            "example": "emitter:off('data')"
          }
        },
        "usage_example": "local events = require('events')\nlocal emitter = events.new()\nemitter:on('ready', function() print('Started!') end)\nemitter:emit('ready')"
      },
      {
        "name": "util",
        "description": "General utility functions",
        "api": {
          "inspect": {
            "signature": "util.inspect(value: any): string",
            "description": "Pretty-print any Lua value for debugging",
            "example": "print(util.inspect({a=1, b={c=2}}))"
          },
          "sleep": {
            "signature": "util.sleep(seconds: number)",
            "description": "Sleep/pause execution",
            "example": "util.sleep(1.5) -- Sleep 1.5 seconds"
          },
          "time": {
            "signature": "util.time(): number",
            "description": "Get current Unix timestamp",
            "example": "local now = util.time()"
          }
        },
        "usage_example": "local util = require('util')\nprint(util.inspect({name='test', values={1,2,3}}))"
      },
      {
        "name": "table",
        "description": "Extended table operations beyond standard library",
        "api": {
          "merge": {
            "signature": "table.merge(t1: table, t2: table): table",
            "description": "Merge two tables (shallow)",
            "example": "local result = table.merge({a=1}, {b=2}) -- {a=1, b=2}"
          },
          "clone": {
            "signature": "table.clone(t: table): table",
            "description": "Create shallow copy of table",
            "example": "local copy = table.clone(original)"
          },
          "keys": {
            "signature": "table.keys(t: table): table",
            "description": "Get array of table keys",
            "example": "local k = table.keys({a=1, b=2}) -- {'a', 'b'}"
          },
          "values": {
            "signature": "table.values(t: table): table",
            "description": "Get array of table values",
            "example": "local v = table.values({a=1, b=2}) -- {1, 2}"
          }
        },
        "usage_example": "local tbl = require('table')\nlocal config = tbl.merge(defaults, user_config)"
      }
    ],
    "features": [
      "Lua 5.4 runtime",
      "Module system with require()",
      "Global package installation",
      "Security sandboxing",
      "Cross-platform support"
    ]
  },
  "constraints": {
    "security": {
      "sandboxing": true,
      "restricted_operations": [
        "os.execute() - Blocked for security",
        "dofile() - Blocked to prevent code injection",
        "load() - Restricted runtime code loading"
      ],
      "memory_limits": "Configurable, default safe limits",
      "instruction_limits": "Prevents infinite loops"
    },
    "limitations": [
      "No Windows support yet (macOS/Linux only)",
      "Lua 5.4 only (not compatible with 5.1/5.2)",
      "Single-threaded execution"
    ]
  },
  "best_practices": [
    "Use require() for code organization",
    "Install packages globally for CLI tools",
    "Use built-in modules instead of external dependencies",
    "Handle errors with pcall() for robust scripts",
    "Keep scripts focused and modular"
  ],
  "common_errors": [
    {
      "pattern": "ModuleNotFoundError",
      "cause": "Module not installed or wrong path",
      "solution": "Use 'hype install' or check hype_modules/"
    },
    {
      "pattern": "SecurityError: os.execute blocked",
      "cause": "Attempting restricted operation",
      "solution": "Use built-in modules like fs or http instead"
    }
  ],
  "examples": {
    "basic_script": {
      "description": "Run a simple Lua script",
      "code": "print('Hello from Hype!')",
      "command": "hype hello.lua"
    },
    "with_arguments": {
      "description": "Pass arguments to script",
      "code": "print('Args:', table.concat(arg, ', '))",
      "command": "hype script.lua arg1 arg2"
    },
    "use_module": {
      "description": "Use built-in http module",
      "code": "local http = require('http')\nlocal res = http.get('https://api.github.com')\nprint(res.body)",
      "command": "hype api.lua"
    }
  }
}
```

**Implementation**:
```rust
// src/cli/agent.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AgentDocumentation {
    schema_version: String,
    tool: ToolInfo,
    capabilities: Capabilities,
    constraints: Constraints,
    best_practices: Vec<String>,
    common_errors: Vec<CommonError>,
    examples: HashMap<String, Example>,
}

pub fn generate_agent_docs() -> String {
    let doc = AgentDocumentation {
        schema_version: "1.0.0".to_string(),
        tool: ToolInfo {
            name: "hype-rs".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            // ... etc
        },
        // ... populate all fields
    };
    
    serde_json::to_string_pretty(&doc).unwrap()
}
```

**Pros**:
- ✅ Industry standard format (JSON)
- ✅ Formal schema validation possible
- ✅ Easy to parse in any language
- ✅ Good tooling support (jq, JSON parsers)
- ✅ TypeScript types can be auto-generated
- ✅ Self-validating with JSON Schema

**Cons**:
- ❌ Verbose for humans to read
- ❌ No comments allowed in pure JSON
- ❌ Schema definition separate file
- ❌ May feel "heavy" for simple tool

**Complexity**: Medium (requires JSON schema definition)

---

### Solution 2: Markdown with YAML Frontmatter

**Overview**: Output structured Markdown with YAML frontmatter for metadata.

**Example Output**:
```markdown
---
schema_version: 1.0.0
tool:
  name: hype-rs
  version: 0.1.0
  description: A fast, lightweight Lua runtime with package management
capabilities:
  commands:
    - name: run
      usage: hype [script] [args...]
    - name: install
      usage: hype install [path]
  modules: [fs, http, path, events, util, table]
  features:
    - Lua 5.4 runtime
    - Module system with require()
    - Global package installation
---

# Hype-RS Agent Documentation

## Tool Overview

**hype-rs** is a high-performance Lua runtime written in Rust. It provides:
- Fast script execution (~50ms startup)
- Package management system
- Built-in modules for common tasks
- Security sandboxing

## Capabilities

### Commands

**run** - Execute a Lua script
```bash
hype script.lua arg1 arg2
hype run script.lua --verbose
```

**install** - Install a package globally
```bash
hype install
hype install ./my-package
```

**list** - List installed packages
```bash
hype list
hype list --json
```

### Built-in Modules

#### `fs` - File System
```lua
local fs = require('fs')
local content = fs.readFile('data.txt')
fs.writeFile('output.txt', content)
```

#### `http` - HTTP Client
```lua
local http = require('http')
local response = http.get('https://api.example.com')
print(response.body)
```

## Constraints

### Security Sandboxing
- `os.execute()` is **blocked** for security
- `dofile()` is **blocked** to prevent code injection
- Memory and instruction limits prevent resource exhaustion

### Limitations
- Windows support not yet available (macOS/Linux only)
- Lua 5.4 only
- Single-threaded execution

## Best Practices

1. **Use require() for modularity** - Organize code into modules
2. **Install packages globally** - For reusable CLI tools
3. **Use built-in modules** - Avoid external dependencies
4. **Error handling** - Use pcall() for robust scripts
5. **Keep scripts focused** - Single responsibility principle

## Common Errors

### ModuleNotFoundError
**Cause**: Module not installed or incorrect path  
**Solution**: Run `hype install` or check `hype_modules/` directory

### SecurityError: os.execute blocked
**Cause**: Attempting to execute shell commands  
**Solution**: Use built-in `fs` or `http` modules instead

## Examples

### Basic Script Execution
```lua
-- hello.lua
print("Hello from Hype!")
```
```bash
hype hello.lua
```

### Using Built-in Modules
```lua
-- api.lua
local http = require('http')
local json = require('json')

local response = http.get('https://api.github.com/users/octocat')
local data = json.decode(response.body)
print(data.name)
```
```bash
hype api.lua
```

### Creating a CLI Tool
```json
{
  "name": "my-tool",
  "version": "1.0.0",
  "bin": {
    "mytool": "bin/main.lua"
  }
}
```
```bash
hype install
mytool --help
```

---

*Generated by hype-rs v0.1.0 - https://github.com/twilson63/hype-rs*
```

**Implementation**:
```rust
pub fn generate_agent_docs() -> String {
    let yaml_metadata = serde_yaml::to_string(&metadata()).unwrap();
    let markdown_body = include_str!("agent_docs_template.md");
    
    format!("---\n{}\n---\n\n{}", yaml_metadata, markdown_body)
}
```

**Pros**:
- ✅ Human and machine readable
- ✅ Rich formatting with code blocks
- ✅ Structured metadata in frontmatter
- ✅ Easy to maintain (separate template file)
- ✅ GitHub/IDE friendly rendering
- ✅ Can include examples with syntax highlighting

**Cons**:
- ❌ Requires YAML + Markdown parsers
- ❌ Frontmatter not universal standard
- ❌ Harder to query specific fields
- ❌ Mixed format may confuse some tools
- ❌ Larger output size

**Complexity**: Medium-High (template system needed)

---

### Solution 3: Custom DSL Format

**Overview**: Design a custom Domain-Specific Language optimized for LLM consumption.

**Example Output**:
```
AGENT_DOCS v1.0.0

TOOL hype-rs 0.1.0
  description: A fast, lightweight Lua runtime with package management
  repository: https://github.com/twilson63/hype-rs

CAPABILITIES
  COMMAND run
    usage: hype [script] [args...]
    example: hype script.lua arg1 arg2
    example: hype run script.lua --verbose
  END

  COMMAND install
    usage: hype install [path]
    example: hype install
    example: hype install ./my-package
  END

  MODULE fs
    description: File system operations
    functions: readFile, writeFile, exists, mkdir, readDir
    example:
      local fs = require('fs')
      local content = fs.readFile('data.txt')
      fs.writeFile('output.txt', content)
    end
  END

  MODULE http
    description: HTTP client for API requests
    functions: get, post, put, delete
    example:
      local http = require('http')
      local response = http.get('https://api.example.com')
      print(response.body)
    end
  END

CONSTRAINTS
  SECURITY sandboxing=enabled
    blocked: os.execute() - Blocked for security
    blocked: dofile() - Blocked to prevent code injection
    blocked: load() - Restricted runtime code loading
    memory_limits: Configurable, default safe limits
    instruction_limits: Prevents infinite loops
  END

  LIMITATIONS
    - No Windows support yet (macOS/Linux only)
    - Lua 5.4 only (not compatible with 5.1/5.2)
    - Single-threaded execution
  END

BEST_PRACTICES
  - Use require() for code organization
  - Install packages globally for CLI tools
  - Use built-in modules instead of external dependencies
  - Handle errors with pcall() for robust scripts
  - Keep scripts focused and modular

COMMON_ERRORS
  ERROR ModuleNotFoundError
    cause: Module not installed or wrong path
    solution: Use 'hype install' or check hype_modules/
  END

  ERROR SecurityError: os.execute blocked
    cause: Attempting restricted operation
    solution: Use built-in modules like fs or http instead
  END

EXAMPLES
  BASIC_SCRIPT "Run a simple Lua script"
    code:
      print('Hello from Hype!')
    command: hype hello.lua
  END

  WITH_ARGUMENTS "Pass arguments to script"
    code:
      print('Args:', table.concat(arg, ', '))
    command: hype script.lua arg1 arg2
  END

  USE_MODULE "Use built-in http module"
    code:
      local http = require('http')
      local res = http.get('https://api.github.com')
      print(res.body)
    command: hype api.lua
  END
```

**Implementation**:
```rust
pub fn generate_agent_docs() -> String {
    let mut output = String::new();
    
    output.push_str("AGENT_DOCS v1.0.0\n\n");
    output.push_str(&format!("TOOL {} {}\n", PKG_NAME, PKG_VERSION));
    // ... build string programmatically
    
    output
}
```

**Pros**:
- ✅ Optimized specifically for LLMs
- ✅ Compact and efficient
- ✅ Clear structure with keywords
- ✅ Easy to generate programmatically
- ✅ Human-readable with practice
- ✅ No external dependencies

**Cons**:
- ❌ Custom format requires documentation
- ❌ No existing tooling support
- ❌ Need to write custom parser for tooling
- ❌ Not a standard format
- ❌ May not be familiar to developers
- ❌ Limited ecosystem

**Complexity**: Low (simple string building)

---

## 5. Solution Comparison

### 5.1 Feature Matrix

| Feature | JSON Schema | Markdown+YAML | Custom DSL |
|---------|------------|---------------|------------|
| Machine Readable | ✅ Excellent | ⚠️ Good | ⚠️ Good |
| Human Readable | ❌ Verbose | ✅ Excellent | ✅ Good |
| Parsing Ease | ✅ Native support | ⚠️ Dual format | ❌ Custom parser |
| Tooling Support | ✅ Excellent | ✅ Good | ❌ None |
| LLM Optimized | ✅ Good | ✅ Good | ✅ Excellent |
| Maintainability | ✅ Good | ⚠️ Template needed | ✅ Excellent |
| Size | ❌ Verbose | ❌ Large | ✅ Compact |
| Standards | ✅ JSON Schema | ⚠️ Informal | ❌ None |
| Validation | ✅ Built-in | ⚠️ Manual | ❌ Manual |

### 5.2 Effort Comparison

| Solution | Setup Time | Maintenance | Learning Curve | Testing |
|----------|-----------|-------------|----------------|---------|
| JSON Schema | 3-4 hours | Low | Low | Easy |
| Markdown+YAML | 4-5 hours | Medium | Low | Medium |
| Custom DSL | 2-3 hours | Low | Medium | Hard |

### 5.3 Use Case Fit

**JSON Schema:**
- ✅ Best for: Tool integrations, IDE plugins, automated processing
- ✅ Best when: Formal validation needed
- ❌ Not ideal for: Human-first reading

**Markdown+YAML:**
- ✅ Best for: Human + machine consumption
- ✅ Best when: Rich formatting needed
- ❌ Not ideal for: Simple data extraction

**Custom DSL:**
- ✅ Best for: LLM-specific optimization
- ✅ Best when: Full control over format needed
- ❌ Not ideal for: Third-party tool integration

---

## 6. Recommended Solution

### 6.1 Selected Solution: **JSON Schema Format** (Solution 1)

**Rationale:**

1. **Industry Standard**: JSON is universally supported
2. **Tool Integration**: Easy for IDEs, extensions, and automation
3. **LLM Friendly**: Modern LLMs excel at parsing JSON
4. **Validation**: JSON Schema provides formal validation
5. **Future-Proof**: Established standard with long-term support
6. **TypeScript Integration**: Can auto-generate types for tooling

**Trade-offs Accepted:**
- Less human-readable than Markdown (acceptable - target is machines)
- Larger file size (acceptable - still < 50KB)
- Requires schema file (one-time cost)

### 6.2 Hybrid Approach

For maximum value, implement JSON as primary with optional formats:

```bash
hype --agent              # Default: JSON
hype --agent --format json     # Explicit JSON
hype --agent --format markdown # Optional: Markdown
```

This gives flexibility while keeping JSON as the canonical format.

### 6.3 Schema Design

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "title": "Hype-RS Agent Documentation",
  "type": "object",
  "required": ["schema_version", "tool", "capabilities"],
  "properties": {
    "schema_version": {
      "type": "string",
      "pattern": "^\\d+\\.\\d+\\.\\d+$"
    },
    "tool": {
      "type": "object",
      "required": ["name", "version", "description"],
      "properties": {
        "name": { "type": "string" },
        "version": { "type": "string" },
        "description": { "type": "string" },
        "repository": { "type": "string", "format": "uri" }
      }
    }
    // ... full schema definition
  }
}
```

---

## 7. Implementation Plan

### Phase 1: Core Implementation (2-3 hours)

**Objective**: Implement basic `--agent` flag with JSON output

**Tasks:**

1. **Add CLI flag** (30 min)
   ```rust
   // src/cli/mod.rs
   #[derive(Parser)]
   pub struct Cli {
       #[arg(long, help = "Output agent documentation in JSON format")]
       agent: bool,
       // ... existing fields
   }
   ```

2. **Create agent module** (1 hour)
   ```rust
   // src/cli/agent.rs
   mod structures;
   mod generator;
   mod schema;
   
   pub use generator::generate_agent_docs;
   ```

3. **Define data structures** (1 hour)
   ```rust
   // src/cli/agent/structures.rs
   #[derive(Serialize, Deserialize, JsonSchema)]
   pub struct AgentDocumentation {
       schema_version: String,
       tool: ToolInfo,
       capabilities: Capabilities,
       constraints: Constraints,
       best_practices: Vec<String>,
       common_errors: Vec<CommonError>,
       examples: HashMap<String, Example>,
   }
   
   // ... all nested structures
   ```

4. **Implement generator** (30 min)
   ```rust
   // src/cli/agent/generator.rs
   pub fn generate_agent_docs() -> Result<String> {
       let doc = build_documentation();
       Ok(serde_json::to_string_pretty(&doc)?)
   }
   
   fn build_documentation() -> AgentDocumentation {
       AgentDocumentation {
           schema_version: "1.0.0".to_string(),
           tool: build_tool_info(),
           capabilities: build_capabilities(),
           constraints: build_constraints(),
           best_practices: build_best_practices(),
           common_errors: build_common_errors(),
           examples: build_examples(),
       }
   }
   ```

**Deliverables:**
- ✅ `--agent` flag functional
- ✅ JSON output generated
- ✅ Basic documentation populated

---

### Phase 2: Content Population (1-2 hours)

**Objective**: Fill in comprehensive documentation content

**Tasks:**

1. **Populate capabilities** (30 min)
   - List all commands with examples
   - Document all built-in modules
   - List all features

2. **Document constraints** (30 min)
   - Security restrictions
   - Platform limitations
   - Known issues

3. **Add best practices** (15 min)
   - Curated list from experience
   - Based on common use cases

4. **Common errors section** (30 min)
   - Gather from issues/support
   - Pattern + solution format

5. **Create examples** (15 min)
   - Cover common scenarios
   - Include code + command

**Deliverables:**
- ✅ Complete documentation content
- ✅ All sections populated
- ✅ Accurate and up-to-date

---

### Phase 3: Validation & Schema (30 min - 1 hour)

**Objective**: Add JSON Schema validation

**Tasks:**

1. **Create JSON Schema file** (20 min)
   ```bash
   # schemas/agent-docs-v1.schema.json
   {
     "$schema": "https://json-schema.org/draft/2020-12/schema",
     "title": "Hype-RS Agent Documentation v1",
     // ... full schema
   }
   ```

2. **Add schema validation** (15 min)
   ```rust
   #[cfg(test)]
   mod tests {
       #[test]
       fn test_agent_docs_valid_schema() {
           let docs = generate_agent_docs().unwrap();
           let schema = include_str!("../../../schemas/agent-docs-v1.schema.json");
           // Validate docs against schema
       }
   }
   ```

3. **Add --validate flag** (15 min)
   ```rust
   #[arg(long, help = "Validate agent docs against schema")]
   validate_agent: bool,
   ```

**Deliverables:**
- ✅ JSON Schema file created
- ✅ Validation tests passing
- ✅ Schema published

---

### Phase 4: Documentation & Testing (1 hour)

**Objective**: Document the feature and add tests

**Tasks:**

1. **Update README** (15 min)
   ```markdown
   ### For AI Agents and LLMs
   
   Get machine-readable documentation:
   ```bash
   hype --agent
   ```
   
   This outputs structured JSON optimized for LLM consumption.
   ```

2. **Create usage guide** (20 min)
   ```bash
   # docs/agent-documentation.md
   # Agent Documentation Guide
   
   ## Overview
   The `--agent` flag provides machine-readable documentation...
   
   ## Usage
   ## Schema
   ## Examples
   ```

3. **Add integration tests** (20 min)
   ```rust
   #[test]
   fn test_agent_flag_outputs_json() {
       let output = Command::new("hype")
           .arg("--agent")
           .output()
           .expect("Failed to execute");
       
       let json: serde_json::Value = 
           serde_json::from_slice(&output.stdout).unwrap();
       
       assert_eq!(json["schema_version"], "1.0.0");
       assert!(json["capabilities"].is_object());
   }
   ```

4. **Add to help text** (5 min)
   ```rust
   /// Output agent documentation in JSON format
   ///
   /// Generates machine-readable documentation optimized
   /// for LLMs and AI coding assistants.
   #[arg(long)]
   agent: bool,
   ```

**Deliverables:**
- ✅ README updated
- ✅ Usage guide created
- ✅ Tests passing
- ✅ Help text updated

---

### Phase 5: Optional Enhancements (1 hour)

**Objective**: Add nice-to-have features

**Tasks:**

1. **Add format option** (20 min)
   ```rust
   #[arg(long, value_enum, default_value = "json")]
   agent_format: AgentFormat,
   
   enum AgentFormat {
       Json,
       Yaml,
       Markdown,
   }
   ```

2. **Add filter option** (20 min)
   ```rust
   #[arg(long, help = "Filter agent docs by section")]
   agent_section: Option<String>,
   ```
   
   ```bash
   hype --agent --agent-section capabilities
   # Output only capabilities section
   ```

3. **Add compact mode** (20 min)
   ```rust
   #[arg(long, help = "Output compact JSON (no pretty print)")]
   agent_compact: bool,
   ```

**Deliverables:**
- ✅ Multiple output formats
- ✅ Section filtering
- ✅ Compact output option

---

## 8. Success Criteria

### 8.1 Functional Tests

✅ **Basic Functionality**:
- [ ] `hype --agent` outputs valid JSON
- [ ] JSON validates against schema
- [ ] All sections populated with content
- [ ] Version matches `hype --version`
- [ ] Exit code 0 on success

✅ **Content Completeness** (Critical for Tool Use):
- [ ] **Every built-in module documented** (fs, http, path, events, util, table)
- [ ] **Every function has complete signature** with parameter names and types
- [ ] **Every parameter documented** with type and description
- [ ] **Every function has return type** documented
- [ ] **Response/return structures fully documented** (e.g., HTTP Response format)
- [ ] **All possible errors listed** for each function
- [ ] **Usage example for each function** (not just module-level)
- [ ] **At least 10 complete examples** covering common use cases

✅ **Content Quality**:
- [ ] All commands documented with examples
- [ ] Security constraints clearly stated
- [ ] At least 10 common errors documented with solutions
- [ ] Best practices for each module type

✅ **Integration**:
- [ ] Works standalone (no dependencies)
- [ ] Output parseable by `jq`
- [ ] Compatible with LLM context windows (< 50KB)
- [ ] Deterministic output (same version = same output)

### 8.2 LLM Validation Tests (Critical)

Test with actual tool-use LLMs to ensure self-contained documentation:

**Test 1: API Discovery Without External Docs**
```
Setup: LLM has access only to `hype --agent` output (no README, no external docs)
Prompt: "What HTTP methods are available in hype-rs?"
Expected: Correctly lists get, post, put, delete with signatures
Success Criteria: LLM doesn't say "I don't know" or hallucinate methods
```

**Test 2: Function Signature Accuracy**
```
Setup: LLM has access only to `hype --agent` output
Prompt: "What parameters does http.post take?"
Expected: url (string), body (any/table), optional options (table)
Success Criteria: LLM provides exact signature with types
```

**Test 3: Return Type Understanding**
```
Setup: LLM has access only to `hype --agent` output
Prompt: "What does http.get return? What fields are in the response?"
Expected: Returns Response table with status, body, headers, ok fields
Success Criteria: LLM describes complete response structure
```

**Test 4: End-to-End Code Generation**
```
Setup: LLM has access only to `hype --agent` output
Prompt: "Write code to fetch JSON from API and save to file"
Expected: Correct code using http.get and fs.writeFile with proper error handling
Success Criteria: 
  - Code compiles and runs
  - Uses correct function signatures
  - Handles response.ok correctly
  - No hallucinated APIs
```

**Test 5: Error Handling Knowledge**
```
Setup: LLM has access only to `hype --agent` output
Prompt: "What errors can occur when using fs.writeFile?"
Expected: PermissionDenied, DiskFull (from errors list)
Success Criteria: LLM lists actual documented errors, doesn't guess
```

**Test 6: Constraint Awareness**
```
Setup: LLM has access only to `hype --agent` output
Prompt: "How do I execute a shell command in hype-rs?"
Expected: States os.execute() is blocked, suggests using fs or http modules
Success Criteria: LLM doesn't suggest blocked operations
```

**Test 7: Module Interaction**
```
Setup: LLM has access only to `hype --agent` output
Prompt: "How do I build a file path that works on all platforms?"
Expected: Use path.join() from path module
Success Criteria: LLM knows path module exists and its purpose
```

### 8.3 Performance Metrics

- [ ] Execution time < 100ms
- [ ] Output size < 50KB
- [ ] Memory usage < 10MB
- [ ] No network calls
- [ ] No file system reads

---

## 9. Future Enhancements

### 9.1 Short-term (Next Release)

- [ ] **OpenAPI-style schema** for API-like documentation
- [ ] **Version compatibility matrix** showing feature availability by version
- [ ] **Interactive mode** with `--agent-query "how to install packages"`
- [ ] **Generate TypeScript types** from schema for tooling

### 9.2 Medium-term

- [ ] **Machine-readable examples** with test cases
- [ ] **Performance characteristics** (benchmarks, resource usage)
- [ ] **Dependency graph** showing module relationships
- [ ] **Migration guides** for version upgrades
- [ ] **Agent prompt templates** optimized for different LLMs

### 9.3 Long-term

- [ ] **Publish schema** to schema registry (schema.org)
- [ ] **Universal agent docs** standard for Rust CLI tools
- [ ] **Agent SDK** for programmatic access
- [ ] **Real-time validation** service
- [ ] **Community contributions** to examples and patterns

---

## 10. Risk Assessment

### 10.1 Technical Risks

**Risk 1: Schema Evolution**  
**Severity**: Medium  
**Mitigation**:
- Version schema explicitly
- Maintain backward compatibility
- Document breaking changes
- Provide migration guide

**Risk 2: Content Staleness**  
**Severity**: Medium  
**Mitigation**:
- Generate from code where possible
- Automated validation in CI
- Review process for updates
- Link to version releases

**Risk 3: Output Size**  
**Severity**: Low  
**Mitigation**:
- Monitor size in tests
- Compact mode available
- Section filtering option
- Optimize JSON structure

### 10.2 Adoption Risks

**Risk 1: Low Usage**  
**Severity**: Low  
**Mitigation**:
- Document in README
- Share with AI tool communities
- Provide examples
- Low maintenance cost anyway

**Risk 2: LLM Incompatibility**  
**Severity**: Low  
**Mitigation**:
- Test with multiple LLMs
- Follow best practices
- Gather feedback
- Iterate based on usage

---

## 11. Example Usage

## 11. Example Usage Scenarios

### 11.1 Tool Use LLM - API Call Generation

**Scenario**: LLM needs to generate code that calls GitHub API

```
System Prompt:
You have access to hype-rs, a Lua runtime. 
Run `hype --agent` to get complete API documentation.
The output is self-contained - you don't need to access external docs.

User: "Create a Lua script that fetches user info from GitHub API and prints the name"

LLM Process:
1. Runs: hype --agent
2. Parses JSON, finds http module
3. Reads http.get signature: http.get(url: string, options?: table): Response
4. Reads Response format: {status: number, body: any, headers: table, ok: boolean}
5. Sees example: local res = http.get('https://api.github.com/users/octocat')
6. Generates correct code:

Generated Code:
```lua
local http = require('http')

local response = http.get('https://api.github.com/users/octocat')

if response.ok then
  print('Name:', response.body.name)
  print('Bio:', response.body.bio)
else
  print('Error:', response.status)
end
```

Command: hype github_user.lua

Why this works:
- LLM knew http module exists (from modules list)
- LLM knew exact function signature (from api.get)
- LLM knew response structure (from response_format)
- LLM knew body is auto-parsed JSON (from description)
- LLM had example to pattern-match (from usage_example)
```

### 11.2 Tool Use LLM - File Operations

**Scenario**: LLM needs to read config file and process it

```
User: "Read config.json, modify the port to 8080, and write it back"

LLM Process:
1. Checks --agent output for fs module
2. Finds fs.readFile(path: string): string
3. Finds fs.writeFile(path: string, content: string): boolean
4. Notes: no built-in JSON module mentioned
5. Generates code using standard Lua JSON:

Generated Code:
```lua
local fs = require('fs')

-- Read config file
local content = fs.readFile('config.json')

-- Parse JSON (using Lua's built-in JSON if available)
local config = {port = 3000, host = 'localhost'} -- simplified for example
config.port = 8080

-- Write back
local json_str = string.format('{"port": %d, "host": "%s"}', config.port, config.host)
fs.writeFile('config.json', json_str)
print('Updated port to 8080')
```

Why this works:
- LLM knew exact fs function signatures
- LLM knew readFile returns string (not table)
- LLM knew writeFile takes string (not table)
- LLM understood it needs to handle JSON serialization
```

### 11.3 Tool Use LLM - Error Handling

**Scenario**: LLM generates robust code with error handling

```
User: "Download a file from URL and save it, handle errors"

LLM Process:
1. Checks http.get errors: can fail with network errors
2. Checks fs.writeFile errors: PermissionDenied, DiskFull
3. Generates code with proper error handling:

Generated Code:
```lua
local http = require('http')
local fs = require('fs')

-- Try to download
local response = http.get('https://example.com/data.json')

if not response.ok then
  print('Download failed with status:', response.status)
  os.exit(1)
end

-- Try to write file
local success, err = pcall(function()
  fs.writeFile('data.json', response.body)
end)

if not success then
  print('Failed to write file:', err)
  os.exit(1)
end

print('Successfully downloaded and saved data.json')
```

Why this works:
- LLM saw response.ok in response_format
- LLM saw possible errors in fs.writeFile documentation
- LLM used pcall based on Lua best practices
```

### 11.4 IDE Integration Example

**VSCode Extension**:
```typescript
import { exec } from 'child_process';

// On extension activation
async function loadHypeDocumentation() {
  const agentDocs = await execAsync('hype --agent');
  const docs = JSON.parse(agentDocs);
  
  // Register autocomplete for modules
  docs.capabilities.modules.forEach(module => {
    registerCompletionProvider(module.name, module.api);
  });
  
  // Register hover documentation
  docs.capabilities.modules.forEach(module => {
    Object.entries(module.api).forEach(([funcName, funcDoc]) => {
      registerHoverProvider(
        `${module.name}.${funcName}`,
        formatDocumentation(funcDoc)
      );
    });
  });
}

// When user types "http.get("
function provideCompletion(position) {
  // Show signature: http.get(url: string, options?: table): Response
  // Show description: "Perform HTTP GET request"
  // Show example: local res = http.get('https://api.github.com')
}
```

### 11.5 Automated Testing from Docs

**Test Generator**:
```bash
# Generate tests from agent docs
hype --agent | jq '.examples' | \
  jq -r 'to_entries[] | 
    "# Test: \(.key)\n\(.value.code)\n# Expected command: \(.value.command)\n"'

# Output:
# Test: basic_script
# print('Hello from Hype!')
# Expected command: hype hello.lua

# Test: use_module
# local http = require('http')
# local res = http.get('https://api.github.com')
# print(res.body)
# Expected command: hype api.lua
```

---

## 12. Complete Example: Self-Contained Tool Use

### 12.1 The Goal

An LLM should be able to generate correct hype-rs code for ANY built-in capability using ONLY the `--agent` output.

### 12.2 Test Case

**User Request**: "Create a script that:
1. Reads a list of URLs from urls.txt (one per line)
2. Fetches each URL
3. Saves the responses to individual files
4. Handles errors gracefully"

**What LLM needs from `--agent`**:
- ✅ How to read file (fs.readFile signature)
- ✅ How to write file (fs.writeFile signature)  
- ✅ How to make HTTP requests (http.get signature)
- ✅ What HTTP response looks like (response_format)
- ✅ How to handle errors (common_errors)
- ✅ Lua string splitting (standard library, noted in constraints)

**Generated Code** (using only --agent info):
```lua
local fs = require('fs')
local http = require('http')
local path = require('path')

-- Read URLs from file
local urls_content = fs.readFile('urls.txt')
local urls = {}
for url in urls_content:gmatch('[^\n]+') do
  table.insert(urls, url)
end

print('Found ' .. #urls .. ' URLs to fetch')

-- Fetch each URL
for i, url in ipairs(urls) do
  print('Fetching ' .. url .. '...')
  
  local response = http.get(url)
  
  if response.ok then
    -- Generate filename from index
    local filename = 'response_' .. i .. '.txt'
    
    -- Save response
    local success = fs.writeFile(filename, response.body)
    
    if success then
      print('  ✓ Saved to ' .. filename)
    else
      print('  ✗ Failed to write ' .. filename)
    end
  else
    print('  ✗ HTTP error: ' .. response.status)
  end
end

print('Done!')
```

**Why this works**:
- LLM found all function signatures in --agent output
- LLM understood response structure from response_format
- LLM used string.gmatch from Lua standard library (mentioned in capabilities)
- LLM knew about success/error patterns from common_errors section
- **NO external documentation was needed**