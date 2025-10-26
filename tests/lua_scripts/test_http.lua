print("═════════════════════════════════════════════════")
print("  HTTP Module Test")
print("═════════════════════════════════════════════════\n")

print("--- Step 1: Loading HTTP Module ---")
local http = require("http")
print("✓ HTTP module loaded successfully")
print()

print("--- Step 2: Module Structure ---")
if http then
    print("Module ID: " .. (http.__id or "unknown"))
    print("Module Description: " .. (http.__desc or "none"))
    print()
    
    print("Available functions:")
    if http.get then
        print("  ✓ http.get")
        print("    " .. (http.get.__signature or ""))
    end
    if http.post then
        print("  ✓ http.post")
        print("    " .. (http.post.__signature or ""))
    end
    if http.put then
        print("  ✓ http.put")
        print("    " .. (http.put.__signature or ""))
    end
    if http.delete then
        print("  ✓ http.delete")
        print("    " .. (http.delete.__signature or ""))
    end
    if http.patch then
        print("  ✓ http.patch")
        print("    " .. (http.patch.__signature or ""))
    end
    if http.head then
        print("  ✓ http.head")
        print("    " .. (http.head.__signature or ""))
    end
    if http.fetch then
        print("  ✓ http.fetch")
        print("    " .. (http.fetch.__signature or ""))
    end
    if http.postJson then
        print("  ✓ http.postJson")
        print("    " .. (http.postJson.__signature or ""))
    end
    if http.putJson then
        print("  ✓ http.putJson")
        print("    " .. (http.putJson.__signature or ""))
    end
else
    print("✗ HTTP module not found")
end
print()

print("--- Step 3: Usage Examples (Planned) ---")
print()
print("When fully implemented, you will be able to:")
print()
print("1. Simple GET request:")
print("   local response = http.get('https://api.example.com/data')")
print("   print(response.status)  -- 200")
print("   print(response.body)    -- Response content")
print()
print("2. POST with JSON:")
print("   local data = {name = 'Alice', age = 30}")
print("   local response = http.postJson('https://api.example.com/users', data)")
print("   local result = response.json()")
print()
print("3. Custom headers:")
print("   local response = http.post('https://api.example.com/data', {")
print("     body = 'custom data',")
print("     headers = {")
print("       ['Content-Type'] = 'application/json',")
print("       ['Authorization'] = 'Bearer token123'")
print("     }")
print("   })")
print()
print("4. Universal fetch API:")
print("   local response = http.fetch('https://api.example.com/data', {")
print("     method = 'POST',")
print("     body = '{\"key\": \"value\"}',")
print("     headers = {['Content-Type'] = 'application/json'},")
print("     timeout = 5000  -- 5 seconds")
print("   })")
print()
print("5. Check response status:")
print("   local response = http.get('https://api.example.com/data')")
print("   if response.ok() then")
print("     print('Success:', response.text())")
print("   else")
print("     print('Error:', response.status, response.statusText)")
print("   end")
print()

print("═════════════════════════════════════════════════")
print("  HTTP Module Test Complete")
print("═════════════════════════════════════════════════")
