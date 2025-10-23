print("Error handling test:")
local status, result = pcall(function()
    return 10 / 2
end)

if status then
    print("Division successful: " .. result)
else
    print("Division failed: " .. result)
end

local status2, result2 = pcall(function()
    return 10 / 0
end)

if status2 then
    print("This won't print")
else
    print("Error caught: " .. tostring(result2))
end
