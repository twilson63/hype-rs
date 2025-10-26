# Project Request Protocol (PRP): HTTP Module Lua Bindings

**Project ID**: PRP-006  
**Status**: 📋 Proposed  
**Priority**: High  
**Created**: 2025-10-26  
**Author**: Claude (AI Assistant)  
**Estimated Effort**: 3-4 days  

---

## Executive Summary

Implement Lua function bindings for the existing HTTP module, enabling Lua scripts to make HTTP requests through a clean, idiomatic API. The Rust HTTP client (571 lines) is fully implemented and tested, but requires Lua callable functions to bridge between Lua tables/values and Rust types.

**Current State**: HTTP module exists at Rust layer with complete implementation (HttpClient, HttpResponse, HttpError).  
**Desired State**: Lua scripts can call `require("http")` and use functions like `http.get(url)`, `http.post(url, options)`, etc.  
**Gap**: No Lua-to-Rust function bindings exist; `require()` returns JSON metadata instead of callable functions.

---

## Table of Contents

- [1. Project Overview](#1-project-overview)
- [2. Current State Analysis](#2-current-state-analysis)
- [3. Technical Requirements](#3-technical-requirements)
- [4. Proposed Solutions](#4-proposed-solutions)
  - [Solution 1: mlua UserData Pattern](#solution-1-mlua-userdata-pattern)
  - [Solution 2: Direct Function Registration](#solution-2-direct-function-registration)
  - [Solution 3: Hybrid JSON + Function Table](#solution-3-hybrid-json--function-table)
- [5. Solution Comparison](#5-solution-comparison)
- [6. Recommended Solution](#6-recommended-solution)
- [7. Implementation Plan](#7-implementation-plan)
- [8. Success Criteria](#8-success-criteria)
- [9. Risk Assessment](#9-risk-assessment)
- [10. Future Enhancements](#10-future-enhancements)

---

## 1. Project Overview

### 1.1 Background

The HTTP module for hype-rs has been fully implemented at the Rust layer:

```
src/modules/builtins/http/
├── mod.rs          - HttpModule (BuiltinModule trait)
├── client.rs       - HttpClient with reqwest + tokio (255 lines)
├── error.rs        - HttpError enum (83 lines)
└── response.rs     - HttpResponse struct (112 lines)
```

**Key Capabilities (Rust Layer)**:
- ✅ All HTTP methods (GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS)
- ✅ JSON request/response parsing
- ✅ Custom headers support
- ✅ Timeout control
- ✅ Connection pooling (10 idle/host)
- ✅ HTTP/2 and TLS/HTTPS support
- ✅ Comprehensive error handling

**Current Limitation**: The `BuiltinModule` trait's `exports()` method returns JSON metadata, not callable Lua functions. This means:

```lua
local http = require("http")
-- Returns: {__id = "http", get = {__fn = "get", __desc = "..."}, ...}
-- NOT: {get = function(url) ... end, post = function(...) ... end}

http.get("https://api.github.com/users/octocat")  -- ERROR: attempt to call table
```

### 1.2 Project Goals

1. **Primary Goal**: Create Lua callable functions that bridge to the Rust HttpClient
2. **API Design**: Provide idiomatic Lua API matching JavaScript fetch() patterns
3. **Type Safety**: Handle Lua ↔ Rust type conversions safely
4. **Error Handling**: Convert Rust errors to Lua errors with meaningful messages
5. **Performance**: Minimize overhead in the binding layer

### 1.3 Target API (Lua)

```lua
local http = require("http")

-- Simple GET
local response = http.get("https://api.github.com/users/octocat")
print(response.status)          -- 200
print(response.statusText)      -- "OK"
print(response.ok())            -- true

-- Parse JSON
local user = response.json()
print(user.name)                -- "The Octocat"

-- POST with options
local res = http.post("https://api.example.com/posts", {
    body = '{"title": "Hello"}',
    headers = {["Content-Type"] = "application/json"}
})

-- Universal fetch
local res = http.fetch("https://api.example.com/data", {
    method = "POST",
    body = "data",
    headers = {["Authorization"] = "Bearer token"},
    timeout = 5000
})

-- Convenience methods
local res = http.postJson("https://api.example.com/users", {
    name = "Alice",
    email = "alice@example.com"
})
```

---

## 2. Current State Analysis

### 2.1 Existing Architecture

**Module Loading Flow**:
```
Lua: require("http")
  ↓
src/lua/require.rs: setup_require_fn()
  ↓
src/modules/loader.rs: ModuleLoader.require()
  ↓
src/modules/builtins/mod.rs: BuiltinRegistry.load()
  ↓
src/modules/builtins/http/mod.rs: HttpModule.exports()
  ↓
Returns: JsonValue (metadata)
  ↓
src/lua/require.rs: json_to_lua()
  ↓
Lua receives: Table with metadata
```

**Problem**: `json_to_lua()` converts JSON to Lua tables, not functions.

### 2.2 What Works

✅ **Rust HTTP Client**: Fully functional, tested, production-ready  
✅ **Module Registration**: HTTP registered as builtin  
✅ **Feature Gating**: Compiles with/without `http` feature  
✅ **Error Types**: Comprehensive error handling  
✅ **Tests**: 231/235 passing (98.3%)  

### 2.3 What's Missing

❌ **Lua Function Bridge**: No way to call Rust functions from Lua  
❌ **Type Conversions**: No Lua table → Rust struct conversions  
❌ **Response Wrapping**: HttpResponse not exposed to Lua  
❌ **Error Conversion**: Rust errors not converted to Lua errors  

### 2.4 Technical Debt

- `require()` system only handles JSON exports, not functions
- BuiltinModule trait assumes static JSON metadata
- No pattern for stateful builtin modules (HttpClient needs tokio Runtime)

---

## 3. Technical Requirements

### 3.1 Functional Requirements

| ID | Requirement | Priority | Complexity |
|----|-------------|----------|------------|
| FR-1 | Lua functions for all HTTP methods (GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS) | Must Have | Medium |
| FR-2 | Support for custom headers as Lua table | Must Have | Medium |
| FR-3 | Support for request body (string) | Must Have | Low |
| FR-4 | Timeout configuration per request | Must Have | Low |
| FR-5 | Response object with status, headers, body | Must Have | Medium |
| FR-6 | `response.json()` method for parsing JSON | Must Have | Medium |
| FR-7 | `response.text()` method for text content | Must Have | Low |
| FR-8 | `response.ok()` method for 2xx status check | Must Have | Low |
| FR-9 | Universal `fetch(url, options)` API | Must Have | High |
| FR-10 | Convenience `postJson(url, data)` method | Should Have | Medium |
| FR-11 | Convenience `putJson(url, data)` method | Should Have | Medium |
| FR-12 | Error handling with meaningful messages | Must Have | Medium |

### 3.2 Non-Functional Requirements

| ID | Requirement | Target | Priority |
|----|-------------|--------|----------|
| NFR-1 | Function call overhead | < 1ms | High |
| NFR-2 | Memory efficiency | No leaks | Must Have |
| NFR-3 | Thread safety | Safe with tokio | Must Have |
| NFR-4 | API consistency | Match existing builtins | High |
| NFR-5 | Documentation | Complete API docs | High |
| NFR-6 | Test coverage | > 90% | High |
| NFR-7 | Backwards compatibility | No breaking changes | Medium |

### 3.3 Integration Requirements

- **mlua**: Use mlua 0.9 for Lua bindings
- **tokio**: Runtime must work with blocking Lua execution
- **serde**: JSON serialization for request/response bodies
- **Feature flags**: Maintain `http` feature gate
- **Module system**: Integrate with existing `require()` infrastructure

### 3.4 API Design Constraints

1. **Lua Idiomatic**: Use Lua conventions (1-indexed tables, nil for optional)
2. **Error Handling**: Use Lua's pcall/error system, not custom result types
3. **Immutability**: Response objects should be immutable
4. **No Global State**: Each `require("http")` should be isolated
5. **Memory Safe**: No dangling pointers or use-after-free

---

## 4. Proposed Solutions

### Solution 1: mlua UserData Pattern

**Approach**: Create mlua UserData types for HttpClient and HttpResponse, register methods on them.

#### Architecture

```rust
// src/modules/builtins/http/lua_bindings.rs

use mlua::{Lua, Table, UserData, UserDataMethods};

// HttpClient wrapper for Lua
struct LuaHttpClient {
    client: Arc<Mutex<HttpClient>>,
}

impl UserData for LuaHttpClient {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        // GET method
        methods.add_method("get", |lua, this, url: String| {
            let client = this.client.lock().unwrap();
            let response = client.get(&url)
                .map_err(|e| mlua::Error::external(e))?;
            Ok(LuaHttpResponse::new(response))
        });
        
        // POST method
        methods.add_method("post", |lua, this, (url, options): (String, Option<Table>)| {
            let client = this.client.lock().unwrap();
            let (body, headers) = parse_options(lua, options)?;
            let response = client.post(&url, body, headers)
                .map_err(|e| mlua::Error::external(e))?;
            Ok(LuaHttpResponse::new(response))
        });
        
        // ... more methods
    }
}

// HttpResponse wrapper for Lua
struct LuaHttpResponse {
    response: HttpResponse,
}

impl UserData for LuaHttpResponse {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("json", |lua, this, ()| {
            let json = this.response.json()
                .map_err(|e| mlua::Error::external(e))?;
            lua_json_to_table(lua, &json)
        });
        
        methods.add_method("text", |lua, this, ()| {
            Ok(this.response.text())
        });
        
        methods.add_method("ok", |lua, this, ()| {
            Ok(this.response.ok())
        });
    }
    
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("status", |lua, this| {
            Ok(this.response.status)
        });
        
        fields.add_field_method_get("statusText", |lua, this| {
            Ok(this.response.status_text.clone())
        });
        
        // ... more fields
    }
}

// Register with Lua
pub fn register_http_module(lua: &Lua) -> mlua::Result<Table> {
    let http = lua.create_table()?;
    
    let client = LuaHttpClient {
        client: Arc::new(Mutex::new(HttpClient::new()?)),
    };
    
    // Create closure that captures client
    let client_arc = Arc::new(client);
    
    let get_fn = lua.create_function({
        let client = client_arc.clone();
        move |lua, url: String| {
            client.get(lua, url)
        }
    })?;
    
    http.set("get", get_fn)?;
    
    Ok(http)
}
```

**Integration Point**:
```rust
// src/modules/builtins/mod.rs

#[cfg(feature = "http")]
pub fn load_http_module(lua: &Lua) -> Result<Table> {
    http::lua_bindings::register_http_module(lua)
}
```

#### Pros

✅ **Type Safety**: mlua handles type checking and conversions  
✅ **Memory Management**: mlua manages lifetimes and garbage collection  
✅ **Methods on Objects**: Natural `response.json()` syntax  
✅ **Error Handling**: Automatic Lua error conversion  
✅ **Documentation**: mlua provides introspection capabilities  
✅ **Idiomatic**: Follows mlua best practices  

#### Cons

❌ **Complexity**: Requires UserData boilerplate for each type  
❌ **State Management**: Need Arc<Mutex<>> for HttpClient  
❌ **Method Overhead**: UserData method calls have small overhead  
❌ **Integration Effort**: Need to modify BuiltinModule trait or bypass it  
❌ **Learning Curve**: Developers need to understand mlua UserData  

#### Scoring

| Criterion | Score | Weight | Total |
|-----------|-------|--------|-------|
| Implementation Complexity | 6/10 | 0.20 | 1.2 |
| Performance | 8/10 | 0.15 | 1.2 |
| Maintainability | 9/10 | 0.20 | 1.8 |
| API Quality | 10/10 | 0.25 | 2.5 |
| Integration Ease | 6/10 | 0.20 | 1.2 |
| **Total** | | | **7.9/10** |

---

### Solution 2: Direct Function Registration

**Approach**: Register free functions directly in the HTTP table, use closures to capture HttpClient.

#### Architecture

```rust
// src/modules/builtins/http/lua_bindings.rs

use mlua::{Lua, Table, Value};
use std::sync::Arc;

pub fn create_http_module(lua: &Lua) -> mlua::Result<Table> {
    let http_table = lua.create_table()?;
    
    // Create a shared HttpClient
    let client = Arc::new(
        HttpClient::new()
            .map_err(|e| mlua::Error::external(e))?
    );
    
    // GET function
    {
        let client = client.clone();
        let get_fn = lua.create_function(move |lua, url: String| {
            let response = client.get(&url)
                .map_err(|e| mlua::Error::external(e))?;
            create_response_table(lua, response)
        })?;
        http_table.set("get", get_fn)?;
    }
    
    // POST function
    {
        let client = client.clone();
        let post_fn = lua.create_function(move |lua, (url, options): (String, Option<Table>)| {
            let (body, headers) = parse_post_options(lua, options)?;
            let response = client.post(&url, body, headers)
                .map_err(|e| mlua::Error::external(e))?;
            create_response_table(lua, response)
        })?;
        http_table.set("post", post_fn)?;
    }
    
    // FETCH function (universal)
    {
        let client = client.clone();
        let fetch_fn = lua.create_function(move |lua, (url, options): (String, Option<Table>)| {
            let opts = parse_fetch_options(lua, options)?;
            let response = client.fetch(
                &opts.method,
                &url,
                opts.body,
                opts.headers,
                opts.timeout,
            ).map_err(|e| mlua::Error::external(e))?;
            create_response_table(lua, response)
        })?;
        http_table.set("fetch", fetch_fn)?;
    }
    
    Ok(http_table)
}

fn create_response_table<'lua>(lua: &'lua Lua, response: HttpResponse) -> mlua::Result<Table<'lua>> {
    let table = lua.create_table()?;
    
    // Properties
    table.set("status", response.status)?;
    table.set("statusText", response.status_text.clone())?;
    table.set("body", response.body.clone())?;
    
    // Headers
    let headers = lua.create_table()?;
    for (k, v) in &response.headers {
        headers.set(k.as_str(), v.as_str())?;
    }
    table.set("headers", headers)?;
    
    // Methods
    let body_clone = response.body.clone();
    table.set("text", lua.create_function(move |_, ()| {
        Ok(body_clone.clone())
    })?)?;
    
    let body_for_json = response.body.clone();
    table.set("json", lua.create_function(move |lua, ()| {
        let json: serde_json::Value = serde_json::from_str(&body_for_json)
            .map_err(|e| mlua::Error::external(e))?;
        json_to_lua_value(lua, &json)
    })?)?;
    
    let status = response.status;
    table.set("ok", lua.create_function(move |_, ()| {
        Ok(status >= 200 && status < 300)
    })?)?;
    
    Ok(table)
}

struct FetchOptions {
    method: String,
    body: Option<String>,
    headers: Option<HashMap<String, String>>,
    timeout: Option<u64>,
}

fn parse_fetch_options(lua: &Lua, options: Option<Table>) -> mlua::Result<FetchOptions> {
    let Some(opts) = options else {
        return Ok(FetchOptions {
            method: "GET".to_string(),
            body: None,
            headers: None,
            timeout: None,
        });
    };
    
    Ok(FetchOptions {
        method: opts.get::<_, Option<String>>("method")?.unwrap_or("GET".to_string()),
        body: opts.get::<_, Option<String>>("body")?,
        headers: parse_headers(lua, opts.get("headers")?)?,
        timeout: opts.get::<_, Option<u64>>("timeout")?,
    })
}
```

**Integration**:
```rust
// src/modules/builtins/mod.rs

impl BuiltinRegistry {
    pub fn load_for_lua(&mut self, lua: &Lua, name: &str) -> Result<Value> {
        match name {
            #[cfg(feature = "http")]
            "http" => {
                http::lua_bindings::create_http_module(lua)
                    .map(Value::Table)
                    .map_err(|e| HypeError::Execution(e.to_string()))
            }
            _ => {
                // Fall back to JSON-based loading
                self.load(name).and_then(|json| {
                    json_to_lua(lua, &json)
                        .map_err(|e| HypeError::Execution(e.to_string()))
                })
            }
        }
    }
}
```

#### Pros

✅ **Simplicity**: No UserData boilerplate, just functions  
✅ **Flexibility**: Easy to customize function signatures  
✅ **Performance**: Direct function calls, minimal overhead  
✅ **Table-Based Response**: Natural Lua idiom  
✅ **Straightforward**: Easier to understand and debug  
✅ **Less Code**: ~30% less code than UserData approach  

#### Cons

❌ **No Type Safety**: Manual type checking and conversions  
❌ **Memory Duplication**: Response data cloned into table  
❌ **Method Closures**: Each response method creates new closure  
❌ **No Introspection**: Can't inspect UserData properties  
❌ **Error Prone**: More manual error handling  

#### Scoring

| Criterion | Score | Weight | Total |
|-----------|-------|--------|-------|
| Implementation Complexity | 8/10 | 0.20 | 1.6 |
| Performance | 7/10 | 0.15 | 1.05 |
| Maintainability | 7/10 | 0.20 | 1.4 |
| API Quality | 8/10 | 0.25 | 2.0 |
| Integration Ease | 9/10 | 0.20 | 1.8 |
| **Total** | | | **7.85/10** |

---

### Solution 3: Hybrid JSON + Function Table

**Approach**: Keep JSON metadata from BuiltinModule, but add callable functions to the returned table.

#### Architecture

```rust
// src/modules/builtins/http/lua_bindings.rs

pub fn augment_http_exports(lua: &Lua, json_exports: JsonValue) -> mlua::Result<Table> {
    // Start with JSON metadata converted to table
    let http_table = json_to_lua_table(lua, &json_exports)?;
    
    // Create shared HttpClient
    let client = Arc::new(
        HttpClient::new()
            .map_err(|e| mlua::Error::external(e))?
    );
    
    // Add actual callable functions
    register_http_functions(lua, &http_table, client)?;
    
    Ok(http_table)
}

fn register_http_functions(
    lua: &Lua, 
    table: &Table, 
    client: Arc<HttpClient>
) -> mlua::Result<()> {
    // Replace __fn markers with actual functions
    
    // GET
    if let Ok(get_meta) = table.get::<_, Table>("get") {
        let client = client.clone();
        let get_fn = lua.create_function(move |lua, url: String| {
            let response = client.get(&url)
                .map_err(|e| mlua::Error::external(e))?;
            create_response_table(lua, response)
        })?;
        
        // Keep metadata
        get_meta.set("__call", get_fn.clone())?;
        
        // Also allow direct call: http.get(url)
        table.set("get", get_fn)?;
    }
    
    // Similar for other methods...
    
    Ok(())
}

// src/modules/builtins/mod.rs

impl BuiltinRegistry {
    pub fn load_with_functions(&mut self, lua: &Lua, name: &str) -> Result<Table> {
        // Get JSON exports
        let json_exports = self.load(name)?;
        
        // Augment with functions if special module
        match name {
            #[cfg(feature = "http")]
            "http" => {
                http::lua_bindings::augment_http_exports(lua, json_exports)
                    .map_err(|e| HypeError::Execution(e.to_string()))
            }
            _ => {
                // Regular JSON-based modules
                json_to_lua_table(lua, &json_exports)
                    .map_err(|e| HypeError::Execution(e.to_string()))
            }
        }
    }
}
```

#### Pros

✅ **Backwards Compatible**: Keeps existing BuiltinModule trait  
✅ **Metadata Preserved**: Documentation strings remain in table  
✅ **Gradual Migration**: Other modules can stay JSON-only  
✅ **Introspectable**: Metadata available for help systems  
✅ **Flexible**: Can mix static data with dynamic functions  

#### Cons

❌ **Complexity**: Two-stage loading process  
❌ **Confusion**: Mixed metaphor (JSON + functions)  
❌ **Overhead**: Extra table manipulations  
❌ **Maintenance**: Need to keep JSON and functions in sync  
❌ **Unclear Semantics**: What does `__fn` marker mean?  

#### Scoring

| Criterion | Score | Weight | Total |
|-----------|-------|--------|-------|
| Implementation Complexity | 5/10 | 0.20 | 1.0 |
| Performance | 7/10 | 0.15 | 1.05 |
| Maintainability | 6/10 | 0.20 | 1.2 |
| API Quality | 7/10 | 0.25 | 1.75 |
| Integration Ease | 8/10 | 0.20 | 1.6 |
| **Total** | | | **6.6/10** |

---

## 5. Solution Comparison

### 5.1 Summary Table

| Criterion | Solution 1 (UserData) | Solution 2 (Functions) | Solution 3 (Hybrid) |
|-----------|----------------------|----------------------|-------------------|
| **Overall Score** | 7.9/10 | 7.85/10 | 6.6/10 |
| Lines of Code | ~400 | ~280 | ~350 |
| Implementation Time | 3-4 days | 2-3 days | 3-4 days |
| Learning Curve | High | Medium | High |
| Type Safety | Excellent | Good | Good |
| Performance | Excellent | Very Good | Good |
| API Quality | Excellent | Very Good | Good |
| Maintainability | Excellent | Very Good | Fair |
| Extensibility | Excellent | Good | Fair |

### 5.2 Performance Comparison

**Benchmark Estimates** (per request):

| Operation | Solution 1 | Solution 2 | Solution 3 |
|-----------|-----------|-----------|-----------|
| `http.get()` call | 0.1ms | 0.08ms | 0.12ms |
| Response creation | 0.2ms | 0.15ms | 0.18ms |
| `response.json()` | 0.5ms | 0.6ms | 0.6ms |
| Memory per response | 2KB | 3KB | 3.5KB |

*Note: Network latency (10-500ms) dominates these differences*

### 5.3 Code Example Comparison

**Solution 1 (UserData)**:
```rust
impl UserData for LuaHttpClient {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("get", |lua, this, url: String| { ... });
    }
}
```

**Solution 2 (Functions)**:
```rust
let get_fn = lua.create_function(move |lua, url: String| {
    let response = client.get(&url)?;
    create_response_table(lua, response)
})?;
http_table.set("get", get_fn)?;
```

**Solution 3 (Hybrid)**:
```rust
let json_exports = module.exports()?;
let table = json_to_lua_table(lua, &json_exports)?;
register_http_functions(lua, &table, client)?;
```

### 5.4 Maintenance Comparison

**Adding a new HTTP method**:

- **Solution 1**: Add UserData method (5 lines) + impl (15 lines) = 20 lines
- **Solution 2**: Add closure + registration (12 lines) = 12 lines  
- **Solution 3**: Add JSON metadata + function + registration (25 lines) = 25 lines

---

## 6. Recommended Solution

### 6.1 Selected Solution: **Solution 2 - Direct Function Registration**

**Rationale**:

1. **Simplicity**: Most straightforward implementation with least boilerplate
2. **Performance**: Direct function calls with minimal overhead
3. **Integration**: Easier to integrate with existing module system
4. **Maintainability**: Less code to maintain, easier to understand
5. **Practical**: Gets us working Lua bindings fastest (2-3 days vs 3-4 days)
6. **Idiomatic Lua**: Table-based responses feel natural in Lua

**Why not Solution 1?**
- While UserData is more "proper" for OOP, the added complexity isn't justified
- HTTP requests are typically one-shot operations, not long-lived objects
- The 0.15-point score difference doesn't warrant the extra complexity
- We can always refactor to UserData later if needed

**Why not Solution 3?**
- Adds unnecessary complexity by trying to maintain backwards compatibility with a system that isn't being used yet
- Mixed metaphor creates confusion
- Lowest score (6.6/10) indicates it's not the best approach

### 6.2 Solution 2 Architecture Details

```
┌─────────────────────────────────────────────────────────────┐
│                    Lua Script Layer                          │
│  local http = require("http")                                │
│  local response = http.get("https://api.github.com")         │
│  print(response.status)                                      │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│            src/lua/require.rs (Modified)                     │
│  - Check if module needs function bindings                   │
│  - Call BuiltinRegistry.load_with_lua()                      │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│         src/modules/builtins/mod.rs (Modified)               │
│  - New method: load_with_lua(lua, name)                      │
│  - Routes http → create_http_module()                        │
│  - Routes others → json_to_lua()                             │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│    src/modules/builtins/http/lua_bindings.rs (New)          │
│                                                              │
│  pub fn create_http_module(lua: &Lua) -> Result<Table>      │
│    ├─ Create Arc<HttpClient>                                │
│    ├─ Register get_fn (closure)                             │
│    ├─ Register post_fn (closure)                            │
│    ├─ Register fetch_fn (closure)                           │
│    └─ Register convenience methods                          │
│                                                              │
│  fn create_response_table(lua, response) -> Result<Table>   │
│    ├─ Set properties: status, statusText, headers, body     │
│    ├─ Set method: json() → parse body as JSON               │
│    ├─ Set method: text() → return body                      │
│    └─ Set method: ok() → check 2xx status                   │
└─────────────────────────────────────────────────────────────┘
```

### 6.3 Key Components

**1. Lua Bindings Module** (`src/modules/builtins/http/lua_bindings.rs`)
- `create_http_module()` - Main entry point
- `create_response_table()` - Converts HttpResponse → Lua table
- `parse_fetch_options()` - Parses Lua options table
- `parse_headers()` - Converts Lua table → HashMap
- `json_to_lua_value()` - Converts serde_json::Value → mlua::Value

**2. Integration Points**
- Modify `src/modules/builtins/mod.rs` to add `load_with_lua()` method
- Modify `src/lua/require.rs` to call `load_with_lua()` for function-based modules
- Add HTTP module to list of function-based modules

**3. Error Handling**
- Convert Rust `HttpError` → mlua::Error::external()
- Provide detailed error messages with context
- Stack traces preserved through mlua

---

## 7. Implementation Plan

### Phase 1: Foundation (Day 1)

**Tasks**:
1. Create `src/modules/builtins/http/lua_bindings.rs`
2. Implement `create_http_module()` skeleton
3. Implement basic `http.get()` function
4. Implement `create_response_table()` with properties only (no methods)
5. Add unit test for basic GET request

**Deliverables**:
- ✅ Basic GET request working from Lua
- ✅ Response table with status, statusText, body
- ✅ 1 passing Rust test
- ✅ Compile without warnings

**Acceptance**:
```lua
local http = require("http")
local response = http.get("https://httpbin.org/get")
assert(response.status == 200)
assert(type(response.body) == "string")
```

### Phase 2: Response Methods (Day 2 - Morning)

**Tasks**:
1. Add `response.json()` method
2. Add `response.text()` method
3. Add `response.ok()` method
4. Add `response.headers` table
5. Implement `json_to_lua_value()` helper

**Deliverables**:
- ✅ Full response object API
- ✅ JSON parsing working
- ✅ Headers accessible
- ✅ 3 passing Rust tests

**Acceptance**:
```lua
local http = require("http")
local response = http.get("https://api.github.com/users/octocat")
local user = response.json()
assert(user.login == "octocat")
assert(response.ok() == true)
```

### Phase 3: HTTP Methods (Day 2 - Afternoon)

**Tasks**:
1. Implement `http.post()` with options
2. Implement `http.put()`
3. Implement `http.delete()`
4. Implement `parse_post_options()` helper
5. Add tests for each method

**Deliverables**:
- ✅ POST, PUT, DELETE methods working
- ✅ Request body support
- ✅ Custom headers support
- ✅ 6 passing Rust tests

**Acceptance**:
```lua
local http = require("http")
local response = http.post("https://httpbin.org/post", {
    body = '{"test": true}',
    headers = {["Content-Type"] = "application/json"}
})
assert(response.status == 200)
```

### Phase 4: Universal Fetch (Day 3 - Morning)

**Tasks**:
1. Implement `http.fetch()` with full options
2. Implement `parse_fetch_options()` helper
3. Add timeout support
4. Add method support (PATCH, HEAD, OPTIONS)
5. Add tests for all options

**Deliverables**:
- ✅ Universal fetch API working
- ✅ All HTTP methods supported
- ✅ Timeout configuration
- ✅ 4 passing Rust tests

**Acceptance**:
```lua
local http = require("http")
local response = http.fetch("https://httpbin.org/post", {
    method = "POST",
    body = "data",
    headers = {["X-Custom"] = "value"},
    timeout = 5000
})
assert(response.ok())
```

### Phase 5: Convenience Methods (Day 3 - Afternoon)

**Tasks**:
1. Implement `http.postJson()`
2. Implement `http.putJson()`
3. Add Lua table → JSON serialization
4. Add tests for JSON methods

**Deliverables**:
- ✅ Convenience methods working
- ✅ Automatic JSON serialization
- ✅ 2 passing Rust tests

**Acceptance**:
```lua
local http = require("http")
local response = http.postJson("https://httpbin.org/post", {
    name = "Alice",
    age = 30
})
local result = response.json()
assert(result.json.name == "Alice")
```

### Phase 6: Error Handling (Day 4 - Morning)

**Tasks**:
1. Improve error messages with context
2. Add error type information
3. Test error scenarios (timeout, invalid URL, network error)
4. Document error handling patterns

**Deliverables**:
- ✅ Detailed error messages
- ✅ Error type preservation
- ✅ 5 error scenario tests
- ✅ Error handling docs

**Acceptance**:
```lua
local http = require("http")
local ok, err = pcall(function()
    return http.get("https://invalid-domain-xyz.com")
end)
assert(not ok)
assert(type(err) == "string")
assert(err:match("error sending request"))
```

### Phase 7: Integration & Testing (Day 4 - Afternoon)

**Tasks**:
1. Integrate with `src/lua/require.rs`
2. Update `src/modules/builtins/mod.rs`
3. Create comprehensive Lua test file
4. Run full test suite
5. Fix any integration issues

**Deliverables**:
- ✅ Full integration complete
- ✅ `require("http")` working
- ✅ All tests passing
- ✅ No regressions

**Acceptance**:
```bash
cargo test --features http --lib
# All HTTP tests passing
# No regressions in other tests
```

### Phase 8: Documentation & Examples (Day 5)

**Tasks**:
1. Update `docs/modules/http-api.md` with Lua examples
2. Create `examples/http-lua-demo.lua`
3. Update `docs/modules/builtin-modules.md`
4. Add inline code documentation
5. Create troubleshooting guide

**Deliverables**:
- ✅ Complete API documentation
- ✅ Working Lua examples
- ✅ Troubleshooting guide
- ✅ Inline docs for all functions

---

## 8. Success Criteria

### 8.1 Functional Success Criteria

| ID | Criterion | Verification |
|----|-----------|--------------|
| SC-1 | All HTTP methods (GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS) work from Lua | Manual test + automated test |
| SC-2 | Request options (body, headers, timeout) work correctly | Automated tests |
| SC-3 | Response object provides all properties and methods | Automated tests |
| SC-4 | JSON parsing works for valid JSON responses | Automated tests |
| SC-5 | Error handling provides meaningful messages | Error scenario tests |
| SC-6 | `require("http")` loads successfully | Integration test |
| SC-7 | Convenience methods (postJson, putJson) work | Automated tests |
| SC-8 | Timeout configuration works as expected | Timeout test with httpbin.org/delay |

### 8.2 Non-Functional Success Criteria

| ID | Criterion | Target | Verification |
|----|-----------|--------|--------------|
| NSC-1 | Test coverage | > 90% | `cargo tarpaulin` |
| NSC-2 | No memory leaks | 0 leaks | `valgrind` or sanitizers |
| NSC-3 | Function call overhead | < 1ms | Benchmark |
| NSC-4 | Documentation completeness | 100% public APIs | Manual review |
| NSC-5 | Build time increase | < 10 seconds | `cargo build --features http` |
| NSC-6 | Binary size increase | < 2MB | Compare release builds |
| NSC-7 | No test regressions | 0 new failures | `cargo test` |

### 8.3 API Quality Criteria

**Must demonstrate**:
1. ✅ Idiomatic Lua API (tables, nil for optional, pcall for errors)
2. ✅ Consistent naming (camelCase for methods, match JS fetch API)
3. ✅ Intuitive usage (minimal boilerplate)
4. ✅ Good error messages (actionable, with context)
5. ✅ Complete documentation (examples for all features)

### 8.4 Integration Criteria

1. ✅ Works with existing module system (`require()`)
2. ✅ Compatible with feature flags (builds with/without `http`)
3. ✅ No breaking changes to other modules
4. ✅ Thread-safe with tokio runtime
5. ✅ Memory-safe (no unsafe code in bindings layer)

### 8.5 Acceptance Test

**Final integration test** (`tests/lua_scripts/test_http_complete.lua`):

```lua
print("HTTP Module Complete Integration Test")

local http = require("http")

-- Test 1: Simple GET
print("Test 1: GET request")
local response = http.get("https://httpbin.org/get")
assert(response.status == 200, "GET failed")
assert(response.ok(), "Response not OK")
print("✓ GET working")

-- Test 2: POST with JSON
print("Test 2: POST with JSON")
local res = http.postJson("https://httpbin.org/post", {
    message = "Hello from Hype-RS"
})
assert(res.status == 200, "POST failed")
local data = res.json()
assert(data.json.message == "Hello from Hype-RS", "JSON not echoed")
print("✓ POST JSON working")

-- Test 3: Custom headers
print("Test 3: Custom headers")
local res = http.get("https://httpbin.org/headers")
assert(res.status == 200, "Headers request failed")
print("✓ Headers working")

-- Test 4: Fetch with options
print("Test 4: Universal fetch")
local res = http.fetch("https://httpbin.org/post", {
    method = "POST",
    body = "test data",
    headers = {["Content-Type"] = "text/plain"},
    timeout = 10000
})
assert(res.status == 200, "Fetch failed")
print("✓ Fetch working")

-- Test 5: Error handling
print("Test 5: Error handling")
local ok, err = pcall(function()
    return http.get("https://invalid-domain-xyz.com")
end)
assert(not ok, "Should have failed")
assert(type(err) == "string", "Error should be string")
print("✓ Error handling working")

-- Test 6: Response methods
print("Test 6: Response methods")
local res = http.get("https://httpbin.org/json")
assert(res.ok(), "Should be OK")
assert(type(res.text()) == "string", "text() should return string")
local json = res.json()
assert(type(json) == "table", "json() should return table")
print("✓ Response methods working")

print("\n✅ All integration tests passed!")
```

**Pass criteria**: All assertions pass, no errors

---

## 9. Risk Assessment

### 9.1 Technical Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|------------|
| **mlua version incompatibility** | Low | High | Pin mlua version, test thoroughly |
| **Memory leaks in closures** | Medium | High | Use Arc properly, add leak tests |
| **Tokio runtime conflicts** | Medium | High | Ensure single runtime, document threading |
| **Type conversion errors** | Medium | Medium | Extensive type checking, error handling |
| **Performance bottlenecks** | Low | Medium | Benchmark early, optimize if needed |
| **Error message quality** | Medium | Low | User testing, iterate on messages |

### 9.2 Integration Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|------------|
| **Breaking existing module system** | Low | High | Extensive testing, feature flags |
| **Lua version incompatibility** | Low | Medium | Test with Lua 5.4 (vendored) |
| **Build complexity** | Low | Low | Clear documentation, CI tests |
| **Feature flag issues** | Medium | Medium | Test both with/without features |

### 9.3 Timeline Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|------------|
| **Underestimated complexity** | Medium | Medium | Build incrementally, test early |
| **Debug time for tokio issues** | Medium | High | Start with simple sync tests first |
| **API design iterations** | Medium | Medium | Reference existing HTTP libraries |
| **Documentation time** | Low | Low | Write docs alongside code |

### 9.4 Mitigation Strategies

1. **Incremental Development**: Build one feature at a time, test thoroughly
2. **Early Integration**: Integrate with require() system in Phase 1
3. **Comprehensive Testing**: Write tests for each feature immediately
4. **Reference Implementation**: Study ureq-lua, mlua examples
5. **Fallback Plan**: If major issues arise, can use simpler blocking API
6. **Code Review**: Have another developer review design before Phase 3

---

## 10. Future Enhancements

### 10.1 Short-Term (Next Sprint)

1. **Streaming Responses**: Support for large file downloads
   ```lua
   http.getStream("https://example.com/largefile.zip", function(chunk)
       file:write(chunk)
   end)
   ```

2. **Request Interceptors**: Middleware for logging, auth
   ```lua
   http.interceptors.request.use(function(config)
       config.headers["X-API-Key"] = os.getenv("API_KEY")
       return config
   end)
   ```

3. **Response Interceptors**: Automatic retry, caching
   ```lua
   http.interceptors.response.use(function(response)
       if response.status == 429 then
           -- Retry after delay
       end
       return response
   end)
   ```

### 10.2 Medium-Term (Next Quarter)

4. **Multipart Form Data**: File uploads
   ```lua
   http.postMultipart("https://api.example.com/upload", {
       file = {
           filename = "photo.jpg",
           content = file_content,
           content_type = "image/jpeg"
       }
   })
   ```

5. **Cookie Management**: Persistent cookies
   ```lua
   local jar = http.createCookieJar()
   jar:set("session", "abc123")
   http.get("https://example.com", {cookies = jar})
   ```

6. **WebSocket Support**: Real-time communication
   ```lua
   local ws = http.websocket("wss://example.com/socket")
   ws:on("message", function(data) print(data) end)
   ws:send("Hello")
   ```

### 10.3 Long-Term (Future Releases)

7. **HTTP/3 Support**: QUIC protocol
8. **Connection Pooling Control**: Custom pool settings
9. **Proxy Configuration**: SOCKS, HTTP proxy support
10. **Certificate Pinning**: Enhanced security
11. **Progress Callbacks**: Upload/download progress
12. **Rate Limiting**: Built-in rate limiter
13. **Retry with Exponential Backoff**: Automatic retry logic
14. **Request Cancellation**: Abort in-flight requests
15. **Response Caching**: HTTP cache implementation

### 10.4 API Stability

**Versioning Strategy**:
- Current implementation: `v1.0` (semantic versioning)
- Breaking changes require major version bump
- New features: minor version bump
- Bug fixes: patch version bump

**Deprecation Policy**:
- 6 month notice for breaking changes
- Deprecation warnings in code
- Migration guide provided

---

## Appendices

### Appendix A: Code Structure

```
src/modules/builtins/http/
├── mod.rs                  - HttpModule, exports()
├── client.rs               - HttpClient (Rust implementation)
├── error.rs                - HttpError enum
├── response.rs             - HttpResponse struct
└── lua_bindings.rs         - NEW: Lua function bindings
    ├── create_http_module()
    ├── create_response_table()
    ├── parse_fetch_options()
    ├── parse_headers()
    └── json_to_lua_value()

src/modules/builtins/
└── mod.rs                  - MODIFIED: Add load_with_lua()

src/lua/
└── require.rs              - MODIFIED: Call load_with_lua()

tests/
├── http_lua_bindings_test.rs  - NEW: Rust tests
└── lua_scripts/
    └── test_http_bindings.lua  - NEW: Lua integration tests

docs/modules/
└── http-api.md             - UPDATED: Add Lua examples

examples/
└── http-lua-demo.lua       - NEW: Complete demo
```

### Appendix B: Dependencies

**New Dependencies**: None (use existing mlua 0.9)

**Version Requirements**:
- mlua = "0.9" (already present)
- tokio = "1.0" (already present, feature-gated)
- reqwest = "0.12" (already present, feature-gated)
- serde_json = "1.0" (already present)

### Appendix C: Reference Implementations

1. **mlua Examples**: https://github.com/khvzak/mlua/tree/main/examples
2. **ureq-lua**: Similar blocking HTTP library for Lua
3. **lua-http**: Popular Lua HTTP library (reference for API design)
4. **reqwest examples**: Rust reqwest crate patterns

### Appendix D: Testing Strategy

**Unit Tests** (Rust):
- Test each Lua function independently
- Mock HTTP responses where possible
- Test type conversions (Lua ↔ Rust)
- Test error handling paths

**Integration Tests** (Lua):
- Test complete workflows
- Use httpbin.org for live tests
- Test error scenarios
- Test with real JSON APIs

**Performance Tests**:
- Benchmark function call overhead
- Benchmark JSON parsing
- Benchmark response creation
- Compare with native Rust calls

### Appendix E: Documentation Checklist

- [ ] API reference updated (docs/modules/http-api.md)
- [ ] Builtin modules doc updated (docs/modules/builtin-modules.md)
- [ ] Inline code comments for all public functions
- [ ] Examples for each HTTP method
- [ ] Error handling examples
- [ ] Troubleshooting guide
- [ ] Migration guide (if needed)
- [ ] Changelog entry

---

## Approval & Sign-off

**Prepared by**: Claude (AI Assistant)  
**Date**: 2025-10-26  
**Status**: ✅ Ready for Review  

**Review Checklist**:
- [ ] Technical approach validated
- [ ] Resource requirements approved
- [ ] Timeline acceptable
- [ ] Risks assessed and mitigated
- [ ] Success criteria clear and measurable
- [ ] Implementation plan detailed and actionable

**Approvals**:
- [ ] Technical Lead: _________________ Date: _______
- [ ] Product Owner: _________________ Date: _______
- [ ] Engineering Manager: ___________ Date: _______

---

**Document Version**: 1.0  
**Last Updated**: 2025-10-26  
**Next Review**: Before implementation begins
