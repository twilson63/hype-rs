local process = require("process")

print("=== Process Module Demo ===\n")

print("1. System Information:")
print("   Platform: " .. process.platform)
print("   Architecture: " .. process.arch)
print("   Process ID: " .. process.pid)
print("   Current Directory: " .. process.cwd())

print("\n2. Command-line Arguments:")
print("   Total arguments: " .. #process.argv)
for i, arg in ipairs(process.argv) do
    print("   argv[" .. i .. "] = " .. arg)
end

print("\n3. Environment Variables:")
print("   PATH = " .. (process.env.PATH or "not set"))
print("   HOME = " .. (process.env.HOME or process.env.USERPROFILE or "not set"))
print("   USER = " .. (process.env.USER or process.env.USERNAME or "not set"))

print("\n4. Working with Environment Variables:")
process.env.MY_APP_CONFIG = "development"
process.env.MY_APP_PORT = "3000"
print("   Set MY_APP_CONFIG = " .. process.env.MY_APP_CONFIG)
print("   Set MY_APP_PORT = " .. process.env.MY_APP_PORT)

print("\n5. Using getenv/setenv:")
process.setenv("MY_APP_DEBUG", "true")
local debug = process.getenv("MY_APP_DEBUG")
print("   MY_APP_DEBUG = " .. debug)

print("\n6. Directory Operations:")
local original_dir = process.cwd()
print("   Original directory: " .. original_dir)

local temp_dir = process.env.TMPDIR or process.env.TEMP or "/tmp"
print("   Changing to temp directory: " .. temp_dir)
process.chdir(temp_dir)
print("   New directory: " .. process.cwd())

print("   Restoring original directory...")
process.chdir(original_dir)
print("   Current directory: " .. process.cwd())

print("\n7. Cross-platform Path Handling:")
if process.platform == "windows" then
    print("   Running on Windows")
    print("   User directory: " .. (process.env.USERPROFILE or "unknown"))
elseif process.platform == "macos" then
    print("   Running on macOS")
    print("   Home directory: " .. (process.env.HOME or "unknown"))
else
    print("   Running on " .. process.platform)
    print("   Home directory: " .. (process.env.HOME or "unknown"))
end

print("\n8. Configuration from Environment:")
local config = {
    debug = process.env.DEBUG == "true" or process.env.DEBUG == "1",
    port = tonumber(process.env.PORT) or 8080,
    host = process.env.HOST or "localhost",
    environment = process.env.NODE_ENV or process.env.ENV or "production"
}
print("   Configuration loaded from environment:")
print("     debug = " .. tostring(config.debug))
print("     port = " .. config.port)
print("     host = " .. config.host)
print("     environment = " .. config.environment)

print("\n=== Demo Complete ===")
