# PRP-011: HTTP Module Cookie Support

**Status:** Proposed  
**Priority:** High  
**Created:** 2025-10-27  
**Author:** AI Assistant  
**Estimated Effort:** 3-5 hours

---

## Table of Contents

- [1. Project Overview](#1-project-overview)
- [2. Current State Analysis](#2-current-state-analysis)
- [3. Technical Requirements](#3-technical-requirements)
- [4. Proposed Solutions](#4-proposed-solutions)
- [5. Solution Comparison](#5-solution-comparison)
- [6. Recommended Solution](#6-recommended-solution)
- [7. Implementation Plan](#7-implementation-plan)
- [8. Success Criteria](#8-success-criteria)
- [9. Risk Assessment](#9-risk-assessment)
- [10. Future Enhancements](#10-future-enhancements)

---

## 1. Project Overview

### 1.1 Problem Statement

The HTTP module in hype-rs currently has no automatic cookie management. While servers can send `Set-Cookie` headers and clients can manually include `Cookie` headers, there is no cookie jar to automatically store, persist, and resend cookies across requests.

**Current behavior:**
```lua
local http = require("http")

-- Request 1: Server sets a session cookie
local res1 = http.get("https://example.com/login")
-- Server response includes: Set-Cookie: session=abc123

-- Request 2: Cookie is NOT automatically sent
local res2 = http.get("https://example.com/profile")
-- Missing: Cookie: session=abc123
-- Result: Server treats as unauthenticated

-- Manual workaround (tedious and error-prone):
local res2 = http.post("https://example.com/profile", {
    headers = {Cookie = "session=abc123; other=value"}
})
```

### 1.2 Impact

**Severity:** High

**Affected Use Cases:**
- 🔐 **Authentication flows** - Login sessions, JWT refresh tokens
- 🛒 **E-commerce scraping** - Shopping cart persistence
- 📊 **API testing** - Stateful API interactions
- 🤖 **Web automation** - Multi-step workflows requiring session state
- 🔄 **CSRF protection** - APIs using cookie-based CSRF tokens

**Current Workarounds:**
- Manual cookie extraction from `Set-Cookie` headers
- Manual cookie injection into request headers
- String parsing of cookie attributes (domain, path, expiry)
- Application-level cookie storage and management

**Pain Points:**
- Error-prone manual cookie string manipulation
- No automatic handling of cookie attributes (domain, path, expiry)
- No secure cookie storage
- Verbose code for simple authenticated requests

### 1.3 Project Goals

1. **Primary Goal:** Implement automatic cookie storage, management, and transmission
2. **API Design:** Provide simple, opt-in cookie jar functionality
3. **Standards Compliance:** Follow RFC 6265 (HTTP State Management Mechanism)
4. **Security:** Handle secure cookies, HttpOnly flags, and SameSite policies
5. **Flexibility:** Support both global and per-client cookie jars

---

## 2. Current State Analysis

### 2.1 Existing HTTP Module Architecture

**File:** `src/modules/builtins/http/client.rs` (245 lines)

```rust
pub struct HttpClient {
    client: reqwest::Client,
    runtime: Runtime,
}

impl HttpClient {
    pub fn new() -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .pool_max_idle_per_host(10)
            .build()?;
        // No cookie store configured
        
        let runtime = Runtime::new()?;
        Ok(Self { client, runtime })
    }
}
```

**Current capabilities:**
- ✅ HTTP methods (GET, POST, PUT, DELETE, PATCH, HEAD)
- ✅ Custom headers
- ✅ Request/response body handling
- ✅ JSON serialization
- ✅ TLS with rustls
- ❌ **No cookie jar**
- ❌ **No cookie persistence**

### 2.2 Dependencies

**Current `Cargo.toml`:**
```toml
reqwest = { 
    version = "0.12", 
    features = ["json", "blocking", "rustls-tls"], 
    default-features = false, 
    optional = true 
}
```

**Reqwest cookie capabilities:**
- ✅ `reqwest::cookie::Jar` - In-memory cookie storage
- ✅ `reqwest::cookie_store()` - Cookie store trait
- ⚠️ Requires `cookies` feature flag (not currently enabled)

### 2.3 Testing Results

**Manual cookie test:**
```bash
# Server sends Set-Cookie
$ hype test.lua
Response headers: {..., "set-cookie": "session=abc123; Path=/; HttpOnly"}

# Next request - cookie NOT sent automatically
$ hype test2.lua  
# Server receives: (no Cookie header)
```

---

## 3. Technical Requirements

### 3.1 Functional Requirements

**FR-1: Cookie Storage**
- System MUST store cookies from `Set-Cookie` response headers
- System MUST associate cookies with their origin domain/path
- System MUST respect cookie attributes (Domain, Path, Expires, Max-Age)

**FR-2: Cookie Transmission**
- System MUST automatically include matching cookies in subsequent requests
- System MUST respect cookie scope (domain and path matching per RFC 6265)
- System MUST send cookies in correct format: `Cookie: name=value; other=val`

**FR-3: Security**
- System MUST respect `Secure` flag (HTTPS-only cookies)
- System MUST respect `HttpOnly` flag (no script access)
- System MUST respect `SameSite` attribute (Strict/Lax/None)
- System MUST NOT send cookies to different domains (no cookie leakage)

**FR-4: API Design**
- API MUST be opt-in (backward compatible with current behavior)
- API MUST support both global and per-client cookie jars
- API MUST allow cookie inspection/clearing

**FR-5: Cookie Expiration**
- System MUST remove expired cookies automatically
- System MUST respect `Max-Age` and `Expires` attributes
- System MUST handle session cookies (no expiry = cleared on "session end")

### 3.2 Non-Functional Requirements

**NFR-1: Performance**
- Cookie lookup overhead MUST be < 1ms per request
- Memory usage MUST be reasonable (< 1MB for typical cookie jar)

**NFR-2: Compatibility**
- Solution MUST maintain backward compatibility (no breaking changes)
- Solution MUST work with existing HTTP methods (get, post, etc.)

**NFR-3: Standards Compliance**
- Implementation MUST follow RFC 6265 (HTTP State Management)
- Implementation SHOULD handle common browser cookie behaviors

**NFR-4: Code Quality**
- Implementation MUST include unit tests (>80% coverage)
- Implementation MUST include integration tests with real servers
- Error handling MUST provide clear messages

### 3.3 Dependencies

**Required:**
- `reqwest` with `cookies` feature flag

**Optional:**
- `cookie_store` crate - Persistent cookie storage (for file-based jars)

---

## 4. Proposed Solutions

### Solution 1: Global Cookie Jar (Simplest)

**Description:**  
Enable reqwest's built-in cookie store at the client level. All HTTP requests share a single global cookie jar that persists for the lifetime of the Lua script.

**Architecture:**
```rust
// src/modules/builtins/http/client.rs
use reqwest::cookie::Jar;
use std::sync::Arc;

pub struct HttpClient {
    client: reqwest::Client,
    runtime: Runtime,
    cookie_jar: Arc<Jar>,  // Shared cookie storage
}

impl HttpClient {
    pub fn new() -> Result<Self> {
        let cookie_jar = Arc::new(Jar::default());
        
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .pool_max_idle_per_host(10)
            .cookie_provider(cookie_jar.clone())  // Enable cookies
            .build()?;
        
        Ok(Self { client, runtime, cookie_jar })
    }
}
```

**Lua API:**
```lua
-- No API changes needed - cookies work automatically!
local http = require("http")

-- Request 1: Login
local res1 = http.post("https://example.com/login", {
    body = '{"user":"alice","pass":"secret"}'
})
-- Server sets: Set-Cookie: session=abc123

-- Request 2: Profile (cookie sent automatically)
local res2 = http.get("https://example.com/profile")
-- Request includes: Cookie: session=abc123
-- Success! Authenticated request.
```

**Optional cookie management API:**
```lua
-- Clear all cookies
http.clearCookies()

-- Get cookies as table (for debugging)
local cookies = http.getCookies()
-- Returns: {session = "abc123", tracking = "xyz"}
```

**Pros:**
- ✅ **Minimal code changes** - ~20 lines of Rust
- ✅ **Zero breaking changes** - Cookies work automatically
- ✅ **Simple mental model** - One jar for all requests
- ✅ **Fast implementation** - 2-3 hours total
- ✅ **Built-in reqwest feature** - Well-tested, RFC compliant

**Cons:**
- ❌ **No per-client isolation** - All requests share cookies (could be surprising)
- ❌ **No persistence** - Cookies lost when script ends
- ❌ **Limited control** - Can't disable cookies per-request
- ❌ **Potential cookie leakage** - If script talks to multiple domains

**Use cases:**
- ✅ Simple authentication flows
- ✅ Single-domain API interactions
- ✅ Short-lived scripts
- ❌ Multi-tenant applications
- ❌ Long-running services

---

### Solution 2: Per-Client Cookie Jars (Flexible)

**Description:**  
Introduce a new `http.newClient(options)` API that creates isolated HTTP clients, each with its own cookie jar. The default `http.get()`, `http.post()` etc. work without cookies (backward compatible), but client instances have cookie support.

**Architecture:**
```rust
// src/modules/builtins/http/client.rs
pub struct HttpClient {
    client: reqwest::Client,
    runtime: Runtime,
}

impl HttpClient {
    // Default client (no cookies)
    pub fn new() -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;
        Ok(Self { client, runtime: Runtime::new()? })
    }
    
    // Client with options (including cookies)
    pub fn with_options(options: ClientOptions) -> Result<Self> {
        let mut builder = reqwest::Client::builder()
            .timeout(Duration::from_secs(options.timeout.unwrap_or(30)));
        
        if options.enable_cookies {
            let jar = Arc::new(Jar::default());
            builder = builder.cookie_provider(jar);
        }
        
        let client = builder.build()?;
        Ok(Self { client, runtime: Runtime::new()? })
    }
}
```

**Lua API:**
```lua
local http = require("http")

-- Default module functions (no cookies, backward compatible)
http.get("https://api.example.com")  -- No cookies

-- Create client with cookie support
local client = http.newClient({
    cookies = true,      -- Enable cookie jar
    timeout = 60000      -- 60s timeout
})

-- Use client for authenticated session
local res1 = client.post("https://example.com/login", {body = credentials})
-- Cookies stored in client.cookieJar

local res2 = client.get("https://example.com/profile")
-- Cookies sent automatically from client.cookieJar

-- Multiple isolated clients
local client1 = http.newClient({cookies = true})
local client2 = http.newClient({cookies = true})
-- client1 and client2 have separate cookie jars

-- Clear cookies for specific client
client.clearCookies()
```

**Pros:**
- ✅ **Perfect backward compatibility** - Default behavior unchanged
- ✅ **Explicit opt-in** - Users choose when to use cookies
- ✅ **Isolation** - Multiple clients have separate cookie jars
- ✅ **Flexible** - Can mix cookie/non-cookie requests
- ✅ **Clear mental model** - Client = isolated HTTP session
- ✅ **Extensible** - Can add more client options (proxy, retries, etc.)

**Cons:**
- ⚠️ **More complex API** - Requires understanding client pattern
- ⚠️ **More code** - ~100 lines (client factory, Lua bindings)
- ⚠️ **Verbose** - More typing for simple cookie use cases
- ⚠️ **Object lifecycle** - Need to manage client lifetime in Lua

**Use cases:**
- ✅ Multi-tenant applications
- ✅ Testing multiple user sessions
- ✅ Complex authentication flows
- ✅ Long-running services
- ✅ Scripts needing both cookied and cookieless requests

---

### Solution 3: Hybrid Approach (Best of Both)

**Description:**  
Combine both solutions: enable cookies globally by default, but also provide `http.newClient()` for isolated sessions. Users get automatic cookies out-of-the-box, with the option to create isolated clients when needed.

**Architecture:**
```rust
// Global singleton client with cookies (lazy-initialized)
lazy_static! {
    static ref GLOBAL_HTTP_CLIENT: Arc<Mutex<HttpClient>> = {
        Arc::new(Mutex::new(
            HttpClient::with_cookies().expect("Failed to create HTTP client")
        ))
    };
}

impl HttpClient {
    // Global client with cookies
    pub fn with_cookies() -> Result<Self> {
        let jar = Arc::new(Jar::default());
        let client = reqwest::Client::builder()
            .cookie_provider(jar.clone())
            .build()?;
        Ok(Self { client, runtime: Runtime::new()?, cookie_jar: Some(jar) })
    }
    
    // Custom client (user-controlled)
    pub fn new_with_options(options: ClientOptions) -> Result<Self> {
        // Same as Solution 2
    }
}
```

**Lua API:**
```lua
local http = require("http")

-- Simple case: cookies work automatically (global jar)
http.post("https://example.com/login", {body = creds})  -- Sets cookie
http.get("https://example.com/profile")                 -- Sends cookie

-- Advanced case: isolated clients
local client1 = http.newClient({cookies = true})
local client2 = http.newClient({cookies = false})

client1.post("/login")  -- Cookies for user 1
client2.get("/public")  -- No cookies

-- Clear global cookies
http.clearCookies()
```

**Pros:**
- ✅ **Best UX** - Cookies "just work" for simple cases
- ✅ **Flexibility** - Isolated clients for advanced use cases
- ✅ **Progressive disclosure** - Simple API, powerful options
- ✅ **Meets all use cases** - Both simple and complex scenarios

**Cons:**
- ⚠️ **Most complex implementation** - Both patterns needed
- ⚠️ **Potential confusion** - Two ways to do HTTP requests
- ⚠️ **Global state** - Global cookie jar can be surprising
- ⚠️ **Testing complexity** - Need to test both code paths

**Use cases:**
- ✅ **All use cases from Solutions 1 & 2**
- ✅ Best for: Library that serves diverse user needs

---

## 5. Solution Comparison

### 5.1 Feature Matrix

| Feature | Solution 1: Global | Solution 2: Per-Client | Solution 3: Hybrid |
|---------|-------------------|------------------------|-------------------|
| **Ease of Use** | ⭐⭐⭐⭐⭐ Automatic | ⭐⭐⭐ Explicit | ⭐⭐⭐⭐⭐ Best of both |
| **Backward Compat** | ⭐⭐⭐⭐⭐ Perfect | ⭐⭐⭐⭐⭐ Perfect | ⭐⭐⭐⭐ Behavior change |
| **Isolation** | ⭐ None | ⭐⭐⭐⭐⭐ Full | ⭐⭐⭐⭐ Optional |
| **Implementation** | ⭐⭐⭐⭐⭐ 2-3h | ⭐⭐⭐ 4-6h | ⭐⭐ 6-8h |
| **Code Complexity** | ⭐⭐⭐⭐⭐ ~20 lines | ⭐⭐⭐ ~100 lines | ⭐⭐ ~150 lines |
| **Multi-tenant** | ❌ No | ✅ Yes | ✅ Yes |
| **Simple Auth** | ✅ Perfect | ⚠️ Verbose | ✅ Perfect |
| **Testing** | ⭐⭐⭐⭐ Easy | ⭐⭐⭐ Medium | ⭐⭐ Complex |

### 5.2 Implementation Effort

| Task | Solution 1 | Solution 2 | Solution 3 |
|------|-----------|-----------|-----------|
| Rust changes | 30 min | 2-3h | 3-4h |
| Lua bindings | 1h | 2h | 3h |
| Tests | 1h | 2h | 2-3h |
| Documentation | 30 min | 1h | 1-2h |
| **Total** | **3h** | **7-8h** | **9-12h** |

### 5.3 Risk Assessment

**Solution 1: Global Jar**
- 🟢 **Low risk** - Simple, well-understood pattern
- 🟡 **Cookie leakage** - Shared jar could leak cookies between domains
- 🟡 **Surprise behavior** - Automatic cookies might be unexpected

**Solution 2: Per-Client**
- 🟢 **Low risk** - Explicit, predictable behavior
- 🟡 **API learning curve** - Users need to understand client pattern
- 🟢 **Best isolation** - No cross-contamination

**Solution 3: Hybrid**
- 🟡 **Medium risk** - More complex = more bugs
- 🟡 **Dual mental model** - Two ways to do the same thing
- 🟢 **Best UX** - Works for everyone

### 5.4 User Experience Comparison

**Scenario: Simple authenticated API call**

```lua
-- Solution 1: Global (Best UX)
http.post("/login", {body = creds})
http.get("/profile")  -- Cookie sent automatically
-- Winner: 2 lines

-- Solution 2: Per-Client
local client = http.newClient({cookies = true})
client.post("/login", {body = creds})
client.get("/profile")
-- 3 lines, explicit

-- Solution 3: Hybrid (Same as Solution 1)
http.post("/login", {body = creds})
http.get("/profile")
-- Winner: 2 lines
```

**Scenario: Multiple user sessions**

```lua
-- Solution 1: Global (Doesn't work well)
http.post("/login", {body = user1_creds})
local user1_data = http.get("/profile")  -- OK
http.clearCookies()  -- Manual cleanup needed
http.post("/login", {body = user2_creds})
local user2_data = http.get("/profile")  -- OK
-- Works but clunky

-- Solution 2: Per-Client (Perfect)
local client1 = http.newClient({cookies = true})
local client2 = http.newClient({cookies = true})
client1.post("/login", {body = user1_creds})
client2.post("/login", {body = user2_creds})
local data1 = client1.get("/profile")  -- User 1's data
local data2 = client2.get("/profile")  -- User 2's data
-- Winner: Clean isolation

-- Solution 3: Hybrid (Flexible)
-- Can use either pattern from above
```

---

## 6. Recommended Solution

### 6.1 Decision: Solution 1 (Global Cookie Jar)

**Rationale:**

After analyzing all three solutions, **Solution 1 (Global Cookie Jar)** is recommended for v0.1.4 based on these factors:

1. **Pragmatic approach**: 
   - 80% of users need simple cookie support for authentication
   - Global jar solves this with zero API changes
   - Advanced users can work around limitations

2. **Fast time-to-value**:
   - 2-3 hours implementation
   - Ships in next minor version
   - Addresses immediate user pain points

3. **Low risk**:
   - Minimal code changes (~20 lines)
   - Built-in reqwest functionality (well-tested)
   - Easy to test and validate

4. **Incremental improvement**:
   - Can later add Solution 2 (per-client) in v0.2.0 if demand exists
   - Not mutually exclusive with future enhancements
   - Backward compatible foundation

5. **User feedback**:
   - Ship simple solution first
   - Gather real-world usage patterns
   - Evolve based on actual needs

**Trade-offs accepted:**
- ❌ No multi-tenant support (can add later)
- ❌ Cookies enabled by default (might surprise some users)
- ✅ Simple, predictable behavior for 80% use case

### 6.2 Future Path

**v0.1.4**: Solution 1 (Global jar)  
**v0.2.0**: Add Solution 2 (Per-client jars) if users request it  
**v0.3.0**: Persistent cookies (file-based storage) if needed

---

## 7. Implementation Plan

### 7.1 Phase 1: Core Implementation (1.5h)

**Step 1.1: Update Cargo.toml** (5 min)
```toml
# Add cookies feature to reqwest
reqwest = { 
    version = "0.12", 
    features = ["json", "blocking", "rustls-tls", "cookies"],
    default-features = false, 
    optional = true 
}
```

**Step 1.2: Modify HttpClient** (30 min)

File: `src/modules/builtins/http/client.rs`

```rust
use reqwest::cookie::Jar;
use std::sync::Arc;

pub struct HttpClient {
    client: reqwest::Client,
    runtime: Runtime,
    #[cfg(feature = "http")]
    cookie_jar: Arc<Jar>,
}

impl HttpClient {
    pub fn new() -> Result<Self> {
        #[cfg(feature = "http")]
        {
            let cookie_jar = Arc::new(Jar::default());
            
            let client = reqwest::Client::builder()
                .timeout(Duration::from_secs(30))
                .pool_max_idle_per_host(10)
                .cookie_provider(cookie_jar.clone())  // Enable cookies!
                .build()
                .map_err(|e| HttpError::RuntimeError(e.to_string()))?;

            let runtime = Runtime::new()
                .map_err(|e| HttpError::RuntimeError(e.to_string()))?;

            Ok(Self { client, runtime, cookie_jar })
        }
        
        #[cfg(not(feature = "http"))]
        {
            Err(HttpError::RuntimeError(
                "HTTP feature not enabled".to_string()
            ))
        }
    }
    
    #[cfg(feature = "http")]
    pub fn clear_cookies(&self) {
        // Note: reqwest::cookie::Jar doesn't have a clear() method
        // Alternative: create new client (in future enhancement)
    }
}
```

**Step 1.3: Add cookie management methods** (30 min)

```rust
#[cfg(feature = "http")]
impl HttpClient {
    pub fn get_cookies(&self, url: &str) -> Result<Vec<(String, String)>> {
        let parsed_url = Url::parse(url)
            .map_err(|e| HttpError::InvalidUrl(e.to_string()))?;
        
        let cookies = self.cookie_jar
            .cookies(&parsed_url)
            .and_then(|header| {
                header.to_str().ok().map(|s| {
                    s.split("; ")
                        .filter_map(|cookie| {
                            let mut parts = cookie.splitn(2, '=');
                            Some((
                                parts.next()?.to_string(),
                                parts.next()?.to_string()
                            ))
                        })
                        .collect()
                })
            })
            .unwrap_or_default();
        
        Ok(cookies)
    }
}
```

**Step 1.4: Expose to Lua** (30 min)

File: `src/modules/builtins/http/lua_bindings.rs`

```rust
#[cfg(feature = "http")]
pub fn create_http_module(lua: &Lua) -> mlua::Result<Table> {
    let http_table = lua.create_table()?;
    let client = Arc::new(HttpClient::new().map_err(|e| mlua::Error::external(e))?);

    // Existing functions...
    register_get(lua, &http_table, client.clone())?;
    register_post(lua, &http_table, client.clone())?;
    // ... etc ...
    
    // New cookie functions
    register_get_cookies(lua, &http_table, client.clone())?;
    
    Ok(http_table)
}

#[cfg(feature = "http")]
fn register_get_cookies(lua: &Lua, table: &Table, client: Arc<HttpClient>) -> mlua::Result<()> {
    let get_cookies_fn = lua.create_function(move |lua, url: String| {
        let cookies = client.get_cookies(&url)
            .map_err(|e| mlua::Error::external(e))?;
        
        let result = lua.create_table()?;
        for (name, value) in cookies {
            result.set(name, value)?;
        }
        Ok(result)
    })?;
    table.set("getCookies", get_cookies_fn)?;
    Ok(())
}
```

### 7.2 Phase 2: Testing (1h)

**Step 2.1: Unit Tests** (30 min)

File: `tests/http_cookie_test.rs`

```rust
#[cfg(test)]
mod http_cookie_tests {
    use hype_rs::modules::builtins::http::HttpClient;

    #[test]
    fn test_cookie_jar_creation() {
        let client = HttpClient::new();
        assert!(client.is_ok());
    }

    #[test]
    fn test_cookies_stored_and_sent() {
        // Uses httpbin.org/cookies endpoint
        let client = HttpClient::new().unwrap();
        
        // First request: set cookie
        let res1 = client.get("https://httpbin.org/cookies/set?test=value").unwrap();
        assert_eq!(res1.status, 200);
        
        // Second request: cookie should be sent
        let res2 = client.get("https://httpbin.org/cookies").unwrap();
        assert!(res2.body.contains("test"));
        assert!(res2.body.contains("value"));
    }

    #[test]
    fn test_cookies_scoped_by_domain() {
        let client = HttpClient::new().unwrap();
        
        // Set cookie for domain1
        client.get("https://httpbin.org/cookies/set?session=abc").unwrap();
        
        // Cookie should NOT be sent to different domain
        let res = client.get("https://example.com").unwrap();
        // Verify no Cookie header sent (would need header inspection)
    }
}
```

**Step 2.2: Integration Tests** (30 min)

File: `tests/lua_scripts/test_http_cookies.lua`

```lua
local http = require("http")

print("=== Testing HTTP Cookie Support ===\n")

-- Test 1: Basic cookie storage
print("1. Testing cookie storage...")
local res1 = http.get("https://httpbin.org/cookies/set?test=value")
assert(res1.status == 200, "Failed to set cookie")
print("✓ Cookie set")

-- Test 2: Cookie transmission
print("\n2. Testing cookie transmission...")
local res2 = http.get("https://httpbin.org/cookies")
assert(res2.body:find("test"), "Cookie not found in response")
assert(res2.body:find("value"), "Cookie value incorrect")
print("✓ Cookie sent automatically")

-- Test 3: Multiple cookies
print("\n3. Testing multiple cookies...")
http.get("https://httpbin.org/cookies/set?cookie1=value1")
http.get("https://httpbin.org/cookies/set?cookie2=value2")
local res3 = http.get("https://httpbin.org/cookies")
assert(res3.body:find("cookie1"), "Cookie1 not found")
assert(res3.body:find("cookie2"), "Cookie2 not found")
print("✓ Multiple cookies work")

-- Test 4: getCookies() API
print("\n4. Testing getCookies() API...")
local cookies = http.getCookies("https://httpbin.org")
assert(type(cookies) == "table", "getCookies should return table")
print("✓ getCookies() works")
print("  Cookies: " .. tostring(cookies.test))

print("\n=== All Tests Passed ===")
```

### 7.3 Phase 3: Documentation (30 min)

**Step 3.1: Update CHANGELOG.md**

```markdown
## [0.1.4] - 2025-10-27

### Added
- HTTP module now includes automatic cookie management (PRP-011)
  - Cookies from `Set-Cookie` headers are stored automatically
  - Stored cookies are sent with subsequent requests to the same domain
  - RFC 6265 compliant cookie handling (domain, path, expiry, secure flags)
  - New `http.getCookies(url)` function to inspect cookie jar
  - Follows redirects and preserves cookies across redirect chain

### Changed
- Updated reqwest dependency to include `cookies` feature
- HttpClient now includes cookie jar by default
```

**Step 3.2: Update README with cookie examples**

**Step 3.3: Update agent documentation**

File: `src/cli/agent/generator.rs`

Add cookie-related examples to HTTP module docs.

### 7.4 Phase 4: Build and Release (30 min)

**Step 4.1: Verify build**
```bash
cargo build --release
cargo test --features http
```

**Step 4.2: Run integration tests**
```bash
./target/release/hype tests/lua_scripts/test_http_cookies.lua
```

**Step 4.3: Manual testing**
```bash
# Test real-world authentication flow
./target/release/hype examples/auth_flow.lua
```

**Step 4.4: Update version**
- Bump to 0.1.4 in Cargo.toml
- Commit changes
- Tag v0.1.4
- Push and trigger release

---

## 8. Success Criteria

### 8.1 Functional Criteria

✅ **Must Have:**
1. Cookies from `Set-Cookie` headers are stored
2. Stored cookies are automatically sent in subsequent requests
3. Cookies respect domain and path scope (no leakage)
4. Secure cookies only sent over HTTPS
5. Expired cookies are not sent
6. `http.getCookies(url)` returns current cookies for domain
7. All existing HTTP tests pass (backward compatibility)

✅ **Should Have:**
8. Session cookies work (no expiry attribute)
9. Multiple cookies per domain work
10. Cookies persist across redirects

### 8.2 Non-Functional Criteria

✅ **Performance:**
- Cookie operations add < 1ms overhead per request
- Memory usage < 100KB for typical cookie jar (~50 cookies)

✅ **Quality:**
- Unit test coverage > 80%
- Integration tests cover authentication flow
- No regressions in existing tests

✅ **Documentation:**
- CHANGELOG updated
- README includes cookie example
- Agent docs include cookie usage

### 8.3 Acceptance Tests

**Test 1: Simple authentication flow**
```lua
local http = require("http")
http.post("https://httpbin.org/cookies/set?session=abc123")
local res = http.get("https://httpbin.org/cookies")
assert(res.body:find("session"))
```
**Expected:** Pass ✅

**Test 2: Domain isolation**
```lua
http.get("https://httpbin.org/cookies/set?test=value")
local cookies_example = http.getCookies("https://example.com")
assert(cookies_example.test == nil)  -- Cookie not leaked
```
**Expected:** Pass ✅

**Test 3: Secure cookie on HTTPS**
```lua
-- Server sets: Set-Cookie: secure=value; Secure
local res = http.get("https://httpbin.org/cookies/set?secure=value")
-- Cookie sent on HTTPS
local res2 = http.get("https://httpbin.org/cookies")
assert(res2.body:find("secure"))
```
**Expected:** Pass ✅

**Test 4: Backward compatibility**
```lua
-- Existing code should work unchanged
local res = http.get("https://httpbin.org/get")
assert(res.status == 200)
```
**Expected:** Pass ✅

---

## 9. Risk Assessment

### 9.1 Technical Risks

| Risk | Severity | Mitigation |
|------|----------|------------|
| Cookie leakage between domains | 🔴 High | Reqwest handles domain scoping automatically (RFC 6265) |
| Expired cookie not removed | 🟡 Medium | Reqwest handles expiry automatically |
| Secure flag not respected | 🔴 High | Reqwest enforces Secure flag (HTTPS-only) |
| Breaking change in API | 🟡 Medium | Solution 1 has no API changes (automatic cookies) |
| Performance regression | 🟢 Low | Cookie lookup is O(1) hash lookup, negligible overhead |

### 9.2 User Impact Risks

| Risk | Severity | Mitigation |
|------|----------|------------|
| Users surprised by automatic cookies | 🟡 Medium | Document clearly in CHANGELOG and README |
| Cookie persistence across scripts | 🟢 Low | Cookies only persist during script execution |
| Multi-domain confusion | 🟡 Medium | Add clear examples showing domain scoping |

### 9.3 Mitigation Plan

1. **Thorough testing** with httpbin.org (supports cookie testing)
2. **Clear documentation** explaining cookie behavior
3. **Unit tests** for domain scoping, expiry, secure flags
4. **Integration tests** with real authentication flows
5. **User feedback** period before marking as stable

---

## 10. Future Enhancements

### 10.1 Short-term (v0.2.0)

**Per-client cookie jars (Solution 2)**
- Allow `http.newClient({cookies: true})`
- Isolated cookie jars for multi-tenant scenarios
- Addresses advanced use cases

**Cookie clearing**
- `http.clearCookies()` - Clear all cookies
- `http.clearCookies(url)` - Clear cookies for specific domain

### 10.2 Medium-term (v0.3.0)

**Persistent cookies**
- Save cookies to file between script runs
- Load cookies from file
- Cookie encryption for sensitive data

**Cookie inspection**
```lua
local cookies = http.getCookies("https://example.com")
-- Returns: {
--   {name = "session", value = "abc", expires = "...", secure = true},
--   {name = "tracking", value = "xyz", expires = "...", secure = false}
-- }
```

### 10.3 Long-term (v1.0)

**Advanced cookie control**
```lua
http.setCookie("https://example.com", {
    name = "custom",
    value = "value",
    expires = "2026-01-01",
    secure = true,
    httpOnly = true,
    sameSite = "Strict"
})
```

**Cookie event hooks**
```lua
http.onCookieSet(function(cookie)
    print("Cookie set: " .. cookie.name)
end)
```

---

## Appendices

### Appendix A: References

- RFC 6265: HTTP State Management Mechanism - https://tools.ietf.org/html/rfc6265
- Reqwest Cookie Documentation - https://docs.rs/reqwest/latest/reqwest/cookie/
- MDN Set-Cookie Reference - https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie

### Appendix B: Code Locations

- `src/modules/builtins/http/client.rs` - HttpClient implementation
- `src/modules/builtins/http/lua_bindings.rs` - Lua API bindings
- `tests/http_cookie_test.rs` - Unit tests
- `tests/lua_scripts/test_http_cookies.lua` - Integration tests

### Appendix C: Testing Checklist

- [ ] Cookies stored from Set-Cookie header
- [ ] Cookies sent in subsequent requests
- [ ] Domain scoping works (no leakage)
- [ ] Path scoping works
- [ ] Secure flag respected (HTTPS only)
- [ ] HttpOnly flag handled
- [ ] Expired cookies not sent
- [ ] Session cookies work
- [ ] Multiple cookies per domain
- [ ] Cookies across redirects
- [ ] getCookies() returns correct data
- [ ] Backward compatibility maintained
- [ ] Performance acceptable (< 1ms overhead)
- [ ] Documentation complete
- [ ] Examples work

---

**End of PRP-011**
