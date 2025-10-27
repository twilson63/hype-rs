# os - Operating System Information

> **Cross-platform utilities for system information, hardware stats, and environment details.**

## Table of Contents
- [Import](#import)
- [System Information](#system-information)
- [Hardware Information](#hardware-information)
- [User Information](#user-information)
- [Network Information](#network-information)
- [Constants](#constants)
- [Examples](#examples)

---

## Import

```lua
local os = require("os")
```

---

## System Information

### os.platform()

Get operating system platform.

**Returns:** `string` - Platform identifier: `"linux"`, `"macos"`, `"windows"`, `"freebsd"`, `"openbsd"`

**Example:**
```lua
local os = require("os")

local platform = os.platform()
print(platform)  -- "macos" or "linux" or "windows"

-- Platform-specific logic
if platform == "windows" then
    print("Running on Windows")
elseif platform == "macos" then
    print("Running on macOS")
elseif platform == "linux" then
    print("Running on Linux")
end
```

---

### os.arch()

Get CPU architecture.

**Returns:** `string` - Architecture: `"x86_64"`, `"aarch64"`, `"arm"`, `"x86"`

**Example:**
```lua
local os = require("os")

local arch = os.arch()
print(arch)  -- "x86_64" or "aarch64"

-- Architecture-specific code
if arch == "aarch64" then
    print("ARM64 architecture (Apple Silicon, AWS Graviton)")
elseif arch == "x86_64" then
    print("x86-64 architecture (Intel/AMD)")
end
```

---

### os.hostname()

Get system hostname.

**Returns:** `string` - System hostname

**Example:**
```lua
local os = require("os")

local hostname = os.hostname()
print("Hostname:", hostname)  -- "MacBook-Pro.local"

-- Use in logging
function log(message)
    local host = os.hostname()
    print("[" .. host .. "] " .. message)
end
```

---

### os.homedir()

Get user's home directory path.

**Returns:** `string` - Home directory path

**Example:**
```lua
local os = require("os")

local home = os.homedir()
print(home)  -- "/Users/john" or "C:\Users\john"

-- Cross-platform config path
local config_path = home .. "/.myapp/config.json"

-- Platform-specific
local platform = os.platform()
if platform == "windows" then
    config_path = home .. "\\AppData\\myapp\\config.json"
end
```

---

### os.tmpdir()

Get system temporary directory path.

**Returns:** `string` - Temp directory path

**Example:**
```lua
local os = require("os")

local tmpdir = os.tmpdir()
print(tmpdir)  -- "/tmp" or "C:\Users\user\AppData\Local\Temp"

-- Create temp file path
local temp_file = tmpdir .. "/myapp_" .. os.time() .. ".tmp"

-- Use for cache
local cache_dir = tmpdir .. "/myapp-cache"
```

---

### os.uptime()

Get system uptime in seconds.

**Returns:** `number` - Seconds since system boot

**Example:**
```lua
local os = require("os")

local uptime = os.uptime()
print("Uptime:", uptime .. " seconds")

-- Format uptime
local function format_uptime(seconds)
    local days = math.floor(seconds / 86400)
    local hours = math.floor((seconds % 86400) / 3600)
    local mins = math.floor((seconds % 3600) / 60)
    return string.format("%dd %dh %dm", days, hours, mins)
end

print("System uptime:", format_uptime(uptime))
-- "System uptime: 5d 12h 34m"
```

---

### os.loadavg()

Get system load average (Unix-like systems only).

**Returns:** `table` - Load averages: `[1min, 5min, 15min]`

**Example:**
```lua
local os = require("os")

-- Unix/Linux/macOS only
if os.platform() ~= "windows" then
    local load = os.loadavg()
    print("Load average (1m):", load[1])
    print("Load average (5m):", load[2])
    print("Load average (15m):", load[3])
    
    -- Health check
    if load[1] > 2.0 then
        print("Warning: High system load!")
    end
end

-- On Windows, returns [0, 0, 0]
```

---

## Hardware Information

### os.cpus()

Get CPU information.

**Returns:** `table` - Array of CPU info:
  - `model: string` - CPU model name
  - `speed: number` - CPU speed in MHz

**Example:**
```lua
local os = require("os")

local cpus = os.cpus()

-- CPU count
print("CPU cores:", #cpus)

-- First CPU info
print("Model:", cpus[1].model)
print("Speed:", cpus[1].speed .. " MHz")

-- All CPUs
for i, cpu in ipairs(cpus) do
    print(string.format("CPU %d: %s @ %d MHz", 
        i, cpu.model, cpu.speed))
end

-- Detect Apple Silicon
if cpus[1].model:find("Apple") then
    print("Running on Apple Silicon")
end
```

---

### os.totalmem()

Get total system memory in bytes.

**Returns:** `number` - Total memory in bytes

**Example:**
```lua
local os = require("os")

local total = os.totalmem()
print("Total memory:", total .. " bytes")

-- Format as GB
local total_gb = total / (1024 * 1024 * 1024)
print(string.format("Total memory: %.2f GB", total_gb))

-- Check minimum requirements
if total_gb < 4 then
    print("Warning: Low memory system")
end
```

---

### os.freemem()

Get free system memory in bytes.

**Returns:** `number` - Free memory in bytes

**Example:**
```lua
local os = require("os")

local free = os.freemem()
local total = os.totalmem()

-- Calculate usage
local used = total - free
local usage_percent = (used / total) * 100

print(string.format("Memory: %.1f%% used", usage_percent))
print(string.format("Free: %.2f GB", free / (1024^3)))
print(string.format("Used: %.2f GB", used / (1024^3)))

-- Memory warning
if usage_percent > 90 then
    print("Warning: High memory usage!")
end
```

---

## User Information

### os.userInfo()

Get current user information.

**Returns:** `table` - User info:
  - `username: string` - Username
  - `homedir: string` - Home directory path
  - `uid: number|nil` - User ID (Unix only)
  - `gid: number|nil` - Group ID (Unix only)
  - `shell: string|nil` - Shell path (Unix only)

**Example:**
```lua
local os = require("os")

local user = os.userInfo()
print("Username:", user.username)
print("Home:", user.homedir)

-- Unix-specific
if user.uid then
    print("UID:", user.uid)
    print("GID:", user.gid)
    print("Shell:", user.shell)
end

-- Check if running as root (Unix)
if user.uid == 0 then
    print("Warning: Running as root!")
end
```

---

## Network Information

### os.networkInterfaces()

Get network interfaces information.

**Returns:** `table` - Array of network interfaces:
  - `name: string` - Interface name (e.g., "en0", "eth0")
  - `mac: string` - MAC address

**Example:**
```lua
local os = require("os")

local interfaces = os.networkInterfaces()

-- List all interfaces
for _, iface in ipairs(interfaces) do
    print(string.format("%s: %s", iface.name, iface.mac))
end

-- Find Wi-Fi interface
for _, iface in ipairs(interfaces) do
    if iface.name:find("^en") or iface.name:find("^wl") then
        print("Wi-Fi MAC:", iface.mac)
        break
    end
end

-- Count interfaces
print("Network interfaces:", #interfaces)
```

---

## Constants

### os.EOL

End-of-line marker for the current platform.

**Type:** `string`
**Values:**
- Unix/Linux/macOS: `"\n"`
- Windows: `"\r\n"`

**Example:**
```lua
local os = require("os")

print("EOL marker:", os.EOL)  -- "\n" or "\r\n"

-- Write platform-appropriate line endings
local function write_lines(filename, lines)
    local content = table.concat(lines, os.EOL)
    -- Write content...
end

-- Parse platform-appropriate lines
local function parse_file(content)
    local lines = {}
    for line in content:gmatch("[^" .. os.EOL .. "]+") do
        table.insert(lines, line)
    end
    return lines
end
```

---

## Examples

### System Health Monitor

```lua
local os = require("os")
local time = require("time")

function system_health()
    local total_mem = os.totalmem()
    local free_mem = os.freemem()
    local used_mem = total_mem - free_mem
    local mem_percent = (used_mem / total_mem) * 100
    
    print("=== System Health ===")
    print("Platform:", os.platform(), os.arch())
    print("Hostname:", os.hostname())
    print("Uptime:", os.uptime() .. "s")
    print("CPUs:", #os.cpus())
    print(string.format("Memory: %.1f%% (%.2f GB / %.2f GB)",
        mem_percent,
        used_mem / (1024^3),
        total_mem / (1024^3)))
    
    if os.platform() ~= "windows" then
        local load = os.loadavg()
        print(string.format("Load: %.2f %.2f %.2f", 
            load[1], load[2], load[3]))
    end
end

system_health()
```

### Cross-Platform Paths

```lua
local os = require("os")

-- Get config directory
function get_config_dir()
    local home = os.homedir()
    local platform = os.platform()
    
    if platform == "windows" then
        return home .. "\\AppData\\Roaming\\myapp"
    elseif platform == "macos" then
        return home .. "/Library/Application Support/myapp"
    else  -- Linux
        return home .. "/.config/myapp"
    end
end

-- Get cache directory
function get_cache_dir()
    local platform = os.platform()
    
    if platform == "windows" then
        return os.tmpdir() .. "\\myapp-cache"
    else
        return os.tmpdir() .. "/myapp-cache"
    end
end

print("Config:", get_config_dir())
print("Cache:", get_cache_dir())
```

### System Report

```lua
local os = require("os")
local json = require("json")

function generate_system_report()
    local report = {
        platform = os.platform(),
        arch = os.arch(),
        hostname = os.hostname(),
        uptime = os.uptime(),
        user = os.userInfo(),
        cpu = {
            count = #os.cpus(),
            model = os.cpus()[1].model,
            speed = os.cpus()[1].speed
        },
        memory = {
            total = os.totalmem(),
            free = os.freemem(),
            used = os.totalmem() - os.freemem()
        },
        network = os.networkInterfaces()
    }
    
    return json.encode(report, true)
end

print(generate_system_report())
```

### Resource Monitor

```lua
local os = require("os")
local time = require("time")

function monitor_resources(interval_ms, duration_ms)
    local start = time.now()
    local samples = {}
    
    while time.elapsed(start) < duration_ms do
        local total = os.totalmem()
        local free = os.freemem()
        local usage = ((total - free) / total) * 100
        
        table.insert(samples, {
            timestamp = time.now(),
            memory_percent = usage
        })
        
        time.sleep(interval_ms)
    end
    
    -- Calculate statistics
    local sum = 0
    local max = 0
    local min = 100
    
    for _, sample in ipairs(samples) do
        sum = sum + sample.memory_percent
        max = math.max(max, sample.memory_percent)
        min = math.min(min, sample.memory_percent)
    end
    
    local avg = sum / #samples
    
    print(string.format("Memory usage over %dms:", duration_ms))
    print(string.format("  Average: %.1f%%", avg))
    print(string.format("  Min: %.1f%%", min))
    print(string.format("  Max: %.1f%%", max))
end

-- Monitor for 10 seconds, sample every second
monitor_resources(1000, 10000)
```

### Environment Detection

```lua
local os = require("os")

function detect_environment()
    local info = {
        platform = os.platform(),
        arch = os.arch(),
        is_mac = false,
        is_linux = false,
        is_windows = false,
        is_arm = false,
        is_64bit = false,
        is_container = false
    }
    
    -- Platform detection
    info.is_mac = info.platform == "macos"
    info.is_linux = info.platform == "linux"
    info.is_windows = info.platform == "windows"
    
    -- Architecture detection
    info.is_arm = info.arch:find("arm") ~= nil or 
                  info.arch:find("aarch64") ~= nil
    info.is_64bit = info.arch:find("64") ~= nil
    
    -- Container detection (simple check)
    local fs = require("fs")
    info.is_container = fs.existsSync("/.dockerenv") or
                       fs.existsSync("/run/.containerenv")
    
    return info
end

local env = detect_environment()
print("Platform:", env.platform)
print("ARM:", env.is_arm)
print("64-bit:", env.is_64bit)
print("Container:", env.is_container)
```

---

## Performance Notes

- All operations are fast (< 1ms)
- `loadavg()` - Unix only, returns `[0,0,0]` on Windows
- `networkInterfaces()` - May be slow on systems with many interfaces
- User/system info cached by OS, safe to call frequently

---

## Platform Differences

| Function | Linux | macOS | Windows | FreeBSD |
|----------|-------|-------|---------|---------|
| platform() | ✅ | ✅ | ✅ | ✅ |
| arch() | ✅ | ✅ | ✅ | ✅ |
| hostname() | ✅ | ✅ | ✅ | ✅ |
| homedir() | ✅ | ✅ | ✅ | ✅ |
| tmpdir() | ✅ | ✅ | ✅ | ✅ |
| cpus() | ✅ | ✅ | ✅ | ✅ |
| totalmem() | ✅ | ✅ | ✅ | ✅ |
| freemem() | ✅ | ✅ | ✅ | ✅ |
| uptime() | ✅ | ✅ | ✅ | ✅ |
| loadavg() | ✅ | ✅ | ⚠️ Returns [0,0,0] | ✅ |
| networkInterfaces() | ✅ | ✅ | ✅ | ✅ |
| userInfo() | ✅ Full | ✅ Full | ⚠️ No uid/gid/shell | ✅ Full |

---

## Error Handling

```lua
-- Most functions handle errors gracefully
local ok, hostname = pcall(os.hostname)
if not ok then
    print("Could not get hostname:", hostname)
end

-- User info may fail
local ok, user = pcall(os.userInfo)
if ok then
    print("Username:", user.username)
else
    print("Could not get user info")
end
```

---

## See Also

- [Process Module](./process.md) - Process control and environment variables
- [Examples](../../examples/os-demo.lua) - More examples
- [Tests](../../tests/os_module_test.rs) - Test suite

---

**Module**: os  
**Functions**: 12 + 1 constant  
**Status**: ✅ Production Ready  
**Last Updated**: October 27, 2025
