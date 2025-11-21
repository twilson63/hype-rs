# Homebrew Formula Setup for hype-rs v0.4.2

This document provides comprehensive instructions for setting up and maintaining the Homebrew formula for hype-rs.

## Quick Start (Automated)

We've provided an automated script to handle all Homebrew setup tasks:

```bash
./homebrew-update.sh
```

This script will:
1. Clone the `homebrew-tap` repository (or pull latest)
2. Create/update the formula at `Formula/hype.rb`
3. Verify Ruby syntax
4. Commit changes with proper message
5. Provide instructions for pushing to remote

## Manual Setup (Step-by-Step)

If you prefer to do this manually, follow these steps:

### Step 1: Clone the Homebrew Tap Repository

```bash
git clone git@github.com:twilson63/homebrew-tap.git
cd homebrew-tap
```

### Step 2: Create or Update the Formula

Create the file `Formula/hype.rb`:

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

### Step 3: Verify Formula Syntax

```bash
ruby -c Formula/hype.rb
```

Expected output: `Syntax OK`

### Step 4: Test the Formula (Optional but Recommended)

```bash
# Install locally
brew install --build-from-source ./Formula/hype.rb

# Verify installation
hype --version

# Run formula tests
brew test-bot --only-formulae Formula/hype.rb

# Uninstall after testing
brew uninstall hype
```

### Step 5: Commit Changes

```bash
git add Formula/hype.rb

git commit -m "chore: Bump hype to v0.4.2

- Update URL to v0.4.2 release archive
- Update SHA256 checksum
- New feature: Direct file/directory fallback for module resolver"
```

### Step 6: Push to Remote

```bash
git push origin master
```

## Updating the Formula for Future Releases

When a new version of hype-rs is released, follow this process:

### 1. Get Release Information

```bash
# Navigate to GitHub release page
# https://github.com/twilson63/hype-rs/releases/tag/vX.Y.Z

# Download the binary
curl -fsSL https://github.com/twilson63/hype-rs/releases/download/vX.Y.Z/hype-x86_64-apple-darwin.tar.gz -o hype.tar.gz

# Get SHA256
shasum -a 256 hype.tar.gz
# Or from GitHub release asset:
curl -fsSL https://github.com/twilson63/hype-rs/releases/download/vX.Y.Z/hype-x86_64-apple-darwin.tar.gz.sha256
```

### 2. Update the Formula

```ruby
class Hype < Formula
  desc "A high-performance Lua scripting engine with CLI interface and package management"
  homepage "https://github.com/twilson63/hype-rs"
  url "https://github.com/twilson63/hype-rs/releases/download/vX.Y.Z/hype-x86_64-apple-darwin.tar.gz"
  sha256 "ACTUAL_SHA256_HASH_HERE"
  license "MIT OR Apache-2.0"

  def install
    bin.install "hype"
  end

  test do
    assert_match /version/, shell_output("#{bin}/hype --version")
  end
end
```

Replace:
- `X.Y.Z` with actual version number
- `ACTUAL_SHA256_HASH_HERE` with the actual SHA256 from the release

### 3. Commit and Push

```bash
git add Formula/hype.rb
git commit -m "chore: Bump hype to vX.Y.Z"
git push origin master
```

## Installation Instructions for Users

Once the formula is pushed to the tap, users can install hype with:

### First Time Setup

```bash
# Add the tap
brew tap twilson63/tap https://github.com/twilson63/homebrew-tap.git

# Install
brew install hype
```

### For Existing Users

```bash
# Update tap
brew tap-pin twilson63/tap  # If pinning
brew update

# Upgrade
brew upgrade hype
```

### Direct Installation

```bash
# Install without tap (if formula is in main Homebrew)
brew install twilson63/tap/hype
```

## Troubleshooting

### Formula Syntax Errors

If you get Ruby syntax errors when verifying:

```bash
ruby -c Formula/hype.rb
```

Common issues:
- Missing colons in key-value pairs
- Unclosed strings or blocks
- Incorrect indentation (use 2 spaces)

### Installation Failures

If users encounter issues during installation:

```bash
# Reinstall tap
brew tap --repair

# Clear cache
brew cleanup

# Try installing again
brew install hype
```

### Version Mismatch

If installed version doesn't match expected:

```bash
# Check installed version
hype --version

# Check formula version
cat $(brew --prefix)/Library/Taps/twilson63/homebrew-tap/Formula/hype.rb | grep 'version'

# Update both if needed
brew upgrade hype
```

## Formula Components Explained

```ruby
class Hype < Formula                    # Formula class name (must be PascalCase)
  desc "..."                            # Short description (shown in brew search)
  homepage "..."                        # Homepage URL
  url "..."                             # Direct download URL
  sha256 "..."                          # SHA256 checksum for verification
  license "..."                         # License identifier

  def install                           # Installation method
    bin.install "hype"                  # Install binary to bin directory
  end

  test do                               # Test to verify installation
    assert_match /version/, shell_output("#{bin}/hype --version")
  end
end
```

## Platform Support

Currently, the formula supports:

- ✅ macOS (Intel x86_64)
- ⚠️ macOS (Apple Silicon) - Would require aarch64 build
- ❌ Linux - Would require additional platform builds

### Adding Platform Support

To support additional platforms:

1. Cross-compile hype for each platform
2. Create separate archives for each platform
3. Use conditional installation in the formula:

```ruby
class Hype < Formula
  # ... header info ...

  on_macos do
    on_intel do
      url "...x86_64-apple-darwin..."
      sha256 "..."
    end
    on_arm do
      url "...aarch64-apple-darwin..."
      sha256 "..."
    end
  end

  on_linux do
    url "...x86_64-unknown-linux-gnu..."
    sha256 "..."
  end

  def install
    bin.install "hype"
  end

  test do
    assert_match /version/, shell_output("#{bin}/hype --version")
  end
end
```

## Release Checklist

When releasing a new version of hype-rs:

- [ ] Version bumped in Cargo.toml
- [ ] CHANGELOG.md updated
- [ ] Binaries built and tested
- [ ] GitHub release created with binaries
- [ ] SHA256 checksums generated
- [ ] Homebrew formula updated
- [ ] Homebrew formula tested
- [ ] Changes committed to homebrew-tap
- [ ] Changes pushed to GitHub
- [ ] Announcement ready (if major release)

## Version-Specific Information

### v0.4.2 (Current)

**Release Date:** November 21, 2025

**New Features:**
- Direct file/directory fallback for module resolver
- Simpler project structures without hype_modules/

**Download URL:**
```
https://github.com/twilson63/hype-rs/releases/download/v0.4.2/hype-x86_64-apple-darwin.tar.gz
```

**SHA256:**
```
9fc16b47f92638c9732557f62565c519bb63056b761888aa43df354203c87728
```

## Resources

- Homebrew Formula API: https://docs.brew.sh/Formula-Cookbook
- Homebrew Tap Creation: https://docs.brew.sh/How-to-Create-and-Maintain-a-Tap
- hype-rs GitHub: https://github.com/twilson63/hype-rs
- homebrew-tap Repository: https://github.com/twilson63/homebrew-tap

## Support

For issues with the Homebrew formula:

1. Check formula syntax: `ruby -c Formula/hype.rb`
2. Reinstall: `brew reinstall hype`
3. Report issues: https://github.com/twilson63/hype-rs/issues

For Homebrew general issues:

- Homebrew Documentation: https://brew.sh/
- Homebrew Discourse: https://discourse.brew.sh/
