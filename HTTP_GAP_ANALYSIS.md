# HTTP Module - Comprehensive Gap Analysis

**Date:** 2025-10-27  
**Version:** 0.1.4 (post-cookie implementation)  
**Status:** Analysis Complete

---

## Executive Summary

After implementing cookie support, the HTTP module is **solid for 80% of use cases** but has **gaps for advanced scenarios**. This document identifies what's missing and recommends priorities.

### Quick Status

| Category | Status | Priority |
|----------|--------|----------|
| ✅ **Working Well** | 10 features | - |
| ⚠️ **Needs Improvement** | 6 features | High-Medium |
| ❌ **Not Supported** | 12 features | Low-Future |

---

## ✅ What's Working Well (Already Supported)

### 1. Core HTTP Methods ✅
**Status:** Fully working  
**Methods:** GET, POST, PUT, DELETE, PATCH, HEAD, fetch()

```lua
http.get(url)
http.post(url, {body = data, headers = {...}})
http.fetch(url, {method = "PATCH", ...})
```

**Assessment:** Complete, no gaps

---

### 2. Redirects ✅
**Status:** Automatic, configurable  
**Default:** Follows up to 10 redirects

```lua
http.get("https://httpbin.org/redirect/2")  -- Automatic
http.fetch(url, {followRedirects = false})  -- Can disable
```

**Assessment:** ✅ Works perfectly

---

### 3. Compression ✅
**Status:** Automatic (transparent)  
**Supported:** gzip, deflate, brotli

```lua
http.get("https://httpbin.org/gzip")  -- Auto-decompressed
```

**Assessment:** ✅ Perfect, users don't need to know

---

### 4. Connection Pooling ✅
**Status:** Enabled (10 idle connections per host)

```lua
-- 3 requests reuse connection
for i = 1, 3 do
    http.get("https://example.com/api")
end
```

**Assessment:** ✅ Excellent performance

---

### 5. HTTP/2 ✅
**Status:** Automatic (reqwest default)

**Assessment:** ✅ Modern protocol support

---

### 6. TLS/HTTPS ✅
**Status:** Full support with rustls  
**Features:** TLS 1.2, TLS 1.3, SNI, certificate validation

```lua
http.get("https://secure-site.com")  -- Just works
```

**Assessment:** ✅ Excellent after v0.1.3 rustls fix

---

### 7. Custom Headers ✅
**Status:** Fully supported

```lua
http.post(url, {
    headers = {
        ["Authorization"] = "Bearer token",
        ["X-Custom"] = "value"
    }
})
```

**Assessment:** ✅ Complete

---

### 8. Timeouts ✅
**Status:** Configurable per request

```lua
http.fetch(url, {timeout = 5000})  -- 5 seconds
```

**Assessment:** ✅ Works in fetch(), should extend to all methods

---

### 9. Cookie Support ✅ (NEW in v0.1.4)
**Status:** Automatic cookie jar

```lua
http.post("/login", {body = creds})  -- Cookie stored
http.get("/profile")                 -- Cookie sent
```

**Assessment:** ✅ Just implemented, works great

---

### 10. Error Handling ✅
**Status:** Comprehensive error types

```lua
local ok, err = pcall(http.get, "https://invalid")
-- Returns clear error messages
```

**Assessment:** ✅ Good error reporting

---

## ⚠️ Needs Improvement (Partially Supported)

### 1. Form Handling ⚠️

**Current Status:** Manual encoding required

**What's Missing:**
- No built-in URL encoding helper
- No form data builder
- No multipart/form-data support

**Current Workaround:**
```lua
-- URL-encoded form (manual)
local body = "field1=value1&field2=value2"
http.post(url, {
    body = body,
    headers = {["Content-Type"] = "application/x-www-form-urlencoded"}
})

-- Multipart form (very manual)
local boundary = "----FormBoundary"
local body = string.format(
    "--%s\r\nContent-Disposition: form-data; name=\"file\"; filename=\"test.txt\"\r\n\r\n%s\r\n--%s--\r\n",
    boundary, file_content, boundary
)
http.post(url, {
    body = body,
    headers = {["Content-Type"] = "multipart/form-data; boundary=" .. boundary}
})
```

**Recommendation:** Add helpers

```lua
-- Proposed API
http.postForm(url, {
    field1 = "value1",
    field2 = "value2"
})

http.uploadFile(url, {
    file = {filename = "test.txt", content = "data"},
    field1 = "value1"
})
```

**Priority:** 🔴 HIGH  
**Effort:** Medium (4-6 hours)  
**Use Cases:** Web scraping, API integrations, file uploads

---

### 2. Authentication Helpers ⚠️

**Current Status:** Manual header construction

**What's Missing:**
- No Basic Auth helper
- No Bearer token helper
- No OAuth helpers

**Current Workaround:**
```lua
-- Basic Auth (manual base64)
local base64 = require("base64")  -- Would need to add
local auth = base64.encode("user:pass")
http.get(url, {headers = {Authorization = "Basic " .. auth}})

-- Bearer (manual)
http.get(url, {headers = {Authorization = "Bearer " .. token}})
```

**Recommendation:** Add auth helpers

```lua
-- Proposed API
http.get(url, {
    auth = {
        type = "basic",
        username = "user",
        password = "pass"
    }
})

http.get(url, {
    auth = {
        type = "bearer",
        token = "abc123"
    }
})
```

**Priority:** 🟡 MEDIUM  
**Effort:** Low (2-3 hours)  
**Use Cases:** API authentication, protected resources

---

### 3. Binary Data Handling ⚠️

**Current Status:** Works but suboptimal

**What's Missing:**
- No way to detect binary vs text
- No base64 encoding option
- No byte array representation

**Current Behavior:**
```lua
local res = http.get("https://httpbin.org/image/png")
-- res.body is string with binary data (works but awkward)
```

**Issues:**
- Large binary files consume memory
- No way to process binary data cleanly in Lua
- String concatenation can corrupt binary data

**Recommendation:** Add binary handling options

```lua
-- Option 1: Detect binary
local res = http.get(url)
if res.isBinary then
    -- Save to file instead of processing
    file:write(res.body)
end

-- Option 2: Base64 encoding
local res = http.get(url, {encoding = "base64"})
-- res.body is base64 string (safe for Lua)

-- Option 3: Direct to file
http.downloadToFile(url, "/tmp/image.png")
```

**Priority:** 🟡 MEDIUM  
**Effort:** Low-Medium (3-4 hours)  
**Use Cases:** Image downloads, binary APIs, file downloads

---

### 4. Large File Handling ⚠️

**Current Status:** All responses buffered in memory

**What's Missing:**
- No streaming download API
- No progress callbacks
- Memory issues with files > 100MB

**Problem:**
```lua
-- This loads entire 1GB file into memory!
local res = http.get("https://example.com/bigfile.zip")
-- Lua process uses 1GB+ RAM
```

**Recommendation:** Add streaming download

```lua
-- Proposed API: Download to file
http.downloadToFile(url, "/tmp/large.zip", {
    onProgress = function(downloaded, total)
        print(string.format("Progress: %.1f%%", downloaded/total*100))
    end
})

-- Proposed API: Chunked streaming
http.stream(url, function(chunk)
    -- Process chunk by chunk
    file:write(chunk)
end)
```

**Priority:** 🟡 MEDIUM  
**Effort:** High (8-12 hours)  
**Use Cases:** Large downloads, file servers, backups

---

### 5. Request/Response Inspection ⚠️

**Current Status:** Limited visibility

**What's Missing:**
- No request headers inspection
- No timing information
- No response size before download
- No redirect chain visibility

**Current:**
```lua
local res = http.get(url)
-- Can see: status, statusText, body, headers
-- Cannot see: request headers, timing, redirects followed
```

**Recommendation:** Enhanced response object

```lua
-- Proposed API
local res = http.get(url, {verbose = true})

-- Additional fields:
print(res.timing.total)        -- Total time (ms)
print(res.timing.dns)          -- DNS lookup time
print(res.timing.connect)      -- Connection time
print(res.timing.tls)          -- TLS handshake time

print(#res.redirects)          -- Number of redirects
for i, r in ipairs(res.redirects) do
    print(r.url, r.status)     -- Redirect chain
end

print(res.request.headers)     -- Request headers sent
print(res.request.url)         -- Final URL (after redirects)
```

**Priority:** 🟢 LOW  
**Effort:** Medium (4-6 hours)  
**Use Cases:** Debugging, performance analysis, monitoring

---

### 6. Timeout Granularity ⚠️

**Current Status:** Only available in fetch()

**What's Missing:**
- Timeout not available in get/post/put/delete
- Cannot set different timeouts for connect vs read
- No per-method timeout configuration

**Current:**
```lua
-- Only works in fetch()
http.fetch(url, {timeout = 5000})

-- Doesn't work in other methods
http.get(url, {timeout = 5000})  -- Option ignored!
```

**Recommendation:** Extend timeout to all methods

```lua
-- Proposed: All methods accept timeout
http.get(url, {timeout = 5000})
http.post(url, {body = data, timeout = 10000})

-- Advanced: Different timeout types
http.get(url, {
    connectTimeout = 3000,  -- 3s to connect
    readTimeout = 30000     -- 30s to read response
})
```

**Priority:** 🔴 HIGH  
**Effort:** Low (2-3 hours)  
**Use Cases:** Unreliable networks, slow servers, user control

---

## ❌ Not Supported (Missing Features)

### 1. Custom HTTP Methods ❌

**Status:** PATCH, HEAD work, but arbitrary methods may not

**What's Missing:**
- Cannot use WebDAV methods (PROPFIND, MKCOL, etc.)
- Cannot use custom application methods

**Workaround:** Limited to standard methods

**Recommendation:** Support arbitrary methods

```lua
http.fetch(url, {method = "PROPFIND"})
http.fetch(url, {method = "CUSTOM_METHOD"})
```

**Priority:** 🟢 LOW (rare use case)  
**Effort:** Low (1-2 hours)

---

### 2. Retry Logic ❌

**Status:** No automatic retry on failure

**What's Missing:**
- No retry on network errors
- No retry on specific status codes (500, 502, 503)
- No exponential backoff

**Workaround:** Manual retry loop

```lua
-- Manual retry (user must implement)
local max_retries = 3
for i = 1, max_retries do
    local ok, res = pcall(http.get, url)
    if ok and res.status == 200 then
        break
    end
    os.execute("sleep 1")  -- Wait between retries
end
```

**Recommendation:** Add retry configuration

```lua
http.get(url, {
    retry = {
        count = 3,
        delay = 1000,           -- Initial delay (ms)
        backoff = "exponential", -- or "linear"
        on = {500, 502, 503}    -- Retry these status codes
    }
})
```

**Priority:** 🟡 MEDIUM  
**Effort:** Medium (4-6 hours)  
**Use Cases:** Unreliable APIs, rate-limited services

---

### 3. Rate Limiting ❌

**Status:** No built-in rate limiting

**What's Missing:**
- No automatic request throttling
- No queue management
- No rate limit detection

**Workaround:** Manual rate limiting

```lua
-- User must implement
local last_request = os.time()
while os.time() - last_request < 1 do
    -- Wait
end
http.get(url)
last_request = os.time()
```

**Recommendation:** Add rate limiter

```lua
-- Proposed API
local client = http.newClient({
    rateLimit = {
        requests = 10,
        per = 60  -- 10 requests per 60 seconds
    }
})

client.get(url)  -- Automatically throttled
```

**Priority:** 🟡 MEDIUM  
**Effort:** Medium (4-6 hours)  
**Use Cases:** API rate limits, web scraping, politeness

---

### 4. Request Cancellation ❌

**Status:** Cannot abort in-flight requests

**What's Missing:**
- No way to cancel long-running request
- No timeout abort callback
- Blocking operations cannot be interrupted

**Problem:**
```lua
-- This blocks until complete (no way to cancel)
local res = http.get("https://slow-server.com/bigfile")
-- If user wants to cancel, tough luck!
```

**Recommendation:** Async API with cancellation

```lua
-- Proposed (requires async support)
local req = http.getAsync(url)
-- ... user hits cancel button ...
req:cancel()
```

**Priority:** 🟢 LOW (requires async Lua, complex)  
**Effort:** Very High (would need async runtime)  
**Blockers:** Lua is synchronous by design

---

### 5. Progress Callbacks ❌

**Status:** No progress tracking

**What's Missing:**
- No upload progress
- No download progress
- No way to show progress bars

**Problem:**
```lua
-- User sees nothing for 30 seconds on large upload
http.post(url, {body = large_file})  -- Black box
```

**Recommendation:** Progress callbacks

```lua
http.post(url, {
    body = data,
    onProgress = function(uploaded, total)
        print(string.format("Uploading: %d/%d bytes", uploaded, total))
    end
})

http.downloadToFile(url, path, {
    onProgress = function(downloaded, total)
        local pct = (downloaded / total) * 100
        print(string.format("Downloading: %.1f%%", pct))
    end
})
```

**Priority:** 🟡 MEDIUM  
**Effort:** High (8-12 hours, requires streaming)  
**Use Cases:** Large uploads/downloads, user feedback

---

### 6. WebSockets ❌

**Status:** Not supported

**What's Missing:**
- No WebSocket client
- No bidirectional communication
- No real-time protocols

**Recommendation:** WebSocket support

```lua
-- Proposed API
local ws = http.websocket("wss://echo.websocket.org")
ws.onMessage = function(msg)
    print("Received: " .. msg)
end
ws.send("Hello")
```

**Priority:** 🟡 MEDIUM  
**Effort:** Very High (16+ hours)  
**Use Cases:** Real-time apps, chat, streaming data

---

### 7. Server-Sent Events (SSE) ❌

**Status:** Not supported (no streaming)

**What's Missing:**
- Cannot consume SSE streams
- No long-lived connections with events

**Recommendation:** SSE client

```lua
http.sse(url, {
    onMessage = function(event)
        print(event.data)
    end
})
```

**Priority:** 🟢 LOW  
**Effort:** High (8-12 hours)  
**Use Cases:** Real-time notifications, live updates

---

### 8. HTTP/3 (QUIC) ❌

**Status:** Not supported

**What's Missing:**
- Only HTTP/1.1 and HTTP/2
- No QUIC protocol

**Recommendation:** Low priority (HTTP/2 is sufficient)

**Priority:** 🟢 VERY LOW  
**Effort:** High (requires experimental reqwest feature)

---

### 9. Proxy Configuration ❌

**Status:** Not exposed

**What's Missing:**
- No HTTP proxy support
- No HTTPS proxy support
- No SOCKS proxy support

**Problem:**
```lua
-- Cannot use corporate proxy
http.get(url)  -- Fails behind proxy
```

**Recommendation:** Proxy configuration

```lua
-- Proposed API
http.get(url, {
    proxy = "http://proxy.company.com:8080"
})

-- Or global proxy
local client = http.newClient({
    proxy = {
        http = "http://proxy:8080",
        https = "https://proxy:8443",
        auth = {username = "user", password = "pass"}
    }
})
```

**Priority:** 🔴 HIGH (enterprise use)  
**Effort:** Low-Medium (3-5 hours)  
**Use Cases:** Corporate networks, tunneling

---

### 10. Certificate Pinning ❌

**Status:** Uses system certificate store only

**What's Missing:**
- Cannot pin specific certificates
- Cannot use custom CA certificates
- Cannot disable certificate verification (intentionally)

**Recommendation:** Custom certificate support

```lua
http.get(url, {
    certificates = {
        ca = "/path/to/custom-ca.pem",
        cert = "/path/to/client-cert.pem",
        key = "/path/to/client-key.pem"
    }
})
```

**Priority:** 🟢 LOW (security risk if misused)  
**Effort:** Medium (4-6 hours)

---

### 11. Request/Response Middleware ❌

**Status:** No interceptor pattern

**What's Missing:**
- Cannot intercept requests before sending
- Cannot modify responses after receiving
- No logging middleware
- No authentication middleware

**Problem:**
```lua
-- Want to add auth header to all requests
-- Must manually add to each request
http.get(url1, {headers = {Authorization = auth}})
http.post(url2, {headers = {Authorization = auth}})
http.put(url3, {headers = {Authorization = auth}})
-- Repetitive!
```

**Recommendation:** Middleware/interceptor pattern

```lua
-- Proposed API
http.addInterceptor({
    before = function(request)
        request.headers.Authorization = "Bearer " .. token
        return request
    end,
    after = function(response)
        print("Request took: " .. response.timing.total .. "ms")
        return response
    end
})

-- Now all requests automatically have auth header
http.get(url1)  -- Auth added automatically
http.post(url2) -- Auth added automatically
```

**Priority:** 🟡 MEDIUM  
**Effort:** High (10-15 hours)  
**Use Cases:** Logging, auth, debugging, monitoring

---

### 12. HTTP Caching ❌

**Status:** No cache support

**What's Missing:**
- No Cache-Control header handling
- No ETag validation
- No conditional requests (If-Modified-Since)

**Problem:**
```lua
-- Re-downloads same resource every time
http.get("https://api.com/static/data.json")  -- Full download
http.get("https://api.com/static/data.json")  -- Full download again
```

**Recommendation:** HTTP caching

```lua
local client = http.newClient({
    cache = true,
    cacheDir = "/tmp/hype-cache"
})

-- First call: full download
client.get(url)

-- Second call: uses cache if valid
client.get(url)  -- Returns cached version
```

**Priority:** 🟢 LOW  
**Effort:** Very High (16+ hours)  
**Use Cases:** Performance optimization, offline support

---

## Priority Matrix

### Must Have (v0.2.0)
1. 🔴 **Proxy Support** - Enterprise requirement
2. 🔴 **Timeout in all methods** - User control
3. 🔴 **Form helpers** - Common use case

**Total Effort:** 9-14 hours

### Should Have (v0.3.0)
4. 🟡 **Authentication helpers** - Better UX
5. 🟡 **Binary data handling** - Common need
6. 🟡 **Retry logic** - Reliability
7. 🟡 **Rate limiting** - API compliance
8. 🟡 **Request/Response inspection** - Debugging

**Total Effort:** 20-30 hours

### Nice to Have (v1.0)
9. 🟢 **Progress callbacks** - UX enhancement
10. 🟢 **WebSockets** - Real-time apps
11. 🟢 **Large file handling** - Streaming
12. 🟢 **Middleware pattern** - Power users

**Total Effort:** 40-60 hours

### Future (v2.0+)
- HTTP caching
- SSE support
- HTTP/3
- Certificate pinning
- Request cancellation (requires async)

---

## Recommended Roadmap

### v0.1.4 ✅
- [x] Cookie support (DONE)

### v0.2.0 (Next Release) - 15 hours
- [ ] Proxy configuration
- [ ] Timeout in all HTTP methods
- [ ] Form data helpers (postForm, uploadFile)
- [ ] Basic Auth helper

### v0.3.0 (3-6 months) - 25 hours
- [ ] Binary data handling (base64, detection)
- [ ] Retry logic with backoff
- [ ] Rate limiting
- [ ] Enhanced response inspection (timing, redirects)
- [ ] Bearer token auth helper

### v1.0.0 (6-12 months) - 50 hours
- [ ] Progress callbacks
- [ ] WebSocket support
- [ ] Large file streaming (downloadToFile)
- [ ] Request/Response middleware
- [ ] Full multipart form support

---

## User Impact Analysis

### Current State (v0.1.4)
**Satisfaction:** 80% of use cases covered

**What works great:**
- Simple API calls ✅
- JSON APIs ✅
- Authentication (manual) ✅
- Cookies ✅
- Basic file uploads ✅

**What's painful:**
- Forms require manual encoding
- No proxy support (enterprise blocker)
- Large files consume memory
- No retry (must implement manually)
- No rate limiting

### After v0.2.0
**Satisfaction:** 90% of use cases

**New capabilities:**
- Enterprise proxy support ✅
- Form data APIs ✅
- Better timeout control ✅
- Auth helpers ✅

### After v0.3.0
**Satisfaction:** 95% of use cases

**New capabilities:**
- Automatic retries ✅
- Rate limiting ✅
- Binary handling ✅
- Better debugging ✅

---

## Conclusion

The HTTP module is **production-ready for most use cases** but has **clear gaps for advanced scenarios**. The recommended approach:

1. **v0.2.0:** Focus on enterprise needs (proxy, forms, auth)
2. **v0.3.0:** Add reliability features (retry, rate limiting)
3. **v1.0.0:** Add advanced features (WebSockets, streaming, middleware)

**Priority 1 (Must Have):** Proxy, Timeout everywhere, Forms  
**Priority 2 (Should Have):** Auth, Binary, Retry, Rate limiting  
**Priority 3 (Nice to Have):** Progress, WebSockets, Streaming

---

**Document Status:** Complete  
**Next Action:** Review with maintainer, prioritize v0.2.0 features
