local http = require("http")

print("Testing postForm...")
local res1 = http.postForm("https://httpbin.org/post", {username = "test"})
print("Status:", res1.status)

print("\nTesting Basic Auth...")
local res2 = http.get("https://httpbin.org/basic-auth/user/pass", {
    auth = {username = "user", password = "pass"}
})
print("Status:", res2.status)

print("\nTesting Bearer Token...")
local res3 = http.get("https://httpbin.org/bearer", {authToken = "token123"})
print("Status:", res3.status)

print("\nAll quick tests completed!")
