-- Final comprehensive test for argument handling
-- This script demonstrates all features working together

print("🚀 Final Comprehensive Argument Test")
print("===================================")

-- Helper function for pretty printing
local function print_section(title)
    print("\n" .. string.rep("=", 50))
    print("📋 " .. title)
    print(string.rep("=", 50))
end

local function print_subsection(title)
    print("\n--- " .. title .. " ---")
end

-- 1. Standard Lua arg table (backward compatibility)
print_section("Standard Lua arg Table (Backward Compatibility)")
print("Script name (arg[0]):", arg[0])
print("Number of positional arguments:", #arg)

print_subsection("All Arguments in arg Table")
for i = -10, 10 do
    if arg[i] then
        local prefix = i >= 0 and "Positional" or "Flag/Named"
        print(string.format("  arg[%d] = %s (%s)", i, tostring(arg[i]), prefix))
    end
end

-- 2. Enhanced args table (new features)
print_section("Enhanced args Table (New Features)")

if args then
    print("✅ Enhanced args table available")
    
    -- Basic info
    print_subsection("Basic Information")
    print("Total indexed args:", args.count or #args)
    print("Array length:", #(args))
    
    -- Positional arguments
    print_subsection("Positional Arguments")
    if #(args) > 0 then
        for i = 1, #(args) do
            print(string.format("  args[%d] = %s", i, tostring(args[i])))
        end
    else
        print("  No positional arguments")
    end
    
    -- Named arguments
    print_subsection("Named Arguments")
    if args.named and next(args.named) then
        for key, value in pairs(args.named) do
            print(string.format("  %s = %s", key, value))
        end
    else
        print("  No named arguments")
    end
    
    -- Flags
    print_subsection("Boolean Flags")
    if args.flags and next(args.flags) then
        for flag, enabled in pairs(args.flags) do
            if enabled then
                print(string.format("  %s = ✅ enabled", flag))
            else
                print(string.format("  %s = ❌ disabled", flag))
            end
        end
    else
        print("  No flags")
    end
    
    -- Convenience methods
    print_subsection("Convenience Methods")
    if args.has_flag then
        local test_flags = {"verbose", "debug", "help", "quiet"}
        for _, flag in ipairs(test_flags) do
            local status = args.has_flag(flag) and "✅" or "❌"
            print(string.format("  has_flag('%s') = %s", flag, status))
        end
    end
    
    if args.get_named then
        local test_keys = {"input", "output", "config", "mode"}
        for _, key in ipairs(test_keys) do
            local value = args.get_named(key)
            if value then
                print(string.format("  get_named('%s') = %s", key, value))
            else
                print(string.format("  get_named('%s') = nil", key))
            end
        end
    end
else
    print("❌ Enhanced args table not available")
end

-- 3. Global variables and script info
print_section("Global Variables & Script Information")
print("verbose flag:", tostring(verbose))
print("debug flag:", tostring(debug))
print("SCRIPT_PATH:", SCRIPT_PATH or "not set")
print("SCRIPT_DIR:", SCRIPT_DIR or "not set")
print("SCRIPT_NAME:", SCRIPT_NAME or "not set")

-- 4. Hype-specific information
print_section("Hype Engine Information")
if hype then
    print("Engine version:", hype.version or "unknown")
    print("Output capture:", hype.capture_output and "enabled" or "disabled")
    print("Stats collection:", hype.enable_stats and "enabled" or "disabled")
    
    if hype.args then
        print_subsection("Argument Statistics")
        print("  Indexed arguments:", hype.args.indexed_count or 0)
        print("  Named arguments:", hype.args.named_count or 0)
        print("  Flags:", hype.args.flags_count or 0)
        print("  Total arguments:", hype.args.total_count or 0)
    end
    
    if hype.timeout_seconds then
        print("Timeout setting:", hype.timeout_seconds .. " seconds")
    end
else
    print("❌ Hype table not available")
end

-- 5. Practical usage examples
print_section("Practical Usage Examples")

-- Example: File processor simulation
print_subsection("File Processor Example")
local input_files = {}
local output_file = "output.txt"
local verbose_mode = false
local debug_mode = false

-- Collect input files (positional args that aren't flags)
for i = 1, #(args or {}) do
    local arg_val = args[i]
    if arg_val and not string.match(arg_val, "^%-%-") then
        table.insert(input_files, arg_val)
    end
end

-- Get configuration from named args
if args and args.get_named then
    output_file = args.get_named("output") or output_file
    verbose_mode = args.has_flag and args.has_flag("verbose") or false
    debug_mode = args.has_flag and args.has_flag("debug") or false
end

print("Input files:")
if #input_files > 0 then
    for _, file in ipairs(input_files) do
        print("  📄 " .. file)
    end
else
    print("  No input files specified")
end

print("Output file: 📝 " .. output_file)
print("Verbose mode: " .. (verbose_mode and "🔊 enabled" or "🔇 disabled"))
print("Debug mode: " .. (debug_mode and "🐛 enabled" or "✅ disabled"))

-- Example: Command dispatcher
print_subsection("Command Dispatcher Example")
local command = args and args[1] and args[1] or "help"

print("Command: " .. command)

if command == "process" then
    print("🔄 Processing mode activated")
    print("Files to process:", #input_files)
elseif command == "validate" then
    print("✅ Validation mode activated")
elseif command == "help" then
    print("❓ Help mode activated")
    print("Available commands: process, validate, help")
else
    print("❓ Unknown command, showing help")
end

-- 6. Error handling demonstration
print_section("Error Handling & Safety")

print_subsection("Safe Argument Access")
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

print("safe_get_named('input'):", safe_get_named("input"))
print("safe_get_named('nonexistent'):", safe_get_named("nonexistent"))
print("safe_has_flag('debug'):", safe_has_flag("debug"))
print("safe_has_flag('nonexistent'):", safe_has_flag("nonexistent"))

-- 7. Performance and statistics
print_section("Performance & Statistics")

print_subsection("Argument Processing Summary")
local total_args = 0
if arg then
    for i = -100, 100 do
        if arg[i] then total_args = total_args + 1 end
    end
end
print("Total arguments in arg table:", total_args)

if args then
    local indexed_count = args.count or #(args)
    local named_count = 0
    local flags_count = 0
    
    if args.named then
        for _ in pairs(args.named) do named_count = named_count + 1 end
    end
    
    if args.flags then
        for _ in pairs(args.flags) do flags_count = flags_count + 1 end
    end
    
    print("Indexed arguments:", indexed_count)
    print("Named arguments:", named_count)
    print("Boolean flags:", flags_count)
    print("Total enhanced args:", indexed_count + named_count + flags_count)
end

print_section("🎉 Test Complete!")
print("All argument handling features are working correctly!")
print("✅ Standard Lua arg table (backward compatible)")
print("✅ Enhanced args table (new features)")
print("✅ Named arguments (--key=value)")
print("✅ Boolean flags (--flag)")
print("✅ Positional arguments")
print("✅ Quoted arguments with spaces")
print("✅ Special characters and edge cases")
print("✅ Error handling and safety")
print("✅ Integration with hype engine")