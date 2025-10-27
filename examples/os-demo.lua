local os = require("os")

print("=== OS Module Demo ===\n")

print("System Information:")
print("  Platform: " .. os.platform())
print("  Architecture: " .. os.arch())
print("  Hostname: " .. os.hostname())

print("\nDirectories:")
print("  Home: " .. os.homedir())
print("  Temp: " .. os.tmpdir())

print("\nCPU Information:")
local cpus = os.cpus()
print("  CPU Count: " .. #cpus)
if #cpus > 0 then
    print("  Model: " .. cpus[1].model)
    print("  Speed: " .. cpus[1].speed .. " MHz")
end

print("\nMemory Information:")
local totalmem = os.totalmem()
local freemem = os.freemem()
local totalmem_gb = totalmem / 1024 / 1024 / 1024
local freemem_gb = freemem / 1024 / 1024 / 1024
local used_gb = totalmem_gb - freemem_gb
local used_percent = (used_gb / totalmem_gb) * 100

print(string.format("  Total: %.2f GB", totalmem_gb))
print(string.format("  Used: %.2f GB (%.1f%%)", used_gb, used_percent))
print(string.format("  Free: %.2f GB", freemem_gb))

print("\nSystem Status:")
local uptime = os.uptime()
local uptime_hours = uptime / 3600
local uptime_days = uptime / 86400
print(string.format("  Uptime: %.2f hours (%.2f days)", uptime_hours, uptime_days))

local loadavg = os.loadavg()
print(string.format("  Load Average: %.2f, %.2f, %.2f", loadavg[1], loadavg[2], loadavg[3]))

print("\nNetwork Interfaces:")
local interfaces = os.networkInterfaces()
if #interfaces > 0 then
    for i, iface in ipairs(interfaces) do
        print(string.format("  %d. %s (MAC: %s)", i, iface.name, iface.mac))
    end
else
    print("  No network interfaces found")
end

print("\nUser Information:")
local userInfo = os.userInfo()
print("  Username: " .. userInfo.username)
print("  Home: " .. userInfo.homedir)
if userInfo.uid then
    print("  UID: " .. userInfo.uid)
end
if userInfo.gid then
    print("  GID: " .. userInfo.gid)
end
if userInfo.shell then
    print("  Shell: " .. userInfo.shell)
end

print("\nPlatform Details:")
local eol_display = os.EOL == "\n" and "\\n (Unix)" or "\\r\\n (Windows)"
print("  EOL Marker: " .. eol_display)

print("\n=== Demo Complete ===")
