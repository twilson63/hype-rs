-- Simple test script for argument handling

print("=== Simple Argument Test ===")

-- Test basic args table
print("Script name:", arg and arg[0] or "unknown")

-- Test indexed arguments
if args then
    print("Number of indexed args:", args.count or #args)
    
    print("Indexed arguments:")
    for i = 1, #(args) do
        if args[i] then
            print(string.format("  args[%d] = %s", i, tostring(args[i])))
        end
    end
    
    -- Test named arguments
    if args.named then
        print("Named arguments:")
        for key, value in pairs(args.named) do
            print(string.format("  %s = %s", key, value))
        end
    end
    
    -- Test flags
    if args.flags then
        print("Flags:")
        for flag, enabled in pairs(args.flags) do
            if enabled then
                print(string.format("  %s = true", flag))
            end
        end
    end
    
    -- Test convenience methods
    if args.has_flag then
        print("Flag tests:")
        print("  Has 'verbose':", args.has_flag("verbose"))
        print("  Has 'debug':", args.has_flag("debug"))
    end
    
    if args.get_named then
        print("Named argument tests:")
        print("  Name:", args.get_named("name") or "not set")
        print("  Count:", args.get_named("count") or "not set")
    end
else
    print("No args table available")
end

-- Test global flags
print("Global flags:")
print("  verbose:", tostring(verbose))
print("  debug:", tostring(debug))

print("=== Test Complete ===")