local http = require('http')
local utils = require('../lib/utils')
local args = _G.args or {}

if #args < 2 or args[1] == "--help" or args[1] == "-h" then
    print("Usage: hpost <url> <data> [options]")
    print("")
    print("Send data to a URL using HTTP POST")
    print("")
    print("Arguments:")
    print("  url          URL to post to")
    print("  data         Data to send (use @file.json to read from file)")
    print("")
    print("Options:")
    print("  --json       Set Content-Type to application/json")
    print("  --headers    Show response headers")
    print("  --verbose    Show detailed request information")
    print("  -h, --help   Show this help message")
    print("")
    print("Examples:")
    print("  hpost https://httpbin.org/post 'hello=world'")
    print("  hpost https://httpbin.org/post '{\"name\":\"test\"}' --json")
    print("  hpost https://httpbin.org/post @data.json --json")
    os.exit(args[1] == "--help" or args[1] == "-h" and 0 or 1)
end

local url = args[1]
local data = args[2]
local use_json = false
local show_headers = false
local verbose = false

for i = 3, #args do
    if args[i] == "--json" then
        use_json = true
    elseif args[i] == "--headers" then
        show_headers = true
    elseif args[i] == "--verbose" then
        verbose = true
    end
end

if data:sub(1, 1) == "@" then
    local filename = data:sub(2)
    local fs = require('fs')
    local file_success, content = pcall(function()
        return fs.readFileSync(filename)
    end)
    
    if not file_success then
        print("Error: Failed to read file: " .. filename)
        os.exit(1)
    end
    
    data = content
end

if verbose then
    print("Request: POST " .. url)
    if use_json then
        print("Content-Type: application/json")
    end
    print("Data length: " .. #data .. " bytes")
    print("")
end

local options = {
    body = data
}

if use_json then
    options.headers = {
        ["Content-Type"] = "application/json"
    }
end

local success, response = pcall(function()
    return http.post(url, options)
end)

if not success then
    print("Error: Failed to make request")
    print(response)
    os.exit(1)
end

if not response:ok() then
    print("Error: HTTP " .. response.status .. " " .. response.statusText)
    os.exit(1)
end

if show_headers then
    print("Status: " .. response.status .. " " .. response.statusText)
    print("Headers:")
    for key, value in pairs(response.headers or {}) do
        print("  " .. key .. ": " .. value)
    end
    print("")
end

if use_json then
    local json_success, parsed = pcall(function()
        return response:json()
    end)
    
    if json_success then
        print(utils.pretty_json(parsed))
    else
        print(response:text())
    end
else
    print(response:text())
end
