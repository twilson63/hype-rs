-- Test script for quoted arguments and special characters

print("Quoted Arguments & Special Characters Test")
print("========================================")

print("Script name:", arg[0])
print("Total arguments:", #arg)

-- Show all arguments with their indices
print("\nAll arguments:")
for i = -10, #arg do
    if arg[i] then
        -- Show the argument and its length
        local arg_str = tostring(arg[i])
        print(string.format("arg[%d] = '%s' (length: %d)", i, arg_str, #arg_str))
    end
end

-- Test enhanced args table if available
if args then
    print("\nEnhanced args table:")
    
    -- Show indexed args
    print("Indexed arguments:")
    for i = 1, #(args) do
        if args[i] then
            print(string.format("  args[%d] = '%s'", i, tostring(args[i])))
        end
    end
    
    -- Show named args
    if args.named then
        print("Named arguments:")
        for key, value in pairs(args.named) do
            print(string.format("  %s = '%s'", key, value))
        end
    end
    
    -- Show flags
    if args.flags then
        print("Flags:")
        for flag, enabled in pairs(args.flags) do
            print(string.format("  %s = %s", flag, tostring(enabled)))
        end
    end
end

print("\n=== Test Complete ===")