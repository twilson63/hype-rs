local t = {1, 2, 3, 4, 5}
print("Table operations:")
print("Table: " .. table.concat(t, ", "))
print("Length: " .. #t)
print("Sum: " .. (t[1] + t[2] + t[3] + t[4] + t[5]))

local person = {name = "Alice", age = 30}
print("Person: " .. person.name .. ", " .. person.age .. " years old")
