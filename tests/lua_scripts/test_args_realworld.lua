-- Real-world argument usage examples

print("Real-world Argument Usage Examples")
print("===================================")

-- Example 1: File processing script
print("\n--- File Processing Example ---")
print("Input files:")
for i = 1, #arg do
    if not string.match(arg[i], "^%-%-") then
        print("  Processing: " .. arg[i])
    end
end

if args and args.get_named then
    local output = args.get_named("output") or "output.txt"
    print("Output file: " .. output)
    
    if args.has_flag and args.has_flag("verbose") then
        print("Verbose mode enabled")
    end
end

-- Example 2: Configuration handling
print("\n--- Configuration Example ---")
if args and args.named then
    print("Configuration:")
    for key, value in pairs(args.named) do
        print("  " .. key .. " = " .. value)
    end
end

if args and args.flags then
    print("Feature flags:")
    for flag, enabled in pairs(args.flags) do
        if enabled then
            print("  " .. flag .. " = enabled")
        end
    end
end

-- Example 3: Command simulation
print("\n--- Command Simulation ---")
local command = arg[1] or "help"
print("Command: " .. command)

if command == "process" then
    print("Processing files...")
    for i = 2, #arg do
        if not string.match(arg[i], "^%-%-") then
            print("  -> " .. arg[i])
        end
    end
elseif command == "config" then
    print("Configuration mode")
    if args and args.get_named then
        local key = args.get_named("key")
        local value = args.get_named("value")
        if key and value then
            print("Setting " .. key .. " = " .. value)
        end
    end
else
    print("Unknown command or no command provided")
end

-- Example 4: Argument validation
print("\n--- Argument Validation ---")
local function validate_required(name)
    if args and args.get_named then
        local value = args.get_named(name)
        if not value then
            print("Error: Required argument '--" .. name .. "' is missing")
            return false
        else
            print("✓ " .. name .. " = " .. value)
            return true
        end
    end
    return false
end

local function validate_flag(name)
    if args and args.has_flag then
        if args.has_flag(name) then
            print("✓ Flag '" .. name .. "' is set")
            return true
        else
            print("- Flag '" .. name .. "' is not set")
            return false
        end
    end
    return false
end

-- Simulate required arguments
validate_required("input")
validate_required("output")

-- Simulate optional flags
validate_flag("debug")
validate_flag("verbose")

print("\n=== Examples Complete ===")