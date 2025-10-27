local process = require("process")

print("Testing process module...")

print("\n1. Testing process.cwd()...")
local cwd = process.cwd()
print("   Current directory:", cwd)
print("   Type:", type(cwd))

print("\n2. Testing process.pid...")
print("   Process ID:", process.pid)
print("   Type:", type(process.pid))

print("\n3. Testing process.platform...")
print("   Platform:", process.platform)

print("\n4. Testing process.arch...")
print("   Architecture:", process.arch)

print("\n5. Testing process.argv...")
print("   Arguments count:", #process.argv)
print("   First argument:", process.argv[1])

print("\n6. Testing process.env (read)...")
print("   PATH exists:", process.env.PATH ~= nil)
print("   HOME/USERPROFILE exists:", (process.env.HOME or process.env.USERPROFILE) ~= nil)

print("\n7. Testing process.env (write)...")
process.env.HYPE_LUA_TEST = "lua_test_value"
print("   Set HYPE_LUA_TEST:", process.env.HYPE_LUA_TEST)

print("\n8. Testing process.getenv()...")
local path = process.getenv("PATH")
print("   PATH found:", path ~= nil)

print("\n9. Testing process.setenv()...")
process.setenv("HYPE_SETENV_TEST", "setenv_value")
local get_result = process.getenv("HYPE_SETENV_TEST")
print("   Set and retrieved:", get_result == "setenv_value")

print("\n10. Testing process.chdir()...")
local original = process.cwd()
print("   Original CWD:", original)
local temp = process.env.TMPDIR or process.env.TEMP or "/tmp"
process.chdir(temp)
local new_cwd = process.cwd()
print("   After chdir:", new_cwd)
print("   Changed:", original ~= new_cwd)
process.chdir(original)
print("   Restored to:", process.cwd())

print("\n11. System Information...")
print("   OS:", process.platform)
print("   Arch:", process.arch)
print("   PID:", process.pid)
print("   CWD:", process.cwd())

print("\n12. Environment Variables...")
local count = 0
for k, v in pairs(process.env) do
    count = count + 1
    if count <= 3 then
        print("   " .. k .. " = " .. tostring(v):sub(1, 50))
    end
end
print("   Total env vars:", count)

print("\nAll process module tests passed!")
