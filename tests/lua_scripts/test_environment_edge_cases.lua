-- Environment variable edge cases test
print("=== Environment Variable Edge Cases Test ===")

-- Test numeric keys
print("Testing numeric keys:")
env[123] = "numeric_key_value"
print("env[123]:", env[123])

-- Test empty string key
print("\nTesting empty string key:")
local success, error_msg = pcall(function()
    env[""] = "empty_key_value"
    print("env['']:", env[""])
end)
if not success then
    print("Empty key error:", error_msg)
end

-- Test very long variable name
print("\nTesting very long variable name:")
local long_name = string.rep("A", 100)
local success, error_msg = pcall(function()
    env[long_name] = "long_name_value"
    print("Long name set successfully")
end)
if not success then
    print("Long name error:", error_msg)
end

-- Test special characters in variable name
print("\nTesting special characters:")
local special_names = {"VAR-NAME", "VAR.NAME", "VAR_NAME", "VAR NAME"}
for _, name in ipairs(special_names) do
    local success, error_msg = pcall(function()
        env[name] = "special_char_value"
        print(string.format("'%s' set successfully", name))
    end)
    if not success then
        print(string.format("'%s' error: %s", name, error_msg))
    end
end

-- Test case sensitivity
print("\nTesting case sensitivity:")
env.case_test = "lowercase"
env.CASE_TEST = "uppercase"
print("env.case_test:", env.case_test)
print("env.CASE_TEST:", env.CASE_TEST)

-- Test nil values
print("\nTesting nil values:")
env.nil_test = "not_nil"
print("Before nil:", env.nil_test)
env.nil_test = nil
print("After nil:", env.nil_test or "nil")

-- Test table values (should fail)
print("\nTesting table values:")
local success, error_msg = pcall(function()
    env.table_test = {key = "value"}
    print("Table set successfully")
end)
if not success then
    print("Table error:", error_msg)
end

-- Test function values (should fail)
print("\nTesting function values:")
local success, error_msg = pcall(function()
    env.function_test = function() return "test" end
    print("Function set successfully")
end)
if not success then
    print("Function error:", error_msg)
end

print("=== Edge cases test completed ===")