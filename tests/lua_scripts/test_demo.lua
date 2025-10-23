print("=== Hype-RS Test Demo ===")
print()

print("1. Basic Output")
print("Hello from Lua!")
print()

print("2. Variables and Math")
local x = 10
local y = 20
local sum = x + y
print("x = " .. x .. ", y = " .. y .. ", sum = " .. sum)
print()

print("3. Tables")
local person = {
    name = "Alice",
    age = 30,
    city = "New York"
}
print("Person: " .. person.name .. ", age " .. person.age .. " from " .. person.city)
print()

print("4. Functions")
local function greet(name)
    return "Hello, " .. name .. "!"
end
print(greet("Bob"))
print()

print("5. Loops")
print("Counting from 1 to 5:")
for i = 1, 5 do
    print("  " .. i)
end
print()

print("6. Script Arguments")
if arg then
    print("Number of arguments: " .. (#arg - 1))
    for i = 1, #arg do
        print("  arg[" .. i .. "] = " .. arg[i])
    end
else
    print("No arguments passed")
end
print()

print("=== Test Complete ===")
