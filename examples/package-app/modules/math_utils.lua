-- Math Utilities Module
-- Provides mathematical operations and utilities
-- Used by package-app example

local function validate_number(value, name)
    if type(value) ~= "number" then
        error(name .. " must be a number, got " .. type(value))
    end
end

local function validate_positive(value, name)
    validate_number(value, name)
    if value < 0 then
        error(name .. " must be positive, got " .. value)
    end
end

module.exports = {
    version = "1.0.0",
    
    add = function(a, b)
        validate_number(a, "a")
        validate_number(b, "b")
        return a + b
    end,
    
    subtract = function(a, b)
        validate_number(a, "a")
        validate_number(b, "b")
        return a - b
    end,
    
    multiply = function(a, b)
        validate_number(a, "a")
        validate_number(b, "b")
        return a * b
    end,
    
    divide = function(a, b)
        validate_number(a, "a")
        validate_number(b, "b")
        if b == 0 then
            error("Cannot divide by zero")
        end
        return a / b
    end,
    
    modulo = function(a, b)
        validate_number(a, "a")
        validate_number(b, "b")
        if b == 0 then
            error("Cannot perform modulo with zero")
        end
        return a % b
    end,
    
    power = function(base, exponent)
        validate_number(base, "base")
        validate_number(exponent, "exponent")
        return base ^ exponent
    end,
    
    square_root = function(n)
        validate_positive(n, "n")
        return n ^ 0.5
    end,
    
    absolute = function(n)
        validate_number(n, "n")
        return n < 0 and -n or n
    end,
    
    min = function(...)
        local args = {...}
        if #args == 0 then
            error("min requires at least one argument")
        end
        for _, v in ipairs(args) do
            validate_number(v, "argument")
        end
        local result = args[1]
        for i = 2, #args do
            if args[i] < result then
                result = args[i]
            end
        end
        return result
    end,
    
    max = function(...)
        local args = {...}
        if #args == 0 then
            error("max requires at least one argument")
        end
        for _, v in ipairs(args) do
            validate_number(v, "argument")
        end
        local result = args[1]
        for i = 2, #args do
            if args[i] > result then
                result = args[i]
            end
        end
        return result
    end,
    
    average = function(...)
        local args = {...}
        if #args == 0 then
            error("average requires at least one argument")
        end
        local sum = 0
        for _, v in ipairs(args) do
            validate_number(v, "argument")
            sum = sum + v
        end
        return sum / #args
    end,
    
    factorial = function(n)
        validate_number(n, "n")
        if n < 0 then
            error("Cannot compute factorial of negative number")
        end
        if n ~= math.floor(n) then
            error("Factorial requires integer input")
        end
        if n == 0 or n == 1 then
            return 1
        end
        local result = 1
        for i = 2, n do
            result = result * i
        end
        return result
    end,
    
    gcd = function(a, b)
        validate_number(a, "a")
        validate_number(b, "b")
        a = math.floor(a)
        b = math.floor(b)
        while b ~= 0 do
            local temp = b
            b = a % b
            a = temp
        end
        return math.abs(a)
    end,
}
