local time = require("time")

print("=== Time Module Integration Tests ===\n")

print("1. Testing time.now()...")
local now = time.now()
assert(type(now) == "number", "now should return number")
assert(now > 0, "now should be positive")
print("   ✓ Current timestamp: " .. now .. " ms")

print("\n2. Testing time.nowSeconds()...")
local sec = time.nowSeconds()
assert(type(sec) == "number", "nowSeconds should return number")
assert(sec > 0, "nowSeconds should be positive")
print("   ✓ Current timestamp: " .. sec .. " seconds")

print("\n3. Testing time.nowNanos()...")
local nanos = time.nowNanos()
assert(type(nanos) == "number", "nowNanos should return number")
assert(nanos > 0, "nowNanos should be positive")
print("   ✓ Nanoseconds available")

print("\n4. Testing ISO conversions...")
local timestamp = 1609459200000
local iso = time.toISO(timestamp)
assert(type(iso) == "string", "toISO should return string")
assert(string.find(iso, "2021"), "ISO string should contain year")
print("   ✓ toISO: " .. iso)
local parsed = time.fromISO(iso)
assert(parsed == timestamp, "fromISO should match original")
print("   ✓ fromISO roundtrip works")

print("\n5. Testing time.format()...")
local formatted = time.format(timestamp, "%Y-%m-%d")
assert(formatted == "2021-01-01", "format should work")
print("   ✓ Formatted date: " .. formatted)

print("\n6. Testing time.date()...")
local d = time.date(timestamp)
assert(type(d) == "table", "date should return table")
assert(d.year == 2021, "year should be 2021")
assert(d.month == 1, "month should be 1")
assert(d.day == 1, "day should be 1")
print("   ✓ Date: " .. d.year .. "-" .. d.month .. "-" .. d.day)
print("   ✓ Time: " .. d.hour .. ":" .. d.minute .. ":" .. d.second)
print("   ✓ Weekday: " .. d.weekday)

print("\n7. Testing time component extractors...")
assert(time.year(timestamp) == 2021, "year() should work")
assert(time.month(timestamp) == 1, "month() should work")
assert(time.day(timestamp) == 1, "day() should work")
print("   ✓ year(), month(), day() work")
local h = time.hour(timestamp)
local m = time.minute(timestamp)
local s = time.second(timestamp)
assert(h >= 0 and h < 24, "hour should be valid")
assert(m >= 0 and m < 60, "minute should be valid")
assert(s >= 0 and s < 60, "second should be valid")
print("   ✓ hour(), minute(), second() work")

print("\n8. Testing current time components...")
local current_date = time.date()
assert(current_date.year >= 2021, "current year should be valid")
assert(current_date.month >= 1 and current_date.month <= 12, "current month should be valid")
print("   ✓ Current date: " .. current_date.year .. "-" .. current_date.month .. "-" .. current_date.day)

print("\n9. Testing time.sleep()...")
local start = time.now()
time.sleep(50)
local elapsed_sleep = time.now() - start
assert(elapsed_sleep >= 50, "sleep should wait at least specified time")
print("   ✓ Slept for ~" .. math.floor(elapsed_sleep) .. " ms")

print("\n10. Testing time.elapsed()...")
local start_time = time.now()
time.sleep(20)
local elapsed_time = time.elapsed(start_time)
assert(elapsed_time >= 20, "elapsed should calculate time difference")
print("   ✓ Elapsed: " .. math.floor(elapsed_time) .. " ms")

print("\n11. Testing time.duration()...")
assert(time.duration(500) == "500ms", "500ms format failed")
assert(time.duration(1500) == "1.500s", "1.500s format failed")
assert(time.duration(65000) == "1m 5s", "1m 5s format failed")
assert(time.duration(3665000) == "1h 1m 5s", "1h 1m 5s format failed")
assert(time.duration(90065000) == "1d 1h 1m", "1d 1h 1m format failed")
print("   ✓ Duration formatting works")
print("     500ms → " .. time.duration(500))
print("     65s → " .. time.duration(65000))
print("     3665s → " .. time.duration(3665000))
print("     25h → " .. time.duration(90065000))

print("\n12. Testing practical use case - timing operations...")
local op_start = time.now()
local sum = 0
for i = 1, 10000 do
    sum = sum + i
end
local op_duration = time.elapsed(op_start)
print("   ✓ Loop took: " .. time.duration(op_duration))

print("\n13. Testing timestamp comparisons...")
local t1 = time.now()
time.sleep(10)
local t2 = time.now()
assert(t2 > t1, "timestamps should increase")
print("   ✓ Timestamp ordering works")

print("\n14. Testing date arithmetic...")
local now_ts = time.now()
local one_hour = 3600 * 1000
local future = now_ts + one_hour
local now_hour = time.hour(now_ts)
local future_hour = time.hour(future)
print("   ✓ Current hour: " .. now_hour)
print("   ✓ Hour + 1: " .. future_hour)

print("\n=== All Time Module Tests Passed! ===")
