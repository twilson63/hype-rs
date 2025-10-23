-- Environment variable writing test
print("=== Environment Variable Write Test ===")

-- Test setting a new variable
print("Setting TEST_VAR to 'hello_world'")
env.TEST_VAR = "hello_world"
print("TEST_VAR after setting:", env.TEST_VAR)

-- Test setting via method
print("Setting TEST_VAR2 via set() method")
local success = env.set("TEST_VAR2", "method_value")
print("set() success:", success)
print("TEST_VAR2:", env.TEST_VAR2)

-- Test updating existing variable
print("Updating TEST_VAR to 'updated_value'")
env.TEST_VAR = "updated_value"
print("TEST_VAR after update:", env.TEST_VAR)

-- Test setting numeric values
env.NUMBER_VAR = 42
print("NUMBER_VAR:", env.NUMBER_VAR)
print("Type of NUMBER_VAR:", type(env.NUMBER_VAR))

-- Test setting boolean values
env.BOOL_VAR = true
print("BOOL_VAR:", env.BOOL_VAR)
print("Type of BOOL_VAR:", type(env.BOOL_VAR))

-- Test unsetting variables
print("Unsetting TEST_VAR")
env.unset("TEST_VAR")
print("TEST_VAR after unset:", env.TEST_VAR or "not set")

-- Test setting to nil (should unset)
env.TEST_VAR2 = nil
print("TEST_VAR2 after setting to nil:", env.TEST_VAR2 or "not set")

print("=== Write test completed ===")