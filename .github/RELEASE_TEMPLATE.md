# Release vX.Y.Z

## What's Changed

### ✨ Features
- New feature 1 description
- New feature 2 description

### 🐛 Bug Fixes
- Bug fix 1 description
- Bug fix 2 description

### 📚 Documentation
- Documentation improvement 1
- Documentation improvement 2

### 🔧 Internal Changes
- Internal refactoring or improvements
- Dependency updates

### ⚠️ Breaking Changes
(If any - remove this section if none)
- Breaking change 1 with migration instructions
- Breaking change 2 with migration instructions

## Installation

### Quick Install (Recommended)

**macOS and Linux:**
```bash
curl -fsSL https://raw.githubusercontent.com/twilson63/hype-rs/master/install.sh | sh
```

### Install Specific Version

```bash
curl -fsSL https://raw.githubusercontent.com/twilson63/hype-rs/master/install.sh | sh -s -- --version vX.Y.Z
```

### From Cargo

```bash
cargo install hype-rs --version X.Y.Z
```

### Download Binaries

Download pre-built binaries for your platform from the assets below.

## Supported Platforms

- macOS (Intel): `hype-x86_64-apple-darwin.tar.gz`
- macOS (Apple Silicon): `hype-aarch64-apple-darwin.tar.gz`
- Linux (x86_64): `hype-x86_64-unknown-linux-gnu.tar.gz`
- Linux (x86_64, static): `hype-x86_64-unknown-linux-musl.tar.gz`
- Linux (ARM64): `hype-aarch64-unknown-linux-gnu.tar.gz`

## Checksums

SHA256 checksums are provided for all binaries (`.sha256` files).

Verify your download:
```bash
shasum -a 256 -c hype-<platform>.tar.gz.sha256
```

## Upgrading

If you installed via the install script:
```bash
curl -fsSL https://raw.githubusercontent.com/twilson63/hype-rs/master/install.sh | sh
```

If you installed via Cargo:
```bash
cargo install hype-rs --force
```

## Full Changelog

**Full Changelog**: https://github.com/twilson63/hype-rs/compare/vX.Y.Z-1...vX.Y.Z

## Contributors

Thank you to everyone who contributed to this release! 🎉

---

**Questions or Issues?**
- 📖 [Documentation](https://github.com/twilson63/hype-rs/tree/master/docs)
- 🐛 [Report a Bug](https://github.com/twilson63/hype-rs/issues/new)
- 💬 [Discussions](https://github.com/twilson63/hype-rs/discussions)
