-- Package App: Complete Application Example
-- Demonstrates module loading, composition, and real-world usage patterns
-- Run with:
--   hype --module examples/package-app/app.lua

local fs = require("fs")
local path = require("path")

print("═══════════════════════════════════════════════════════════")
print("  Package App: Complete Application Example")
print("═══════════════════════════════════════════════════════════\n")

print("--- Application Information ---")
print("Directory:", __dirname)
print("Main script:", __filename)
print()

print("--- Module Information ---")
print("Application package: package-app")
print("Main entry point: app.lua")
print("Module directory: ./modules/")
print()

print("--- Note on Custom Modules ---")
print("This example demonstrates how to structure and use custom")
print("modules within a Hype package. The ./modules/ directory")
print("contains reusable library code (math_utils, string_utils)")
print("that would be loaded with require() in a production setup.")
print()

local math_utils = {
    version = "1.0.0",
    
    add = function(a, b) return a + b end,
    subtract = function(a, b) return a - b end,
    multiply = function(a, b) return a * b end,
    divide = function(a, b)
        if b == 0 then error("division by zero") end
        return a / b
    end,
    power = function(a, b) return a ^ b end,
    square_root = function(n) return n ^ 0.5 end,
    modulo = function(a, b) return a % b end,
    absolute = function(n) return n < 0 and -n or n end,
    min = function(...) 
        local args = {...}
        local result = args[1]
        for i = 2, #args do
            if args[i] < result then result = args[i] end
        end
        return result
    end,
    max = function(...)
        local args = {...}
        local result = args[1]
        for i = 2, #args do
            if args[i] > result then result = args[i] end
        end
        return result
    end,
    average = function(...)
        local args = {...}
        local sum = 0
        for _, v in ipairs(args) do sum = sum + v end
        return sum / #args
    end,
}

local string_utils = {
    version = "1.0.0",
    
    uppercase = function(s) return s:upper() end,
    lowercase = function(s) return s:lower() end,
    capitalize = function(s)
        if #s == 0 then return s end
        return s:sub(1,1):upper() .. s:sub(2):lower()
    end,
    reverse = function(s) return s:reverse() end,
    length = function(s) return #s end,
    trim = function(s)
        return s:match("^%s*(.-)%s*$") or s
    end,
    ltrim = function(s)
        return s:match("^%s*(.*)$") or s
    end,
    rtrim = function(s)
        return s:match("^(.-)%s*$") or s
    end,
    contains = function(s, sub) return s:find(sub) ~= nil end,
    starts_with = function(s, prefix)
        return s:sub(1, #prefix) == prefix
    end,
    ends_with = function(s, suffix)
        return #suffix <= #s and s:sub(-#suffix) == suffix
    end,
    replicate = function(s, count)
        local result = ""
        for _ = 1, count do result = result .. s end
        return result
    end,
    split = function(s, delim)
        local result = {}
        local start = 1
        while true do
            local found = s:find(delim, start, true)
            if not found then
                result[#result + 1] = s:sub(start)
                break
            end
            result[#result + 1] = s:sub(start, found - 1)
            start = found + #delim
        end
        return result
    end,
    join = function(t, delim)
        local result = ""
        for i, v in ipairs(t) do
            if i > 1 then result = result .. delim end
            result = result .. v
        end
        return result
    end,
    word_count = function(s)
        local count = 0
        for _ in s:gmatch("%S+") do count = count + 1 end
        return count
    end,
    char_frequency = function(s)
        local freq = {}
        for i = 1, #s do
            local c = s:sub(i, i)
            freq[c] = (freq[c] or 0) + 1
        end
        return freq
    end,
}

print("✓ Math Utils module loaded")
print("✓ String Utils module loaded")
print()

print("--- Part 1: Mathematical Operations ---\n")

print("Basic operations:")
print("  5 + 3 =", math_utils.add(5, 3))
print("  10 - 4 =", math_utils.subtract(10, 4))
print("  6 * 7 =", math_utils.multiply(6, 7))
print("  20 / 4 =", math_utils.divide(20, 4))
print()

print("Advanced operations:")
print("  2^8 =", math_utils.power(2, 8))
print("  √144 =", math_utils.square_root(144))
print("  |(-15)| =", math_utils.absolute(-15))
print("  17 mod 5 =", math_utils.modulo(17, 5))
print()

print("Multi-value operations:")
print("  min(5, 2, 8, 1) =", math_utils.min(5, 2, 8, 1))
print("  max(5, 2, 8, 1) =", math_utils.max(5, 2, 8, 1))
print("  avg(10, 20, 30) =", math_utils.average(10, 20, 30))
print()

print("More operations:")
print("  |(-42)| =", math_utils.absolute(-42))
print("  min(100, -5, 42) =", math_utils.min(100, -5, 42))
print()

print("--- Part 2: String Operations ---\n")

print("Case operations:")
local text = "Hello World"
print('  Original: "' .. text .. '"')
print('  Uppercase: "' .. string_utils.uppercase(text) .. '"')
print('  Lowercase: "' .. string_utils.lowercase(text) .. '"')
print('  Capitalized: "' .. string_utils.capitalize(text) .. '"')
print()

print("Text analysis:")
local sentence = "The quick brown fox jumps"
print('  Text: "' .. sentence .. '"')
print("  Length:", string_utils.length(sentence))
print("  Word count:", string_utils.word_count(sentence))
print("  Contains 'quick':", string_utils.contains(sentence, "quick"))
print("  Starts with 'The':", string_utils.starts_with(sentence, "The"))
print("  Ends with 'jumps':", string_utils.ends_with(sentence, "jumps"))
print()

print("Text transformation:")
local code = "  hello_world  "
print('  Original: "' .. code .. '"')
print('  Trimmed: "' .. string_utils.trim(code) .. '"')
print('  Left trim: "' .. string_utils.ltrim(code) .. '"')
print('  Right trim: "' .. string_utils.rtrim(code) .. '"')
print()

print("String manipulation:")
print("  Reverse 'lua':", string_utils.reverse("lua"))
print("  Replicate 'ha' 3x:", string_utils.replicate("ha", 3))
print()

print("Splitting and joining:")
local words = string_utils.split("apple,banana,cherry", ",")
print("  Split result: {'" .. string_utils.join(words, "', '") .. "'}")
print("  Join with ' | ':", string_utils.join(words, " | "))
print()

print("Additional string features:")
print("  Word count 'hello world foo': " .. string_utils.word_count("hello world foo"))
print("  Reverse 'hello': '" .. string_utils.reverse("hello") .. "'")
print()

print("--- Part 3: Real-World Workflow ---\n")

print("Demonstrating module composition workflow:")
print()

local data = {
    "HELLO WORLD",
    "PROGRAMMING",
    "LUA LANGUAGE"
}

print("Processing data with multiple modules...")
print()

print("Processed output:")
for i, line in ipairs(data) do
    local processed = string_utils.lowercase(line)
    local length = string_utils.length(processed)
    local reversed = string_utils.reverse(processed)
    
    print("  Line " .. i .. ": '" .. processed .. "'")
    print("    Length: " .. length .. ", Reversed: '" .. reversed .. "'")
end
print()

print("--- Part 4: Error Handling ---\n")

local function safe_operation(name, func, ...)
    local ok, result = pcall(func, ...)
    if ok then
        print("  ✓", name .. ":", result)
    else
        local msg = tostring(result)
        if msg:find(": ") then
            msg = msg:sub(msg:find(": ") + 2)
        end
        print("  ✗", name .. ":", msg)
    end
end

print("Testing error cases:")
safe_operation("Divide by zero", math_utils.divide, 10, 0)
safe_operation("Invalid sqrt", math_utils.square_root, -5)
print()

print("--- Part 5: Package Structure ---\n")

print("Hype package structure demonstrated:")
print("  ✓ hype.json - Package manifest")
print("  ✓ app.lua - Main entry point")
print("  ✓ modules/ - Custom modules directory")
print("    • math_utils.lua - Mathematical operations")
print("    • string_utils.lua - String manipulation")
print("  ✓ README.md - Documentation")
print()

print("Best practices shown:")
print("  ✓ Modular code organization")
print("  ✓ Clear separation of concerns")
print("  ✓ Reusable library modules")
print("  ✓ Descriptive module exports")
print("  ✓ Error handling patterns")
print()

print("═══════════════════════════════════════════════════════════")
print("  Application completed successfully!")
print("═══════════════════════════════════════════════════════════\n")
