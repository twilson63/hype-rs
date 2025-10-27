# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
