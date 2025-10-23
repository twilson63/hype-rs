-- Basic environment variable access test
print("=== Basic Environment Variable Test ===")

-- Test reading common environment variables
print("PATH:", env.PATH or "not set")
print("HOME:", env.HOME or "not set")
print("USER:", env.USER or "not set")

-- Test reading via string indexing
print("SHELL via string index:", env["SHELL"] or "not set")

-- Test reading non-existent variable
print("NON_EXISTENT:", env.NON_EXISTENT or "not set")

-- Test using the get method
local path_value = env.get("PATH")
if path_value then
    print("PATH via get():", path_value)
else
    print("PATH via get(): not set")
end

-- Test exists method
print("PATH exists:", env.exists("PATH"))
print("NON_EXISTENT exists:", env.exists("NON_EXISTENT"))

-- Test is_sensitive method
print("Is PATH sensitive?", env.is_sensitive("PATH"))
print("Is PASSWORD sensitive?", env.is_sensitive("PASSWORD"))
print("Is API_KEY sensitive?", env.is_sensitive("API_KEY"))

print("=== Basic test completed ===")