-- HTTP Module Demo
-- This demonstrates the HTTP module API (once require() is fully integrated)

print("═══════════════════════════════════════════")
print("  HTTP Module Demo")
print("═══════════════════════════════════════════\n")

-- Load the HTTP module
local http = require("http")

print("--- Example 1: Simple GET Request ---\n")

-- Fetch user data from GitHub API
local response = http.get("https://api.github.com/users/octocat")

if response.ok() then
    local user = response.json()
    print("✓ Successfully fetched GitHub user")
    print("  Name:", user.name)
    print("  Location:", user.location or "N/A")
    print("  Public Repos:", user.public_repos)
    print("  Followers:", user.followers)
    print("  Bio:", user.bio or "No bio")
else
    print("✗ Request failed:", response.status, response.statusText)
end

print("\n--- Example 2: POST JSON Data ---\n")

-- Create a new post (using JSONPlaceholder test API)
local newPost = {
    title = "Hello from Hype-RS",
    body = "This is a test post from the HTTP module",
    userId = 1
}

local createResponse = http.postJson("https://jsonplaceholder.typicode.com/posts", newPost)

if createResponse.ok() then
    local created = createResponse.json()
    print("✓ Successfully created post")
    print("  Post ID:", created.id)
    print("  Title:", created.title)
else
    print("✗ Failed to create post")
end

print("\n--- Example 3: Fetch with Custom Options ---\n")

-- Use the universal fetch API with options
local fetchResponse = http.fetch("https://api.github.com/repos/rust-lang/rust", {
    method = "GET",
    headers = {
        ["Accept"] = "application/vnd.github.v3+json",
        ["User-Agent"] = "Hype-RS-HTTP-Client"
    },
    timeout = 10000  -- 10 second timeout
})

if fetchResponse.ok() then
    local repo = fetchResponse.json()
    print("✓ Successfully fetched Rust repo info")
    print("  Name:", repo.name)
    print("  Stars:", repo.stargazers_count)
    print("  Forks:", repo.forks_count)
    print("  Language:", repo.language)
    print("  Description:", repo.description)
else
    print("✗ Failed to fetch repo info")
end

print("\n--- Example 4: Error Handling ---\n")

-- Demonstrate error handling with pcall
local ok, result = pcall(function()
    return http.get("https://this-domain-does-not-exist-12345.com")
end)

if ok then
    print("✓ Request succeeded (unexpected)")
else
    print("✓ Error handled gracefully:")
    print("  Error:", result)
end

print("\n--- Example 5: Checking Response Status ---\n")

-- Try to fetch a non-existent resource
local notFoundResponse = http.get("https://api.github.com/users/this-user-definitely-does-not-exist-xyz")

print("  Status:", notFoundResponse.status)
print("  Status Text:", notFoundResponse.statusText)
print("  Is OK (2xx)?", notFoundResponse.ok())

if notFoundResponse.status == 404 then
    print("✓ Correctly identified 404 Not Found")
end

print("\n--- Example 6: Multiple Requests ---\n")

local users = {"octocat", "torvalds", "gvanrossum"}

print("Fetching multiple GitHub users:")
for _, username in ipairs(users) do
    local userResponse = http.get("https://api.github.com/users/" .. username)
    
    if userResponse.ok() then
        local userData = userResponse.json()
        print(string.format("  ✓ %s - %d repos", userData.login, userData.public_repos))
    else
        print(string.format("  ✗ %s - Failed", username))
    end
end

print("\n═══════════════════════════════════════════")
print("  Demo Complete!")
print("═══════════════════════════════════════════")
