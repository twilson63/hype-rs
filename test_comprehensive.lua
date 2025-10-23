-- Comprehensive test for the hype-rs execution engine
print("=== Comprehensive Test Suite ===")

-- Test basic operations
print("\n1. Basic Operations:")
local x = 10
local y = 20
print("10 + 20 =", x + y)

-- Test table operations
print("\n2. Table Operations:")
local person = {
    name = "John Doe",
    age = 30,
    hobbies = {"reading", "coding", "gaming"}
}
print("Person name:", person.name)
print("Person age:", person.age)
print("First hobby:", person.hobbies[1])

-- Test string operations
print("\n3. String Operations:")
local message = "Hello, World!"
print("Original:", message)
print("Upper:", string.upper(message))
print("Length:", #message)

-- Test math operations
print("\n4. Math Operations:")
print("PI:", math.pi)
print("Random:", math.random(1, 100))
print("Square root of 16:", math.sqrt(16))

-- Test conditional logic
print("\n5. Conditional Logic:")
local score = 85
if score >= 90 then
    print("Grade: A")
elseif score >= 80 then
    print("Grade: B")
elseif score >= 70 then
    print("Grade: C")
else
    print("Grade: F")
end

-- Test loops
print("\n6. Loops:")
print("Counting to 5:")
for i = 1, 5 do
    print("  Count:", i)
end

-- Test functions
print("\n7. Functions:")
local function factorial(n)
    if n <= 1 then
        return 1
    else
        return n * factorial(n - 1)
    end
end
print("5! =", factorial(5))

-- Test arguments
print("\n8. Arguments:")
if #args > 0 then
    print("Arguments received:")
    for i, arg in ipairs(args) do
        print("  Arg " .. i .. ": " .. arg)
    end
else
    print("No arguments provided")
end

-- Test hype globals
print("\n9. Hype Globals:")
print("Version:", hype.version or "unknown")
print("Debug mode:", debug or false)
print("Verbose mode:", verbose or false)
print("Script path:", SCRIPT_PATH or "unknown")
print("Script directory:", SCRIPT_DIR or "unknown")
print("Script name:", SCRIPT_NAME or "unknown")

-- Test error handling
print("\n10. Error Handling:")
local success, error_msg = pcall(function()
    -- This will cause an error
    local result = nil + 5
    return result
end)

if success then
    print("No error occurred")
else
    print("Caught error:", error_msg)
end

-- Test memory usage simulation
print("\n11. Memory Usage:")
local large_table = {}
for i = 1, 1000 do
    large_table[i] = {
        id = i,
        name = "Item " .. i,
        data = string.rep("x", 100)
    }
end
print("Created table with", #large_table, "items")

print("\n=== Test Suite Complete ===")