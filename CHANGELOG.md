# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
