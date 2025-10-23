# Hype-RS: A Lua Runtime Written in Rust

A fast, lightweight command-line Lua runtime written in Rust that enables you to execute Lua scripts from the terminal. Hype-RS serves as the foundation for a Lua runtime ecosystem, starting with robust script execution capabilities.

## Features

✨ **Core Features**
- Execute Lua scripts from the command line
- Pass arguments to Lua scripts
- Access environment variables from within scripts
- Comprehensive error handling with meaningful messages
- Security sandboxing to prevent dangerous operations
- Cross-platform support (Windows, macOS, Linux)

🚀 **Performance**
- Fast startup time (~50ms for simple scripts)
- Low memory footprint
- Optimized bytecode execution
- Efficient resource management

🔒 **Security**
- Memory limits to prevent resource exhaustion
- Instruction limits to prevent infinite loops
- Sandboxing to restrict file and OS operations
- Safe execution environment for untrusted scripts

## Installation

### From Cargo

```bash
cargo install hype-rs
```

### Build from Source

```bash
git clone https://github.com/your-org/hype-rs
cd hype-rs
cargo build --release
./target/release/hype --version
```

## Usage

### Basic Script Execution

Create a Lua script (`hello.lua`):

```lua
print("Hello, World!")
```

Run it with Hype-RS:

```bash
hype hello.lua
```

Output:
```
Hello, World!
```

### Passing Arguments to Scripts

Create a script (`greet.lua`):

```lua
local name = arg[1] or "World"
print("Hello, " .. name .. "!")
```

Run with arguments:

```bash
hype greet.lua Alice
```

Output:
```
Hello, Alice!
```

### Accessing Environment Variables

Create a script (`env_example.lua`):

```lua
local home = os.getenv("HOME")
print("Home directory: " .. home)

-- Access via env global table
if env then
    print("PATH: " .. env.PATH)
end
```

Run:

```bash
hype env_example.lua
```

### Command-Line Options

```bash
# Show help
hype --help

# Show version
hype --version

# Enable verbose output
hype --verbose script.lua

# Enable debug mode
hype --debug script.lua

# Set execution timeout (in seconds)
hype --timeout 30 long_running_script.lua

# Combine flags
hype --verbose --debug --timeout 60 script.lua
```

## CLI Reference

```
USAGE:
    hype [OPTIONS] <SCRIPT> [ARGS]...

ARGUMENTS:
    <SCRIPT>      Path to the Lua script to execute
    [ARGS]...     Arguments to pass to the Lua script

OPTIONS:
    -v, --verbose    Enable verbose output
    --debug          Enable debug mode
    --timeout <SEC>  Set execution timeout in seconds
    -h, --help       Print help information
    -V, --version    Print version information
```

## Script Arguments

Arguments passed on the command line are available in the Lua script:

- `arg[0]` - Script name
- `arg[1]`, `arg[2]`, ... - Script arguments (positive indices)
- `arg[-1]`, `arg[-2]`, ... - Program name and options (negative indices)

Example (`args_test.lua`):

```lua
print("Script: " .. arg[0])
print("Number of arguments: " .. (#arg - 1))

for i = 1, #arg do
    print("arg[" .. i .. "] = " .. arg[i])
end
```

Run:

```bash
hype args_test.lua foo bar baz
```

Output:

```
Script: args_test.lua
Number of arguments: 3
arg[1] = foo
arg[2] = bar
arg[3] = baz
```

## Supported Lua Features

### Standard Libraries

Hype-RS includes support for the following Lua standard libraries:

- **base** - Basic functions (print, tostring, tonumber, etc.)
- **string** - String manipulation (string.sub, string.format, etc.)
- **table** - Table operations (table.insert, table.concat, etc.)
- **math** - Mathematical functions (math.sin, math.sqrt, etc.)
- **io** - File I/O operations (io.open, io.read, io.write)
- **os** - Operating system interface (limited for security)

### Language Features

- Variables and assignments
- Functions and closures
- Tables and metatables
- Control flow (if/else, for, while, repeat)
- Operators (arithmetic, comparison, logical)
- String concatenation and formatting
- Coroutines (basic support)

### Limitations

For security reasons, the following are restricted:

- `os.execute()` - Shell command execution
- `os.system()` - System calls
- `dofile()` - Loading external files
- `load()` - Runtime code loading
- `package.loadlib()` - Dynamic library loading
- `debug.getinfo()` - Debug introspection (in sandboxed mode)

## Examples

### Example 1: Simple Calculator

Create `calc.lua`:

```lua
local a = tonumber(arg[1]) or 0
local b = tonumber(arg[2]) or 0
local op = arg[3] or "+"

local result
if op == "+" then result = a + b
elseif op == "-" then result = a - b
elseif op == "*" then result = a * b
elseif op == "/" then result = a / b
else result = 0 end

print(a .. " " .. op .. " " .. b .. " = " .. result)
```

Run:

```bash
hype calc.lua 10 5 "*"
```

### Example 2: File Processing

Create `process_files.lua`:

```lua
local pattern = arg[1] or "*.txt"
print("Processing files matching: " .. pattern)

-- Note: Some file operations may be restricted by security settings
for file in io.popen("ls " .. pattern):lines() do
    print("Found: " .. file)
end
```

### Example 3: Configuration Processor

Create `config.lua`:

```lua
local config = {
    app_name = "MyApp",
    version = "1.0.0",
    debug = true,
}

function config:print_info()
    print("Application: " .. self.app_name)
    print("Version: " .. self.version)
    print("Debug mode: " .. tostring(self.debug))
end

config:print_info()
```

Run:

```bash
hype config.lua
```

## Performance Benchmarks

Typical startup times on modern hardware:

- Simple print statement: ~50-100ms
- Script with 1000 iterations: ~150-200ms
- Complex calculation (Fibonacci): ~300-500ms

Memory usage:

- Idle Lua state: ~5-10MB
- Simple script execution: ~10-20MB
- Complex script with tables: ~50-100MB

*Note: Benchmarks vary based on system configuration and script complexity.*

## Architecture

```
hype-rs/
├── src/
│   ├── main.rs              # Entry point and CLI routing
│   ├── cli/                 # Command-line interface
│   │   ├── mod.rs
│   │   ├── parser.rs        # Argument parsing with clap
│   │   └── commands.rs      # CLI command handlers
│   ├── lua/                 # Lua runtime integration
│   │   ├── mod.rs
│   │   ├── state.rs         # Lua state management
│   │   ├── error.rs         # Error types and handling
│   │   ├── security.rs      # Security sandbox
│   │   ├── lifecycle.rs     # State lifecycle
│   │   ├── env.rs           # Environment variables
│   │   ├── path.rs          # Path resolution
│   │   └── debug.rs         # Debug support
│   ├── engine/              # Execution engine
│   │   ├── executor.rs      # Script execution orchestrator
│   │   ├── output.rs        # Output capture
│   │   ├── timeout.rs       # Timeout handling
│   │   └── stats.rs         # Execution statistics
│   ├── error/               # Error handling
│   │   └── mod.rs
│   └── file_io/             # File I/O operations
│       └── mod.rs
├── Cargo.toml               # Project manifest
└── README.md                # This file
```

## Contributing

We welcome contributions! See [CONTRIBUTING.md](./.github/CONTRIBUTING.md) for detailed guidelines on:

- Setting up your development environment
- Code style and conventions
- Testing requirements
- Submitting pull requests
- Reporting bugs and requesting features

## Documentation

Comprehensive documentation is available in the [docs/](./docs/) directory:

- **[Module System Guide](./docs/modules/)** - Learn how to use the module system with `require()`
- **[Getting Started with Modules](./docs/modules/getting-started.md)** - Quick examples and tutorials
- **[require() API Reference](./docs/modules/require-api.md)** - Full API documentation
- **[Built-in Modules](./docs/modules/builtin-modules.md)** - Available modules (fs, path, events, util, table)
- **[Testing Guide](./docs/testing.md)** - How to run tests and test coverage information
- **[Performance Benchmarks](./docs/performance.md)** - Performance metrics and optimization details
- **[Contributing Guide](./.github/CONTRIBUTING.md)** - Guidelines for contributing to the project

See [docs/README.md](./docs/README.md) for the complete documentation hub.

## Roadmap

### Completed Features ✅

- [x] Module system (`require()` support) - Phase 4 Complete
- [x] Built-in modules (fs, path, events, util, table)
- [x] Module caching and resolution
- [x] Comprehensive testing (265 tests, 97%+ coverage)
- [x] Performance benchmarking

### Upcoming Features

- [ ] Full standard library implementation
- [ ] Debugging support (breakpoints, stepping)
- [ ] REPL (interactive mode)
- [ ] Package manager integration
- [ ] Performance optimizations
- [ ] C API for embedding

### Future Enhancements

- Multi-script execution
- Script caching and precompilation
- Remote script execution
- Hot reloading
- Real-time profiling
- Custom module system

## Troubleshooting

### Script Not Found

```bash
$ hype nonexistent.lua
Error: Script not found: nonexistent.lua
```

**Solution**: Ensure the script file exists in the current directory or provide the full path.

### Permission Denied

```bash
$ hype restricted.lua
Error: Permission denied reading file: restricted.lua
```

**Solution**: Check file permissions and ensure the user has read access.

### Timeout Error

```bash
$ hype --timeout 5 long_script.lua
Error: Script execution timed out after Duration { secs: 5, nanos: 0 }
```

**Solution**: Increase the timeout value or optimize the script to run faster.

### Memory Limit Exceeded

```bash
$ hype memory_intensive.lua
Error: Memory limit exceeded
```

**Solution**: Review the script for memory leaks or increase the memory limit in configuration.

## Performance Tips

1. **Use tables efficiently**: Minimize table operations in tight loops
2. **Cache string operations**: String concatenation is expensive; use table.concat()
3. **Avoid excessive function calls**: Function calls have overhead; consider inlining
4. **Pre-compile calculations**: Calculate once, store in variables
5. **Use local variables**: Local variables are faster than globals

## Security Considerations

Hype-RS provides a sandboxed environment for script execution:

1. **File system access** is restricted to safe operations
2. **OS commands** cannot be executed directly
3. **Memory and CPU** usage are limited
4. **External code loading** is disabled by default

For untrusted scripts, consider:

- Running in a containerized environment
- Setting strict resource limits
- Using the `--timeout` flag
- Monitoring resource usage

## License

Hype-RS is dual-licensed under:

- Apache License 2.0
- MIT License

You may choose either license for your use of this software.

## Support

- **GitHub Issues**: Report bugs and request features
- **Documentation**: See [docs/README.md](./docs/README.md) for full documentation
- **Contributing**: See [CONTRIBUTING.md](./.github/CONTRIBUTING.md) to contribute
- **Examples**: Browse example scripts in the `examples/` directory

## Changelog

### v0.1.0 (Initial Release)

- Basic script execution
- Command-line argument passing
- Error handling and reporting
- Environment variable access
- Security sandboxing
- Cross-platform support

---

**Built with ❤️ in Rust** | [GitHub](https://github.com/your-org/hype-rs) | [Documentation](https://docs.hype-rs.dev)
