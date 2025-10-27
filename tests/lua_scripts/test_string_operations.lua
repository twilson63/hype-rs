local string = require("string")

print("=== String Module Integration Tests ===\n")

print("1. Testing string.split()...")
local parts = string.split("apple,banana,cherry", ",")
assert(#parts == 3 and parts[1] == "apple", "split with delimiter failed")
local chars = string.split("hi", "")
assert(#chars == 2 and chars[1] == "h" and chars[2] == "i", "split by char failed")
print("   ✓ split works")

print("\n2. Testing string.trim()...")
assert(string.trim("  hello  ") == "hello", "trim failed")
assert(string.trim("world") == "world", "trim no-op failed")
print("   ✓ trim works")

print("\n3. Testing string.trimStart() and trimEnd()...")
assert(string.trimStart("  hello  ") == "hello  ", "trimStart failed")
assert(string.trimEnd("  hello  ") == "  hello", "trimEnd failed")
print("   ✓ trimStart/trimEnd work")

print("\n4. Testing string.startsWith() and endsWith()...")
assert(string.startsWith("hello world", "hello") == true, "startsWith failed")
assert(string.startsWith("hello world", "world") == false, "startsWith negative failed")
assert(string.endsWith("hello world", "world") == true, "endsWith failed")
assert(string.endsWith("hello world", "hello") == false, "endsWith negative failed")
print("   ✓ startsWith/endsWith work")

print("\n5. Testing string.contains()...")
assert(string.contains("hello world", "lo wo") == true, "contains failed")
assert(string.contains("hello", "xyz") == false, "contains negative failed")
print("   ✓ contains works")

print("\n6. Testing string.padStart() and padEnd()...")
assert(string.padStart("5", 3) == "  5", "padStart default failed")
assert(string.padStart("5", 3, "0") == "005", "padStart with fill failed")
assert(string.padEnd("5", 3) == "5  ", "padEnd default failed")
assert(string.padEnd("5", 3, "0") == "500", "padEnd with fill failed")
print("   ✓ padStart/padEnd work")

print("\n7. Testing string.repeat()...")
local repeat_fn = string["repeat"]
assert(repeat_fn("ha", 3) == "hahaha", "repeat failed")
assert(repeat_fn("x", 0) == "", "repeat zero failed")
print("   ✓ repeat works")

print("\n8. Testing string.replace()...")
local text = "hello hello hello"
assert(string.replace(text, "l", "L", 2) == "heLLo hello hello", "replace with count failed")
print("   ✓ replace works")

print("\n9. Testing string.replaceAll()...")
assert(string.replaceAll("foo foo", "foo", "bar") == "bar bar", "replaceAll failed")
print("   ✓ replaceAll works")

print("\n10. Testing string.toUpperCase() and toLowerCase()...")
assert(string.toUpperCase("hello") == "HELLO", "toUpperCase failed")
assert(string.toLowerCase("HELLO") == "hello", "toLowerCase failed")
print("   ✓ toUpperCase/toLowerCase work")

print("\n11. Testing string.capitalize()...")
assert(string.capitalize("hello") == "Hello", "capitalize failed")
assert(string.capitalize("world") == "World", "capitalize another failed")
print("   ✓ capitalize works")

print("\n12. Testing string.lines()...")
local lines = string.lines("line1\nline2\nline3")
assert(#lines == 3, "lines count failed")
assert(lines[1] == "line1" and lines[2] == "line2", "lines content failed")
print("   ✓ lines works")

print("\n13. Testing string.chars()...")
local chars = string.chars("abc")
assert(#chars == 3, "chars count failed")
assert(chars[1] == "a" and chars[2] == "b" and chars[3] == "c", "chars content failed")
print("   ✓ chars works")

print("\n14. Testing combined operations...")
local message = "  Hello World  "
message = string.trim(message)
assert(message == "Hello World", "step 1 failed")
message = string.toLowerCase(message)
assert(message == "hello world", "step 2 failed")
message = string.replaceAll(message, "world", "Lua")
assert(message == "hello Lua", "step 3 failed")
message = string.capitalize(message)
assert(message == "Hello Lua", "step 4 failed")
print("   ✓ Combined operations work")

print("\n15. Testing practical use case - parsing CSV...")
local csv = "name,age,city"
local headers = string.split(csv, ",")
assert(#headers == 3 and headers[1] == "name", "CSV parsing failed")
print("   ✓ CSV parsing works")

print("\n16. Testing practical use case - formatting...")
local num = "42"
local formatted = string.padStart(num, 5, "0")
assert(formatted == "00042", "Number formatting failed")
print("   ✓ Number formatting works")

print("\n17. Testing Unicode support...")
local unicode = "Hello 世界 🌍"
assert(string.contains(unicode, "世界"), "Unicode contains failed")
assert(string.toUpperCase("hello") == "HELLO", "ASCII uppercase works")
print("   ✓ Unicode support works")

print("\n=== All String Module Tests Passed! ===")
