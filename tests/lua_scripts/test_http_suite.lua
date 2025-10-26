print("=== HTTP Lua Bindings Test Suite ===\n")

local http = require('http')
local passed = 0
local failed = 0

local function test(name, fn)
    io.write("Test: " .. name .. " ... ")
    local success, err = pcall(fn)
    if success then
        print("✓")
        passed = passed + 1
    else
        print("✗")
        print("  Error: " .. tostring(err))
        failed = failed + 1
    end
end

test("Module loads successfully", function()
    assert(http ~= nil)
    assert(type(http) == "table")
end)

test("All HTTP methods exist", function()
    assert(type(http.get) == "function")
    assert(type(http.post) == "function")
    assert(type(http.put) == "function")
    assert(type(http.delete) == "function")
    assert(type(http.patch) == "function")
    assert(type(http.head) == "function")
    assert(type(http.fetch) == "function")
    assert(type(http.postJson) == "function")
    assert(type(http.putJson) == "function")
end)

test("GET request returns response", function()
    local response = http.get("https://httpbin.org/get")
    assert(response ~= nil)
    assert(type(response) == "table")
    assert(type(response.status) == "number")
    assert(type(response.statusText) == "string")
    assert(type(response.headers) == "table")
    assert(type(response.body) == "string")
end)

test("Response has methods", function()
    local response = http.get("https://httpbin.org/get")
    assert(type(response.json) == "function")
    assert(type(response.text) == "function")
    assert(type(response.ok) == "function")
end)

test("Response.ok() works", function()
    local response = http.get("https://httpbin.org/get")
    assert(response:ok() == true)
end)

test("Response.text() works", function()
    local response = http.get("https://httpbin.org/get")
    local text = response:text()
    assert(type(text) == "string")
    assert(#text > 0)
end)

test("Response.json() works", function()
    local response = http.get("https://httpbin.org/get")
    local data = response:json()
    assert(type(data) == "table")
    assert(data.url ~= nil)
end)

test("GET with query parameters", function()
    local response = http.get("https://httpbin.org/get?foo=bar")
    assert(response:ok())
    local data = response:json()
    assert(data.args.foo == "bar")
end)

test("POST with JSON body", function()
    local response = http.postJson("https://httpbin.org/post", { test = "value", num = 42 })
    assert(response:ok())
    local data = response:json()
    assert(data.json.test == "value")
    assert(data.json.num == 42)
end)

test("PUT with JSON body", function()
    local response = http.putJson("https://httpbin.org/put", { updated = true })
    assert(response:ok())
    local data = response:json()
    assert(data.json.updated == true)
end)

test("DELETE request", function()
    local response = http.delete("https://httpbin.org/delete")
    assert(response:ok())
end)

test("HEAD request", function()
    local response = http.head("https://httpbin.org/get")
    assert(response:ok())
    assert(response.body == "")
end)

test("fetch() with options", function()
    local response = http.fetch("https://httpbin.org/get", { method = "GET" })
    assert(response:ok())
end)

test("fetch() with headers", function()
    local response = http.fetch("https://httpbin.org/headers", {
        headers = { ["X-Test"] = "value" }
    })
    assert(response:ok())
    local data = response:json()
    assert(data.headers["X-Test"] == "value")
end)

test("404 response is not ok()", function()
    local response = http.get("https://httpbin.org/status/404")
    assert(response.status == 404)
    assert(not response:ok())
end)

test("Invalid URL throws error", function()
    local success = pcall(function()
        http.get("not-a-url")
    end)
    assert(success == false)
end)

print("\n=== Results ===")
print("Passed: " .. passed)
print("Failed: " .. failed)
print("Total:  " .. (passed + failed))

if failed == 0 then
    print("\n✓ All tests passed!")
else
    print("\n✗ Some tests failed")
end
