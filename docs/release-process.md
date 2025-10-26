# Release Process

This document describes the release process for hype-rs, including version management, automated builds, and distribution.

## Overview

Hype-rs uses an automated release system powered by GitHub Actions and cargo-dist. When you push a version tag, the system automatically:

1. Builds binaries for all supported platforms
2. Runs the full test suite
3. Creates a GitHub release with release notes
4. Uploads binary artifacts and checksums
5. Makes the release available for download

## Supported Platforms

### Tier 1 (Fully Supported)
- **macOS (Intel)**: `x86_64-apple-darwin`
- **macOS (Apple Silicon)**: `aarch64-apple-darwin`
- **Linux (x86_64)**: `x86_64-unknown-linux-gnu`

### Tier 2 (Supported)
- **Linux (ARM64)**: `aarch64-unknown-linux-gnu`
- **Linux (musl - static)**: `x86_64-unknown-linux-musl`

## Release Workflow

### 1. Prepare for Release

Before creating a release, ensure:

- [ ] All tests pass: `cargo test`
- [ ] Code is formatted: `cargo fmt`
- [ ] No clippy warnings: `cargo clippy -- -D warnings`
- [ ] Documentation is up to date
- [ ] CHANGELOG.md includes all changes

### 2. Version Bump

Update the version in `Cargo.toml`:

```toml
[package]
version = "0.2.0"  # Update this
```

Follow [Semantic Versioning](https://semver.org/):
- **MAJOR** (1.0.0): Breaking changes
- **MINOR** (0.2.0): New features, backward compatible
- **PATCH** (0.1.1): Bug fixes, backward compatible

### 3. Update CHANGELOG

Add release notes to `CHANGELOG.md`:

```markdown
## [0.2.0] - 2025-10-27

### Added
- New feature 1
- New feature 2

### Changed
- Improvement 1
- Improvement 2

### Fixed
- Bug fix 1
- Bug fix 2
```

### 4. Commit and Tag

```bash
# Commit version bump
git add Cargo.toml CHANGELOG.md
git commit -m "chore: bump version to 0.2.0"

# Create annotated tag
git tag -a v0.2.0 -m "Release v0.2.0"

# Push to remote
git push origin master
git push origin v0.2.0
```

**Important**: Always use annotated tags (`-a`) and include the `v` prefix (e.g., `v0.2.0`).

### 5. Monitor GitHub Actions

1. Go to: https://github.com/twilson63/hype-rs/actions
2. Watch the "Release" workflow
3. Ensure all jobs complete successfully:
   - Create release
   - Build artifacts (all platforms)
   - Upload artifacts
   - Publish release

The workflow typically takes 10-15 minutes to complete.

### 6. Verify Release

After the workflow completes:

1. Check the release page: https://github.com/twilson63/hype-rs/releases
2. Verify all artifacts are uploaded:
   - `hype-x86_64-apple-darwin.tar.gz`
   - `hype-aarch64-apple-darwin.tar.gz`
   - `hype-x86_64-unknown-linux-gnu.tar.gz`
   - `hype-x86_64-unknown-linux-musl.tar.gz`
   - `hype-aarch64-unknown-linux-gnu.tar.gz`
   - Checksum files (`.sha256`)
3. Check release notes are generated

### 7. Test Installation

Test the installation on different platforms:

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/twilson63/hype-rs/master/install.sh | sh

# Verify
hype --version
```

### 8. Announce Release (Optional)

- Update project README if needed
- Post to social media or announcement channels
- Notify users of breaking changes

## Release Checklist

Complete checklist for creating a release:

- [ ] Run full test suite: `cargo test`
- [ ] Run linter: `cargo clippy -- -D warnings`
- [ ] Format code: `cargo fmt`
- [ ] Update version in `Cargo.toml`
- [ ] Update `CHANGELOG.md` with release notes
- [ ] Commit changes: `git commit -am "chore: bump version to X.Y.Z"`
- [ ] Create annotated tag: `git tag -a vX.Y.Z -m "Release vX.Y.Z"`
- [ ] Push commits: `git push origin master`
- [ ] Push tag: `git push origin vX.Y.Z`
- [ ] Monitor GitHub Actions workflow
- [ ] Verify release on GitHub
- [ ] Test installation script
- [ ] Update documentation if needed
- [ ] Announce release (if major)

## Hotfix Releases

For urgent bug fixes:

1. Create a hotfix branch from the release tag:
   ```bash
   git checkout -b hotfix/v0.2.1 v0.2.0
   ```

2. Make the fix and commit

3. Update version to patch (0.2.0 → 0.2.1)

4. Tag and push:
   ```bash
   git tag -a v0.2.1 -m "Hotfix v0.2.1"
   git push origin hotfix/v0.2.1
   git push origin v0.2.1
   ```

## Pre-releases

For alpha, beta, or release candidate versions:

1. Use pre-release version format: `0.2.0-alpha.1`, `0.2.0-beta.1`, `0.2.0-rc.1`

2. Tag with pre-release suffix:
   ```bash
   git tag -a v0.2.0-beta.1 -m "Beta release v0.2.0-beta.1"
   git push origin v0.2.0-beta.1
   ```

3. The GitHub release will be marked as a pre-release automatically

## Troubleshooting

### Build Fails for Specific Platform

1. Check the GitHub Actions logs for the specific job
2. Common issues:
   - Cross-compilation tools not installed
   - Target not added to Rust toolchain
   - Platform-specific dependencies missing

### Tag Already Exists

If you need to recreate a tag:

```bash
# Delete local tag
git tag -d v0.2.0

# Delete remote tag
git push origin :refs/tags/v0.2.0

# Recreate and push
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin v0.2.0
```

**Warning**: Only do this if the release hasn't been published yet.

### Release Workflow Doesn't Trigger

Ensure:
- Tag follows the pattern `v*.*.*` (e.g., `v0.2.0`)
- Tag is pushed to the remote repository
- GitHub Actions is enabled for the repository

## Rollback

If a release has critical issues:

1. Mark the release as "pre-release" on GitHub
2. Add a warning to the release notes
3. Create a hotfix and release a new version
4. Update documentation to reference the fixed version

## Version History

| Version | Release Date | Highlights |
|---------|--------------|------------|
| 0.1.0   | 2025-10-26   | Initial release |
| ...     | ...          | ... |

## References

- [Semantic Versioning](https://semver.org/)
- [Keep a Changelog](https://keepachangelog.com/)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [cargo-dist Documentation](https://github.com/axodotdev/cargo-dist)

## Support

For questions about the release process:
- Open a discussion on GitHub
- Check existing documentation in `docs/`
- Contact maintainers via GitHub issues
