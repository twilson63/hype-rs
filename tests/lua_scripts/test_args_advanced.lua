-- Advanced argument handling test script

print("=== Advanced Argument Handling Test ===")

-- Helper function to print table contents
function print_table(name, tbl)
    if not tbl then
        print(name .. ": nil")
        return
    end
    
    print(name .. ":")
    for k, v in pairs(tbl) do
        if type(k) == "string" then
            print(string.format("  %s = %s", k, tostring(v)))
        else
            print(string.format("  [%d] = %s", k, tostring(v)))
        end
    end
end

-- Test argument parsing with different types
print("\n--- Argument Type Tests ---")

-- Test string arguments
local name = args.get_named and args.get_named("name") or arg and arg[1] or "World"
print("Hello, " .. name .. "!")

-- Test numeric arguments
local count = args.get_named and args.get_named("count") or arg and arg[2] or "1"
count = tonumber(count) or 1
print("Count:", count)

-- Test boolean flags
local verbose = args.has_flag and args.has_flag("verbose") or false
local debug = args.has_flag and args.has_flag("debug") or false

print("Verbose mode:", verbose)
print("Debug mode:", debug)

-- Test file paths
local input_file = args.get_named and args.get_named("input") or arg and arg[3] or "default.txt"
local output_file = args.get_named and args.get_named("output") or arg and arg[4] or "output.txt"

print("Input file:", input_file)
print("Output file:", output_file)

-- Test complex argument processing
print("\n--- Complex Argument Processing ---")

-- Process all named arguments
if args.named then
    print("Processing named arguments:")
    for key, value in pairs(args.named) do
        -- Try to convert to number
        local num_value = tonumber(value)
        if num_value then
            print(string.format("  %s (number): %g", key, num_value))
        else
            -- Check for boolean-like values
            local lower_value = value:lower()
            if lower_value == "true" or lower_value == "false" then
                print(string.format("  %s (boolean): %s", key, lower_value))
            else
                print(string.format("  %s (string): %s", key, value))
            end
        end
    end
end

-- Process all flags
if args.flags then
    print("Processing flags:")
    for flag, enabled in pairs(args.flags) do
        if enabled then
            print(string.format("  Flag '%s' is enabled", flag))
        end
    end
end

-- Test argument validation simulation
print("\n--- Argument Validation Simulation ---")

function validate_arguments()
    local errors = {}
    
    -- Check required arguments
    if not args.get_named("name") then
        table.insert(errors, "Missing required argument: --name")
    end
    
    if not args.get_named("input") then
        table.insert(errors, "Missing required argument: --input")
    end
    
    -- Validate numeric arguments
    local count_str = args.get_named("count")
    if count_str and not tonumber(count_str) then
        table.insert(errors, "Argument --count must be a number")
    end
    
    -- Validate file extensions
    local input_file = args.get_named("input")
    if input_file and not input_file:match("%.txt$") and not input_file:match("%.lua$") then
        table.insert(errors, "Input file must have .txt or .lua extension")
    end
    
    return #errors == 0, errors
end

local valid, errors = validate_arguments()
if valid then
    print("✓ All arguments are valid")
else
    print("✗ Argument validation failed:")
    for _, error in ipairs(errors) do
        print("  - " .. error)
    end
end

-- Test argument-based configuration
print("\n--- Dynamic Configuration ---")

local config = {
    verbose = args.has_flag("verbose"),
    debug = args.has_flag("debug"),
    dry_run = args.has_flag("dry-run"),
    output_format = args.get_named("format") or "text",
    max_iterations = tonumber(args.get_named("max") or "100"),
}

print("Configuration:")
print_table("config", config)

-- Demonstrate usage based on arguments
print("\n--- Usage Demonstration ---")

if config.verbose then
    print("Running in verbose mode...")
end

if config.debug then
    print("Debug information:")
    print("  Script: " .. (SCRIPT_NAME or "unknown"))
    print("  Args count: " .. (args.count or 0))
end

if config.dry_run then
    print("DRY RUN: Would process " .. input_file .. " and write to " .. output_file)
else
    print("Would process " .. input_file .. " and write to " .. output_file)
    print("Output format: " .. config.output_format)
    print("Max iterations: " .. config.max_iterations)
end

print("\n=== Advanced Test Complete ===")