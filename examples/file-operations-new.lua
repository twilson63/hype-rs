local fs = require("fs")

print("=== Filesystem Operations Demo ===\n")

local demo_dir = "/tmp/hype_demo"
local demo_file = demo_dir .. "/demo.txt"

print("1. Creating directory:", demo_dir)
fs.mkdirSync(demo_dir)

print("2. Writing file:", demo_file)
fs.writeFileSync(demo_file, "Hello from hype-rs filesystem module!")

print("3. Reading file back:")
local content = fs.readFileSync(demo_file)
print("   Content:", content)

print("4. Getting file stats:")
local stat = fs.statSync(demo_file)
print("   Size:", stat.size, "bytes")
print("   Is file:", stat.isFile)
print("   Is directory:", stat.isDirectory)

print("5. Listing directory contents:")
local files = fs.readdirSync(demo_dir)
print("   Found", #files, "file(s):")
for i, file in ipairs(files) do
    print("   -", file)
end

print("6. Cleaning up...")
fs.unlinkSync(demo_file)
fs.rmdirSync(demo_dir)

print("7. Verifying cleanup:")
print("   Directory exists:", fs.existsSync(demo_dir))

print("\n=== Demo Complete ===")
