# Package App: Complete Application Example

A comprehensive example demonstrating the Hype-RS module system with a complete application structure, including custom modules, file I/O, and real-world usage patterns.

## Overview

This example shows how to:
- Organize code with a proper package structure
- Create and use custom modules
- Compose multiple modules in a single application
- Handle file operations with the `fs` module
- Manipulate paths with the `path` module
- Implement error handling patterns
- Build production-quality Lua applications

## Project Structure

```
package-app/
├── hype.json              # Package manifest
├── app.lua                # Main application entry point
├── modules/
│   ├── math_utils.lua     # Mathematical operations
│   └── string_utils.lua   # String manipulation utilities
└── README.md              # This file
```

## Running the Application

### Using Hype-RS Command Line

```bash
hype examples/package-app/app.lua
```

### Using npm-style scripts (from package-app directory)

```bash
hype app.lua
```

## File Descriptions

### hype.json

The package manifest file contains metadata about the application:

```json
{
  "name": "package-app",
  "version": "1.0.0",
  "description": "A complete application example...",
  "main": "app.lua",
  "dependencies": {},
  "scripts": {
    "start": "hype app.lua"
  }
}
```

**Fields:**
- `name`: Package name (must be unique)
- `version`: Semantic version of the application
- `description`: What the application does
- `main`: Entry point script (executed with hype)
- `dependencies`: External module dependencies
- `scripts`: Shortcut commands (like npm)

### modules/math_utils.lua

Mathematical operations module with 14 functions:

#### Basic Operations
- `add(a, b)` - Addition
- `subtract(a, b)` - Subtraction
- `multiply(a, b)` - Multiplication
- `divide(a, b)` - Division (with zero check)
- `modulo(a, b)` - Remainder (with zero check)

#### Advanced Operations
- `power(base, exponent)` - Exponentiation
- `square_root(n)` - Square root (with validation)
- `absolute(n)` - Absolute value
- `factorial(n)` - Factorial (with integer validation)

#### Multi-value Operations
- `min(...)` - Minimum value
- `max(...)` - Maximum value
- `average(...)` - Average/mean
- `gcd(a, b)` - Greatest common divisor

#### Example Usage

```lua
local math_utils = require("modules/math_utils")

print(math_utils.add(5, 3))           -- 8
print(math_utils.power(2, 8))          -- 256
print(math_utils.min(5, 2, 8, 1))      -- 1
print(math_utils.average(10, 20, 30))  -- 20
```

### modules/string_utils.lua

String manipulation module with 18 functions:

#### Case Operations
- `uppercase(str)` - Convert to uppercase
- `lowercase(str)` - Convert to lowercase
- `capitalize(str)` - Capitalize first letter

#### Text Properties
- `length(str)` - String length
- `word_count(str)` - Count words
- `char_frequency(str)` - Count character frequencies
- `reverse(str)` - Reverse string

#### Text Search
- `contains(str, substring)` - Check if contains substring
- `starts_with(str, prefix)` - Check if starts with prefix
- `ends_with(str, suffix)` - Check if ends with suffix

#### Text Transformation
- `trim(str)` - Remove leading/trailing whitespace
- `ltrim(str)` - Remove leading whitespace
- `rtrim(str)` - Remove trailing whitespace
- `repeat(str, count)` - Repeat string N times
- `replace(str, old, new)` - Replace first occurrence
- `replace_all(str, old, new)` - Replace all occurrences

#### String Operations
- `split(str, delimiter)` - Split string into array
- `join(strings, delimiter)` - Join array into string

#### Example Usage

```lua
local string_utils = require("modules/string_utils")

print(string_utils.uppercase("hello"))              -- HELLO
print(string_utils.capitalize("hello world"))       -- Hello world
print(string_utils.contains("hello", "ell"))        -- true
print(string_utils.word_count("hello world foo"))   -- 3
print(string_utils.split("a,b,c", ","))             -- {a, b, c}
```

### app.lua

The main application that demonstrates:

1. **Module Loading**: Using `require()` to load custom modules
2. **Mathematical Operations**: Performing various calculations
3. **String Operations**: Manipulating and analyzing text
4. **File I/O**: Reading and writing files
5. **Error Handling**: Gracefully handling errors with `pcall()`
6. **Module Composition**: Combining multiple modules in a workflow

## Sample Output

When you run the application, you'll see:

```
═══════════════════════════════════════════════════════════
  Package App: Complete Application Example
═══════════════════════════════════════════════════════════

--- Application Information ---
Directory: /path/to/examples/package-app
Main script: /path/to/examples/package-app/app.lua

--- Module Information ---
Math Utils version: 1.0.0
String Utils version: 1.0.0

--- Part 1: Mathematical Operations ---
...
```

## Key Concepts Demonstrated

### 1. Module Loading with Paths

```lua
local module_dir = path.join(__dirname, "modules")
local math_utils = require(path.join(module_dir, "math_utils"))
```

Shows how to load modules from relative paths within a package.

### 2. Error Handling in Modules

Both modules validate inputs and provide clear error messages:

```lua
-- In math_utils.lua
local function validate_number(value, name)
    if type(value) ~= "number" then
        error(name .. " must be a number")
    end
end
```

### 3. Safe Operations

The app uses `pcall()` to handle errors gracefully:

```lua
local function safe_operation(name, func, ...)
    local ok, result = pcall(func, ...)
    if ok then
        print("  ✓", name .. ":", result)
    else
        print("  ✗", name .. ":", result)
    end
end
```

### 4. File Operations

Demonstrates reading and writing files:

```lua
fs.writeFileSync(data_file, content)
local content = fs.readFileSync(data_file)
```

### 5. Module Composition

Shows how to use multiple modules together:

```lua
local lines = string_utils.split(string_utils.trim(content), "\n")
for i, line in ipairs(lines) do
    local processed = string_utils.lowercase(line)
    local length = string_utils.length(processed)
end
```

## Learning Path

1. **Start here**: Run the application to see output
2. **Explore modules**: Open `modules/math_utils.lua` and `modules/string_utils.lua`
3. **Study the main app**: Review `app.lua` to see how modules are used
4. **Modify and experiment**: Change values, add new functions, test error cases
5. **Create your own**: Build modules for your own applications

## Common Tasks

### Adding a New Function to math_utils

1. Open `modules/math_utils.lua`
2. Add your function to the exported table:

```lua
square = function(n)
    validate_number(n, "n")
    return n * n
end,
```

3. Use it in `app.lua`:

```lua
print("3² =", math_utils.square(3))  -- 9
```

### Adding a New Function to string_utils

1. Open `modules/string_utils.lua`
2. Add your function to the exported table:

```lua
count_occurrences = function(str, substring)
    validate_string(str, "str")
    validate_string(substring, "substring")
    local count = 0
    for _ in str:gmatch(substring:gsub("[%(%)%.%+%-%*%?%[%]%^%$%%]", "%%%0")) do
        count = count + 1
    end
    return count
end,
```

3. Test it in `app.lua`

### Creating a Third Module

1. Create `modules/array_utils.lua`
2. Export functions from it:

```lua
module.exports = {
    length = function(arr) return #arr end,
    first = function(arr) return arr[1] end,
    last = function(arr) return arr[#arr] end,
    contains = function(arr, value)
        for _, v in ipairs(arr) do
            if v == value then return true end
        end
        return false
    end,
}
```

3. Load it in `app.lua`:

```lua
local array_utils = require(path.join(module_dir, "array_utils"))
```

## Best Practices

1. **Validate Inputs**: Check parameter types and values
2. **Clear Error Messages**: Help users understand what went wrong
3. **Organize Modules**: One module per file, organized by function
4. **Document Functions**: Include usage examples
5. **Use Semantic Versioning**: For tracking changes
6. **Test Edge Cases**: Handle zeros, negatives, empty strings, etc.

## Extending This Example

### Add Unit Tests

Create `tests/test_math_utils.lua`:

```lua
local math_utils = require("modules/math_utils")

local tests_passed = 0
local tests_failed = 0

local function test(name, func)
    local ok, result = pcall(func)
    if ok then
        tests_passed = tests_passed + 1
        print("✓", name)
    else
        tests_failed = tests_failed + 1
        print("✗", name, result)
    end
end

test("add(5, 3) == 8", function()
    assert(math_utils.add(5, 3) == 8)
end)
```

### Add Configuration

Create `config.lua`:

```lua
module.exports = {
    app_name = "Package App",
    version = "1.0.0",
    debug = false,
    max_precision = 2,
}
```

### Add Data Processing

Extend the file I/O example to process CSV files, JSON data, etc.

## Troubleshooting

### Module not found error

Make sure you're using the correct relative path:

```lua
-- Wrong:
local math_utils = require("math_utils")

-- Correct:
local math_utils = require(path.join(__dirname, "modules", "math_utils"))
```

### Functions not working

Check that the module is properly exporting:

```lua
-- At the end of your module
module.exports = {
    function_name = function(...) end,
}
```

### File I/O errors

Always check if files exist before reading:

```lua
if fs.existsSync(filepath) then
    local content = fs.readFileSync(filepath)
end
```

## References

- [Hype-RS Module System](../docs/modules/)
- [Built-in Modules](../docs/modules/builtin-modules.md)
- [require() API](../docs/modules/require-api.md)
- [Getting Started with Modules](../docs/modules/getting-started.md)

## License

MIT - Feel free to use this example as a template for your own applications.
