-- Script that demonstrates argument help and usage

print("=== Argument Help and Usage Demo ===")

-- Function to display usage information
function show_usage()
    print("Usage: " .. (arg and arg[0] or "script") .. " [options] [arguments]")
    print("")
    print("This script demonstrates comprehensive argument handling.")
    print("")
    print("Options:")
    print("  --name=<string>      Set your name (default: World)")
    print("  --count=<number>      Set iteration count (default: 1)")
    print("  --input=<file>        Input file path")
    print("  --output=<file>       Output file path (default: output.txt)")
    print("  --format=<type>       Output format: text, json, xml (default: text)")
    print("  --max=<number>        Maximum iterations (default: 100)")
    print("")
    print("Flags:")
    print("  --verbose             Enable verbose output")
    print("  --debug               Enable debug information")
    print("  --dry-run             Show what would be done without doing it")
    print("  --help                Show this help message")
    print("")
    print("Examples:")
    print("  " .. (arg and arg[0] or "script") .. " --name=John --verbose")
    print("  " .. (arg and arg[0] or "script") .. " --input=data.txt --output=result.txt --format=json")
    print("  " .. (arg and arg[0] or "script") .. " --count=5 --max=10 --debug")
    print("")
end

-- Check for help flag
if args.has_flag and args.has_flag("help") then
    show_usage()
    return
end

-- Show current argument state
print("Current Arguments:")
print("Script name:", arg and arg[0] or "unknown")

if args then
    print("Total arguments:", args.count or #args)
    
    -- Show indexed arguments
    if #args > 0 then
        print("Positional arguments:")
        for i = 1, #args do
            print(string.format("  [%d] = %s", i, tostring(args[i])))
        end
    end
    
    -- Show named arguments
    if args.named and next(args.named) then
        print("Named arguments:")
        for key, value in pairs(args.named) do
            print(string.format("  --%s = %s", key, value))
        end
    end
    
    -- Show flags
    if args.flags and next(args.flags) then
        print("Flags:")
        for flag, enabled in pairs(args.flags) do
            if enabled then
                print(string.format("  --%s", flag))
            end
        end
    end
else
    print("No argument parsing available")
end

-- Demonstrate argument-based behavior
print("\n--- Behavior Based on Arguments ---")

local name = args.get_named and args.get_named("name") or "World"
local count = tonumber(args.get_named and args.get_named("count") or "1") or 1
local verbose = args.has_flag and args.has_flag("verbose") or false
local debug = args.has_flag and args.has_flag("debug") or false

print(string.format("Hello, %s!", name))

if count > 1 then
    print(string.format("Running %d iterations:", count))
    for i = 1, count do
        if verbose then
            print(string.format("  Iteration %d: Processing...", i))
        else
            print(string.format("  Iteration %d", i))
        end
    end
else
    print("Running single iteration")
end

if debug then
    print("\nDebug Information:")
    print("  Name parameter:", name)
    print("  Count parameter:", count)
    print("  Verbose flag:", verbose)
    print("  Debug flag:", debug)
    
    if hype and hype.args then
        print("  Argument stats:")
        print("    Indexed:", hype.args.indexed_count or 0)
        print("    Named:", hype.args.named_count or 0)
        print("    Flags:", hype.args.flags_count or 0)
    end
end

-- Show file processing example
local input_file = args.get_named and args.get_named("input")
local output_file = args.get_named and args.get_named("output") or "output.txt"
local format = args.get_named and args.get_named("format") or "text"

if input_file then
    print(string.format("\nFile Processing:")
    print("  Input:  %s", input_file)
    print("  Output: %s", output_file)
    print("  Format: %s", format))
    
    if args.has_flag and args.has_flag("dry-run") then
        print("  Mode: DRY RUN (no actual processing)")
    else
        print("  Mode: NORMAL (would process file)")
    end
end

print("\n=== Help Demo Complete ===")
print("Run with --help to see usage information")