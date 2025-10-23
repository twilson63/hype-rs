-- Environment variable listing test
print("=== Environment Variable List Test ===")

-- Test listing all allowed environment variables
print("Listing all accessible environment variables:")
local env_vars = env.list()

if env_vars then
    local count = 0
    for name, value in pairs(env_vars) do
        print(string.format("  %s = %s", name, value))
        count = count + 1
        if count >= 10 then  -- Limit output for readability
            print("  ... (showing first 10 variables)")
            break
        end
    end
    
    -- Count total variables
    local total_count = 0
    for _ in pairs(env_vars) do
        total_count = total_count + 1
    end
    print(string.format("Total accessible variables: %d", total_count))
else
    print("Failed to list environment variables")
end

-- Test filtering for specific patterns
print("\nSearching for PATH-related variables:")
local env_vars = env.list()
for name, value in pairs(env_vars or {}) do
    if string.find(string.upper(name), "PATH") then
        print(string.format("  %s = %s", name, value))
    end
end

print("=== List test completed ===")