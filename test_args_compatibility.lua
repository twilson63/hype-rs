-- Backward compatibility test for argument handling

print("Backward Compatibility Test")
print("============================")

-- Test standard Lua arg table (should always work)
print("\n--- Standard Lua arg Table ---")
if arg then
    print("✓ arg table exists")
    print("✓ arg[0] (script name):", arg[0])
    
    -- Count positive indices (standard Lua behavior)
    local positive_count = 0
    for i = 1, 1000 do
        if arg[i] == nil then break end
        positive_count = positive_count + 1
    end
    print("✓ Positive arguments count:", positive_count)
    
    -- Count negative indices (flags and named args)
    local negative_count = 0
    for i = -1, -100, -1 do
        if arg[i] == nil then break end
        negative_count = negative_count + 1
    end
    print("✓ Negative arguments count:", negative_count)
    
    -- Show all arguments
    print("All arg table entries:")
    for i = -negative_count, positive_count do
        if arg[i] then
            print(string.format("  arg[%d] = %s", i, tostring(arg[i])))
        end
    end
else
    print("✗ arg table missing")
end

-- Test enhanced args table (new feature)
print("\n--- Enhanced args Table ---")
if args then
    print("✓ args table exists")
    
    -- Test basic properties
    print("✓ args.count:", args.count or "not available")
    print("✓ #args:", #(args))
    
    -- Test convenience methods
    if args.has_flag then
        print("✓ args.has_flag method exists")
        local test_flags = {"verbose", "debug", "help", "nonexistent"}
        for _, flag in ipairs(test_flags) do
            local success, result = pcall(args.has_flag, flag)
            if success then
                print("  has_flag('" .. flag .. "'):", result)
            else
                print("  has_flag('" .. flag .. "') error:", result)
            end
        end
    else
        print("✗ args.has_flag method missing")
    end
    
    if args.get_named then
        print("✓ args.get_named method exists")
        local test_keys = {"input", "output", "name", "nonexistent"}
        for _, key in ipairs(test_keys) do
            local success, result = pcall(args.get_named, key)
            if success then
                print("  get_named('" .. key .. "'):", result or "nil")
            else
                print("  get_named('" .. key .. "') error:", result)
            end
        end
    else
        print("✗ args.get_named method missing")
    end
    
    -- Test sub-tables
    if args.named then
        print("✓ args.named table exists")
        print("  Named arguments:")
        for key, value in pairs(args.named) do
            print(string.format("    %s = %s", key, value))
        end
    else
        print("✗ args.named table missing")
    end
    
    if args.flags then
        print("✓ args.flags table exists")
        print("  Flags:")
        for flag, value in pairs(args.flags) do
            print(string.format("    %s = %s", flag, tostring(value)))
        end
    else
        print("✗ args.flags table missing")
    end
    
else
    print("✗ args table missing (fallback mode)")
end

-- Test global variables
print("\n--- Global Variables ---")
print("verbose:", tostring(verbose))
print("debug:", tostring(debug))

-- Test script information globals
print("\n--- Script Information ---")
print("SCRIPT_PATH:", SCRIPT_PATH or "not set")
print("SCRIPT_DIR:", SCRIPT_DIR or "not set")
print("SCRIPT_NAME:", SCRIPT_NAME or "not set")

-- Test hype table
print("\n--- Hype Table ---")
if hype then
    print("✓ hype table exists")
    print("  version:", hype.version or "unknown")
    print("  capture_output:", hype.capture_output or "unknown")
    print("  enable_stats:", hype.enable_stats or "unknown")
    
    if hype.args then
        print("  hype.args statistics:")
        print("    indexed_count:", hype.args.indexed_count or 0)
        print("    named_count:", hype.args.named_count or 0)
        print("    flags_count:", hype.args.flags_count or 0)
        print("    total_count:", hype.args.total_count or 0)
    end
else
    print("✗ hype table missing")
end

print("\n=== Compatibility Test Complete ===")