# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.2] - 2025-11-21

### Added
- **Direct File/Directory Fallback for Module Resolution**
  - `require()` now supports loading Lua files and directories directly from the root directory
  - Enables simpler project structures without requiring `hype_modules/` directory
  - Fallback mechanism searches for: `{module_id}.lua`, `{module_id}/index.lua`, `{module_id}/init.lua`, and directories
  - `hype_modules/` maintains priority - direct files are only used as a fallback
  - Fully backward compatible with existing code

### Module Resolution Order (Updated)
1. Built-in modules (fs, path, events, util, table, http)
2. Relative paths (./ or ../)
3. Absolute paths (if allowed)
4. hype_modules directories
5. ~/.hype/modules directories
6. Direct file/directory fallback (NEW)
7. Error if not found

### Examples
```lua
-- Direct .lua file fallback
local foo = require("foo")  -- Loads ./foo.lua

-- Directory with index.lua fallback
local utils = require("utils")  -- Loads ./utils/index.lua

-- Directory with init.lua fallback
local helpers = require("helpers")  -- Loads ./helpers/init.lua
```

## [0.4.1] - 2025-10-27

### Fixed
- **Critical**: HTTP module now loads correctly via `require("http")`
  - HTTP module existed but was missing from the Lua loader function
  - Users can now use all HTTP features: GET, POST, cookies, auth, proxies, forms
  - All HTTP functions now accessible: `http.get()`, `http.post()`, `http.fetch()`, etc.

### Technical Details
- Added `http` case to `load_with_lua()` switch statement in builtin module registry
- Exported `create_http_module()` function from HTTP module
- HTTP module was already compiled in but unusable - now fully functional

## [0.4.0] - 2025-10-27

### Added
- Crypto module for cryptographic operations
  - `crypto.hash(algorithm, data)` - Hash data with SHA256, SHA512, SHA1, or MD5
  - `crypto.hashFile(algorithm, path)` - Hash file contents
  - `crypto.hmac(algorithm, key, data)` - HMAC signing with SHA256, SHA512, or SHA1
  - `crypto.randomBytes(size)` - Generate cryptographically secure random bytes
  - `crypto.randomInt(min, max)` - Generate secure random integer in range
  - `crypto.randomUUID()` - Generate UUID v4
  - `crypto.base64Encode(data)` - Base64 encode string
  - `crypto.base64Decode(data)` - Base64 decode string
  - `crypto.hexEncode(data)` - Hex encode string
  - `crypto.hexDecode(data)` - Hex decode string
  - `crypto.bcrypt(password, cost?)` - Hash password with bcrypt (cost 4-31, default 12)
  - `crypto.bcryptVerify(password, hash)` - Verify password against bcrypt hash
  - `crypto.timingSafeEqual(a, b)` - Constant-time string comparison
  - Full cryptographic hashing support (SHA256, SHA512, SHA1, MD5)
  - Secure random generation using OS entropy
  - Password hashing with bcrypt work factor control
  - HMAC message authentication
  - Encoding/decoding utilities (Base64, Hex)
  - Timing-safe comparison to prevent timing attacks

- QueryString module for query string parsing and formatting
  - `querystring.parse(queryString)` - Parse query string into table of key-value pairs
  - `querystring.stringify(params)` - Convert table of key-value pairs into query string
  - `querystring.escape(string)` - URL-encode string for query strings (form-urlencoded)
  - `querystring.unescape(string)` - Decode URL-encoded query string component
  - Automatic handling of plus signs (+) as spaces
  - Full percent-encoding support
  - Compliant with application/x-www-form-urlencoded format
  - Roundtrip encoding/decoding with special characters

- URL module for URL parsing and manipulation
  - `url.parse(urlString)` - Parse URL into components (protocol, host, port, path, query, hash, auth)
  - `url.format(components)` - Build URL from components object
  - `url.resolve(base, relative)` - Resolve relative URL against base URL
  - `url.encode(string)` - URL encode string (form-urlencoded)
  - `url.decode(string)` - URL decode string
  - `url.encodeComponent(string)` - Encode URL component (percent-encoding)
  - `url.decodeComponent(string)` - Decode URL component
  - `url.parseQuery(queryString)` - Parse query string to table
  - `url.formatQuery(params)` - Format table as query string
  - Full RFC 3986 URL parsing support
  - Query string parsing and building
  - Relative URL resolution
  - Percent-encoding and form-urlencoded support

- Time module for date and time operations
  - `time.now()` - Get current timestamp in milliseconds
  - `time.nowSeconds()` - Get current timestamp in seconds
  - `time.nowNanos()` - Get current timestamp in nanoseconds
  - `time.format(timestamp, format)` - Format timestamp using custom format string
  - `time.parse(dateString, format)` - Parse date string using custom format
  - `time.toISO(timestamp)` - Convert timestamp to ISO 8601 string
  - `time.fromISO(isoString)` - Parse ISO 8601 string to timestamp
  - `time.date(timestamp?)` - Get date components as table (year, month, day, hour, minute, second, weekday)
  - `time.year(timestamp?)` - Get year from timestamp
  - `time.month(timestamp?)` - Get month (1-12) from timestamp
  - `time.day(timestamp?)` - Get day (1-31) from timestamp
  - `time.hour(timestamp?)` - Get hour (0-23) from timestamp
  - `time.minute(timestamp?)` - Get minute (0-59) from timestamp
  - `time.second(timestamp?)` - Get second (0-59) from timestamp
  - `time.sleep(ms)` - Sleep for specified milliseconds
  - `time.elapsed(start)` - Calculate elapsed time since start timestamp
  - `time.duration(ms)` - Format duration in human-readable form
  - ISO 8601 and custom format support via chrono
  - Timestamp arithmetic and date component extraction
  - Human-readable duration formatting (e.g., "1d 2h 30m")

- String module for enhanced string manipulation utilities
  - `string.split(str, delimiter)` - Split string into array by delimiter
  - `string.trim(str)` - Remove whitespace from both ends
  - `string.trimStart(str)` - Remove leading whitespace
  - `string.trimEnd(str)` - Remove trailing whitespace
  - `string.startsWith(str, prefix)` - Check if string starts with prefix
  - `string.endsWith(str, suffix)` - Check if string ends with suffix
  - `string.contains(str, substring)` - Check if string contains substring
  - `string.padStart(str, length, fill?)` - Pad start of string to length
  - `string.padEnd(str, length, fill?)` - Pad end of string to length
  - `string.repeat(str, count)` - Repeat string count times
  - `string.replace(str, pattern, replacement, count?)` - Replace occurrences of pattern
  - `string.replaceAll(str, pattern, replacement)` - Replace all occurrences of pattern
  - `string.toUpperCase(str)` - Convert to uppercase
  - `string.toLowerCase(str)` - Convert to lowercase
  - `string.capitalize(str)` - Capitalize first letter
  - `string.lines(str)` - Split string into lines
  - `string.chars(str)` - Split string into characters
  - Full UTF-8 Unicode support
  - Zero dependencies (pure Rust implementation)

- OS module for operating system information and utilities
  - `os.platform()` - Get operating system platform ("linux", "macos", "windows", "freebsd", "openbsd")
  - `os.arch()` - Get CPU architecture ("x86_64", "aarch64", "arm", "x86")
  - `os.hostname()` - Get system hostname
  - `os.homedir()` - Get user home directory path
  - `os.tmpdir()` - Get system temp directory path
  - `os.cpus()` - Get CPU information (model, speed)
  - `os.totalmem()` - Get total system memory in bytes
  - `os.freemem()` - Get free system memory in bytes
  - `os.uptime()` - Get system uptime in seconds
  - `os.loadavg()` - Get load average [1, 5, 15 minutes] (Unix-like systems)
  - `os.networkInterfaces()` - Get network interfaces information
  - `os.userInfo()` - Get current user information (username, uid, gid, shell, homedir)
  - `os.EOL` - End of line marker for the platform ("\n" or "\r\n")
  - Cross-platform support for all major operating systems
  - Full Unix/Windows compatibility with platform-specific features

- Process module for process and environment management
  - `process.cwd()` - Get current working directory
  - `process.chdir(path)` - Change working directory
  - `process.env` - Environment variables (readable/writable table with metatable)
  - `process.getenv(key)` - Get environment variable
  - `process.setenv(key, value)` - Set environment variable
  - `process.pid` - Process ID (number)
  - `process.platform` - Operating system ("linux", "macos", "windows")
  - `process.arch` - CPU architecture ("x86_64", "aarch64", etc.)
  - `process.argv` - Command-line arguments (table)
  - `process.exit(code?)` - Exit with code (0-255)
  - Cross-platform support for Windows, macOS, Linux

- JSON module for encoding and decoding JSON data
  - `json.encode(value, pretty?)` - Encode Lua value to JSON string
  - `json.decode(jsonString)` - Decode JSON string to Lua value
  - `json.stringify(value, pretty?)` - Alias for encode
  - `json.parse(jsonString)` - Alias for decode
  - Full Unicode support including emojis
  - Pretty-printing support for readable output
  - Comprehensive error handling for invalid JSON

## [0.3.0] - 2025-10-27

### Added
- Filesystem module with 8 core synchronous operations (PRP-013)
  - `fs.readFileSync(path)` - Read UTF-8 text files
  - `fs.writeFileSync(path, data)` - Write UTF-8 text files
  - `fs.existsSync(path)` - Check file/directory existence
  - `fs.statSync(path)` - Get file metadata (size, type, mtime)
  - `fs.readdirSync(path)` - List directory contents (sorted)
  - `fs.unlinkSync(path)` - Delete files
  - `fs.mkdirSync(path)` - Create directories (recursive by default)
  - `fs.rmdirSync(path)` - Remove empty directories
  
### Technical Details
- UTF-8 text file support only (binary support planned for Phase 2)
- Recursive directory creation by default for safety
- Cross-platform path handling using `std::path`
- Comprehensive error handling with `FsError` enum
- Full Lua integration via `require("fs")`
- 24 unit and integration tests

## [0.2.0] - 2025-10-28

### Added
- HTTP module now supports proxy configuration (PRP-012)
  - HTTP and HTTPS proxy via `{proxy = "http://proxy:8080"}`
  - Works with all HTTP methods (GET, POST, PUT, DELETE, PATCH, HEAD, fetch)
  - Simple string URL format for ease of use
  - Per-request proxy override capability
  
- Form data helpers for easier form submission (PRP-012)
  - `http.postForm(url, fields)` for URL-encoded forms
  - `http.uploadFile(url, options)` for file uploads with multipart/form-data
  - Automatic Content-Type header setting
  - Automatic encoding and boundary management
  - Support for additional metadata fields in file uploads
  
- Authentication helpers (PRP-012)
  - Basic Auth: `{auth = {username = "u", password = "p"}}`
  - Bearer Token: `{authToken = "token"}`
  - Automatic Base64 encoding for Basic Auth (RFC 7617 compliant)
  - Automatic Authorization header construction
  - Works with all HTTP methods

### Changed
- All HTTP methods now accept optional options table for proxy and auth
- Added `base64` and `serde_urlencoded` dependencies
- Enabled `multipart` feature in reqwest dependency
- Bumped version from 0.1.4 to 0.2.0

### Technical Details
- Base64 encoding using standard `base64` crate (RFC 4648 compliant)
- Form encoding using `serde_urlencoded` (RFC 1738 compliant)
- Multipart forms using reqwest's built-in multipart support (RFC 7578 compliant)
- Full backward compatibility maintained - all existing code continues to work
- New modules: `auth.rs`, `forms.rs` for better code organization

## [0.1.4] - 2025-10-27

### Added
- HTTP module now includes automatic cookie management (PRP-011)
  - Cookies from `Set-Cookie` headers are automatically stored in a cookie jar
  - Stored cookies are automatically sent with subsequent requests to the same domain
  - RFC 6265 compliant cookie handling (domain, path, expiry, secure flags, HttpOnly, SameSite)
  - New `http.getCookies(url)` function to inspect cookies for a given domain
  - Cookies are properly scoped by domain and path (no cookie leakage)
  - Secure cookies only sent over HTTPS connections
  - Automatic handling of session cookies and expiration
  - Cookies persist across redirects
  - Works seamlessly with all HTTP methods (GET, POST, PUT, DELETE, PATCH, HEAD, fetch)

### Changed
- Updated reqwest dependency to include `cookies` feature flag
- HttpClient now includes Arc<Jar> cookie jar by default
- All HTTP requests automatically participate in cookie management

### Technical Details
- Built on reqwest's battle-tested cookie store implementation
- Zero breaking changes - cookies work automatically for existing code
- Minimal performance overhead (< 1ms per request)
- Full backward compatibility maintained

## [0.1.3] - 2025-10-26

### Fixed
- HTTP module now uses rustls instead of native-tls for better TLS compatibility
  - Fixes connection failures to servers with limited TLS 1.2 cipher suite support
  - Improved TLS 1.3 support and cipher suite negotiation
  - Resolves "handshake failure" errors with certain HTTPS endpoints
  - All HTTPS connections now work consistently across different server configurations

### Changed
- Updated reqwest dependency to use `rustls-tls` feature instead of default `native-tls`
  - Better cross-platform TLS compatibility
  - More predictable TLS behavior across different operating systems

## [0.1.2] - 2025-01-26

### Added
- LLM agent documentation via `hype agent` command (PRP-009)
  - Complete API reference for all built-in modules
  - Machine-readable JSON output optimized for LLM consumption
  - Function signatures, parameters, return types, and examples
  - Best practices and common error patterns

### Fixed
- HTTP module now properly validates and encodes URLs according to RFC 3986 (PRP-010)
  - Added `url` crate dependency for RFC-compliant URL parsing
  - Tilde (~) and other unreserved characters now handled correctly in URL paths
  - Invalid URLs now produce clear, actionable error messages
  - Comprehensive test suite for URL edge cases (34 new tests)
  - All HTTP methods (GET, POST, PUT, DELETE, PATCH, HEAD, fetch) now validate URLs

## [0.1.1] - 2025-01-20

### Added
- Automated release system with GitHub Actions (PRP-008)
- Cross-platform binary builds for macOS (Intel/ARM) and Linux (x86_64/ARM/musl)
- One-command installation script for easy setup
- cargo-dist configuration for streamlined releases
- Release process documentation
- GitHub release notes templates

### BREAKING CHANGES

#### Module Directory Renamed: `node_modules` → `hype_modules`

**What Changed:**
- Module resolution now searches for `hype_modules/` instead of `node_modules/`
- The `~/.hype/modules` directory remains unchanged

**Why:**
- Establishes Hype-RS as an independent Lua runtime ecosystem
- Prevents confusion with Node.js/npm packages
- Aligns with project branding and naming conventions
- Provides clear differentiation from JavaScript runtimes

**Migration:**
Simply rename your module directory:
```bash
mv node_modules hype_modules
```

**For detailed migration steps, see:** [docs/MIGRATION_HYPE_MODULES.md](docs/MIGRATION_HYPE_MODULES.md)

**Rationale:** See [PRPs/hype-modules-rename-prp.md](PRPs/hype-modules-rename-prp.md) for full technical details.

**Impact:**
- All projects must rename `node_modules` to `hype_modules`
- No code changes required - `require()` API remains the same
- Module resolution algorithm unchanged except for directory name

---

## [0.1.0] - Initial Release

### Added
- Core Lua runtime with Lua 5.4 support
- CLI interface with script execution
- Module system with `require()` support
- Built-in modules: fs, path, events, util, table
- Module resolution with directory walking
- Module caching for performance
- Circular dependency detection
- Security sandboxing
- Command-line argument passing
- Environment variable access
- Comprehensive test suite (265 tests, 97% coverage)
- Documentation and examples

### Core Features
- Fast startup time (~50ms for simple scripts)
- Low memory footprint
- Cross-platform support (Windows, macOS, Linux)
- Error handling with meaningful messages
- Timeout support for script execution
- Debug and verbose modes

---

## Version History

| Version | Date | Description |
|---------|------|-------------|
| 0.1.0 | TBD | Initial release with core features |
| Unreleased | TBD | Breaking change: node_modules → hype_modules |

---

## Upgrade Guide

### Upgrading to 0.2.0 (Unreleased)

**Prerequisites:**
- Hype-RS 0.1.0 or later installed
- Project uses `node_modules` for module resolution

**Steps:**
1. Backup your project (optional but recommended)
2. Rename module directory: `mv node_modules hype_modules`
3. Update `.gitignore` if needed
4. Test your application
5. Commit changes

**Estimated Time:** 5 minutes

**No API Changes:** Your Lua code remains exactly the same. Only the filesystem directory name changes.

---

## Support

For issues, questions, or contributions:
- **Issues**: [GitHub Issues](https://github.com/sst/hype-rs/issues)
- **Documentation**: [docs/README.md](docs/README.md)
- **Contributing**: [.github/CONTRIBUTING.md](.github/CONTRIBUTING.md)
