print("Control flow test:")

for i = 1, 5 do
    print("Loop: " .. i)
end

local x = 10
if x > 5 then
    print("x is greater than 5")
elseif x == 5 then
    print("x is equal to 5")
else
    print("x is less than 5")
end

local i = 0
while i < 3 do
    print("While loop: " .. i)
    i = i + 1
end
