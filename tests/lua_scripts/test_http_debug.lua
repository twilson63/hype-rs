print("Debugging POST JSON...")

local http = require('http')

local success, result = pcall(function()
    return http.postJson("https://httpbin.org/post", {
        name = "test",
        value = 42,
        nested = { key = "value" }
    })
end)

if not success then
    print("ERROR: " .. tostring(result))
else
    print("Success!")
    print("Status: " .. result.status)
    print("Status text: " .. result.statusText)
    print("OK: " .. tostring(result:ok()))
    
    if result:ok() then
        local data = result:json()
        print("\nResponse data:")
        for k, v in pairs(data) do
            print("  " .. k .. ": " .. type(v))
        end
    end
end
