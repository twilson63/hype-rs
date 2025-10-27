# Project Request Protocol: Standard Library Implementation Roadmap

**Project Name**: hype-rs Standard Library Modules  
**Project ID**: PRP-014  
**Priority**: ⭐⭐⭐⭐ HIGH  
**Estimated Duration**: 8-10 weeks  
**Target Completion**: Q1 2025

---

## 1. Executive Summary

This document outlines the complete roadmap for implementing a comprehensive standard library for hype-rs, modeled after Node.js and Deno stdlib patterns. The goal is to provide a batteries-included scripting environment that enables real-world application development without external dependencies.

### Current State (v0.3.0)

**Implemented Modules (8):**
- ✅ **fs** - Filesystem operations (8 functions)
- ✅ **json** - JSON encoding/decoding (4 functions)
- ✅ **process** - Process and environment management (10 functions)
- ✅ **path** - Path manipulation (5 functions)
- ✅ **events** - Event emitter (3 methods)
- ✅ **util** - Utility functions (2 functions)
- ✅ **table** - Table operations (4 functions)
- ✅ **http** - HTTP client (8 methods)

**Test Coverage**: 148 tests passing ✅

---

## 2. Remaining Standard Library Modules

### Priority 1: Core System Modules (4-5 weeks)

#### 2.1 **os** Module
**Priority**: ⭐⭐⭐⭐⭐ CRITICAL  
**Effort**: 1-2 days  
**Target**: v0.4.0

System information and operating system utilities.

**Functions:**
```lua
os.platform()      -- Get OS: "linux", "macos", "windows"
os.arch()          -- Get arch: "x86_64", "aarch64", "arm"
os.hostname()      -- Get hostname
os.homedir()       -- Get home directory path
os.tmpdir()        -- Get temp directory path
os.cpus()          -- Get CPU info: [{model, speed}]
os.totalmem()      -- Total system memory in bytes
os.freemem()       -- Free system memory in bytes
os.uptime()        -- System uptime in seconds
os.loadavg()       -- Load average [1, 5, 15] (Unix only)
os.networkInterfaces() -- Network interfaces info
os.userInfo()      -- Current user info {username, uid, gid, shell, homedir}
os.EOL             -- End of line marker: "\n" or "\r\n"
```

**Dependencies**: None (uses std::env, sys-info crate)  
**Tests**: 15 unit + 10 integration = 25 tests

---

#### 2.2 **string** Module
**Priority**: ⭐⭐⭐⭐ HIGH  
**Effort**: 1 day  
**Target**: v0.4.0

Enhanced string manipulation utilities (extends Lua's built-in string library).

**Functions:**
```lua
string.split(str, delimiter)           -- Split into array
string.trim(str)                       -- Remove whitespace
string.trimStart(str)                  -- Remove leading whitespace
string.trimEnd(str)                    -- Remove trailing whitespace
string.startsWith(str, prefix)         -- Check prefix
string.endsWith(str, suffix)           -- Check suffix
string.contains(str, substring)        -- Check substring
string.padStart(str, length, fill?)    -- Pad start
string.padEnd(str, length, fill?)      -- Pad end
string.repeat(str, count)              -- Repeat string
string.replace(str, pattern, replacement, count?) -- Replace
string.replaceAll(str, pattern, replacement)      -- Replace all
string.toUpperCase(str)                -- Uppercase
string.toLowerCase(str)                -- Lowercase
string.capitalize(str)                 -- Capitalize first letter
string.lines(str)                      -- Split into lines
string.chars(str)                      -- Split into chars
```

**Dependencies**: None (uses Rust std::string)  
**Tests**: 20 unit + 12 integration = 32 tests

---

#### 2.3 **crypto** Module
**Priority**: ⭐⭐⭐⭐ HIGH  
**Effort**: 2-3 days  
**Target**: v0.4.0

Cryptographic operations for security and hashing.

**Functions:**
```lua
-- Hashing
crypto.hash(algorithm, data)           -- Hash: "sha256", "sha1", "md5"
crypto.hashFile(algorithm, path)       -- Hash file contents
crypto.hmac(algorithm, key, data)      -- HMAC signing

-- Random
crypto.randomBytes(size)               -- Secure random bytes
crypto.randomInt(min, max)             -- Secure random integer
crypto.randomUUID()                    -- Generate UUID v4

-- Encoding
crypto.base64Encode(data)              -- Base64 encode
crypto.base64Decode(data)              -- Base64 decode
crypto.hexEncode(data)                 -- Hex encode
crypto.hexDecode(data)                 -- Hex decode

-- Password Hashing
crypto.bcrypt(password, rounds?)       -- Bcrypt hash
crypto.bcryptVerify(password, hash)    -- Verify bcrypt

-- Utilities
crypto.timingSafeEqual(a, b)           -- Constant-time comparison
```

**Dependencies**: sha2, md-5, hmac, rand, base64, bcrypt, hex  
**Tests**: 18 unit + 12 integration = 30 tests

---

#### 2.4 **time** Module
**Priority**: ⭐⭐⭐ MEDIUM  
**Effort**: 1-2 days  
**Target**: v0.4.0

Date and time operations.

**Functions:**
```lua
-- Current Time
time.now()                             -- Current timestamp (ms)
time.nowSeconds()                      -- Current timestamp (seconds)
time.nowNanos()                        -- Current timestamp (nanoseconds)

-- Formatting
time.format(timestamp, format)         -- Format timestamp
time.parse(string, format)             -- Parse date string
time.toISO(timestamp)                  -- ISO 8601 format
time.fromISO(string)                   -- Parse ISO 8601

-- Date Components
time.date(timestamp?)                  -- Get date table {year, month, day, ...}
time.year(timestamp?)                  -- Get year
time.month(timestamp?)                 -- Get month (1-12)
time.day(timestamp?)                   -- Get day (1-31)
time.hour(timestamp?)                  -- Get hour (0-23)
time.minute(timestamp?)                -- Get minute (0-59)
time.second(timestamp?)                -- Get second (0-59)

-- Utilities
time.sleep(ms)                         -- Sleep for milliseconds
time.elapsed(start)                    -- Elapsed time since start
time.duration(ms)                      -- Human-readable duration
```

**Dependencies**: chrono (already in Cargo.toml)  
**Tests**: 16 unit + 10 integration = 26 tests

---

### Priority 2: Web & Network Modules (2-3 weeks)

#### 2.5 **url** Module
**Priority**: ⭐⭐⭐ MEDIUM  
**Effort**: 1 day  
**Target**: v0.4.0

URL parsing and manipulation.

**Functions:**
```lua
-- Parsing
url.parse(urlString)                   -- Parse URL into components
-- Returns: {protocol, host, hostname, port, path, query, hash, auth}

-- Building
url.format(urlObject)                  -- Build URL from components
url.resolve(base, relative)            -- Resolve relative URL

-- Encoding
url.encode(string)                     -- URL encode
url.decode(string)                     -- URL decode
url.encodeComponent(string)            -- Encode URL component
url.decodeComponent(string)            -- Decode URL component

-- Query String
url.parseQuery(queryString)            -- Parse query string
url.formatQuery(table)                 -- Format query string
```

**Dependencies**: url crate (already in Cargo.toml)  
**Tests**: 15 unit + 8 integration = 23 tests

---

#### 2.6 **querystring** Module
**Priority**: ⭐⭐ LOW-MEDIUM  
**Effort**: 0.5 days  
**Target**: v0.4.0

Query string encoding and decoding.

**Functions:**
```lua
querystring.parse(string)              -- Parse query string to table
querystring.stringify(table)           -- Table to query string
querystring.escape(string)             -- Escape query value
querystring.unescape(string)           -- Unescape query value
```

**Dependencies**: serde_urlencoded (already in Cargo.toml)  
**Tests**: 10 unit + 6 integration = 16 tests

---

#### 2.7 **websocket** Module (Optional)
**Priority**: ⭐ LOW  
**Effort**: 3-4 days  
**Target**: v0.5.0+

WebSocket client for real-time communication.

**Functions:**
```lua
-- Client
local ws = websocket.connect(url, options?)
ws:send(message)
ws:receive()
ws:close(code?, reason?)
ws:onMessage(callback)
ws:onError(callback)
ws:onClose(callback)
```

**Dependencies**: tokio-tungstenite  
**Tests**: 12 unit + 8 integration = 20 tests

---

### Priority 3: Data & Testing Modules (1-2 weeks)

#### 2.8 **buffer** Module
**Priority**: ⭐⭐⭐ MEDIUM  
**Effort**: 2-3 days  
**Target**: v0.5.0

Binary data handling.

**Functions:**
```lua
-- Creation
Buffer.from(data, encoding?)           -- Create from string/array
Buffer.alloc(size, fill?)              -- Allocate zeroed buffer
Buffer.allocUnsafe(size)               -- Allocate uninitialized

-- Conversion
buffer:toString(encoding?)             -- To string
buffer:toArray()                       -- To byte array
buffer:toHex()                         -- To hex string

-- Read/Write
buffer:readUInt8(offset)               -- Read unsigned 8-bit
buffer:readUInt16LE(offset)            -- Read unsigned 16-bit LE
buffer:readUInt32LE(offset)            -- Read unsigned 32-bit LE
buffer:writeUInt8(value, offset)       -- Write unsigned 8-bit
buffer:writeUInt16LE(value, offset)    -- Write unsigned 16-bit LE
buffer:writeUInt32LE(value, offset)    -- Write unsigned 32-bit LE

-- Operations
buffer:slice(start, end?)              -- Slice buffer
buffer:copy(target, targetStart?, sourceStart?, sourceEnd?)
buffer:fill(value, offset?, end?)      -- Fill buffer
buffer:equals(other)                   -- Compare buffers
buffer:concat(list)                    -- Concatenate buffers

-- Properties
buffer.length                          -- Buffer length
```

**Dependencies**: None (uses Vec<u8>)  
**Tests**: 20 unit + 12 integration = 32 tests

---

#### 2.9 **assert** Module
**Priority**: ⭐⭐ LOW-MEDIUM  
**Effort**: 1 day  
**Target**: v0.5.0

Assertion utilities for testing.

**Functions:**
```lua
-- Basic Assertions
assert.ok(value, message?)             -- Assert truthy
assert.equal(actual, expected, message?)        -- Shallow equality
assert.deepEqual(actual, expected, message?)    -- Deep equality
assert.notEqual(actual, expected, message?)     -- Not equal
assert.notDeepEqual(actual, expected, message?) -- Not deep equal

-- Type Assertions
assert.isString(value, message?)       -- Assert string
assert.isNumber(value, message?)       -- Assert number
assert.isBoolean(value, message?)      -- Assert boolean
assert.isTable(value, message?)        -- Assert table
assert.isFunction(value, message?)     -- Assert function
assert.isNil(value, message?)          -- Assert nil

-- Comparison
assert.greaterThan(a, b, message?)     -- a > b
assert.lessThan(a, b, message?)        -- a < b
assert.greaterThanOrEqual(a, b, message?)  -- a >= b
assert.lessThanOrEqual(a, b, message?)     -- a <= b

-- Pattern Matching
assert.matches(string, pattern, message?)  -- String matches pattern
assert.contains(array, value, message?)    -- Array contains value

-- Error Handling
assert.throws(fn, message?)            -- Assert function throws
assert.doesNotThrow(fn, message?)      -- Assert function doesn't throw
```

**Dependencies**: None  
**Tests**: 25 unit + 15 integration = 40 tests

---

#### 2.10 **regex** Module
**Priority**: ⭐⭐ LOW-MEDIUM  
**Effort**: 1 day  
**Target**: v0.5.0

Regular expression support.

**Functions:**
```lua
-- Creation
local pattern = regex.new(pattern, flags?)  -- Create regex

-- Matching
pattern:test(string)                   -- Test if matches
pattern:match(string)                  -- Get first match
pattern:matchAll(string)               -- Get all matches
pattern:captures(string)               -- Get capture groups

-- Replacement
pattern:replace(string, replacement)   -- Replace first match
pattern:replaceAll(string, replacement) -- Replace all matches

-- Splitting
pattern:split(string, limit?)          -- Split by pattern

-- Utilities
regex.escape(string)                   -- Escape special chars
```

**Dependencies**: regex (already in Cargo.toml)  
**Tests**: 18 unit + 10 integration = 28 tests

---

### Priority 4: Advanced Modules (2-3 weeks)

#### 2.11 **child_process** Module
**Priority**: ⭐⭐ LOW-MEDIUM  
**Effort**: 2-3 days  
**Target**: v0.6.0

Spawn and manage child processes.

**Functions:**
```lua
-- Synchronous
child_process.execSync(command, options?)       -- Execute command
child_process.spawnSync(command, args, options?) -- Spawn command

-- Asynchronous (if tokio enabled)
child_process.exec(command, callback)
child_process.spawn(command, args, options?)

-- Process Object
process:kill(signal?)
process:wait()
process.pid
process.stdin
process.stdout
process.stderr
```

**Dependencies**: std::process  
**Tests**: 15 unit + 10 integration = 25 tests

---

#### 2.12 **stream** Module
**Priority**: ⭐ LOW  
**Effort**: 3-4 days  
**Target**: v0.6.0+

Stream abstractions for data processing.

**Functions:**
```lua
-- Readable Stream
local readable = stream.Readable:new(options?)
readable:read(size?)
readable:pipe(destination)
readable:on("data", callback)
readable:on("end", callback)

-- Writable Stream
local writable = stream.Writable:new(options?)
writable:write(chunk)
writable:end(chunk?)
writable:on("finish", callback)

-- Transform Stream
local transform = stream.Transform:new(options?)
transform:transform(chunk)

-- Utilities
stream.pipeline(source, ...transforms, destination)
```

**Dependencies**: None (custom implementation)  
**Tests**: 20 unit + 12 integration = 32 tests

---

#### 2.13 **console** Module
**Priority**: ⭐⭐ LOW-MEDIUM  
**Effort**: 0.5 days  
**Target**: v0.4.0

Enhanced console output.

**Functions:**
```lua
console.log(...)                       -- Log to stdout
console.error(...)                     -- Log to stderr
console.warn(...)                      -- Warning (yellow)
console.info(...)                      -- Info (blue)
console.debug(...)                     -- Debug (gray)
console.trace(...)                     -- Log with stack trace

-- Timing
console.time(label)                    -- Start timer
console.timeEnd(label)                 -- End timer and log

-- Formatting
console.table(data)                    -- Display as table
console.dir(object, options?)          -- Display object

-- Grouping
console.group(label?)                  -- Start group
console.groupEnd()                     -- End group

-- Assertions
console.assert(condition, ...message)  -- Assert and log
```

**Dependencies**: colored (for colors)  
**Tests**: 12 unit + 8 integration = 20 tests

---

#### 2.14 **encoding** Module
**Priority**: ⭐⭐ LOW-MEDIUM  
**Effort**: 1-2 days  
**Target**: v0.5.0

Character encoding conversions.

**Functions:**
```lua
-- Text Encoding
encoding.utf8Encode(string)            -- UTF-8 encode
encoding.utf8Decode(bytes)             -- UTF-8 decode
encoding.latin1Encode(string)          -- Latin-1 encode
encoding.latin1Decode(bytes)           -- Latin-1 decode

-- Binary Encoding
encoding.base64Encode(data)            -- Base64 encode
encoding.base64Decode(string)          -- Base64 decode
encoding.base64UrlEncode(data)         -- URL-safe base64
encoding.base64UrlDecode(string)       -- URL-safe base64 decode
encoding.hexEncode(data)               -- Hex encode
encoding.hexDecode(string)             -- Hex decode

-- Utilities
encoding.detect(data)                  -- Detect encoding
encoding.convert(data, from, to)       -- Convert encoding
```

**Dependencies**: base64, hex, encoding_rs  
**Tests**: 15 unit + 10 integration = 25 tests

---

#### 2.15 **compression** Module (Optional)
**Priority**: ⭐ LOW  
**Effort**: 2-3 days  
**Target**: v0.6.0+

Data compression and decompression.

**Functions:**
```lua
-- Gzip
compression.gzip(data, level?)         -- Compress with gzip
compression.gunzip(data)               -- Decompress gzip

-- Deflate
compression.deflate(data, level?)      -- Compress with deflate
compression.inflate(data)              -- Decompress deflate

-- Brotli
compression.brotli(data, level?)       -- Compress with brotli
compression.unbrotli(data)             -- Decompress brotli

-- Utilities
compression.detectFormat(data)         -- Detect compression format
```

**Dependencies**: flate2, brotli  
**Tests**: 12 unit + 8 integration = 20 tests

---

## 3. Implementation Phases

### Phase 1: Core System (v0.4.0) - 2 weeks
- ✅ fs (completed in v0.3.0)
- ✅ json (completed in v0.3.0)
- ✅ process (completed in v0.3.0)
- 🔄 **os** (1-2 days)
- 🔄 **string** (1 day)
- 🔄 **crypto** (2-3 days)
- 🔄 **time** (1-2 days)
- 🔄 **url** (1 day)
- 🔄 **querystring** (0.5 days)
- 🔄 **console** (0.5 days)

**Total**: 8-11 days of work

---

### Phase 2: Data & Testing (v0.5.0) - 2 weeks
- 🔄 **buffer** (2-3 days)
- 🔄 **assert** (1 day)
- 🔄 **regex** (1 day)
- 🔄 **encoding** (1-2 days)

**Total**: 5-7 days of work

---

### Phase 3: Advanced Features (v0.6.0) - 3-4 weeks
- 🔄 **child_process** (2-3 days)
- 🔄 **stream** (3-4 days)
- 🔄 **websocket** (3-4 days)
- 🔄 **compression** (2-3 days)

**Total**: 10-14 days of work

---

## 4. Module Summary Table

| Module | Priority | Effort | Phase | Functions | Dependencies |
|--------|----------|--------|-------|-----------|--------------|
| **os** | ⭐⭐⭐⭐⭐ | 1-2d | v0.4.0 | 13 | sys-info |
| **string** | ⭐⭐⭐⭐ | 1d | v0.4.0 | 17 | None |
| **crypto** | ⭐⭐⭐⭐ | 2-3d | v0.4.0 | 13 | sha2, bcrypt, etc |
| **time** | ⭐⭐⭐ | 1-2d | v0.4.0 | 14 | chrono |
| **url** | ⭐⭐⭐ | 1d | v0.4.0 | 9 | url |
| **querystring** | ⭐⭐ | 0.5d | v0.4.0 | 4 | serde_urlencoded |
| **console** | ⭐⭐ | 0.5d | v0.4.0 | 11 | colored |
| **buffer** | ⭐⭐⭐ | 2-3d | v0.5.0 | 18 | None |
| **assert** | ⭐⭐ | 1d | v0.5.0 | 17 | None |
| **regex** | ⭐⭐ | 1d | v0.5.0 | 7 | regex |
| **encoding** | ⭐⭐ | 1-2d | v0.5.0 | 11 | encoding_rs |
| **child_process** | ⭐⭐ | 2-3d | v0.6.0 | 6 | std::process |
| **stream** | ⭐ | 3-4d | v0.6.0+ | 12 | None |
| **websocket** | ⭐ | 3-4d | v0.6.0+ | 7 | tokio-tungstenite |
| **compression** | ⭐ | 2-3d | v0.6.0+ | 7 | flate2, brotli |

**Totals:**
- **15 new modules** to implement
- **169 new functions** total
- **~30-40 days** of development
- **~400+ tests** to write

---

## 5. Dependencies Required

### New Cargo Dependencies

```toml
[dependencies]
# Already have: mlua, clap, anyhow, tokio, reqwest, url, regex, serde, 
# serde_json, tempfile, chrono, base64, serde_urlencoded

# Need to add:
sys-info = "0.9"              # For os module
colored = "2.0"               # For console colors
sha2 = "0.10"                 # For crypto hashing
md-5 = "0.10"                 # For MD5 hashing
hmac = "0.12"                 # For HMAC
rand = "0.8"                  # For crypto random
bcrypt = "0.15"               # For password hashing
hex = "0.4"                   # For hex encoding
encoding_rs = "0.8"           # For text encoding
flate2 = "1.0"                # For compression
brotli = "3.3"                # For brotli compression
tokio-tungstenite = "0.20"    # For websockets (optional)
uuid = { version = "1.0", features = ["v4"] }  # For UUID generation
```

---

## 6. Success Metrics

### Code Metrics
- ✅ All modules implemented with comprehensive APIs
- ✅ Minimum 20 tests per module (unit + integration)
- ✅ 100% of public APIs documented
- ✅ Zero clippy warnings in new code
- ✅ Example script for each module

### Performance Metrics
- ✅ Crypto operations: <10ms for common operations
- ✅ String operations: O(n) time complexity
- ✅ Buffer operations: Zero-copy where possible
- ✅ Time operations: <1ms for formatting

### Quality Metrics
- ✅ RFC compliance where applicable (URLs, dates, crypto)
- ✅ Cross-platform support (Windows, macOS, Linux)
- ✅ Memory-safe implementations (no unsafe unless necessary)
- ✅ Comprehensive error handling

---

## 7. Implementation Guidelines

### For Each Module:

1. **Create directory structure:**
   ```
   src/modules/builtins/MODULE_NAME/
   ├── error.rs           # Custom error types
   ├── operations.rs      # Core functionality
   ├── lua_bindings.rs    # Lua API bindings
   └── mod.rs            # Module definition
   ```

2. **Write tests:**
   ```
   tests/MODULE_NAME_test.rs              # Integration tests
   tests/lua_scripts/test_MODULE_NAME.lua # Lua tests
   examples/MODULE_NAME-demo.lua          # Example script
   ```

3. **Documentation:**
   - Update `CHANGELOG.md`
   - Add entry to `docs/modules/builtin-modules.md`
   - Create API reference in `docs/modules/MODULE_NAME-api.md`

4. **Register module:**
   - Add to `src/modules/builtins/mod.rs`
   - Add to builtin registry
   - Add to `is_builtin()` and `list()` functions

---

## 8. Timeline

**Estimated Timeline**: 8-10 weeks

- **Week 1-2**: os, string, crypto, time (v0.4.0 core)
- **Week 3**: url, querystring, console (v0.4.0 utilities)
- **Week 4-5**: buffer, assert, regex (v0.5.0)
- **Week 6**: encoding (v0.5.0 complete)
- **Week 7-8**: child_process, stream (v0.6.0)
- **Week 9-10**: websocket, compression (v0.6.0 complete)

---

## 9. Next Steps

### Immediate (Next Module to Implement):
**Recommendation**: Start with **os** module
- Most fundamental after process
- Required by many scripts
- Quick win (1-2 days)
- No complex dependencies

### Following Modules (in order):
1. **os** (system info)
2. **string** (utilities)
3. **crypto** (security)
4. **time** (dates)
5. **url** (web)
6. **console** (output)
7. **querystring** (web utilities)

---

## 10. Appendix

### A. Module Interdependencies

```
os ────┐
       ├──> console (uses os for colors)
       └──> time (uses os for timezone)

process ──> child_process (spawns processes)

buffer ───┐
          ├──> crypto (uses buffers)
          └──> stream (uses buffers)

string ───> regex (string operations)

url ──────> querystring (URL parsing)
```

### B. Feature Flags

Some modules may require feature flags:

```toml
[features]
default = ["http", "crypto"]
crypto = ["sha2", "md-5", "hmac", "bcrypt"]
compression = ["flate2", "brotli"]
websocket = ["tokio-tungstenite", "tokio"]
```

---

**Document Version**: 1.0  
**Last Updated**: 2025-10-27  
**Author**: hype-rs Development Team  
**Status**: APPROVED - Ready for Implementation
