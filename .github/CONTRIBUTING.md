# Contributing to hype-rs

Thank you for your interest in contributing to hype-rs! This document provides guidelines and instructions for contributing.

## Code of Conduct

Be respectful and constructive in all interactions. We're building a welcoming community for all contributors.

## Getting Started

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Cargo (comes with Rust)
- Git

### Setup

1. Fork the repository on GitHub
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/hype-rs.git
   cd hype-rs
   ```
3. Add upstream remote:
   ```bash
   git remote add upstream https://github.com/sst/hype-rs.git
   ```

### Building & Testing

```bash
cargo build              # Debug build
cargo build --release   # Release build
cargo test              # Run all tests
cargo clippy            # Lint with Clippy
cargo fmt               # Format code
```

## Making Changes

### Code Style

- Run `cargo fmt` before committing to ensure consistent formatting
- Run `cargo clippy -- -D warnings` to check for lints
- Follow Rust naming conventions:
  - Functions: `snake_case`
  - Types/Structs/Enums: `PascalCase`
  - Constants: `SCREAMING_SNAKE_CASE`
- Use explicit type annotations for function parameters and return types
- Prefer strong typing with custom error types (`HypeError`, `Result<T>`)

### Module Organization

The project uses domain-driven module organization:

```
src/
├── cli/           # Command-line interface
├── lua/           # Lua integration
├── modules/       # Module system
├── engine/        # Execution engine
├── file_io/       # File operations
└── error/         # Error types
```

Keep modules focused and single-responsibility. See existing modules for patterns.

### Testing

- Write tests for new functionality
- Tests should be in the same file or in `tests/` directory
- Run `cargo test` to ensure all tests pass
- Aim for high coverage of critical paths

Test organization:
- Unit tests: In the module they test, use `#[cfg(test)]`
- Integration tests: In `tests/` directory
- Benchmarks: In `benches/` directory using Criterion

### Documentation

- Document public APIs with doc comments (`///`)
- Include examples in doc comments where helpful
- Keep [docs/](../docs/) files updated for significant changes
- Update [docs/README.md](../docs/README.md) if adding new documentation

## Submitting Changes

### Commit Messages

Write clear, descriptive commit messages:

```
Add module caching support to improve load performance

- Implement cache layer in ModuleLoader
- Add LRU eviction policy (default 100 modules)
- Update tests to verify caching behavior
```

Guidelines:
- First line: 50 characters max, clear summary
- Use imperative mood ("Add" not "Added")
- Reference issues with `Fixes #123` or `Closes #123`
- Blank line before body (if needed)

### Creating a Pull Request

1. Create a feature branch: `git checkout -b feature/description`
2. Make your changes and commit
3. Keep commits focused and logical
4. Push to your fork: `git push origin feature/description`
5. Create a PR on GitHub with:
   - Clear title describing the change
   - Description of what and why
   - References to related issues
   - List of any breaking changes

### Review Process

- At least one maintainer review required
- All CI checks must pass (tests, linting, formatting)
- Be open to feedback and discuss suggestions
- Updates may be requested before merging

## Types of Contributions

### Bug Reports

- Use GitHub Issues to report bugs
- Include: steps to reproduce, expected behavior, actual behavior
- Attach error messages or stack traces if applicable
- Environment: Rust version, OS, architecture

### Feature Requests

- Use GitHub Issues for feature discussions
- Explain the use case and motivation
- Discuss implementation approach before major work
- Consider backward compatibility

### Documentation

- Fix typos and improve clarity
- Add examples or clarifications
- Update for API changes
- Add architecture documentation

## Project Structure

See [docs/README.md](../docs/README.md) for complete documentation structure.

Key files:
- `README.md` - Project overview
- `VISION.md` - Project vision and goals
- `ROADMAP.md` - Future development plans
- `docs/modules/` - Module system documentation
- `docs/testing.md` - Testing guide
- `docs/performance.md` - Performance information

## Questions?

- Check existing documentation in [docs/](../docs/)
- Search closed issues for similar questions
- Open a discussion issue for questions
- Contact maintainers if stuck

## License

By contributing, you agree that your contributions will be licensed under the same license as the project.

---

Thank you for contributing to hype-rs! 🎉
