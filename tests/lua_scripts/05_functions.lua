local function factorial(n)
    if n <= 1 then
        return 1
    else
        return n * factorial(n - 1)
    end
end

print("Factorial function test:")
print("5! = " .. factorial(5))
print("10! = " .. factorial(10))

local function greet(name)
    return "Hello, " .. name .. "!"
end

print(greet("World"))
