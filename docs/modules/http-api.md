# HTTP Module API Reference

> **Status**: ✅ Fully implemented and tested with Lua bindings via `require("http")`

## Overview

The HTTP module provides a comprehensive HTTP client for making web requests from Lua scripts. It supports all standard HTTP methods (GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS) and provides a modern fetch-style API.

## Features

- ✅ **Asynchronous HTTP requests** with blocking Lua API via Tokio runtime
- ✅ **Multiple HTTP methods**: GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS
- ✅ **Modern fetch API** with flexible options
- ✅ **JSON helpers** for posting/putting JSON data
- ✅ **Custom headers** support
- ✅ **Timeout control** per-request
- ✅ **Response parsing** (text, JSON)
- ✅ **Feature-gated** compilation with `http` feature flag

## Installation

The HTTP module is built-in and available when Hype-RS is compiled with the `http` feature:

```bash
cargo build --features http
```

Or use the default features (which include HTTP):

```bash
cargo build
```

## Loading the Module

```lua
local http = require("http")
```

## API Reference

### http.get(url)

Perform an HTTP GET request.

**Parameters:**
- `url` (string): The URL to request

**Returns:**
- `Response` object

**Example:**
```lua
local http = require("http")
local response = http.get("https://api.example.com/users")

if response:ok() then
    print("Status:", response.status)
    print("Body:", response:text())
end
```

---

### http.post(url, options?)

Perform an HTTP POST request.

**Parameters:**
- `url` (string): The URL to request
- `options` (table, optional):
  - `body` (string, optional): Request body
  - `headers` (table, optional): Custom headers as key-value pairs

**Returns:**
- `Response` object

**Example:**
```lua
local http = require("http")
local response = http.post("https://api.example.com/users", {
    body = '{"name": "Alice", "age": 30}',
    headers = {
        ["Content-Type"] = "application/json",
        ["Authorization"] = "Bearer token123"
    }
})
```

---

### http.put(url, options?)

Perform an HTTP PUT request.

**Parameters:**
- `url` (string): The URL to request
- `options` (table, optional):
  - `body` (string, optional): Request body
  - `headers` (table, optional): Custom headers

**Returns:**
- `Response` object

**Example:**
```lua
local http = require("http")
local response = http.put("https://api.example.com/users/123", {
    body = '{"name": "Bob"}',
    headers = {["Content-Type"] = "application/json"}
})
```

---

### http.delete(url, options?)

Perform an HTTP DELETE request.

**Parameters:**
- `url` (string): The URL to request
- `options` (table, optional):
  - `headers` (table, optional): Custom headers

**Returns:**
- `Response` object

**Example:**
```lua
local http = require("http")
local response = http.delete("https://api.example.com/users/123", {
    headers = {["Authorization"] = "Bearer token123"}
})
```

---

### http.patch(url, options?)

Perform an HTTP PATCH request.

**Parameters:**
- `url` (string): The URL to request
- `options` (table, optional):
  - `body` (string, optional): Request body
  - `headers` (table, optional): Custom headers

**Returns:**
- `Response` object

---

### http.head(url, options?)

Perform an HTTP HEAD request.

**Parameters:**
- `url` (string): The URL to request
- `options` (table, optional):
  - `headers` (table, optional): Custom headers

**Returns:**
- `Response` object

---

### http.fetch(url, options?)

Universal fetch API for HTTP requests (similar to JavaScript's fetch).

**Parameters:**
- `url` (string): The URL to request
- `options` (table, optional):
  - `method` (string, optional): HTTP method (default: "GET")
  - `body` (string, optional): Request body
  - `headers` (table, optional): Custom headers
  - `timeout` (number, optional): Request timeout in milliseconds

**Returns:**
- `Response` object

**Example:**
```lua
local http = require("http")
local response = http.fetch("https://api.example.com/data", {
    method = "POST",
    body = '{"query": "search term"}',
    headers = {
        ["Content-Type"] = "application/json",
        ["API-Key"] = "abc123"
    },
    timeout = 5000  -- 5 seconds
})

if response:ok() then
    local data = response:json()
    print("Results:", data.count)
end
```

---

### http.postJson(url, data)

Convenience method for posting JSON data.

**Parameters:**
- `url` (string): The URL to request
- `data` (table): Lua table to be serialized to JSON

**Returns:**
- `Response` object

**Example:**
```lua
local http = require("http")
local user = {
    name = "Charlie",
    email = "charlie@example.com",
    age = 28
}

local response = http.postJson("https://api.example.com/users", user)
local result = response:json()
print("User ID:", result.id)
```

---

### http.putJson(url, data)

Convenience method for putting JSON data.

**Parameters:**
- `url` (string): The URL to request
- `data` (table): Lua table to be serialized to JSON

**Returns:**
- `Response` object

**Example:**
```lua
local http = require("http")
local updates = {name = "Charles"}
local response = http.putJson("https://api.example.com/users/123", updates)
```

---

## Response Object

All HTTP methods return a `Response` object with the following properties and methods:

### Properties

- `status` (number): HTTP status code (e.g., 200, 404, 500)
- `statusText` (string): HTTP status text (e.g., "OK", "Not Found")
- `headers` (table): Response headers as key-value pairs
- `body` (string): Raw response body

### Methods

#### response.ok()

Check if the response status indicates success (200-299).

**Returns:**
- `boolean`: true if status is 2xx, false otherwise

**Example:**
```lua
if response:ok() then
    print("Success!")
else
    print("Error:", response.status, response.statusText)
end
```

---

#### response.text()

Get the response body as a string.

**Returns:**
- `string`: Response body

**Example:**
```lua
local html = response:text()
print("Page length:", #html)
```

---

#### response.json()

Parse the response body as JSON.

**Returns:**
- `table`: Parsed JSON data

**Throws:**
- Error if JSON parsing fails

**Example:**
```lua
local data = response:json()
print("User name:", data.name)
print("User email:", data.email)
```

---

## Error Handling

The HTTP module provides detailed error types:

- **NetworkError**: Connection or network-related errors
- **TimeoutError**: Request exceeded timeout duration
- **InvalidUrl**: Malformed URL
- **RequestError**: Error building or sending request
- **ResponseError**: HTTP error status (4xx, 5xx)
- **JsonParseError**: Failed to parse JSON response
- **RuntimeError**: Internal runtime error

**Example:**
```lua
local http = require("http")

local ok, response = pcall(function()
    return http.get("https://invalid-domain-xyz.com")
end)

if not ok then
    print("Request failed:", response)
else
    print("Success:", response.status)
end
```

---

## Complete Examples

### Example 1: Fetching JSON Data

```lua
local http = require("http")

local response = http.get("https://api.github.com/users/torvalds")

if response:ok() then
    local user = response:json()
    print("Name:", user.name)
    print("Location:", user.location)
    print("Public Repos:", user.public_repos)
else
    print("Failed:", response.status, response.statusText)
end
```

### Example 2: Creating a Resource

```lua
local http = require("http")

local newPost = {
    title = "Hello World",
    body = "This is my first post",
    userId = 1
}

local response = http.postJson("https://jsonplaceholder.typicode.com/posts", newPost)

if response:ok() then
    local created = response:json()
    print("Created post with ID:", created.id)
end
```

### Example 3: Authenticated Request

```lua
local http = require("http")

local response = http.fetch("https://api.example.com/protected", {
    method = "GET",
    headers = {
        ["Authorization"] = "Bearer " .. os.getenv("API_TOKEN"),
        ["Accept"] = "application/json"
    }
})

if response.status == 401 then
    print("Unauthorized - check your token")
elseif response:ok() then
    local data = response:json()
    print("Data:", data)
end
```

### Example 4: Timeout Handling

```lua
local http = require("http")

local ok, response = pcall(function()
    return http.fetch("https://httpbin.org/delay/10", {
        timeout = 3000  -- 3 second timeout
    })
end)

if not ok then
    print("Request timed out or failed")
else
    print("Response:", response:text())
end
```

### Example 5: File Upload (Form Data)

```lua
local http = require("http")

local fileContent = fs.readFileSync("data.txt")

local response = http.post("https://api.example.com/upload", {
    body = fileContent,
    headers = {
        ["Content-Type"] = "text/plain",
        ["Content-Length"] = tostring(#fileContent)
    }
})

print("Upload status:", response.status)
```

---

## Implementation Details

### Architecture

The HTTP module is implemented in Rust using:
- **reqwest**: HTTP client library (v0.12)
- **tokio**: Async runtime for handling requests
- **serde_json**: JSON serialization/deserialization

The module uses a blocking API on the Lua side while leveraging async I/O underneath via Tokio's `Runtime::block_on()`.

### Performance Considerations

- **Connection Pooling**: The HTTP client maintains a connection pool (max 10 idle connections per host)
- **HTTP/1.1 & HTTP/2**: Supports both protocols with automatic negotiation
- **Timeouts**: Default timeout is 30 seconds, configurable per-request
- **Memory**: Response bodies are loaded into memory; use streaming for large files (future enhancement)

### Security

- **HTTPS**: Full TLS support via rustls
- **Certificate Validation**: Certificates are validated by default
- **Redirects**: Automatic redirect following (up to 10 by default)
- **Proxy Support**: System proxy settings are respected

---

## Building from Source

To enable the HTTP module:

```bash
# Build with HTTP support (default)
cargo build --release

# Build without HTTP support
cargo build --release --no-default-features

# Build with specific features
cargo build --release --features http
```

---

## Future Enhancements

Planned features:

- [ ] Streaming response bodies for large downloads
- [ ] Multipart form data support
- [ ] Cookie jar management
- [ ] Custom certificate validation
- [ ] Proxy configuration from Lua
- [ ] WebSocket support
- [ ] Progress callbacks for uploads/downloads
- [ ] Response caching
- [ ] Request retry with exponential backoff
- [ ] Rate limiting

---

## See Also

- [Built-in Modules Overview](./builtin-modules.md)
- [Module System](./README.md)
- [Getting Started](./getting-started.md)
- [HTTP Module PRP](../../PRPs/http-module-prp.md)

---

## Contributing

Found a bug or want to contribute? See [CONTRIBUTING.md](../../.github/CONTRIBUTING.md) for guidelines.

---

## License

MIT OR Apache-2.0
