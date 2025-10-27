local time = require("time")

print("=== Time Module Demo ===\n")

print("1. Current Time:")
local now = time.now()
print("  Milliseconds: " .. now)
print("  Seconds: " .. time.nowSeconds())
print("  ISO format: " .. time.toISO(now))

print("\n2. Date Components:")
local current = time.date()
print(string.format("  Date: %04d-%02d-%02d", current.year, current.month, current.day))
print(string.format("  Time: %02d:%02d:%02d", current.hour, current.minute, current.second))
print("  Weekday: " .. current.weekday .. " (0=Monday)")

print("\n3. Formatting Timestamps:")
local timestamp = 1609459200000
print("  Timestamp: " .. timestamp)
print("  ISO: " .. time.toISO(timestamp))
print("  Date: " .. time.format(timestamp, "%Y-%m-%d"))
print("  Full: " .. time.format(timestamp, "%Y-%m-%d %H:%M:%S"))
print("  Custom: " .. time.format(timestamp, "%B %d, %Y"))

print("\n4. Parsing Dates:")
local iso_string = "2021-06-15T10:30:00+00:00"
local parsed = time.fromISO(iso_string)
print("  Parsed: " .. iso_string)
print("  Timestamp: " .. parsed)
print("  Year: " .. time.year(parsed))

print("\n5. Duration Formatting:")
local durations = {
    {ms = 250, desc = "Quarter second"},
    {ms = 5000, desc = "Five seconds"},
    {ms = 90000, desc = "Minute and a half"},
    {ms = 3665000, desc = "Hour and minute"},
    {ms = 90065000, desc = "Day and hour"}
}
for _, d in ipairs(durations) do
    print("  " .. d.desc .. ": " .. time.duration(d.ms))
end

print("\n6. Measuring Execution Time:")
local start = time.now()
local result = 0
for i = 1, 100000 do
    result = result + i
end
local elapsed = time.elapsed(start)
print("  Computed sum: " .. result)
print("  Time taken: " .. time.duration(elapsed))

print("\n7. Sleep Demo:")
print("  Sleeping for 100ms...")
local sleep_start = time.now()
time.sleep(100)
local sleep_duration = time.elapsed(sleep_start)
print("  Actually slept: " .. math.floor(sleep_duration) .. "ms")

print("\n8. Time Travel (Date Math):")
local base = time.now()
local minute = 60 * 1000
local hour = 60 * minute
local day = 24 * hour

print("  Now: " .. time.format(base, "%H:%M:%S"))
print("  +1 hour: " .. time.format(base + hour, "%H:%M:%S"))
print("  +1 day: " .. time.format(base + day, "%Y-%m-%d"))

print("\n9. Birthday Calculator:")
local birthday = time.fromISO("1990-05-15T00:00:00+00:00")
local age_ms = time.now() - birthday
local age_days = math.floor(age_ms / (24 * 3600 * 1000))
local age_years = math.floor(age_days / 365.25)
print("  Birthday: " .. time.format(birthday, "%B %d, %Y"))
print("  Days alive: " .. age_days)
print("  Age: ~" .. age_years .. " years")

print("\n10. Countdown Timer:")
local countdown_start = time.now()
local target = countdown_start + 500
print("  Starting countdown...")
while time.now() < target do
    local remaining = target - time.now()
    if remaining % 100 < 10 then
        print("    " .. math.ceil(remaining) .. "ms remaining...")
    end
end
print("  Countdown complete!")

print("\n11. Timestamp Comparison:")
local timestamps = {
    time.fromISO("2021-01-01T00:00:00+00:00"),
    time.fromISO("2022-06-15T12:00:00+00:00"),
    time.fromISO("2023-12-31T23:59:59+00:00")
}
print("  Sorted dates:")
table.sort(timestamps)
for _, ts in ipairs(timestamps) do
    print("    " .. time.format(ts, "%Y-%m-%d %H:%M:%S"))
end

print("\n12. Scheduling Info:")
local schedule = {
    {name = "Daily backup", next = time.now() + (24 * 3600 * 1000)},
    {name = "Weekly report", next = time.now() + (7 * 24 * 3600 * 1000)},
    {name = "Monthly review", next = time.now() + (30 * 24 * 3600 * 1000)}
}
print("  Scheduled tasks:")
for _, task in ipairs(schedule) do
    local until_next = task.next - time.now()
    print("    " .. task.name .. ": " .. time.duration(until_next))
end

print("\n=== Demo Complete ===")
