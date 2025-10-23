-- Test script for timeout
print("Starting long-running script...")

-- Simulate a long operation
local start = os.clock()
while os.clock() - start < 10 do
    -- Do nothing, just wait
end

print("This should not be reached due to timeout")