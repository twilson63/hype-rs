local string = require("string")

print("=== String Module Demo ===\n")

print("1. Splitting strings:")
local csv = "apple,banana,cherry,date"
local fruits = string.split(csv, ",")
for i, fruit in ipairs(fruits) do
    print("  " .. i .. ". " .. fruit)
end

print("\n2. Trimming whitespace:")
local messy = "   too much space   "
print("  Before: '" .. messy .. "'")
print("  After:  '" .. string.trim(messy) .. "'")

print("\n3. Checking prefixes and suffixes:")
local filename = "document.pdf"
if string.endsWith(filename, ".pdf") then
    print("  ✓ " .. filename .. " is a PDF file")
end

print("\n4. Padding for alignment:")
local items = {
    {name = "Apple", price = 1.50},
    {name = "Banana", price = 0.75},
    {name = "Cherry", price = 2.00}
}
print("\n  Item       Price")
local repeat_fn = string["repeat"]
print("  " .. repeat_fn("-", 20))
for _, item in ipairs(items) do
    local name = string.padEnd(item.name, 10)
    local price = string.padStart(string.format("%.2f", item.price), 6)
    print("  " .. name .. " $" .. price)
end

print("\n5. Text transformations:")
local text = "hello world"
print("  Original:    " .. text)
print("  Uppercase:   " .. string.toUpperCase(text))
print("  Lowercase:   " .. string.toLowerCase(text))
print("  Capitalized: " .. string.capitalize(text))

print("\n6. Pattern replacement:")
local log = "ERROR: file not found ERROR: permission denied ERROR: timeout"
local cleaned = string.replaceAll(log, "ERROR:", "⚠️ ")
print("  " .. cleaned)

print("\n7. Repeating patterns:")
local repeat_fn = string["repeat"]
local border = repeat_fn("=", 40)
print("\n" .. border)
print("  " .. string.padStart("CENTERED TEXT", 26))
print(border)

print("\n8. Line processing:")
local multiline = [[First line
Second line
Third line]]
local lines = string.lines(multiline)
print("\n  Processing " .. #lines .. " lines:")
for i, line in ipairs(lines) do
    print("    Line " .. i .. ": " .. string.trim(line))
end

print("\n9. Character analysis:")
local word = "Hello"
local chars = string.chars(word)
print("\n  Breaking '" .. word .. "' into characters:")
for i, char in ipairs(chars) do
    print("    [" .. i .. "] = '" .. char .. "'")
end

print("\n10. Practical example - URL parsing:")
local url = "https://example.com/api/users"
if string.startsWith(url, "https://") then
    print("\n  ✓ Secure connection (HTTPS)")
    local path = string.split(url, "/")
    print("  Endpoint: /" .. path[#path])
end

print("\n11. String manipulation pipeline:")
local input = "  MESSY    TeXt WiTh   SpAcEs  "
print("\n  Input:  '" .. input .. "'")
local step1 = string.trim(input)
print("  Step 1 (trim): '" .. step1 .. "'")
local step2 = string.toLowerCase(step1)
print("  Step 2 (lower): '" .. step2 .. "'")
local step3 = string.replaceAll(step2, "   ", " ")
print("  Step 3 (normalize spaces): '" .. step3 .. "'")
local step4 = string.capitalize(step3)
print("  Step 4 (capitalize): '" .. step4 .. "'")

print("\n12. Formatting output:")
local status = {
    {service = "API", status = "UP", uptime = "99.9%"},
    {service = "Database", status = "UP", uptime = "100%"},
    {service = "Cache", status = "DOWN", uptime = "95.2%"}
}
print("\n  Service Status Report")
print("  " .. repeat_fn("-", 40))
for _, s in ipairs(status) do
    local service = string.padEnd(s.service, 12)
    local status = string.padEnd(s.status, 8)
    local uptime = string.padStart(s.uptime, 8)
    print("  " .. service .. status .. uptime)
end

print("\n=== Demo Complete ===")
