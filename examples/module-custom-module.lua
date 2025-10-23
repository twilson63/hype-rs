-- Custom Module with Exports
-- Demonstrates creating and using custom modules with module.exports pattern
--
-- Run with:
--   hype --module examples/module-custom-module.lua
-- or:
--   hype -m examples/module-custom-module.lua

print("═══════════════════════════════════════════════════════")
print("  Custom Module Export Example")
print("═══════════════════════════════════════════════════════\n")

-- First, let's define what a custom module looks like
-- In a real project, this would be in a separate file

print("--- Understanding module.exports pattern ---\n")

print([[
In a real custom module file (e.g., mymodule.lua):

  -- Define private functions
  local function add(a, b)
    return a + b
  end
  
  local function subtract(a, b)
    return a - b
  end
  
  -- Export the public API
  module.exports = {
    add = add,
    subtract = subtract,
    version = "1.0.0"
  }

When you require() this module, you get back the
module.exports table with only the public functions.
]])

print("\n--- Simulating a custom math utilities module ---\n")

-- Simulate what a custom module would define
local MathUtils = {
    -- Add two numbers
    add = function(a, b)
        return a + b
    end,
    
    -- Subtract two numbers
    subtract = function(a, b)
        return a - b
    end,
    
    -- Multiply two numbers
    multiply = function(a, b)
        return a * b
    end,
    
    -- Divide two numbers with error handling
    divide = function(a, b)
        if b == 0 then
            return nil, "Cannot divide by zero"
        end
        return a / b
    end,
    
    -- Calculate power
    power = function(base, exponent)
        return base ^ exponent
    end,
    
    -- Module metadata
    __id = "math-utils",
    version = "1.0.0",
    author = "Hype-RS Examples"
}

print("Module functions available:")
print("  • add(a, b)")
print("  • subtract(a, b)")
print("  • multiply(a, b)")
print("  • divide(a, b)")
print("  • power(base, exponent)")
print()

-- Example 1: Using exported functions
print("--- Example 1: Using module functions ---")
print("Math operations:")
print("  add(10, 5) =", MathUtils.add(10, 5))
print("  subtract(10, 5) =", MathUtils.subtract(10, 5))
print("  multiply(10, 5) =", MathUtils.multiply(10, 5))

local result, err = MathUtils.divide(10, 5)
if result then
    print("  divide(10, 5) =", result)
else
    print("  divide(10, 5) error:", err)
end

print("  power(2, 8) =", MathUtils.power(2, 8))
print()

-- Example 2: Module metadata
print("--- Example 2: Module metadata ---")
print("Module ID:", MathUtils.__id)
print("Module version:", MathUtils.version)
print("Module author:", MathUtils.author)
print()

-- Example 3: Error handling in modules
print("--- Example 3: Error handling ---")
print("Testing divide by zero:")
local result, err = MathUtils.divide(100, 0)
if result then
    print("  Result:", result)
else
    print("  Error caught:", err)
    print("  ✓ Error handling works correctly")
end
print()

-- Example 4: Private vs Public pattern explanation
print("--- Example 4: Private vs Public functions ---")
print([[
In the custom module:
  local function privateFunc() ... end  -- NOT in exports
  
  module.exports = {
    publicFunc = function() ... end     -- In exports
  }

When you require() the module, only functions in
module.exports are accessible. Private functions
are hidden from external code.

This enables:
  ✓ Encapsulation
  ✓ API boundaries
  ✓ Implementation hiding
]])
print()

-- Example 5: Chaining operations
print("--- Example 5: Chaining module operations ---")
local operation_sequence = {
    {"add", {20, 30}},
    {"multiply", {10}},
    {"power", {2}}
}

local value = 5
print("Starting value:", value)

for _, op in ipairs(operation_sequence) do
    local func_name = op[1]
    local args = op[2]
    local func = MathUtils[func_name]
    
    if func_name == "multiply" then
        value = func(value, args[1])
        print("After multiply(" .. value / args[1] .. ", " .. args[1] .. "):", value)
    elseif func_name == "add" then
        value = func(value, args[1])
        print("After add(result, " .. args[1] .. "):", value)
    elseif func_name == "power" then
        value = func(value, args[1])
        print("After power(result, " .. args[1] .. "):", value)
    end
end
print()

-- Example 6: Returning complex values
print("--- Example 6: Module returning complex structures ---")

local ComplexModule = {
    __id = "complex-module",
    
    -- Function returning a table (complex value)
    createVector = function(x, y, z)
        return {
            x = x, y = y, z = z,
            magnitude = function(self)
                return math.sqrt(self.x^2 + self.y^2 + self.z^2)
            end,
            add = function(self, other)
                return ComplexModule.createVector(
                    self.x + other.x,
                    self.y + other.y,
                    self.z + other.z
                )
            end
        }
    end
}

local v1 = ComplexModule.createVector(1, 2, 3)
local v2 = ComplexModule.createVector(4, 5, 6)
local v_sum = v1:add(v2)

print("Vector 1: (" .. v1.x .. ", " .. v1.y .. ", " .. v1.z .. ")")
print("Vector 2: (" .. v2.x .. ", " .. v2.y .. ", " .. v2.z .. ")")
print("Sum: (" .. v_sum.x .. ", " .. v_sum.y .. ", " .. v_sum.z .. ")")
print("Magnitude of sum:", string.format("%.2f", v_sum:magnitude()))
print()

print("═══════════════════════════════════════════════════════")
print("  Example completed successfully!")
print("═══════════════════════════════════════════════════════\n")
