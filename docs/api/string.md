# string - String Manipulation Utilities

> **Enhanced string operations with full Unicode support for text processing.**

## Table of Contents
- [Import](#import)
- [Splitting & Parsing](#splitting--parsing)
- [Trimming](#trimming)
- [Pattern Matching](#pattern-matching)
- [Padding](#padding)
- [Transformation](#transformation)
- [Case Conversion](#case-conversion)
- [Examples](#examples)

---

## Import

```lua
local string = require("string")
```

**Note:** The string module extends Lua's built-in `string` table with additional functions.

---

## Splitting & Parsing

### string.split(str, delimiter)

Split string into array by delimiter.

**Parameters:**
- `str: string` - String to split
- `delimiter: string` - Delimiter to split on

**Returns:** `table` - Array of substrings (1-indexed)

**Example:**
```lua
local string = require("string")

-- Split by comma
local parts = string.split("a,b,c", ",")
-- {"a", "b", "c"}

-- Split by space
local words = string.split("hello world lua", " ")
-- {"hello", "world", "lua"}

-- Split path
local path_parts = string.split("/home/user/file.txt", "/")
-- {"", "home", "user", "file.txt"}

-- Empty string
local empty = string.split("", ",")
-- {""}

-- No delimiter found
local single = string.split("hello", ",")
-- {"hello"}
```

---

### string.lines(str)

Split string into lines (by `\n` or `\r\n`).

**Parameters:**
- `str: string` - String to split

**Returns:** `table` - Array of lines

**Example:**
```lua
local string = require("string")

local text = "line1\nline2\nline3"
local lines = string.lines(text)
-- {"line1", "line2", "line3"}

-- Windows line endings
local windows = "a\r\nb\r\nc"
local win_lines = string.lines(windows)
-- {"a", "b", "c"}

-- Empty lines preserved
local with_empty = "a\n\nb"
local result = string.lines(with_empty)
-- {"a", "", "b"}
```

---

### string.chars(str)

Split string into individual characters.

**Parameters:**
- `str: string` - String to split

**Returns:** `table` - Array of characters (Unicode-aware)

**Example:**
```lua
local string = require("string")

-- ASCII
local chars = string.chars("hello")
-- {"h", "e", "l", "l", "o"}

-- Unicode
local emoji = string.chars("Hello 🌍")
-- {"H", "e", "l", "l", "o", " ", "🌍"}

-- Count characters
local text = "café"
local count = #string.chars(text)
print(count)  -- 4 (not 5 bytes)
```

---

## Trimming

### string.trim(str)

Remove whitespace from both ends.

**Parameters:**
- `str: string` - String to trim

**Returns:** `string` - Trimmed string

**Example:**
```lua
local string = require("string")

print(string.trim("  hello  "))  -- "hello"
print(string.trim("\t\ntest\n\t"))  -- "test"
print(string.trim("no-trim"))  -- "no-trim"

-- Whitespace: space, tab, newline, carriage return
```

---

### string.trimStart(str)

Remove leading whitespace.

**Parameters:**
- `str: string` - String to trim

**Returns:** `string` - Trimmed string

**Example:**
```lua
local string = require("string")

print(string.trimStart("  hello"))  -- "hello"
print(string.trimStart("  hello  "))  -- "hello  "
```

---

### string.trimEnd(str)

Remove trailing whitespace.

**Parameters:**
- `str: string` - String to trim

**Returns:** `string` - Trimmed string

**Example:**
```lua
local string = require("string")

print(string.trimEnd("hello  "))  -- "hello"
print(string.trimEnd("  hello  "))  -- "  hello"
```

---

## Pattern Matching

### string.startsWith(str, prefix)

Check if string starts with prefix.

**Parameters:**
- `str: string` - String to check
- `prefix: string` - Prefix to match

**Returns:** `boolean` - `true` if starts with prefix

**Example:**
```lua
local string = require("string")

print(string.startsWith("hello world", "hello"))  -- true
print(string.startsWith("hello world", "world"))  -- false
print(string.startsWith("", ""))  -- true

-- Case sensitive
print(string.startsWith("Hello", "hello"))  -- false

-- Use cases
if string.startsWith(url, "https://") then
    print("Secure URL")
end
```

---

### string.endsWith(str, suffix)

Check if string ends with suffix.

**Parameters:**
- `str: string` - String to check
- `suffix: string` - Suffix to match

**Returns:** `boolean` - `true` if ends with suffix

**Example:**
```lua
local string = require("string")

print(string.endsWith("file.txt", ".txt"))  -- true
print(string.endsWith("file.txt", ".md"))  -- false

-- File extension check
if string.endsWith(filename, ".lua") then
    print("Lua file")
end
```

---

### string.contains(str, substring)

Check if string contains substring.

**Parameters:**
- `str: string` - String to search
- `substring: string` - Substring to find

**Returns:** `boolean` - `true` if substring found

**Example:**
```lua
local string = require("string")

print(string.contains("hello world", "world"))  -- true
print(string.contains("hello world", "xyz"))  -- false

-- Search in content
if string.contains(email, "@") then
    print("Valid email format")
end
```

---

## Padding

### string.padStart(str, length, fill?)

Pad start of string to target length.

**Parameters:**
- `str: string` - String to pad
- `length: number` - Target length
- `fill?: string` - Fill character (default: `" "`)

**Returns:** `string` - Padded string

**Example:**
```lua
local string = require("string")

-- Space padding
print(string.padStart("5", 3))  -- "  5"
print(string.padStart("42", 5))  -- "   42"

-- Zero padding
print(string.padStart("5", 3, "0"))  -- "005"
print(string.padStart("42", 4, "0"))  -- "0042"

-- Custom padding
print(string.padStart("text", 8, "-"))  -- "----text"

-- Already long enough
print(string.padStart("hello", 3))  -- "hello"
```

---

### string.padEnd(str, length, fill?)

Pad end of string to target length.

**Parameters:**
- `str: string` - String to pad
- `length: number` - Target length
- `fill?: string` - Fill character (default: `" "`)

**Returns:** `string` - Padded string

**Example:**
```lua
local string = require("string")

-- Space padding
print(string.padEnd("5", 3))  -- "5  "

-- Custom padding
print(string.padEnd("Name", 10, "."))  -- "Name......"

-- Table formatting
local function format_row(name, value)
    return string.padEnd(name, 20) .. value
end
```

---

## Transformation

### string.repeat(str, count)

Repeat string count times.

**Parameters:**
- `str: string` - String to repeat
- `count: number` - Number of repetitions

**Returns:** `string` - Repeated string

**Example:**
```lua
local string = require("string")

print(string.repeat("*", 10))  -- "**********"
print(string.repeat("abc", 3))  -- "abcabcabc"
print(string.repeat("x", 0))  -- ""

-- Draw line
local line = string.repeat("-", 50)

-- Pattern
local pattern = string.repeat("ab", 5)  -- "ababababab"
```

---

### string.replace(str, pattern, replacement, count?)

Replace occurrences of pattern (literal string).

**Parameters:**
- `str: string` - String to search
- `pattern: string` - Literal string to replace
- `replacement: string` - Replacement string
- `count?: number` - Max replacements (default: 1)

**Returns:** `string` - Modified string

**Example:**
```lua
local string = require("string")

-- Replace first occurrence
local text = "hello world hello"
print(string.replace(text, "hello", "hi"))
-- "hi world hello"

-- Replace multiple
print(string.replace(text, "hello", "hi", 2))
-- "hi world hi"

-- Replace none (pattern not found)
print(string.replace("abc", "xyz", "123"))  -- "abc"

-- Case sensitive
print(string.replace("Hello", "hello", "hi"))  -- "Hello"
```

---

### string.replaceAll(str, pattern, replacement)

Replace all occurrences of pattern (literal string).

**Parameters:**
- `str: string` - String to search
- `pattern: string` - Literal string to replace
- `replacement: string` - Replacement string

**Returns:** `string` - Modified string

**Example:**
```lua
local string = require("string")

-- Replace all
local text = "foo bar foo baz foo"
print(string.replaceAll(text, "foo", "qux"))
-- "qux bar qux baz qux"

-- Remove characters
print(string.replaceAll("a-b-c-d", "-", ""))  -- "abcd"

-- Sanitize
local safe = string.replaceAll(user_input, "<", "&lt;")
```

---

## Case Conversion

### string.toUpperCase(str)

Convert string to uppercase.

**Parameters:**
- `str: string` - String to convert

**Returns:** `string` - Uppercase string

**Example:**
```lua
local string = require("string")

print(string.toUpperCase("hello"))  -- "HELLO"
print(string.toUpperCase("Hello World"))  -- "HELLO WORLD"

-- Unicode support
print(string.toUpperCase("café"))  -- "CAFÉ"
```

---

### string.toLowerCase(str)

Convert string to lowercase.

**Parameters:**
- `str: string` - String to convert

**Returns:** `string` - Lowercase string

**Example:**
```lua
local string = require("string")

print(string.toLowerCase("HELLO"))  -- "hello"
print(string.toLowerCase("Hello World"))  -- "hello world"

-- Case-insensitive comparison
local a = string.toLowerCase(str1)
local b = string.toLowerCase(str2)
if a == b then
    print("Equal (case-insensitive)")
end
```

---

### string.capitalize(str)

Capitalize first letter, lowercase rest.

**Parameters:**
- `str: string` - String to capitalize

**Returns:** `string` - Capitalized string

**Example:**
```lua
local string = require("string")

print(string.capitalize("hello"))  -- "Hello"
print(string.capitalize("HELLO"))  -- "Hello"
print(string.capitalize("hello world"))  -- "Hello world"

-- Format names
local name = string.capitalize("john")  -- "John"

-- Empty/single char
print(string.capitalize(""))  -- ""
print(string.capitalize("a"))  -- "A"
```

---

## Examples

### Text Processing

```lua
local string = require("string")

-- Parse CSV line
local csv = "John,Doe,30,Engineer"
local fields = string.split(csv, ",")
print("First name:", fields[1])  -- "John"

-- Clean user input
local input = "  User Input  \n\t"
local clean = string.trim(input)

-- Format output
local function format_table(rows)
    for _, row in ipairs(rows) do
        local formatted = string.padEnd(row.name, 20) .. 
                          string.padStart(tostring(row.value), 10)
        print(formatted)
    end
end
```

### String Validation

```lua
local string = require("string")

-- Email validation (basic)
function is_email(str)
    return string.contains(str, "@") and 
           string.contains(str, ".")
end

-- URL validation
function is_https(url)
    return string.startsWith(url, "https://")
end

-- File extension
function get_extension(filename)
    local parts = string.split(filename, ".")
    return parts[#parts]
end
```

### Text Transformation

```lua
local string = require("string")

-- Slugify
function slugify(text)
    local lower = string.toLowerCase(text)
    local slug = string.replaceAll(lower, " ", "-")
    return slug
end

-- Title case (simple)
function title_case(text)
    local words = string.split(text, " ")
    local result = {}
    for _, word in ipairs(words) do
        table.insert(result, string.capitalize(word))
    end
    return table.concat(result, " ")
end

print(title_case("hello world"))  -- "Hello World"
```

### Data Formatting

```lua
local string = require("string")

-- Format currency
function format_currency(amount)
    local str = tostring(amount)
    return "$" .. string.padStart(str, 8, " ")
end

-- Format table
function format_row(cols, widths)
    local parts = {}
    for i, col in ipairs(cols) do
        table.insert(parts, string.padEnd(col, widths[i]))
    end
    return table.concat(parts, " | ")
end

-- Progress bar
function progress_bar(percent, width)
    local filled = math.floor(width * percent / 100)
    local bar = string.repeat("█", filled) .. 
                string.repeat("░", width - filled)
    return bar .. " " .. percent .. "%"
end

print(progress_bar(75, 20))
-- "███████████████░░░░░ 75%"
```

---

## Performance Notes

- All operations are Unicode-aware (UTF-8)
- Pure Rust implementation (zero dependencies)
- Operations are O(n) or better
- No regex engine (use `string.match` for patterns)
- Splitting creates new strings (not views)

---

## Comparison with Lua Built-in

| Operation | Built-in | Enhanced |
|-----------|----------|----------|
| Split | `string.gmatch` | `string.split` ✅ |
| Trim | Manual | `string.trim` ✅ |
| Case | `string.upper` | `toUpperCase` (alias) |
| Starts with | `string.sub` | `string.startsWith` ✅ |
| Contains | `string.find` | `string.contains` ✅ |
| Padding | Manual | `padStart/padEnd` ✅ |
| Replace | `string.gsub` | `replace/replaceAll` ✅ |

---

## Error Handling

```lua
-- Empty delimiter
local ok, err = pcall(function()
    return string.split("abc", "")
end)
-- Works, returns single element table

-- Invalid types automatically coerced or error
local result = string.trim(123)  -- Error: expected string
```

---

## See Also

- [Examples](../../examples/string-demo.lua) - More examples
- [Tests](../../tests/string_module_test.rs) - Test suite
- [Lua Built-in](https://www.lua.org/manual/5.4/manual.html#6.4) - Standard string library

---

**Module**: string  
**Functions**: 17  
**Status**: ✅ Production Ready  
**Last Updated**: October 27, 2025
