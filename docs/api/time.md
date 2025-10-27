# time - Date and Time Operations

> **Comprehensive time utilities for timestamps, formatting, parsing, and duration calculations.**

## Table of Contents
- [Import](#import)
- [Current Time](#current-time)
- [Formatting & Parsing](#formatting--parsing)
- [ISO 8601](#iso-8601)
- [Date Components](#date-components)
- [Utilities](#utilities)
- [Examples](#examples)

---

## Import

```lua
local time = require("time")
```

---

## Current Time

### time.now()

Get current timestamp in milliseconds since Unix epoch.

**Returns:** `number` - Milliseconds since 1970-01-01 00:00:00 UTC

**Example:**
```lua
local time = require("time")

local now = time.now()
print(now)  -- 1698451200000

-- Measure execution time
local start = time.now()
-- ... some operation ...
local duration = time.now() - start
print("Took " .. duration .. "ms")
```

---

### time.nowSeconds()

Get current timestamp in seconds since Unix epoch.

**Returns:** `number` - Seconds since 1970-01-01 00:00:00 UTC

**Example:**
```lua
local time = require("time")

local now = time.nowSeconds()
print(now)  -- 1698451200

-- Unix timestamp for logging
local log_time = time.nowSeconds()
```

---

### time.nowNanos()

Get current timestamp in nanoseconds since Unix epoch.

**Returns:** `number` - Nanoseconds since 1970-01-01 00:00:00 UTC

**Example:**
```lua
local time = require("time")

local now = time.nowNanos()
print(now)  -- 1698451200000000000

-- High-precision timing
local start = time.nowNanos()
-- ... operation ...
local elapsed_ns = time.nowNanos() - start
print("Took " .. elapsed_ns .. "ns")
```

---

## Formatting & Parsing

### time.format(timestamp, format)

Format timestamp using custom format string.

**Parameters:**
- `timestamp: number` - Milliseconds since Unix epoch
- `format: string` - Format string (strftime-like)

**Returns:** `string` - Formatted date string

**Format Specifiers:**
- `%Y` - Year (4 digits): 2021
- `%m` - Month (01-12): 01
- `%d` - Day (01-31): 15
- `%H` - Hour 24h (00-23): 14
- `%M` - Minute (00-59): 30
- `%S` - Second (00-59): 45
- `%a` - Weekday short: Mon
- `%A` - Weekday full: Monday
- `%b` - Month short: Jan
- `%B` - Month full: January

**Example:**
```lua
local time = require("time")

local ts = 1609459200000  -- 2021-01-01 00:00:00 UTC

-- Common formats
print(time.format(ts, "%Y-%m-%d"))  -- "2021-01-01"
print(time.format(ts, "%H:%M:%S"))  -- "00:00:00"
print(time.format(ts, "%Y-%m-%d %H:%M:%S"))  -- "2021-01-01 00:00:00"

-- Readable format
print(time.format(ts, "%B %d, %Y"))  -- "January 01, 2021"
print(time.format(ts, "%A, %B %d"))  -- "Friday, January 01"

-- Custom format
local now = time.now()
local log_time = time.format(now, "[%Y-%m-%d %H:%M:%S]")
print(log_time)  -- "[2021-01-01 00:00:00]"
```

---

### time.parse(dateString, format)

Parse date string using custom format.

**Parameters:**
- `dateString: string` - Date string to parse
- `format: string` - Format string matching the input

**Returns:** `number` - Milliseconds since Unix epoch

**Example:**
```lua
local time = require("time")

-- Parse date
local ts = time.parse("2021-01-01", "%Y-%m-%d")
print(ts)  -- 1609459200000

-- Parse datetime
local ts2 = time.parse("2021-01-01 14:30:00", "%Y-%m-%d %H:%M:%S")

-- Parse log timestamp
local log_line = "[2021-01-01 14:30:45] Error occurred"
local ts3 = time.parse("2021-01-01 14:30:45", "%Y-%m-%d %H:%M:%S")
```

---

## ISO 8601

### time.toISO(timestamp)

Convert timestamp to ISO 8601 string.

**Parameters:**
- `timestamp: number` - Milliseconds since Unix epoch

**Returns:** `string` - ISO 8601 formatted string (RFC 3339)

**Example:**
```lua
local time = require("time")

local ts = 1609459200000
local iso = time.toISO(ts)
print(iso)  -- "2021-01-01T00:00:00Z"

-- Current time in ISO format
local now_iso = time.toISO(time.now())
print(now_iso)  -- "2025-10-27T12:30:45Z"

-- API timestamps
local created_at = time.toISO(time.now())
```

---

### time.fromISO(isoString)

Parse ISO 8601 string to timestamp.

**Parameters:**
- `isoString: string` - ISO 8601 formatted string

**Returns:** `number` - Milliseconds since Unix epoch

**Example:**
```lua
local time = require("time")

-- Parse ISO string
local ts = time.fromISO("2021-01-01T00:00:00Z")
print(ts)  -- 1609459200000

-- With timezone
local ts2 = time.fromISO("2021-01-01T00:00:00+00:00")

-- Parse API response
local api_timestamp = time.fromISO(response.created_at)
```

---

## Date Components

### time.date(timestamp?)

Get date components as table.

**Parameters:**
- `timestamp?: number` - Milliseconds (default: current time)

**Returns:** `table` - Date components:
  - `year: number` - Full year (e.g., 2021)
  - `month: number` - Month 1-12
  - `day: number` - Day 1-31
  - `hour: number` - Hour 0-23
  - `minute: number` - Minute 0-59
  - `second: number` - Second 0-59
  - `weekday: number` - Weekday 1-7 (1=Monday, 7=Sunday)

**Example:**
```lua
local time = require("time")

-- Current date
local d = time.date()
print("Year:", d.year)
print("Month:", d.month)
print("Day:", d.day)

-- Specific timestamp
local ts = 1609459200000
local d2 = time.date(ts)
print(d2.year)  -- 2021
print(d2.month)  -- 1
print(d2.day)  -- 1

-- Check if weekend
local weekday = d.weekday
if weekday >= 6 then
    print("It's the weekend!")
end
```

---

### time.year(timestamp?)

Get year from timestamp.

**Parameters:**
- `timestamp?: number` - Milliseconds (default: current time)

**Returns:** `number` - Year (e.g., 2021)

**Example:**
```lua
local time = require("time")

print(time.year())  -- 2025
print(time.year(1609459200000))  -- 2021
```

---

### time.month(timestamp?)

Get month from timestamp.

**Parameters:**
- `timestamp?: number` - Milliseconds (default: current time)

**Returns:** `number` - Month 1-12

**Example:**
```lua
local time = require("time")

print(time.month())  -- 10
print(time.month(1609459200000))  -- 1
```

---

### time.day(timestamp?)

Get day from timestamp.

**Parameters:**
- `timestamp?: number` - Milliseconds (default: current time)

**Returns:** `number` - Day 1-31

**Example:**
```lua
local time = require("time")

print(time.day())  -- 27
print(time.day(1609459200000))  -- 1
```

---

### time.hour(timestamp?)

Get hour from timestamp.

**Parameters:**
- `timestamp?: number` - Milliseconds (default: current time)

**Returns:** `number` - Hour 0-23

**Example:**
```lua
local time = require("time")

local hour = time.hour()
if hour >= 12 then
    print("Good afternoon!")
else
    print("Good morning!")
end
```

---

### time.minute(timestamp?)

Get minute from timestamp.

**Parameters:**
- `timestamp?: number` - Milliseconds (default: current time)

**Returns:** `number` - Minute 0-59

**Example:**
```lua
local time = require("time")

print(time.minute())  -- 30
```

---

### time.second(timestamp?)

Get second from timestamp.

**Parameters:**
- `timestamp?: number` - Milliseconds (default: current time)

**Returns:** `number` - Second 0-59

**Example:**
```lua
local time = require("time")

print(time.second())  -- 45
```

---

## Utilities

### time.sleep(ms)

Sleep for specified milliseconds (blocking).

**Parameters:**
- `ms: number` - Milliseconds to sleep

**Returns:** `nil`

**Example:**
```lua
local time = require("time")

print("Starting...")
time.sleep(1000)  -- Sleep 1 second
print("Done!")

-- Retry with backoff
local function retry_with_backoff(fn, max_attempts)
    for attempt = 1, max_attempts do
        local ok, result = pcall(fn)
        if ok then
            return result
        end
        time.sleep(1000 * attempt)  -- Exponential backoff
    end
end
```

---

### time.elapsed(start)

Calculate elapsed time since start timestamp.

**Parameters:**
- `start: number` - Start timestamp in milliseconds

**Returns:** `number` - Elapsed milliseconds

**Example:**
```lua
local time = require("time")

local start = time.now()
time.sleep(500)
local elapsed = time.elapsed(start)
print("Elapsed:", elapsed .. "ms")  -- ~500ms

-- Measure operation
local function measure(fn)
    local start = time.now()
    fn()
    return time.elapsed(start)
end

local duration = measure(function()
    -- some operation
end)
print("Took " .. duration .. "ms")
```

---

### time.duration(ms)

Format duration in human-readable form.

**Parameters:**
- `ms: number` - Duration in milliseconds

**Returns:** `string` - Formatted duration (e.g., "2h 30m 15s")

**Formats:**
- `< 1s` - "500ms"
- `< 1m` - "45s"
- `< 1h` - "5m 30s"
- `< 1d` - "2h 15m"
- `>= 1d` - "3d 4h 30m"

**Example:**
```lua
local time = require("time")

print(time.duration(500))  -- "500ms"
print(time.duration(5000))  -- "5s"
print(time.duration(90000))  -- "1m 30s"
print(time.duration(3661000))  -- "1h 1m 1s"
print(time.duration(90061000))  -- "1d 1h 1m"

-- Format elapsed time
local start = time.now()
-- ... operation ...
local elapsed = time.elapsed(start)
print("Operation took " .. time.duration(elapsed))
```

---

## Examples

### Timing Operations

```lua
local time = require("time")

-- Simple timer
local function timer(name, fn)
    local start = time.now()
    fn()
    local elapsed = time.elapsed(start)
    print(name .. " took " .. time.duration(elapsed))
end

timer("Database query", function()
    time.sleep(100)
end)
-- Output: "Database query took 100ms"
```

### Logging with Timestamps

```lua
local time = require("time")

local function log(level, message)
    local timestamp = time.format(time.now(), "%Y-%m-%d %H:%M:%S")
    print("[" .. timestamp .. "] " .. level .. ": " .. message)
end

log("INFO", "Server started")
-- [2025-10-27 12:30:45] INFO: Server started
```

### Date Arithmetic

```lua
local time = require("time")

-- Add days
local function add_days(timestamp, days)
    return timestamp + (days * 24 * 60 * 60 * 1000)
end

local now = time.now()
local tomorrow = add_days(now, 1)
local next_week = add_days(now, 7)

print("Tomorrow:", time.toISO(tomorrow))
print("Next week:", time.toISO(next_week))
```

### Age Calculation

```lua
local time = require("time")

local function calculate_age(birth_timestamp)
    local birth = time.date(birth_timestamp)
    local current = time.date()
    
    local age = current.year - birth.year
    if current.month < birth.month or 
       (current.month == birth.month and current.day < birth.day) then
        age = age - 1
    end
    
    return age
end

local birthdate = time.parse("1990-01-15", "%Y-%m-%d")
print("Age:", calculate_age(birthdate))
```

### Cron-like Scheduling

```lua
local time = require("time")

local function run_at_hour(hour, fn)
    while true do
        local current_hour = time.hour()
        if current_hour == hour then
            fn()
            time.sleep(3600000)  -- Sleep 1 hour
        else
            time.sleep(60000)  -- Check every minute
        end
    end
end

-- Run backup at 2 AM
run_at_hour(2, function()
    print("Running backup at " .. time.format(time.now(), "%H:%M:%S"))
end)
```

### Time Zones (UTC only)

```lua
local time = require("time")

-- Note: All timestamps are in UTC
-- For local time, adjust manually

local function to_local_hour(utc_hour, offset_hours)
    return (utc_hour + offset_hours) % 24
end

local utc_hour = time.hour()
local est_hour = to_local_hour(utc_hour, -5)  -- EST = UTC-5
local pst_hour = to_local_hour(utc_hour, -8)  -- PST = UTC-8

print("UTC:", utc_hour)
print("EST:", est_hour)
print("PST:", pst_hour)
```

---

## Performance Notes

- `time.now()` - Very fast (< 1μs)
- `time.format()` - Fast (< 100μs)
- `time.parse()` - Fast (< 100μs)
- `time.sleep()` - Blocking (use for delays, not async)
- All operations use system clock (UTC)

---

## Common Patterns

**Uptime Tracker:**
```lua
local start_time = time.now()

function get_uptime()
    return time.duration(time.elapsed(start_time))
end

print("Uptime:", get_uptime())
```

**Timeout Handler:**
```lua
local function with_timeout(fn, timeout_ms)
    local start = time.now()
    while time.elapsed(start) < timeout_ms do
        local ok, result = pcall(fn)
        if ok then
            return result
        end
        time.sleep(100)
    end
    error("Operation timed out")
end
```

**Rate Limiting:**
```lua
local last_call = 0
local min_interval = 1000  -- 1 second

function rate_limited_action()
    local now = time.now()
    if now - last_call < min_interval then
        local wait = min_interval - (now - last_call)
        time.sleep(wait)
    end
    last_call = time.now()
    -- Do action
end
```

---

## Error Handling

```lua
-- Invalid format string
local ok, err = pcall(function()
    return time.format(time.now(), "%Z")  -- Invalid specifier
end)

-- Invalid date string
local ok, err = pcall(function()
    return time.parse("invalid", "%Y-%m-%d")
end)

-- Invalid ISO string
local ok, err = pcall(function()
    return time.fromISO("not-iso")
end)
```

---

## Comparison with os.time()

| Operation | os.time() | time module |
|-----------|-----------|-------------|
| Timestamp | Seconds | Milliseconds (precise) |
| Format | os.date() | time.format() (custom) |
| Parse | Limited | time.parse() (flexible) |
| ISO 8601 | Manual | time.toISO() ✅ |
| Components | os.date("*t") | time.date() ✅ |
| Duration | Manual | time.duration() ✅ |
| Sleep | None | time.sleep() ✅ |

---

## See Also

- [Examples](../../examples/time-demo.lua) - More examples
- [Tests](../../tests/time_module_test.rs) - Test suite
- [Chrono docs](https://docs.rs/chrono/) - Underlying library

---

**Module**: time  
**Functions**: 17  
**Status**: ✅ Production Ready  
**Last Updated**: October 27, 2025
