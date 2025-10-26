local http = require('http')
local utils = require('../lib/utils')
local args = _G.args or {}

if #args < 1 or args[1] == "--help" or args[1] == "-h" then
    print("Usage: hfetch <url> [options]")
    print("")
    print("Fetch data from a URL using HTTP GET")
    print("")
    print("Arguments:")
    print("  url          URL to fetch")
    print("")
    print("Options:")
    print("  --json       Parse and pretty-print JSON response")
    print("  --headers    Show response headers")
    print("  --verbose    Show detailed request information")
    print("  -h, --help   Show this help message")
    print("")
    print("Examples:")
    print("  hfetch https://httpbin.org/get")
    print("  hfetch https://api.github.com/users/octocat --json")
    print("  hfetch https://httpbin.org/get --headers")
    os.exit(args[1] == "--help" or args[1] == "-h" and 0 or 1)
end

local url = args[1]
local parse_json = false
local show_headers = false
local verbose = false

for i = 2, #args do
    if args[i] == "--json" then
        parse_json = true
    elseif args[i] == "--headers" then
        show_headers = true
    elseif args[i] == "--verbose" then
        verbose = true
    end
end

if verbose then
    print("Request: GET " .. url)
    print("")
end

local success, response = pcall(function()
    return http.get(url)
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

if parse_json then
    local json_success, data = pcall(function()
        return response:json()
    end)
    
    if json_success then
        print(utils.pretty_json(data))
    else
        print("Error: Failed to parse JSON response")
        print(response:text())
        os.exit(1)
    end
else
    print(response:text())
end
