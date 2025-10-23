-- Comprehensive test script for argument handling

print("=== Argument Handling Test ===")
print("Script name:", arg and arg[0] or "unknown")

-- Test standard Lua arg table
print("\n--- Standard Lua arg table ---")
if arg then
    for i = -10, 10 do
        if arg[i] ~= nil then
            print(string.format("arg[%d] = %s", i, tostring(arg[i])))
        end
    end
else
    print("No arg table available")
end

-- Test enhanced args table
print("\n--- Enhanced args table ---")
if args then
    print("Number of indexed arguments:", args.count or #args)
    
    -- Test indexed arguments
    print("Indexed arguments:")
    for i = 1, #(args) do
        print(string.format("  args[%d] = %s", i, tostring(args[i])))
    end
    
    -- Test named arguments
    if args.named then
        print("Named arguments:")
        for key, value in pairs(args.named) do
            print(string.format("  args.named.%s = %s", key, value))
        end
    end
    
    -- Test flags
    if args.flags then
        print("Flags:")
        for flag, value in pairs(args.flags) do
            print(string.format("  args.flags.%s = %s", flag, tostring(value)))
        end
    end
    
    -- Test convenience methods
    if args.has_flag then
        print("\n--- Flag Tests ---")
        print("Has 'verbose' flag:", args.has_flag("verbose"))
        print("Has 'debug' flag:", args.has_flag("debug"))
        print("Has 'help' flag:", args.has_flag("help"))
    end
    
    if args.get_named then
        print("\n--- Named Argument Tests ---")
        print("Name:", args.get_named("name") or "not set")
        print("Age:", args.get_named("age") or "not set")
        print("Output:", args.get_named("output") or "not set")
    end
else
    print("No args table available")
end

-- Test global flags
print("\n--- Global Flags ---")
print("verbose:", tostring(verbose))
print("debug:", tostring(debug))

-- Test script information
print("\n--- Script Information ---")
print("SCRIPT_PATH:", SCRIPT_PATH or "not set")
print("SCRIPT_DIR:", SCRIPT_DIR or "not set")
print("SCRIPT_NAME:", SCRIPT_NAME or "not set")

-- Test hype information
if hype then
    print("\n--- Hype Information ---")
    print("Hype version:", hype.version or "unknown")
    print("Capture output:", hype.capture_output or "unknown")
    print("Enable stats:", hype.enable_stats or "unknown")
    
    if hype.args then
        print("Argument stats:")
        print("  Indexed count:", hype.args.indexed_count or 0)
        print("  Named count:", hype.args.named_count or 0)
        print("  Flags count:", hype.args.flags_count or 0)
        print("  Total count:", hype.args.total_count or 0)
    end
end

print("\n=== Test Complete ===")