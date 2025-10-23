-- Debug script to check args table structure

print("=== Debug Args Table ===")

-- Check if args table exists
print("args table exists:", args ~= nil)

if args then
    -- Check all keys in args table
    print("All keys in args table:")
    for k, v in pairs(args) do
        print(string.format("  %s = %s", tostring(k), tostring(v)))
    end
    
    -- Check specific keys
    print("Specific key checks:")
    print("  args.count:", args.count)
    print("  args.named:", args.named)
    print("  args.flags:", args.flags)
    print("  args.has_flag:", type(args.has_flag))
    print("  args.get_named:", type(args.get_named))
    
    -- Try to call the methods
    if args.has_flag then
        print("  Calling args.has_flag('verbose'):", args.has_flag("verbose"))
    else
        print("  args.has_flag is nil")
    end
    
    if args.get_named then
        print("  Calling args.get_named('name'):", args.get_named("name"))
    else
        print("  args.get_named is nil")
    end
end

-- Check arg table
print("\n--- arg table ---")
if arg then
    print("arg[0]:", arg[0])
    print("arg[1]:", arg[1])
    print("arg[2]:", arg[2])
    print("arg[-1]:", arg[-1])
    print("arg[-2]:", arg[-2])
end

print("=== Debug Complete ===")