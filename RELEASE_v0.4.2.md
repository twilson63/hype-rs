# Release v0.4.2 - Direct File/Directory Fallback Module Resolution

## Release Summary

**Version:** 0.4.2  
**Release Date:** November 21, 2025  
**GitHub Release:** https://github.com/twilson63/hype-rs/releases/tag/v0.4.2

### Major Feature: Direct File/Directory Fallback Module Resolution

This release adds a new fallback mechanism to the module resolver that enables simpler project structures by allowing `require()` to load Lua files and directories directly from the root directory without requiring them to be in the `hype_modules/` directory.

#### Key Improvements

✨ **Direct File Fallback Support**
- Load `.lua` files directly: `require("foo")` → loads `./foo.lua`
- Load directory modules: `require("utils")` → loads `./utils/index.lua` or `./utils/init.lua`
- Fallback mechanism only activates if module not found in standard locations
- `hype_modules/` maintains priority - fully backward compatible

🎯 **Simplified Module Organization**
- No longer need `hype_modules/` directory for simple projects
- Flexible local module organization without strict hierarchy
- Easier gradual migration from monolithic scripts to modular code

### Testing

✅ 6 new comprehensive tests for fallback behavior  
✅ All 33 resolver tests pass (zero regressions)  
✅ 479+ total tests pass  
✅ Feature fully backward compatible

## Distribution

### Available Downloads

1. **macOS Binary (Intel x86_64)**
   - Archive: `hype-x86_64-apple-darwin.tar.gz` (2.2 MB)
   - Checksum: `hype-x86_64-apple-darwin.tar.gz.sha256`
   - Download: https://github.com/twilson63/hype-rs/releases/download/v0.4.2/hype-x86_64-apple-darwin.tar.gz

### Installation Methods

#### Method 1: Bash Install Script (Recommended)

```bash
curl -fsSL https://raw.githubusercontent.com/twilson63/hype-rs/master/install.sh | sh
```

Or with specific version:
```bash
curl -fsSL https://raw.githubusercontent.com/twilson63/hype-rs/master/install.sh | sh -s -- --version v0.4.2
```

#### Method 2: Homebrew

If you have the Homebrew tap configured:
```bash
brew install twilson63/tap/hype
```

Or upgrade:
```bash
brew upgrade twilson63/tap/hype
```

To add the tap:
```bash
brew tap twilson63/homebrew-tap https://github.com/twilson63/homebrew-tap.git
brew install hype
```

#### Method 3: Manual Download

```bash
# Download
curl -fsSL https://github.com/twilson63/hype-rs/releases/download/v0.4.2/hype-x86_64-apple-darwin.tar.gz -o hype.tar.gz

# Verify checksum (optional)
curl -fsSL https://github.com/twilson63/hype-rs/releases/download/v0.4.2/hype-x86_64-apple-darwin.tar.gz.sha256 -o hype.tar.gz.sha256
sha256sum -c hype-x86_64-apple-darwin.tar.gz.sha256

# Extract
tar xzf hype.tar.gz

# Install
sudo mv hype /usr/local/bin/
chmod +x /usr/local/bin/hype
```

## Homebrew Tap Update

To update the Homebrew tap formula for this release:

### Steps

1. **Clone the Homebrew tap repository** (if it doesn't exist)
   ```bash
   git clone git@github.com:twilson63/homebrew-tap.git
   cd homebrew-tap
   ```

2. **Update the formula** (Formula/hype.rb)
   ```ruby
   class Hype < Formula
     desc "A high-performance Lua scripting engine with CLI interface and package management"
     homepage "https://github.com/twilson63/hype-rs"
     url "https://github.com/twilson63/hype-rs/releases/download/v0.4.2/hype-x86_64-apple-darwin.tar.gz"
     sha256 "9fc16b47f92638c9732557f62565c519bb63056b761888aa43df354203c87728"
     license "MIT OR Apache-2.0"

     def install
       bin.install "hype"
     end

     test do
       assert_match /version/, shell_output("#{bin}/hype --version")
     end
   end
   ```

3. **Get the correct SHA256**
   ```bash
   # From the GitHub release assets
   curl -fsSL https://github.com/twilson63/hype-rs/releases/download/v0.4.2/hype-x86_64-apple-darwin.tar.gz.sha256
   # Or calculate locally
   sha256sum hype-x86_64-apple-darwin.tar.gz | cut -d' ' -f1
   ```

4. **Test the formula**
   ```bash
   brew test-bot --only-formulae Formula/hype.rb
   ```

5. **Commit and push**
   ```bash
   git add Formula/hype.rb
   git commit -m "chore: Bump hype to 0.4.2"
   git push origin master
   ```

### SHA256 Checksum

The official SHA256 for `hype-x86_64-apple-darwin.tar.gz`:

```
9fc16b47f92638c9732557f62565c519bb63056b761888aa43df354203c87728  hype-x86_64-apple-darwin.tar.gz
```

Available at: https://github.com/twilson63/hype-rs/releases/download/v0.4.2/hype-x86_64-apple-darwin.tar.gz.sha256

## Changelog

### Added
- **Direct File/Directory Fallback for Module Resolution**
  - `require()` now supports loading Lua files and directories directly from root
  - Enables simpler project structures without `hype_modules/` directory
  - Fallback mechanism checks for: `.lua` files, `index.lua`, `init.lua`, and directories
  - `hype_modules/` maintains priority over direct files
  - Fully backward compatible with existing code

### Module Resolution Order (Updated)

1. Built-in modules (fs, path, events, util, table, http)
2. Relative paths (./ or ../)
3. Absolute paths (if allowed)
4. hype_modules directories
5. ~/.hype/modules directories
6. **Direct file/directory fallback** ← NEW
7. Error if not found

## Documentation

- See `DIRECT_FILE_FALLBACK.md` for comprehensive feature documentation
- See `CHANGELOG.md` for detailed changelog
- See `examples/direct-file-fallback.lua` for usage examples

## Verification

To verify the installation:

```bash
hype --version
# Output: hype 0.4.2

hype examples/direct-file-fallback.lua
```

## Support

- Report issues: https://github.com/twilson63/hype-rs/issues
- Feature requests: https://github.com/twilson63/hype-rs/discussions
- Documentation: https://github.com/twilson63/hype-rs/tree/master/docs

## Credits

Thank you to all contributors and testers who helped make this release possible!
