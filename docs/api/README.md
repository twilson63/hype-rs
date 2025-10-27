# Hype Lua Standard Library API Documentation

> **All examples in this documentation are tested and verified to work.**

## Overview

The Hype Lua runtime includes a comprehensive standard library with modules for:
- Cryptography and security
- String manipulation
- Date/time operations
- File system operations
- HTTP requests
- JSON parsing
- System information
- And more...

## Quick Start

```lua
-- Import a module
local crypto = require("crypto")
local fs = require("fs")
local json = require("json")

-- Use it
local hash = crypto.hash("sha256", "hello")
local data = fs.read("file.txt")
local obj = json.parse('{"name": "John"}')
```

## Available Modules

### Core Utilities
- [**crypto**](crypto.md) - Cryptographic operations (hashing, HMAC, bcrypt, random, encoding) ✅ Documented
- [**string**](string.md) - Enhanced string manipulation (split, trim, case, padding) ✅ Documented
- [**time**](time.md) - Date and time operations (timestamps, ISO 8601, formatting) ✅ Documented
- [**json**](json.md) - JSON encoding/decoding with Unicode support ✅ Documented

### Web & Networking
- [**http**](http.md) - HTTP client (GET, POST, cookies, auth, proxies)
- [**url**](url.md) - URL parsing and manipulation (RFC 3986 compliant) ✅ Documented
- [**querystring**](querystring.md) - Query string parsing and formatting ✅ Documented

### File System & I/O
- [**fs**](fs.md) - File system operations (read, write, stat, directories) ✅ Documented
- [**path**](path.md) - Cross-platform path manipulation

### System
- [**os**](os.md) - Operating system information (platform, CPU, memory, network) ✅ Documented
- [**process**](process.md) - Process control and environment variables ✅ Documented

### Data Structures
- [**table**](table.md) - Table/array utilities
- [**util**](util.md) - General utilities
- [**events**](events.md) - Event emitter pattern

## Module Count: 14

| Module | Functions | Status | Use Case |
|--------|-----------|--------|----------|
| crypto | 13 | ✅ Stable | Security, hashing, passwords |
| string | 17 | ✅ Stable | Text processing |
| time | 17 | ✅ Stable | Timestamps, formatting |
| url | 9 | ✅ Stable | URL operations |
| querystring | 4 | ✅ Stable | Query parsing |
| os | 13 | ✅ Stable | System info |
| process | 8 | ✅ Stable | Environment, args |
| fs | 15+ | ✅ Stable | File operations |
| json | 4 | ✅ Stable | JSON data |
| http | 7+ | ✅ Stable | Web requests |
| path | 10+ | ✅ Stable | Path utils |
| table | 8+ | ✅ Stable | Array/table ops |
| util | 5+ | ✅ Stable | Misc utilities |
| events | 5+ | ✅ Stable | Event handling |

## Documentation Conventions

### Function Signatures
```lua
-- functionName(param1: type, param2?: type) -> returnType
crypto.hash(algorithm: string, data: string) -> string
```

- `?` indicates optional parameter
- `->` indicates return type
- `|` indicates union types (e.g., `string | nil`)

### Examples
All examples are **tested and working**. They can be copied directly into your code.

```lua
-- ✅ This example works
local result = crypto.hash("sha256", "hello")
print(result)  -- Outputs hash

-- ❌ This will error
local result = crypto.hash("invalid_algo", "hello")
```

### Return Values
- Functions return `nil` on error unless otherwise specified
- Some functions throw errors - these are documented
- Check return values for error handling

## Testing Examples

All documentation examples can be tested:

```bash
# Run example validator
cargo run -- docs/api/test-examples.lua

# Run specific module examples
cargo run -- docs/api/examples/crypto-examples.lua
```

## Version Information

- **Lua Version**: 5.4
- **Hype Version**: 0.4.0
- **Standard Library**: Phase 1 Complete ✅
- **Documented Modules**: 8 of 14 (crypto, string, time, url, querystring, os, process, fs, json)

## Next Steps

1. Browse the [module documentation](crypto.md)
2. Check out [examples directory](../examples/)
3. Read the [migration guide](../MIGRATION_HYPE_MODULES.md)
4. See [performance tips](../performance.md)

## Support

- **Issues**: [GitHub Issues](https://github.com/twilson63/hype-rs/issues)
- **Examples**: See `examples/` directory
- **Tests**: See `tests/` directory for comprehensive test coverage

---

**Documentation Last Updated**: October 27, 2025  
**All Examples Tested**: ✅ Yes
