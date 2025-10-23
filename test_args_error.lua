-- Test script for error handling with arguments

print("Argument Error Handling Test")
print("=============================")

-- Test 1: Accessing non-existent arguments
print("\n--- Test 1: Non-existent arguments ---")
print("arg[999]:", tostring(arg[999]))
print("args[999]:", tostring(args and args[999] or "nil"))

if args and args.get_named then
    print("Non-existent named arg:", args.get_named("nonexistent") or "nil")
end

-- Test 2: Type safety
print("\n--- Test 2: Type safety ---")
print("Type of arg table:", type(arg))
print("Type of args table:", type(args))

if args then
    print("Type of args.count:", type(args.count))
    print("Type of args.named:", type(args.named))
    print("Type of args.flags:", type(args.flags))
    
    if args.has_flag then
        print("Type of args.has_flag:", type(args.has_flag))
        local success, result = pcall(args.has_flag, "test")
        print("args.has_flag('test') works:", success, result)
    end
end

-- Test 3: Edge cases
print("\n--- Test 3: Edge cases ---")
print("#arg (length):", #arg)
if args then
    print("#args (length):", #(args))
    print("args.count:", args.count)
end

-- Test 4: Iteration safety
print("\n--- Test 4: Safe iteration ---")
print("Iterating over arg table:")
for i = -5, 5 do
    if arg[i] ~= nil then
        print(string.format("  arg[%d] = %s", i, tostring(arg[i])))
    end
end

if args and args.named then
    print("Iterating over named args:")
    for key, value in pairs(args.named) do
        print(string.format("  %s = %s", key, value))
    end
end

if args and args.flags then
    print("Iterating over flags:")
    for flag, value in pairs(args.flags) do
        print(string.format("  %s = %s", flag, tostring(value)))
    end
end

-- Test 5: Error recovery
print("\n--- Test 5: Error recovery ---")
local function safe_get_named(key)
    if not args then return "no args table" end
    if not args.get_named then return "no get_named method" end
    
    local success, result = pcall(args.get_named, key)
    if success then
        return result or "nil"
    else
        return "error: " .. tostring(result)
    end
end

local function safe_has_flag(flag)
    if not args then return "no args table" end
    if not args.has_flag then return "no has_flag method" end
    
    local success, result = pcall(args.has_flag, flag)
    if success then
        return tostring(result)
    else
        return "error: " .. tostring(result)
    end
end

print("Safe get_named('input'):", safe_get_named("input"))
print("Safe get_named('nonexistent'):", safe_get_named("nonexistent"))
print("Safe has_flag('debug'):", safe_has_flag("debug"))
print("Safe has_flag('nonexistent'):", safe_has_flag("nonexistent"))

print("\n=== Error Handling Test Complete ===")