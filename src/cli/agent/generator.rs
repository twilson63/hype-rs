use super::structures::*;
use std::collections::HashMap;

pub fn generate_agent_docs() -> Result<String, Box<dyn std::error::Error>> {
    let doc = AgentDocumentation {
        schema_version: "1.0.0".to_string(),
        tool: ToolInfo {
            name: "hype".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: "Fast, secure Lua runtime for scripts, modules, and CLI tools".to_string(),
            repository: "https://github.com/oeo/hype-rs".to_string(),
        },
        capabilities: Capabilities {
            commands: vec![
                Command {
                    name: "run".to_string(),
                    description: "Execute Lua script".to_string(),
                    usage: "hype <script.lua> [args...]".to_string(),
                    examples: vec![
                        "hype app.lua".to_string(),
                        "hype app.lua --flag value arg1 arg2".to_string(),
                    ],
                },
                Command {
                    name: "install".to_string(),
                    description: "Install package globally".to_string(),
                    usage: "hype install [path]".to_string(),
                    examples: vec![
                        "hype install".to_string(),
                        "hype install /path/to/package".to_string(),
                    ],
                },
            ],
            modules: vec![
                create_fs_module(),
                create_http_module(),
                create_path_module(),
                create_events_module(),
                create_util_module(),
                create_table_module(),
            ],
            features: vec![
                "Built-in module system with require()".to_string(),
                "Package management with hype.json manifests".to_string(),
                "Global package installation via ~/.hype/bin".to_string(),
                "Security sandboxing with restricted operations".to_string(),
                "Command-line argument parsing via _args".to_string(),
            ],
        },
        constraints: Constraints {
            security: SecurityConstraints {
                sandboxing: true,
                restricted_operations: vec![
                    "No raw OS command execution".to_string(),
                    "Limited filesystem access outside working directory".to_string(),
                    "No direct memory manipulation".to_string(),
                ],
                memory_limits: "Configurable via Lua state (default: reasonable limits)".to_string(),
                instruction_limits: "Configurable timeout support".to_string(),
            },
            limitations: vec![
                "HTTP module requires 'http' feature flag (enabled by default)".to_string(),
                "Synchronous operations only (no async/await in Lua)".to_string(),
                "Module resolution follows Node.js-style algorithm".to_string(),
            ],
        },
        best_practices: vec![
            "Use require() for loading modules instead of dofile() or loadfile()".to_string(),
            "Access CLI arguments via _args table, not arg or ...".to_string(),
            "Use path module for cross-platform path operations".to_string(),
            "Handle errors with pcall() for robustness".to_string(),
            "Use http.fetch() for full control over HTTP requests".to_string(),
        ],
        common_errors: vec![
            CommonError {
                pattern: "module 'X' not found".to_string(),
                cause: "Module doesn't exist in node_modules, current directory, or builtin modules".to_string(),
                solution: "Check module path, ensure hype.json dependencies are correct, or use correct builtin name".to_string(),
            },
            CommonError {
                pattern: "_args is nil".to_string(),
                cause: "Accessing _args when no arguments were passed".to_string(),
                solution: "Check _args ~= nil before accessing, or use default values".to_string(),
            },
            CommonError {
                pattern: "HTTP feature not enabled".to_string(),
                cause: "Binary compiled without --features http".to_string(),
                solution: "Use official release binaries or rebuild with cargo build --features http".to_string(),
            },
            CommonError {
                pattern: "Invalid URL".to_string(),
                cause: "URL is malformed or missing required components (e.g., protocol)".to_string(),
                solution: "Ensure URL starts with http:// or https:// and is properly formatted. Example: http.get(\"https://example.com/path\")".to_string(),
            },
        ],
        examples: create_examples(),
    };

    let json = serde_json::to_string_pretty(&doc)?;
    Ok(json)
}

fn create_fs_module() -> Module {
    let mut api = HashMap::new();

    api.insert(
        "readFileSync".to_string(),
        FunctionDoc {
            signature: "fs.readFileSync(path: string): string".to_string(),
            description: "Read file contents synchronously as string".to_string(),
            params: Some(HashMap::from([(
                "path".to_string(),
                "string - File path to read".to_string(),
            )])),
            returns: "string - File contents".to_string(),
            errors: Some(vec![
                "File not found".to_string(),
                "Permission denied".to_string(),
                "Invalid UTF-8 encoding".to_string(),
            ]),
            example: r#"local content = fs.readFileSync("config.json")"#.to_string(),
        },
    );

    api.insert(
        "writeFileSync".to_string(),
        FunctionDoc {
            signature: "fs.writeFileSync(path: string, data: string): nil".to_string(),
            description: "Write string data to file synchronously".to_string(),
            params: Some(HashMap::from([
                (
                    "path".to_string(),
                    "string - File path to write".to_string(),
                ),
                ("data".to_string(), "string - Data to write".to_string()),
            ])),
            returns: "nil".to_string(),
            errors: Some(vec![
                "Permission denied".to_string(),
                "Directory doesn't exist".to_string(),
            ]),
            example: r#"fs.writeFileSync("output.txt", "Hello World")"#.to_string(),
        },
    );

    api.insert(
        "existsSync".to_string(),
        FunctionDoc {
            signature: "fs.existsSync(path: string): boolean".to_string(),
            description: "Check if file or directory exists".to_string(),
            params: Some(HashMap::from([(
                "path".to_string(),
                "string - Path to check".to_string(),
            )])),
            returns: "boolean - true if exists, false otherwise".to_string(),
            errors: None,
            example: r#"if fs.existsSync("data.txt") then print("Found") end"#.to_string(),
        },
    );

    api.insert(
        "statSync".to_string(),
        FunctionDoc {
            signature:
                "fs.statSync(path: string): {isFile: boolean, isDirectory: boolean, size: number}"
                    .to_string(),
            description: "Get file/directory statistics".to_string(),
            params: Some(HashMap::from([(
                "path".to_string(),
                "string - Path to stat".to_string(),
            )])),
            returns: "table - {isFile: boolean, isDirectory: boolean, size: number}".to_string(),
            errors: Some(vec!["File not found".to_string()]),
            example: r#"local stat = fs.statSync("file.txt")
if stat.isFile then print("Size: " .. stat.size) end"#
                .to_string(),
        },
    );

    api.insert(
        "readdirSync".to_string(),
        FunctionDoc {
            signature: "fs.readdirSync(path: string): table".to_string(),
            description: "Read directory contents, returns array of filenames".to_string(),
            params: Some(HashMap::from([(
                "path".to_string(),
                "string - Directory path".to_string(),
            )])),
            returns: "table - Array of string filenames".to_string(),
            errors: Some(vec![
                "Directory not found".to_string(),
                "Not a directory".to_string(),
            ]),
            example: r#"local files = fs.readdirSync(".")
for i, name in ipairs(files) do print(name) end"#
                .to_string(),
        },
    );

    api.insert(
        "unlinkSync".to_string(),
        FunctionDoc {
            signature: "fs.unlinkSync(path: string): nil".to_string(),
            description: "Delete file".to_string(),
            params: Some(HashMap::from([(
                "path".to_string(),
                "string - File path to delete".to_string(),
            )])),
            returns: "nil".to_string(),
            errors: Some(vec![
                "File not found".to_string(),
                "Permission denied".to_string(),
            ]),
            example: r#"fs.unlinkSync("temp.txt")"#.to_string(),
        },
    );

    api.insert(
        "mkdirSync".to_string(),
        FunctionDoc {
            signature: "fs.mkdirSync(path: string): nil".to_string(),
            description: "Create directory".to_string(),
            params: Some(HashMap::from([(
                "path".to_string(),
                "string - Directory path to create".to_string(),
            )])),
            returns: "nil".to_string(),
            errors: Some(vec![
                "Directory already exists".to_string(),
                "Permission denied".to_string(),
            ]),
            example: r#"fs.mkdirSync("output")"#.to_string(),
        },
    );

    api.insert(
        "rmdirSync".to_string(),
        FunctionDoc {
            signature: "fs.rmdirSync(path: string): nil".to_string(),
            description: "Remove empty directory".to_string(),
            params: Some(HashMap::from([(
                "path".to_string(),
                "string - Directory path to remove".to_string(),
            )])),
            returns: "nil".to_string(),
            errors: Some(vec![
                "Directory not found".to_string(),
                "Directory not empty".to_string(),
                "Permission denied".to_string(),
            ]),
            example: r#"fs.rmdirSync("temp")"#.to_string(),
        },
    );

    Module {
        name: "fs".to_string(),
        description: "File system operations for reading, writing, and managing files".to_string(),
        api,
        response_format: None,
        usage_example: r#"local fs = require("fs")
local content = fs.readFileSync("input.txt")
fs.writeFileSync("output.txt", content:upper())"#
            .to_string(),
    }
}

fn create_http_module() -> Module {
    let mut api = HashMap::new();

    api.insert(
        "get".to_string(),
        FunctionDoc {
            signature: "http.get(url: string): Response".to_string(),
            description: "Perform HTTP GET request".to_string(),
            params: Some(HashMap::from([
                ("url".to_string(), "string - URL to fetch".to_string()),
            ])),
            returns: "Response - {status: number, statusText: string, body: string, headers: table, text: function, json: function, ok: function}".to_string(),
            errors: Some(vec![
                "Network error".to_string(),
                "Invalid URL".to_string(),
                "Timeout".to_string(),
            ]),
            example: r#"local resp = http.get("https://api.example.com/data")
if resp.ok() then print(resp.body) end"#.to_string(),
        },
    );

    api.insert(
        "post".to_string(),
        FunctionDoc {
            signature:
                "http.post(url: string, options?: {body?: string, headers?: table}): Response"
                    .to_string(),
            description: "Perform HTTP POST request".to_string(),
            params: Some(HashMap::from([
                ("url".to_string(), "string - URL to post to".to_string()),
                (
                    "options".to_string(),
                    "table (optional) - {body: string, headers: table}".to_string(),
                ),
            ])),
            returns: "Response".to_string(),
            errors: Some(vec!["Network error".to_string(), "Invalid URL".to_string()]),
            example: r#"local resp = http.post("https://api.example.com/data", {
  body = "name=value",
  headers = {["Content-Type"] = "application/x-www-form-urlencoded"}
})"#
            .to_string(),
        },
    );

    api.insert(
        "postJson".to_string(),
        FunctionDoc {
            signature: "http.postJson(url: string, data: table): Response".to_string(),
            description: "POST JSON data with automatic Content-Type header".to_string(),
            params: Some(HashMap::from([
                ("url".to_string(), "string - URL to post to".to_string()),
                (
                    "data".to_string(),
                    "table - Lua table to serialize as JSON".to_string(),
                ),
            ])),
            returns: "Response".to_string(),
            errors: Some(vec![
                "Network error".to_string(),
                "JSON serialization error".to_string(),
            ]),
            example: r#"local resp = http.postJson("https://api.example.com/users", {
  name = "Alice",
  age = 30
})"#
            .to_string(),
        },
    );

    api.insert(
        "put".to_string(),
        FunctionDoc {
            signature:
                "http.put(url: string, options?: {body?: string, headers?: table}): Response"
                    .to_string(),
            description: "Perform HTTP PUT request".to_string(),
            params: Some(HashMap::from([
                ("url".to_string(), "string - URL".to_string()),
                (
                    "options".to_string(),
                    "table (optional) - {body: string, headers: table}".to_string(),
                ),
            ])),
            returns: "Response".to_string(),
            errors: Some(vec!["Network error".to_string()]),
            example: r#"local resp = http.put("https://api.example.com/users/1", {body = "data"})"#
                .to_string(),
        },
    );

    api.insert(
        "putJson".to_string(),
        FunctionDoc {
            signature: "http.putJson(url: string, data: table): Response".to_string(),
            description: "PUT JSON data with automatic Content-Type header".to_string(),
            params: Some(HashMap::from([
                ("url".to_string(), "string - URL".to_string()),
                (
                    "data".to_string(),
                    "table - Lua table to serialize as JSON".to_string(),
                ),
            ])),
            returns: "Response".to_string(),
            errors: Some(vec!["Network error".to_string()]),
            example:
                r#"local resp = http.putJson("https://api.example.com/users/1", {name = "Bob"})"#
                    .to_string(),
        },
    );

    api.insert(
        "delete".to_string(),
        FunctionDoc {
            signature: "http.delete(url: string, options?: {headers?: table}): Response"
                .to_string(),
            description: "Perform HTTP DELETE request".to_string(),
            params: Some(HashMap::from([
                ("url".to_string(), "string - URL".to_string()),
                (
                    "options".to_string(),
                    "table (optional) - {headers: table}".to_string(),
                ),
            ])),
            returns: "Response".to_string(),
            errors: Some(vec!["Network error".to_string()]),
            example: r#"local resp = http.delete("https://api.example.com/users/1")"#.to_string(),
        },
    );

    api.insert(
        "patch".to_string(),
        FunctionDoc {
            signature:
                "http.patch(url: string, options?: {body?: string, headers?: table}): Response"
                    .to_string(),
            description: "Perform HTTP PATCH request".to_string(),
            params: Some(HashMap::from([
                ("url".to_string(), "string - URL".to_string()),
                (
                    "options".to_string(),
                    "table (optional) - {body: string, headers: table}".to_string(),
                ),
            ])),
            returns: "Response".to_string(),
            errors: Some(vec!["Network error".to_string()]),
            example:
                r#"local resp = http.patch("https://api.example.com/users/1", {body = "update"})"#
                    .to_string(),
        },
    );

    api.insert(
        "head".to_string(),
        FunctionDoc {
            signature: "http.head(url: string, options?: {headers?: table}): Response".to_string(),
            description: "Perform HTTP HEAD request (no response body)".to_string(),
            params: Some(HashMap::from([
                ("url".to_string(), "string - URL".to_string()),
                (
                    "options".to_string(),
                    "table (optional) - {headers: table}".to_string(),
                ),
            ])),
            returns: "Response".to_string(),
            errors: Some(vec!["Network error".to_string()]),
            example: r#"local resp = http.head("https://api.example.com/file")
print("Content-Length:", resp.headers["content-length"])"#
                .to_string(),
        },
    );

    api.insert(
        "fetch".to_string(),
        FunctionDoc {
            signature: "http.fetch(url: string, options?: {method?: string, body?: string, headers?: table, timeout?: number}): Response".to_string(),
            description: "Full-featured HTTP request with all options".to_string(),
            params: Some(HashMap::from([
                ("url".to_string(), "string - URL".to_string()),
                ("options".to_string(), "table (optional) - {method: string (default GET), body: string, headers: table, timeout: number (milliseconds)}".to_string()),
            ])),
            returns: "Response".to_string(),
            errors: Some(vec!["Network error".to_string(), "Timeout".to_string()]),
            example: r#"local resp = http.fetch("https://api.example.com", {
  method = "POST",
  body = '{"key":"value"}',
  headers = {["Content-Type"] = "application/json"},
  timeout = 5000
})"#.to_string(),
        },
    );

    let mut response_format = HashMap::new();
    response_format.insert(
        "status".to_string(),
        "number - HTTP status code".to_string(),
    );
    response_format.insert(
        "statusText".to_string(),
        "string - HTTP status text".to_string(),
    );
    response_format.insert("body".to_string(), "string - Response body".to_string());
    response_format.insert(
        "headers".to_string(),
        "table - Response headers (lowercase keys)".to_string(),
    );
    response_format.insert(
        "text()".to_string(),
        "function - Returns body as string".to_string(),
    );
    response_format.insert(
        "json()".to_string(),
        "function - Parses body as JSON and returns Lua table".to_string(),
    );
    response_format.insert(
        "ok()".to_string(),
        "function - Returns true if status 200-299".to_string(),
    );

    Module {
        name: "http".to_string(),
        description: "HTTP client for making web requests (requires http feature). URLs are automatically validated and encoded according to RFC 3986. Unreserved characters (a-z, A-Z, 0-9, -, ., _, ~) are preserved. Special characters in paths are handled correctly. Invalid URLs produce clear error messages.".to_string(),
        api,
        response_format: Some(response_format),
        usage_example: r#"local http = require("http")
local resp = http.get("https://api.github.com/users/octocat")
if resp.ok() then
  local data = resp.json()
  print("Name:", data.name)
end

-- Tilde character works correctly
local resp2 = http.get("https://example.com/~username/profile")

-- Already-encoded URLs are preserved
local resp3 = http.get("https://example.com/path%20with%20spaces")"#
            .to_string(),
    }
}

fn create_path_module() -> Module {
    let mut api = HashMap::new();

    api.insert(
        "join".to_string(),
        FunctionDoc {
            signature: "path.join(...: string): string".to_string(),
            description: "Join path segments using platform separator".to_string(),
            params: Some(HashMap::from([(
                "...".to_string(),
                "varargs string - Path segments to join".to_string(),
            )])),
            returns: "string - Joined path".to_string(),
            errors: None,
            example:
                r#"local p = path.join("src", "modules", "main.lua")  -- "src/modules/main.lua""#
                    .to_string(),
        },
    );

    api.insert(
        "dirname".to_string(),
        FunctionDoc {
            signature: "path.dirname(p: string): string".to_string(),
            description: "Get directory name from path".to_string(),
            params: Some(HashMap::from([(
                "p".to_string(),
                "string - File path".to_string(),
            )])),
            returns: "string - Directory portion".to_string(),
            errors: None,
            example: r#"path.dirname("/home/user/file.txt")  -- "/home/user""#.to_string(),
        },
    );

    api.insert(
        "basename".to_string(),
        FunctionDoc {
            signature: "path.basename(p: string): string".to_string(),
            description: "Get filename from path".to_string(),
            params: Some(HashMap::from([(
                "p".to_string(),
                "string - File path".to_string(),
            )])),
            returns: "string - Filename portion".to_string(),
            errors: None,
            example: r#"path.basename("/home/user/file.txt")  -- "file.txt""#.to_string(),
        },
    );

    api.insert(
        "extname".to_string(),
        FunctionDoc {
            signature: "path.extname(p: string): string".to_string(),
            description: "Get file extension including dot".to_string(),
            params: Some(HashMap::from([(
                "p".to_string(),
                "string - File path".to_string(),
            )])),
            returns: "string - Extension with dot (e.g. '.lua'), empty if no extension".to_string(),
            errors: None,
            example: r#"path.extname("script.lua")  -- ".lua""#.to_string(),
        },
    );

    api.insert(
        "resolve".to_string(),
        FunctionDoc {
            signature: "path.resolve(...: string): string".to_string(),
            description: "Resolve path segments into absolute path".to_string(),
            params: Some(HashMap::from([(
                "...".to_string(),
                "varargs string - Path segments".to_string(),
            )])),
            returns: "string - Absolute path".to_string(),
            errors: None,
            example: r#"path.resolve("src", "main.lua")  -- "/current/working/dir/src/main.lua""#
                .to_string(),
        },
    );

    api.insert(
        "relative".to_string(),
        FunctionDoc {
            signature: "path.relative(from: string, to: string): string".to_string(),
            description: "Get relative path from one path to another".to_string(),
            params: Some(HashMap::from([
                ("from".to_string(), "string - Source path".to_string()),
                ("to".to_string(), "string - Target path".to_string()),
            ])),
            returns: "string - Relative path".to_string(),
            errors: None,
            example: r#"path.relative("/home/user", "/home/user/docs")  -- "docs""#.to_string(),
        },
    );

    api.insert(
        "normalize".to_string(),
        FunctionDoc {
            signature: "path.normalize(p: string): string".to_string(),
            description: "Normalize path, resolving . and .. segments".to_string(),
            params: Some(HashMap::from([(
                "p".to_string(),
                "string - Path to normalize".to_string(),
            )])),
            returns: "string - Normalized path".to_string(),
            errors: None,
            example: r#"path.normalize("/home/./user/../user/docs")  -- "/home/user/docs""#
                .to_string(),
        },
    );

    api.insert(
        "sep".to_string(),
        FunctionDoc {
            signature: "path.sep: string".to_string(),
            description: "Platform-specific path separator".to_string(),
            params: None,
            returns: "string - '/' on Unix, '\\' on Windows".to_string(),
            errors: None,
            example: r#"local parts = {"home", "user", "file.txt"}
local p = table.concat(parts, path.sep)"#
                .to_string(),
        },
    );

    Module {
        name: "path".to_string(),
        description: "Cross-platform path manipulation utilities".to_string(),
        api,
        response_format: None,
        usage_example: r#"local path = require("path")
local filepath = path.join("data", "users", "info.json")
local dir = path.dirname(filepath)  -- "data/users"
local ext = path.extname(filepath)  -- ".json""#
            .to_string(),
    }
}

fn create_events_module() -> Module {
    let mut api = HashMap::new();

    api.insert(
        "EventEmitter".to_string(),
        FunctionDoc {
            signature: "EventEmitter.new(): EventEmitter".to_string(),
            description: "Create new event emitter instance".to_string(),
            params: None,
            returns: "EventEmitter - New instance with methods: on, once, off, emit, listeners, removeAllListeners".to_string(),
            errors: None,
            example: r#"local EventEmitter = require("events").EventEmitter
local emitter = EventEmitter.new()"#.to_string(),
        },
    );

    api.insert(
        "on".to_string(),
        FunctionDoc {
            signature: "emitter:on(event: string, listener: function): nil".to_string(),
            description: "Register event listener".to_string(),
            params: Some(HashMap::from([
                ("event".to_string(), "string - Event name".to_string()),
                (
                    "listener".to_string(),
                    "function - Callback function".to_string(),
                ),
            ])),
            returns: "nil".to_string(),
            errors: None,
            example: r#"emitter:on("data", function(value)
  print("Received:", value)
end)"#
                .to_string(),
        },
    );

    api.insert(
        "once".to_string(),
        FunctionDoc {
            signature: "emitter:once(event: string, listener: function): nil".to_string(),
            description: "Register one-time event listener (auto-removed after first call)"
                .to_string(),
            params: Some(HashMap::from([
                ("event".to_string(), "string - Event name".to_string()),
                (
                    "listener".to_string(),
                    "function - Callback function".to_string(),
                ),
            ])),
            returns: "nil".to_string(),
            errors: None,
            example: r#"emitter:once("ready", function()
  print("Ready fired only once")
end)"#
                .to_string(),
        },
    );

    api.insert(
        "off".to_string(),
        FunctionDoc {
            signature: "emitter:off(event: string, listener: function): nil".to_string(),
            description: "Remove specific event listener".to_string(),
            params: Some(HashMap::from([
                ("event".to_string(), "string - Event name".to_string()),
                (
                    "listener".to_string(),
                    "function - Callback to remove".to_string(),
                ),
            ])),
            returns: "nil".to_string(),
            errors: None,
            example: r#"local handler = function(x) print(x) end
emitter:on("data", handler)
emitter:off("data", handler)"#
                .to_string(),
        },
    );

    api.insert(
        "emit".to_string(),
        FunctionDoc {
            signature: "emitter:emit(event: string, ...): nil".to_string(),
            description: "Emit event, calling all registered listeners with arguments".to_string(),
            params: Some(HashMap::from([
                ("event".to_string(), "string - Event name".to_string()),
                (
                    "...".to_string(),
                    "varargs - Arguments passed to listeners".to_string(),
                ),
            ])),
            returns: "nil".to_string(),
            errors: None,
            example: r#"emitter:emit("data", 42, "hello")"#.to_string(),
        },
    );

    api.insert(
        "listeners".to_string(),
        FunctionDoc {
            signature: "emitter:listeners(event: string): table".to_string(),
            description: "Get array of listeners for event".to_string(),
            params: Some(HashMap::from([(
                "event".to_string(),
                "string - Event name".to_string(),
            )])),
            returns: "table - Array of listener functions".to_string(),
            errors: None,
            example: r#"local count = #emitter:listeners("data")"#.to_string(),
        },
    );

    api.insert(
        "removeAllListeners".to_string(),
        FunctionDoc {
            signature: "emitter:removeAllListeners(event?: string): nil".to_string(),
            description: "Remove all listeners for event, or all events if no event specified"
                .to_string(),
            params: Some(HashMap::from([(
                "event".to_string(),
                "string (optional) - Event name, or omit to clear all".to_string(),
            )])),
            returns: "nil".to_string(),
            errors: None,
            example: r#"emitter:removeAllListeners("data")  -- remove all 'data' listeners
emitter:removeAllListeners()  -- remove all listeners"#
                .to_string(),
        },
    );

    Module {
        name: "events".to_string(),
        description: "Event emitter pattern for pub/sub messaging".to_string(),
        api,
        response_format: None,
        usage_example: r#"local EventEmitter = require("events").EventEmitter
local emitter = EventEmitter.new()

emitter:on("message", function(msg)
  print("Got:", msg)
end)

emitter:emit("message", "Hello!")"#
            .to_string(),
    }
}

fn create_util_module() -> Module {
    let mut api = HashMap::new();

    api.insert(
        "inspect".to_string(),
        FunctionDoc {
            signature: "util.inspect(value: any): string".to_string(),
            description: "Convert any Lua value to readable string representation".to_string(),
            params: Some(HashMap::from([(
                "value".to_string(),
                "any - Value to inspect".to_string(),
            )])),
            returns: "string - String representation".to_string(),
            errors: None,
            example: r#"local t = {name = "Alice", age = 30}
print(util.inspect(t))  -- "{name = 'Alice', age = 30}""#
                .to_string(),
        },
    );

    api.insert(
        "format".to_string(),
        FunctionDoc {
            signature: "util.format(format: string, ...): string".to_string(),
            description: "Format string with printf-style placeholders (%s, %d, %f, etc.)".to_string(),
            params: Some(HashMap::from([
                ("format".to_string(), "string - Format string".to_string()),
                ("...".to_string(), "varargs - Values to interpolate".to_string()),
            ])),
            returns: "string - Formatted string".to_string(),
            errors: None,
            example: r#"util.format("User %s has %d points", "Alice", 100)  -- "User Alice has 100 points""#.to_string(),
        },
    );

    api.insert(
        "promisify".to_string(),
        FunctionDoc {
            signature: "util.promisify(fn: function): function".to_string(),
            description: "Convert callback-based function to promise-style (if async supported)"
                .to_string(),
            params: Some(HashMap::from([(
                "fn".to_string(),
                "function - Callback-based function".to_string(),
            )])),
            returns: "function - Promise-returning function".to_string(),
            errors: None,
            example: r#"local promisifiedFn = util.promisify(callbackFn)"#.to_string(),
        },
    );

    api.insert(
        "inherits".to_string(),
        FunctionDoc {
            signature: "util.inherits(constructor: table, superConstructor: table): nil"
                .to_string(),
            description: "Set up prototype inheritance between constructors".to_string(),
            params: Some(HashMap::from([
                (
                    "constructor".to_string(),
                    "table - Child constructor".to_string(),
                ),
                (
                    "superConstructor".to_string(),
                    "table - Parent constructor".to_string(),
                ),
            ])),
            returns: "nil".to_string(),
            errors: None,
            example: r#"util.inherits(ChildClass, ParentClass)"#.to_string(),
        },
    );

    api.insert(
        "deprecate".to_string(),
        FunctionDoc {
            signature: "util.deprecate(fn: function, message: string): function".to_string(),
            description: "Wrap function to emit deprecation warning on first call".to_string(),
            params: Some(HashMap::from([
                (
                    "fn".to_string(),
                    "function - Function to deprecate".to_string(),
                ),
                (
                    "message".to_string(),
                    "string - Deprecation message".to_string(),
                ),
            ])),
            returns: "function - Wrapped function".to_string(),
            errors: None,
            example: r#"local oldFn = util.deprecate(myOldFunction, "Use newFunction instead")"#
                .to_string(),
        },
    );

    Module {
        name: "util".to_string(),
        description: "Utility functions for debugging, formatting, and function manipulation"
            .to_string(),
        api,
        response_format: None,
        usage_example: r#"local util = require("util")
local data = {users = {"Alice", "Bob"}}
print(util.inspect(data))
print(util.format("Found %d users", #data.users))"#
            .to_string(),
    }
}

fn create_table_module() -> Module {
    let mut api = HashMap::new();

    api.insert(
        "merge".to_string(),
        FunctionDoc {
            signature: "table.merge(target: table, source: table): table".to_string(),
            description: "Merge source table into target table (shallow copy)".to_string(),
            params: Some(HashMap::from([
                (
                    "target".to_string(),
                    "table - Destination table".to_string(),
                ),
                ("source".to_string(), "table - Source table".to_string()),
            ])),
            returns: "table - Merged target table".to_string(),
            errors: None,
            example: r#"local t1 = {a = 1}
local t2 = {b = 2}
table.merge(t1, t2)  -- t1 is now {a = 1, b = 2}"#
                .to_string(),
        },
    );

    api.insert(
        "clone".to_string(),
        FunctionDoc {
            signature: "table.clone(tbl: table): table".to_string(),
            description: "Create deep copy of table".to_string(),
            params: Some(HashMap::from([(
                "tbl".to_string(),
                "table - Table to clone".to_string(),
            )])),
            returns: "table - New cloned table".to_string(),
            errors: None,
            example: r#"local original = {x = {y = 1}}
local copy = table.clone(original)
copy.x.y = 2  -- original.x.y still 1"#
                .to_string(),
        },
    );

    api.insert(
        "keys".to_string(),
        FunctionDoc {
            signature: "table.keys(tbl: table): table".to_string(),
            description: "Get array of all keys in table".to_string(),
            params: Some(HashMap::from([(
                "tbl".to_string(),
                "table - Input table".to_string(),
            )])),
            returns: "table - Array of keys".to_string(),
            errors: None,
            example: r#"local t = {name = "Alice", age = 30}
local ks = table.keys(t)  -- {"name", "age"} (order not guaranteed)"#
                .to_string(),
        },
    );

    api.insert(
        "values".to_string(),
        FunctionDoc {
            signature: "table.values(tbl: table): table".to_string(),
            description: "Get array of all values in table".to_string(),
            params: Some(HashMap::from([(
                "tbl".to_string(),
                "table - Input table".to_string(),
            )])),
            returns: "table - Array of values".to_string(),
            errors: None,
            example: r#"local t = {name = "Alice", age = 30}
local vs = table.values(t)  -- {"Alice", 30} (order not guaranteed)"#
                .to_string(),
        },
    );

    api.insert(
        "filter".to_string(),
        FunctionDoc {
            signature: "table.filter(tbl: table, predicate: function): table".to_string(),
            description: "Filter table elements by predicate function".to_string(),
            params: Some(HashMap::from([
                ("tbl".to_string(), "table - Input table".to_string()),
                (
                    "predicate".to_string(),
                    "function(value, key) - Returns true to keep element".to_string(),
                ),
            ])),
            returns: "table - New filtered table".to_string(),
            errors: None,
            example: r#"local nums = {1, 2, 3, 4, 5}
local evens = table.filter(nums, function(v) return v % 2 == 0 end)  -- {2, 4}"#
                .to_string(),
        },
    );

    api.insert(
        "map".to_string(),
        FunctionDoc {
            signature: "table.map(tbl: table, fn: function): table".to_string(),
            description: "Transform table elements using mapping function".to_string(),
            params: Some(HashMap::from([
                ("tbl".to_string(), "table - Input table".to_string()),
                (
                    "fn".to_string(),
                    "function(value, key) - Returns new value".to_string(),
                ),
            ])),
            returns: "table - New mapped table".to_string(),
            errors: None,
            example: r#"local nums = {1, 2, 3}
local doubled = table.map(nums, function(v) return v * 2 end)  -- {2, 4, 6}"#
                .to_string(),
        },
    );

    api.insert(
        "reduce".to_string(),
        FunctionDoc {
            signature: "table.reduce(tbl: table, fn: function, initial: any): any".to_string(),
            description: "Reduce table to single value using accumulator function".to_string(),
            params: Some(HashMap::from([
                ("tbl".to_string(), "table - Input table".to_string()),
                (
                    "fn".to_string(),
                    "function(accumulator, value, key) - Returns new accumulator".to_string(),
                ),
                (
                    "initial".to_string(),
                    "any - Initial accumulator value".to_string(),
                ),
            ])),
            returns: "any - Final accumulated value".to_string(),
            errors: None,
            example: r#"local nums = {1, 2, 3, 4}
local sum = table.reduce(nums, function(acc, v) return acc + v end, 0)  -- 10"#
                .to_string(),
        },
    );

    api.insert(
        "insert".to_string(),
        FunctionDoc {
            signature: "table.insert(tbl: table, value: any) | table.insert(tbl: table, pos: number, value: any): nil".to_string(),
            description: "Insert element at end or specific position in array".to_string(),
            params: Some(HashMap::from([
                ("tbl".to_string(), "table - Target array".to_string()),
                ("pos".to_string(), "number (optional) - Position to insert at".to_string()),
                ("value".to_string(), "any - Value to insert".to_string()),
            ])),
            returns: "nil".to_string(),
            errors: None,
            example: r#"local arr = {1, 2, 3}
table.insert(arr, 4)      -- {1, 2, 3, 4}
table.insert(arr, 1, 0)   -- {0, 1, 2, 3, 4}"#.to_string(),
        },
    );

    api.insert(
        "remove".to_string(),
        FunctionDoc {
            signature: "table.remove(tbl: table, pos?: number): any".to_string(),
            description: "Remove and return element from array at position (default: last)"
                .to_string(),
            params: Some(HashMap::from([
                ("tbl".to_string(), "table - Target array".to_string()),
                (
                    "pos".to_string(),
                    "number (optional) - Position to remove (default: last element)".to_string(),
                ),
            ])),
            returns: "any - Removed value".to_string(),
            errors: None,
            example: r#"local arr = {1, 2, 3, 4}
local last = table.remove(arr)      -- 4, arr is {1, 2, 3}
local first = table.remove(arr, 1)  -- 1, arr is {2, 3}"#
                .to_string(),
        },
    );

    api.insert(
        "contains".to_string(),
        FunctionDoc {
            signature: "table.contains(tbl: table, value: any): boolean".to_string(),
            description: "Check if table contains value".to_string(),
            params: Some(HashMap::from([
                ("tbl".to_string(), "table - Table to search".to_string()),
                ("value".to_string(), "any - Value to find".to_string()),
            ])),
            returns: "boolean - true if value exists in table".to_string(),
            errors: None,
            example: r#"local arr = {"apple", "banana", "cherry"}
table.contains(arr, "banana")  -- true"#
                .to_string(),
        },
    );

    Module {
        name: "table".to_string(),
        description: "Advanced table manipulation functions (extends Lua's built-in table library)"
            .to_string(),
        api,
        response_format: None,
        usage_example: r#"local tbl = require("table")
local nums = {1, 2, 3, 4, 5}
local evens = tbl.filter(nums, function(n) return n % 2 == 0 end)
local doubled = tbl.map(evens, function(n) return n * 2 end)"#
            .to_string(),
    }
}

fn create_examples() -> HashMap<String, Example> {
    let mut examples = HashMap::new();

    examples.insert(
        "hello_world".to_string(),
        Example {
            description: "Basic hello world script".to_string(),
            code: r#"print("Hello, World!")"#.to_string(),
            command: "hype hello.lua".to_string(),
        },
    );

    examples.insert(
        "cli_args".to_string(),
        Example {
            description: "Access command-line arguments".to_string(),
            code: r#"if _args then
  print("Got " .. #_args .. " arguments")
  for i, arg in ipairs(_args) do
    print(i, arg)
  end
else
  print("No arguments")
end"#
                .to_string(),
            command: "hype script.lua arg1 arg2".to_string(),
        },
    );

    examples.insert(
        "file_io".to_string(),
        Example {
            description: "Read and write files".to_string(),
            code: r#"local fs = require("fs")

local content = fs.readFileSync("input.txt")
local upper = content:upper()
fs.writeFileSync("output.txt", upper)
print("Converted to uppercase")"#
                .to_string(),
            command: "hype convert.lua".to_string(),
        },
    );

    examples.insert(
        "http_request".to_string(),
        Example {
            description: "Make HTTP GET request".to_string(),
            code: r#"local http = require("http")

local resp = http.get("https://api.github.com/users/octocat")
if resp.ok() then
  local data = resp.json()
  print("User:", data.login)
  print("Name:", data.name)
  print("Repos:", data.public_repos)
else
  print("Error:", resp.status)
end"#
                .to_string(),
            command: "hype fetch.lua".to_string(),
        },
    );

    examples.insert(
        "http_post_json".to_string(),
        Example {
            description: "POST JSON data".to_string(),
            code: r#"local http = require("http")

local resp = http.postJson("https://httpbin.org/post", {
  name = "Alice",
  age = 30,
  active = true
})

if resp.ok() then
  local result = resp.json()
  print("Posted:", result.json.name)
end"#
                .to_string(),
            command: "hype post.lua".to_string(),
        },
    );

    examples.insert(
        "module_usage".to_string(),
        Example {
            description: "Use custom local module".to_string(),
            code: r#"-- main.lua
local utils = require("./lib/utils")
print(utils.greet("World"))

-- lib/utils.lua
local M = {}

function M.greet(name)
  return "Hello, " .. name .. "!"
end

return M"#
                .to_string(),
            command: "hype main.lua".to_string(),
        },
    );

    examples.insert(
        "event_emitter".to_string(),
        Example {
            description: "Use event emitter pattern".to_string(),
            code: r#"local EventEmitter = require("events").EventEmitter
local emitter = EventEmitter.new()

emitter:on("data", function(value)
  print("Received:", value)
end)

emitter:once("end", function()
  print("Stream ended")
end)

emitter:emit("data", 42)
emitter:emit("data", 100)
emitter:emit("end")"#
                .to_string(),
            command: "hype events.lua".to_string(),
        },
    );

    examples
}
