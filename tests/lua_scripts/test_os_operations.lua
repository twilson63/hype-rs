local os = require("os")

print("=== OS Module Integration Tests ===\n")

print("1. Testing os.platform()...")
local platform = os.platform()
assert(type(platform) == "string", "platform should return string")
assert(platform == "linux" or platform == "macos" or platform == "windows" or platform == "freebsd" or platform == "openbsd", "Invalid platform: " .. platform)
print("   ✓ Platform: " .. platform)

print("\n2. Testing os.arch()...")
local arch = os.arch()
assert(type(arch) == "string", "arch should return string")
assert(arch == "x86_64" or arch == "aarch64" or arch == "arm" or arch == "x86", "Invalid architecture: " .. arch)
print("   ✓ Architecture: " .. arch)

print("\n3. Testing os.hostname()...")
local hostname = os.hostname()
assert(type(hostname) == "string", "hostname should return string")
assert(#hostname > 0, "hostname should not be empty")
print("   ✓ Hostname: " .. hostname)

print("\n4. Testing os.homedir()...")
local homedir = os.homedir()
assert(type(homedir) == "string", "homedir should return string")
assert(#homedir > 0, "homedir should not be empty")
print("   ✓ Home directory: " .. homedir)

print("\n5. Testing os.tmpdir()...")
local tmpdir = os.tmpdir()
assert(type(tmpdir) == "string", "tmpdir should return string")
assert(#tmpdir > 0, "tmpdir should not be empty")
print("   ✓ Temp directory: " .. tmpdir)

print("\n6. Testing os.cpus()...")
local cpus = os.cpus()
assert(type(cpus) == "table", "cpus should return table")
assert(#cpus > 0, "cpus should not be empty")
assert(type(cpus[1].model) == "string", "cpu model should be string")
assert(type(cpus[1].speed) == "number", "cpu speed should be number")
print("   ✓ CPU count: " .. #cpus)
print("   ✓ CPU model: " .. cpus[1].model)
print("   ✓ CPU speed: " .. cpus[1].speed .. " MHz")

print("\n7. Testing os.totalmem()...")
local totalmem = os.totalmem()
assert(type(totalmem) == "number", "totalmem should return number")
assert(totalmem > 0, "totalmem should be positive")
local totalmem_gb = totalmem / 1024 / 1024 / 1024
print(string.format("   ✓ Total memory: %.2f GB", totalmem_gb))

print("\n8. Testing os.freemem()...")
local freemem = os.freemem()
assert(type(freemem) == "number", "freemem should return number")
assert(freemem > 0, "freemem should be positive")
local freemem_gb = freemem / 1024 / 1024 / 1024
print(string.format("   ✓ Free memory: %.2f GB", freemem_gb))

print("\n9. Testing memory relationship...")
assert(freemem <= totalmem, "free memory should be <= total memory")
local used_percent = ((totalmem - freemem) / totalmem) * 100
print(string.format("   ✓ Memory usage: %.1f%%", used_percent))

print("\n10. Testing os.uptime()...")
local uptime = os.uptime()
assert(type(uptime) == "number", "uptime should return number")
assert(uptime > 0, "uptime should be positive")
local uptime_hours = uptime / 3600
print(string.format("   ✓ System uptime: %.2f hours", uptime_hours))

print("\n11. Testing os.loadavg()...")
local loadavg = os.loadavg()
assert(type(loadavg) == "table", "loadavg should return table")
assert(type(loadavg[1]) == "number", "loadavg[1] should be number")
assert(type(loadavg[2]) == "number", "loadavg[2] should be number")
assert(type(loadavg[3]) == "number", "loadavg[3] should be number")
assert(loadavg[1] >= 0, "loadavg[1] should be non-negative")
assert(loadavg[2] >= 0, "loadavg[2] should be non-negative")
assert(loadavg[3] >= 0, "loadavg[3] should be non-negative")
print(string.format("   ✓ Load average: %.2f, %.2f, %.2f", loadavg[1], loadavg[2], loadavg[3]))

print("\n12. Testing os.networkInterfaces()...")
local interfaces = os.networkInterfaces()
assert(type(interfaces) == "table", "networkInterfaces should return table")
print("   ✓ Network interfaces found: " .. #interfaces)
if #interfaces > 0 then
    print("   ✓ First interface: " .. interfaces[1].name .. " (" .. interfaces[1].mac .. ")")
end

print("\n13. Testing os.userInfo()...")
local userInfo = os.userInfo()
assert(type(userInfo) == "table", "userInfo should return table")
assert(type(userInfo.username) == "string", "username should be string")
assert(#userInfo.username > 0, "username should not be empty")
assert(type(userInfo.homedir) == "string", "homedir should be string")
assert(#userInfo.homedir > 0, "homedir should not be empty")
print("   ✓ Username: " .. userInfo.username)
print("   ✓ Home directory: " .. userInfo.homedir)
if userInfo.uid then
    print("   ✓ UID: " .. userInfo.uid)
end
if userInfo.gid then
    print("   ✓ GID: " .. userInfo.gid)
end
if userInfo.shell then
    print("   ✓ Shell: " .. userInfo.shell)
end

print("\n14. Testing os.EOL...")
assert(type(os.EOL) == "string", "EOL should be string")
assert(os.EOL == "\n" or os.EOL == "\r\n", "EOL should be valid line ending")
local eol_display = os.EOL == "\n" and "\\n (Unix)" or "\\r\\n (Windows)"
print("   ✓ EOL: " .. eol_display)

print("\n=== All OS Module Tests Passed! ===")
