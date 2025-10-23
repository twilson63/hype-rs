-- Environment variable security test
print("=== Environment Variable Security Test ===")

-- Test accessing sensitive variables (should be blocked by default)
print("Testing access to sensitive variables:")

local function test_sensitive_access(var_name)
    local success, error_msg = pcall(function()
        local value = env[var_name]
        print(var_name .. ":", value or "nil")
    end)
    
    if not success then
        print(var_name .. ": ACCESS DENIED - " .. error_msg)
    end
end

-- Test various sensitive variable patterns
test_sensitive_access("PASSWORD")
test_sensitive_access("API_KEY")
test_sensitive_access("SECRET_TOKEN")
test_sensitive_access("DATABASE_URL")
test_sensitive_access("MY_APP_PASSWORD")
test_sensitive_access("AWS_SECRET_ACCESS_KEY")

-- Test writing to sensitive variables (should be blocked)
print("\nTesting write access to sensitive variables:")

local function test_sensitive_write(var_name)
    local success, error_msg = pcall(function()
        env[var_name] = "test_value"
        print(var_name .. ": WRITE SUCCESS")
    end)
    
    if not success then
        print(var_name .. ": WRITE DENIED - " .. error_msg)
    end
end

test_sensitive_write("PASSWORD")
test_sensitive_write("SECRET_KEY")
test_sensitive_write("AUTH_TOKEN")

-- Test accessing non-sensitive variables (should work)
print("\nTesting access to non-sensitive variables:")
print("PATH:", env.PATH or "not set")
print("HOME:", env.HOME or "not set")
print("LANG:", env.LANG or "not set")

print("=== Security test completed ===")