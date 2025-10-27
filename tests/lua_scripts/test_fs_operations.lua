local fs = require("fs")

print("Testing fs module...")

local test_dir = "/tmp/hype_fs_test"
local test_file = test_dir .. "/test.txt"

print("\n1. Testing mkdirSync (recursive)...")
fs.mkdirSync(test_dir)
print("   Directory created: " .. test_dir)

print("\n2. Testing existsSync...")
if fs.existsSync(test_dir) then
    print("   Directory exists: true")
else
    print("   ERROR: Directory should exist!")
end

print("\n3. Testing writeFileSync...")
fs.writeFileSync(test_file, "Hello from hype-rs!")
print("   File written: " .. test_file)

print("\n4. Testing readFileSync...")
local content = fs.readFileSync(test_file)
print("   File content: " .. content)

print("\n5. Testing statSync...")
local stat = fs.statSync(test_file)
print("   File size: " .. stat.size)
print("   Is file: " .. tostring(stat.isFile))
print("   Is directory: " .. tostring(stat.isDirectory))

print("\n6. Testing readdirSync...")
local files = fs.readdirSync(test_dir)
print("   Files in directory: " .. #files)
for i, file in ipairs(files) do
    print("     - " .. file)
end

print("\n7. Testing unlinkSync...")
fs.unlinkSync(test_file)
print("   File deleted: " .. test_file)

print("\n8. Testing rmdirSync...")
fs.rmdirSync(test_dir)
print("   Directory removed: " .. test_dir)

print("\n9. Verifying cleanup...")
if not fs.existsSync(test_dir) then
    print("   Cleanup successful: directory no longer exists")
else
    print("   ERROR: Directory should not exist!")
end

print("\nAll fs module tests passed!")
