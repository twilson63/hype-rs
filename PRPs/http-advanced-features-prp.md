# PRP-012: HTTP Advanced Features - Proxy, Forms, and Auth Helpers

**Status:** Proposed  
**Priority:** High  
**Created:** 2025-10-27  
**Author:** AI Assistant  
**Estimated Effort:** 12-18 hours

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

The HTTP module currently requires manual implementation for three common use cases:

**1. Proxy Configuration**
- No way to configure HTTP/HTTPS proxies
- Blocks enterprise users behind corporate firewalls
- Cannot use SOCKS proxies for privacy/tunneling

**2. Form Data Handling**
- Manual URL encoding for form submissions
- Manual multipart/form-data construction for file uploads
- Error-prone string building for common operations

**3. Authentication Helpers**
- Manual Base64 encoding for Basic Auth
- Manual header construction for Bearer tokens
- Repetitive boilerplate for authenticated requests

### 1.2 Current Pain Points

#### Proxy Issue (Enterprise Blocker)
```lua
-- Currently: IMPOSSIBLE without system proxy
http.get("https://api.example.com")  -- Fails behind corporate proxy
-- No workaround available!
```

#### Form Handling Issue
```lua
-- Currently: Manual and error-prone
local body = "username=" .. escape(user) .. "&password=" .. escape(pass)
http.post(url, {
    body = body,
    headers = {["Content-Type"] = "application/x-www-form-urlencoded"}
})

-- File upload: 15+ lines of manual boundary management
local boundary = "----Boundary" .. os.time()
local body = string.format(
    "--%s\r\nContent-Disposition: form-data; name=\"file\"; filename=\"%s\"\r\nContent-Type: %s\r\n\r\n%s\r\n--%s--\r\n",
    boundary, filename, content_type, file_content, boundary
)
```

#### Auth Issue
```lua
-- Currently: Manual encoding
local function base64_encode(str)
    -- 20+ lines of base64 implementation
    -- Or require external base64 library
end

local auth = base64_encode("user:pass")
http.get(url, {headers = {Authorization = "Basic " .. auth}})
```

### 1.3 Impact

**Severity:** High

**Affected Users:**
- 🏢 **Enterprise users** - Cannot use without proxy support (BLOCKER)
- 🔐 **API developers** - Repetitive auth boilerplate (PAIN)
- 📊 **Web scrapers** - Manual form encoding (ERROR-PRONE)
- 📁 **File uploaders** - Complex multipart construction (COMPLEX)

**Market Impact:**
- Enterprise adoption blocked without proxy support
- Competitive disadvantage vs Python requests, Node fetch
- User complaints about missing "basic" features

### 1.4 Project Goals

1. **Primary Goal:** Add proxy, form, and auth helpers to HTTP module
2. **API Design:** Simple, intuitive APIs matching industry standards
3. **Backward Compatibility:** Zero breaking changes to existing code
4. **Standards Compliance:** Follow HTTP standards (RFC 7617 for auth, RFC 7578 for multipart)
5. **Enterprise Ready:** Support corporate proxy configurations

---

## 2. Current State Analysis

### 2.1 Existing HTTP Module Capabilities

**File:** `src/modules/builtins/http/client.rs` (290 lines)

**Current features:**
- ✅ HTTP methods (GET, POST, PUT, DELETE, PATCH, HEAD, fetch)
- ✅ Custom headers
- ✅ Request/response body handling
- ✅ JSON serialization
- ✅ TLS with rustls
- ✅ Cookie jar (v0.1.4)
- ✅ Connection pooling
- ❌ **No proxy support**
- ❌ **No form helpers**
- ❌ **No auth helpers**

### 2.2 Dependencies

**Current `Cargo.toml`:**
```toml
reqwest = { 
    version = "0.12", 
    features = ["json", "blocking", "rustls-tls", "cookies"],
    default-features = false, 
    optional = true 
}
```

**Reqwest capabilities (already available):**
- ✅ `reqwest::Proxy` - Proxy configuration (not exposed)
- ✅ `reqwest::multipart` - Multipart forms (not exposed, requires feature)
- ⚠️ Base64 encoding - Need to add dependency
- ⚠️ URL encoding - Need percent-encoding crate or use reqwest's

### 2.3 Gap Analysis

| Feature | Reqwest Support | Hype-rs Exposed | Gap |
|---------|----------------|-----------------|-----|
| HTTP Proxy | ✅ Yes | ❌ No | HIGH |
| HTTPS Proxy | ✅ Yes | ❌ No | HIGH |
| SOCKS Proxy | ✅ Yes | ❌ No | MEDIUM |
| URL-encoded forms | ⚠️ Manual | ❌ No | MEDIUM |
| Multipart forms | ✅ Yes (feature) | ❌ No | MEDIUM |
| Basic Auth | ⚠️ Manual | ❌ No | HIGH |
| Bearer Auth | ⚠️ Manual | ❌ No | HIGH |

---

## 3. Technical Requirements

### 3.1 Functional Requirements

#### FR-1: Proxy Configuration

**FR-1.1:** System MUST support HTTP proxy configuration
```lua
http.get(url, {proxy = "http://proxy.corp.com:8080"})
```

**FR-1.2:** System MUST support HTTPS proxy configuration
```lua
http.get(url, {proxy = "https://secure-proxy.corp.com:8443"})
```

**FR-1.3:** System MUST support SOCKS5 proxy configuration
```lua
http.get(url, {proxy = "socks5://proxy.example.com:1080"})
```

**FR-1.4:** System MUST support proxy authentication
```lua
http.get(url, {
    proxy = "http://proxy.corp.com:8080",
    proxyAuth = {username = "user", password = "pass"}
})
```

**FR-1.5:** System SHOULD support environment variable proxy detection
- Read `HTTP_PROXY`, `HTTPS_PROXY`, `ALL_PROXY`
- Follow standard Unix proxy conventions

**FR-1.6:** System MUST support per-request proxy override
```lua
-- Global proxy
local client = http.newClient({proxy = "http://proxy:8080"})

-- Override for one request
client.get(url, {proxy = nil})  -- Direct connection
```

#### FR-2: Form Data Handling

**FR-2.1:** System MUST provide URL-encoded form helper
```lua
http.postForm(url, {
    username = "alice",
    password = "secret",
    remember = "true"
})
-- Automatically sets Content-Type: application/x-www-form-urlencoded
-- Automatically URL-encodes values
```

**FR-2.2:** System MUST provide multipart form helper
```lua
http.uploadFile(url, {
    file = {
        filename = "document.pdf",
        content = file_data,
        contentType = "application/pdf"
    },
    description = "Q4 Report"
})
-- Automatically sets Content-Type: multipart/form-data; boundary=...
-- Automatically constructs multipart message
```

**FR-2.3:** System MUST handle special characters in form values
- URL encoding for form-urlencoded
- Proper escaping in multipart boundaries

**FR-2.4:** System SHOULD support multiple file uploads
```lua
http.uploadFiles(url, {
    files = {
        {filename = "doc1.pdf", content = data1},
        {filename = "doc2.pdf", content = data2}
    },
    metadata = "batch upload"
})
```

#### FR-3: Authentication Helpers

**FR-3.1:** System MUST provide Basic Auth helper
```lua
http.get(url, {
    auth = {
        type = "basic",
        username = "user",
        password = "pass"
    }
})
-- Automatically encodes to Base64
-- Sets Authorization: Basic <base64>
```

**FR-3.2:** System MUST provide Bearer token helper
```lua
http.get(url, {
    auth = {
        type = "bearer",
        token = "abc123xyz"
    }
})
-- Sets Authorization: Bearer abc123xyz
```

**FR-3.3:** System SHOULD provide generic auth header helper
```lua
http.get(url, {
    auth = {
        type = "custom",
        value = "CustomScheme token123"
    }
})
-- Sets Authorization: CustomScheme token123
```

### 3.2 Non-Functional Requirements

**NFR-1: Performance**
- Proxy overhead MUST be < 5ms per request
- Form encoding MUST be < 10ms for typical forms
- Base64 encoding MUST be < 1ms for auth headers

**NFR-2: Compatibility**
- Solution MUST maintain backward compatibility (100%)
- Solution MUST work with existing cookie support (v0.1.4)
- Solution MUST work with all HTTP methods

**NFR-3: Standards Compliance**
- Proxy: Follow standard proxy protocols (HTTP CONNECT, SOCKS5)
- Forms: RFC 1738 (URL encoding), RFC 7578 (multipart)
- Auth: RFC 7617 (Basic Auth), RFC 6750 (Bearer tokens)

**NFR-4: Security**
- Proxy credentials MUST NOT be logged
- Auth credentials MUST NOT be logged
- HTTPS proxy MUST validate certificates
- Base64 implementation MUST be correct (no custom impl)

**NFR-5: Code Quality**
- Implementation MUST include unit tests (>80% coverage)
- Implementation MUST include integration tests
- Error messages MUST be clear and actionable

### 3.3 Dependencies

**Required:**
- `base64` crate for Basic Auth encoding
- `percent-encoding` crate for URL encoding (or use reqwest's)
- `multipart` feature for reqwest (already part of reqwest)

**Optional:**
- `serde_urlencoded` for form encoding (simpler API)

---

## 4. Proposed Solutions

### Solution 1: Comprehensive Feature Addition (Full Implementation)

**Description:**  
Add all three features (proxy, forms, auth) with complete APIs including advanced options, per-client configuration, and global defaults.

**Architecture:**

```rust
// src/modules/builtins/http/proxy.rs
pub struct ProxyConfig {
    url: String,
    auth: Option<(String, String)>,
    bypass: Vec<String>,  // No-proxy hosts
}

// src/modules/builtins/http/forms.rs
pub struct FormData {
    fields: HashMap<String, String>,
}

pub struct MultipartForm {
    fields: HashMap<String, String>,
    files: Vec<FileField>,
}

// src/modules/builtins/http/auth.rs
pub enum AuthType {
    Basic { username: String, password: String },
    Bearer { token: String },
    Custom { value: String },
}

impl HttpClient {
    pub fn new_with_proxy(proxy: ProxyConfig) -> Result<Self> { ... }
}
```

**Lua API:**

```lua
-- Proxy support
http.get(url, {proxy = "http://proxy:8080"})
http.get(url, {
    proxy = {
        url = "http://proxy:8080",
        auth = {username = "user", password = "pass"},
        bypass = {"localhost", "*.internal.com"}
    }
})

-- Environment variables
http.get(url, {proxy = "env"})  -- Use HTTP_PROXY env var

-- Forms
http.postForm(url, {field1 = "value1", field2 = "value2"})
http.uploadFile(url, {
    file = {filename = "doc.pdf", content = data},
    description = "metadata"
})
http.uploadFiles(url, {
    files = {{filename = "a.txt", content = d1}, {filename = "b.txt", content = d2}}
})

-- Auth
http.get(url, {auth = {type = "basic", username = "u", password = "p"}})
http.get(url, {auth = {type = "bearer", token = "xyz"}})
http.get(url, {auth = {type = "custom", value = "Scheme creds"}})

-- Per-client configuration
local client = http.newClient({
    proxy = "http://proxy:8080",
    auth = {type = "bearer", token = "global_token"}
})
client.get(url)  -- Uses client's proxy and auth
```

**Pros:**
- ✅ **Complete feature set** - Covers all use cases
- ✅ **Flexible configuration** - Per-request, per-client, global
- ✅ **Advanced options** - Proxy bypass, multiple files, custom auth
- ✅ **Future-proof** - Easy to extend with more features
- ✅ **Enterprise-ready** - Full proxy auth, bypass lists

**Cons:**
- ❌ **High complexity** - ~300 lines of new code
- ❌ **Longer implementation** - 15-20 hours
- ❌ **More testing needed** - Many edge cases
- ❌ **API surface** - More functions to learn
- ❌ **Maintenance burden** - More code to maintain

**Use cases:**
- ✅ Enterprise deployments with complex proxy rules
- ✅ Advanced file upload scenarios
- ✅ Multi-tenant auth requirements
- ✅ Power users needing fine control

---

### Solution 2: Essential Features Only (Minimal Implementation)

**Description:**  
Implement only the most common use cases with simple APIs. No advanced options, no per-client config, just basic functionality.

**Architecture:**

```rust
// All in src/modules/builtins/http/client.rs
impl HttpClient {
    pub fn with_proxy(proxy_url: &str) -> Result<Self> {
        let proxy = reqwest::Proxy::all(proxy_url)?;
        let client = reqwest::Client::builder()
            .proxy(proxy)
            .build()?;
        // ...
    }
}

// Helper functions, not separate modules
fn encode_basic_auth(username: &str, password: &str) -> String {
    use base64::Engine;
    let credentials = format!("{}:{}", username, password);
    base64::engine::general_purpose::STANDARD.encode(credentials)
}

fn url_encode_form(data: HashMap<String, String>) -> String {
    data.iter()
        .map(|(k, v)| format!("{}={}", encode(k), encode(v)))
        .collect::<Vec<_>>()
        .join("&")
}
```

**Lua API:**

```lua
-- Proxy: Simple string only
http.get(url, {proxy = "http://proxy:8080"})

-- Forms: Simple helper
http.postForm(url, {field1 = "value1", field2 = "value2"})

-- File upload: One file only
http.uploadFile(url, "fieldname", {
    filename = "doc.pdf",
    content = file_data
})

-- Auth: Simple object
http.get(url, {auth = {username = "user", password = "pass"}})
http.get(url, {authToken = "bearer_token_here"})
```

**Pros:**
- ✅ **Fast implementation** - 8-12 hours
- ✅ **Simple API** - Easy to learn and use
- ✅ **Low maintenance** - Less code to maintain
- ✅ **Covers 90% of use cases** - Most users don't need advanced features
- ✅ **Easy to extend later** - Can add advanced features in next version

**Cons:**
- ⚠️ **Limited flexibility** - No proxy auth, no bypass lists
- ⚠️ **No per-client config** - Only per-request options
- ⚠️ **Single file upload** - Must call multiple times for multiple files
- ⚠️ **Basic auth only** - Username/password, no bearer token helper
- ⚠️ **No environment variables** - Must specify proxy explicitly

**Use cases:**
- ✅ Simple proxy scenarios
- ✅ Basic form submissions
- ✅ Single file uploads
- ✅ Basic authentication
- ❌ Complex enterprise proxy configurations
- ❌ Bulk file uploads

---

### Solution 3: Hybrid Approach (Progressive Enhancement)

**Description:**  
Start with essential features (Solution 2) but design APIs to allow easy extension. Add advanced features as separate optional parameters that can be added later without breaking changes.

**Architecture:**

```rust
// src/modules/builtins/http/client.rs - Core impl
pub struct RequestOptions {
    proxy: Option<ProxyOption>,
    auth: Option<AuthOption>,
    // ... extensible
}

pub enum ProxyOption {
    Simple(String),
    Advanced(ProxyConfig),  // Can add later
}

pub enum AuthOption {
    Basic { username: String, password: String },
    Bearer(String),
    Custom(String),  // Can add later
}

// Start simple, allow complex later
impl HttpClient {
    pub fn apply_proxy(&mut self, proxy: ProxyOption) -> Result<()> {
        match proxy {
            ProxyOption::Simple(url) => {
                let proxy = reqwest::Proxy::all(url)?;
                // Apply proxy
            }
            ProxyOption::Advanced(config) => {
                // Add in future version
                unimplemented!("Advanced proxy config coming in v0.3.0")
            }
        }
    }
}
```

**Lua API (v0.2.0):**

```lua
-- Phase 1: Simple APIs
http.get(url, {proxy = "http://proxy:8080"})
http.postForm(url, {field1 = "value1"})
http.uploadFile(url, {file = "path/to/file.txt"})
http.get(url, {auth = {username = "u", password = "p"}})
http.get(url, {authToken = "bearer_token"})
```

**Lua API (v0.3.0 - Future):**

```lua
-- Phase 2: Advanced APIs (backward compatible)
http.get(url, {
    proxy = {
        url = "http://proxy:8080",
        auth = {username = "user", password = "pass"},  -- NEW
        bypass = {"localhost"}  -- NEW
    }
})

http.uploadFiles(url, {  -- NEW plural function
    files = {
        {filename = "a.txt", content = data1},
        {filename = "b.txt", content = data2}
    }
})
```

**Pros:**
- ✅ **Fast initial delivery** - 10-14 hours for Phase 1
- ✅ **Future-proof design** - Can add features without breaking
- ✅ **Balanced complexity** - Not too simple, not too complex
- ✅ **User feedback driven** - Add advanced features based on demand
- ✅ **Covers immediate needs** - Proxy, forms, auth work now
- ✅ **Clear upgrade path** - Simple → Advanced is natural

**Cons:**
- ⚠️ **Design overhead** - Must think about future extensions
- ⚠️ **Two-phase delivery** - Some features delayed to v0.3.0
- ⚠️ **API consistency** - Must maintain compatibility between versions

**Use cases:**
- ✅ **All use cases from Solution 2** (Phase 1)
- ✅ **All use cases from Solution 1** (Phase 2, future)
- ✅ Best for: Most users now, power users later

---

## 5. Solution Comparison

### 5.1 Feature Matrix

| Feature | Solution 1: Full | Solution 2: Minimal | Solution 3: Hybrid |
|---------|-----------------|---------------------|-------------------|
| **Proxy** |
| HTTP/HTTPS proxy | ✅ Yes | ✅ Yes | ✅ Yes |
| SOCKS proxy | ✅ Yes | ❌ No | 🔄 Future |
| Proxy auth | ✅ Yes | ❌ No | 🔄 Future |
| Proxy bypass list | ✅ Yes | ❌ No | 🔄 Future |
| Env var support | ✅ Yes | ❌ No | 🔄 Future |
| **Forms** |
| URL-encoded | ✅ Yes | ✅ Yes | ✅ Yes |
| Single file upload | ✅ Yes | ✅ Yes | ✅ Yes |
| Multiple files | ✅ Yes | ❌ No | 🔄 Future |
| Custom content-type | ✅ Yes | ⚠️ Limited | ✅ Yes |
| **Auth** |
| Basic Auth | ✅ Yes | ✅ Yes | ✅ Yes |
| Bearer token | ✅ Yes | ⚠️ String only | ✅ Yes |
| Custom auth | ✅ Yes | ❌ No | 🔄 Future |
| **Config** |
| Per-request | ✅ Yes | ✅ Yes | ✅ Yes |
| Per-client | ✅ Yes | ❌ No | 🔄 Future |
| Global defaults | ✅ Yes | ❌ No | 🔄 Future |

### 5.2 Implementation Effort

| Task | Solution 1 | Solution 2 | Solution 3 |
|------|-----------|-----------|-----------|
| Proxy implementation | 4-5h | 2-3h | 3-4h |
| Forms implementation | 4-5h | 2-3h | 3-4h |
| Auth implementation | 2-3h | 2-3h | 2-3h |
| API design | 2h | 1h | 2h |
| Testing | 4-5h | 2-3h | 3-4h |
| Documentation | 2h | 1h | 1-2h |
| **Total** | **18-22h** | **10-13h** | **14-18h** |

### 5.3 Risk Assessment

**Solution 1: Full Implementation**
- 🟡 **Scope creep risk** - HIGH (many features, many edge cases)
- 🟢 **Technical risk** - LOW (reqwest supports everything)
- 🟡 **Maintenance risk** - MEDIUM (more code to maintain)
- 🟢 **User satisfaction** - HIGH (covers all use cases)
- 🔴 **Time risk** - HIGH (might take longer than estimated)

**Solution 2: Minimal Implementation**
- 🟢 **Scope creep risk** - LOW (limited features)
- 🟢 **Technical risk** - LOW (simple implementation)
- 🟢 **Maintenance risk** - LOW (less code)
- 🟡 **User satisfaction** - MEDIUM (missing advanced features)
- 🟢 **Time risk** - LOW (quick delivery)

**Solution 3: Hybrid Approach**
- 🟢 **Scope creep risk** - LOW (defined phases)
- 🟢 **Technical risk** - LOW (proven patterns)
- 🟢 **Maintenance risk** - LOW (incremental)
- 🟢 **User satisfaction** - HIGH (now + future)
- 🟢 **Time risk** - LOW-MEDIUM (Phase 1 quick)

### 5.4 User Experience Comparison

**Scenario 1: Corporate proxy**

```lua
-- Solution 1 (Full)
http.get(url, {
    proxy = {
        url = "http://proxy:8080",
        auth = {username = "user", password = "pass"},
        bypass = {"*.internal.com"}
    }
})

-- Solution 2 (Minimal)
-- Not supported! Must use system proxy or fail

-- Solution 3 (Hybrid)
-- Phase 1: Simple
http.get(url, {proxy = "http://user:pass@proxy:8080"})
-- Phase 2: Advanced (future)
-- Same as Solution 1
```

**Winner:** Solution 1 for enterprise, Solution 3 for most users

**Scenario 2: Form submission**

```lua
-- All solutions identical
http.postForm("https://example.com/login", {
    username = "alice",
    password = "secret"
})
```

**Winner:** Tie (all solutions provide this)

**Scenario 3: File upload**

```lua
-- Solution 1 & 3
http.uploadFile(url, {
    file = {filename = "doc.pdf", content = data},
    metadata = "Q4 report"
})

-- Solution 2
http.uploadFile(url, "file", {
    filename = "doc.pdf",
    content = data
})
-- metadata: separate request ❌
```

**Winner:** Solutions 1 & 3 (better API)

**Scenario 4: Multi-file upload**

```lua
-- Solution 1
http.uploadFiles(url, {
    files = {
        {filename = "a.pdf", content = d1},
        {filename = "b.pdf", content = d2}
    }
})

-- Solution 2
for i, file in ipairs(files) do
    http.uploadFile(url, "file", file)
end
-- Multiple requests ❌

-- Solution 3
-- Phase 1: Same as Solution 2
-- Phase 2: Same as Solution 1
```

**Winner:** Solution 1 now, Solution 3 future

---

## 6. Recommended Solution

### 6.1 Decision: Solution 3 (Hybrid Approach)

**Rationale:**

1. **Balanced delivery timeline**
   - Phase 1 delivers in 14-18 hours (acceptable)
   - Unblocks enterprise users immediately
   - Provides essential features for 90% of use cases

2. **Risk management**
   - Lower implementation risk than Solution 1
   - Defined scope prevents scope creep
   - User feedback informs Phase 2 priorities

3. **User experience**
   - Simple APIs for common cases (matches Solution 2)
   - Clear upgrade path to advanced features
   - No breaking changes between phases

4. **Future-proof**
   - API design allows natural extension
   - Can add features based on real user demand
   - Not over-engineering for hypothetical needs

5. **Best ROI**
   - 90% of value in 75% of the time
   - Remaining 10% of value can wait for user validation
   - Focuses resources on proven needs

**Trade-offs accepted:**
- ❌ Advanced proxy features delayed (Phase 2)
- ❌ Multiple file uploads delayed (Phase 2)
- ❌ Custom auth schemes delayed (Phase 2)
- ✅ Core functionality delivered quickly
- ✅ Enterprise blocker removed
- ✅ API remains simple

### 6.2 Delivery Phases

**Phase 1 (v0.2.0) - This PRP**

Core features:
- ✅ HTTP/HTTPS proxy (simple URL)
- ✅ URL-encoded forms (`postForm()`)
- ✅ Single file upload (`uploadFile()`)
- ✅ Basic Auth helper
- ✅ Bearer token helper

**Phase 2 (v0.3.0) - Future PRP**

Advanced features (if user demand exists):
- 🔄 Proxy auth and bypass lists
- 🔄 SOCKS proxy support
- 🔄 Multiple file upload (`uploadFiles()`)
- 🔄 Custom auth schemes
- 🔄 Per-client configuration

---

## 7. Implementation Plan

### 7.1 Phase 1: Core Implementation (14-18 hours)

#### Step 1: Add Dependencies (30 min)

**File:** `Cargo.toml`

```toml
[dependencies]
base64 = "0.21"
serde_urlencoded = "0.7"  # For form encoding
```

Enable multipart feature in reqwest:
```toml
reqwest = { 
    version = "0.12", 
    features = ["json", "blocking", "rustls-tls", "cookies", "multipart"],
    default-features = false, 
    optional = true 
}
```

#### Step 2: Implement Proxy Support (3-4 hours)

**File:** `src/modules/builtins/http/client.rs`

```rust
use reqwest::Proxy;

pub struct HttpClient {
    client: reqwest::Client,
    runtime: Runtime,
    cookie_jar: Arc<Jar>,
}

impl HttpClient {
    // Add new constructor with proxy
    pub fn new_with_proxy(proxy_url: &str) -> Result<Self> {
        let cookie_jar = Arc::new(Jar::default());
        
        let proxy = Proxy::all(proxy_url)
            .map_err(|e| HttpError::RuntimeError(format!("Invalid proxy: {}", e)))?;
        
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .pool_max_idle_per_host(10)
            .cookie_provider(cookie_jar.clone())
            .proxy(proxy)
            .build()
            .map_err(|e| HttpError::RuntimeError(e.to_string()))?;

        let runtime = Runtime::new()
            .map_err(|e| HttpError::RuntimeError(e.to_string()))?;

        Ok(Self { client, runtime, cookie_jar })
    }
    
    // Modify existing methods to accept proxy in options
    pub fn get_with_options(&self, url: &str, options: RequestOptions) -> Result<HttpResponse> {
        // If proxy specified, create temporary client with proxy
        if let Some(proxy_url) = options.proxy {
            let temp_client = Self::new_with_proxy(&proxy_url)?;
            return temp_client.get(url);
        }
        self.get(url)
    }
}

struct RequestOptions {
    proxy: Option<String>,
    auth: Option<AuthOptions>,
    // ... other options
}
```

#### Step 3: Implement Form Helpers (3-4 hours)

**File:** `src/modules/builtins/http/forms.rs` (NEW)

```rust
use serde_urlencoded;
use std::collections::HashMap;

pub fn encode_form_urlencoded(fields: HashMap<String, String>) -> Result<String, String> {
    serde_urlencoded::to_string(&fields)
        .map_err(|e| format!("Form encoding error: {}", e))
}

pub fn build_multipart_form(
    fields: HashMap<String, String>,
    file: FileField,
) -> Result<reqwest::blocking::multipart::Form, String> {
    let mut form = reqwest::blocking::multipart::Form::new();
    
    // Add text fields
    for (key, value) in fields {
        form = form.text(key, value);
    }
    
    // Add file
    let part = reqwest::blocking::multipart::Part::bytes(file.content)
        .file_name(file.filename)
        .mime_str(&file.content_type)
        .map_err(|e| format!("Invalid MIME type: {}", e))?;
    
    form = form.part(file.field_name, part);
    
    Ok(form)
}

pub struct FileField {
    field_name: String,
    filename: String,
    content: Vec<u8>,
    content_type: String,
}
```

**File:** `src/modules/builtins/http/client.rs`

```rust
use super::forms;

impl HttpClient {
    pub fn post_form(
        &self,
        url: &str,
        fields: HashMap<String, String>,
    ) -> Result<HttpResponse> {
        let body = forms::encode_form_urlencoded(fields)
            .map_err(|e| HttpError::RequestError(e))?;
        
        let mut headers = HashMap::new();
        headers.insert(
            "Content-Type".to_string(),
            "application/x-www-form-urlencoded".to_string()
        );
        
        self.post(url, Some(body), Some(headers))
    }
    
    pub fn upload_file(
        &self,
        url: &str,
        fields: HashMap<String, String>,
        file: forms::FileField,
    ) -> Result<HttpResponse> {
        let parsed_url = Url::parse(url)
            .map_err(|e| HttpError::RequestError(format!("Invalid URL: {}", e)))?;
        
        let form = forms::build_multipart_form(fields, file)
            .map_err(|e| HttpError::RequestError(e))?;
        
        self.runtime.block_on(async {
            let response = self.client
                .post(parsed_url.as_str())
                .multipart(form)
                .send()
                .await?;
            
            HttpResponse::from_reqwest(response)
                .await
                .map_err(Into::into)
        })
    }
}
```

#### Step 4: Implement Auth Helpers (2-3 hours)

**File:** `src/modules/builtins/http/auth.rs` (NEW)

```rust
use base64::{Engine as _, engine::general_purpose::STANDARD};

pub enum AuthOption {
    Basic { username: String, password: String },
    Bearer(String),
}

impl AuthOption {
    pub fn to_header_value(&self) -> String {
        match self {
            AuthOption::Basic { username, password } => {
                let credentials = format!("{}:{}", username, password);
                let encoded = STANDARD.encode(credentials.as_bytes());
                format!("Basic {}", encoded)
            }
            AuthOption::Bearer(token) => {
                format!("Bearer {}", token)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_auth_encoding() {
        let auth = AuthOption::Basic {
            username: "user".to_string(),
            password: "pass".to_string(),
        };
        let header = auth.to_header_value();
        assert_eq!(header, "Basic dXNlcjpwYXNz");
    }

    #[test]
    fn test_bearer_token() {
        let auth = AuthOption::Bearer("abc123".to_string());
        let header = auth.to_header_value();
        assert_eq!(header, "Bearer abc123");
    }
}
```

**File:** `src/modules/builtins/http/client.rs`

```rust
use super::auth::AuthOption;

impl HttpClient {
    fn apply_auth(&self, headers: &mut HashMap<String, String>, auth: AuthOption) {
        headers.insert(
            "Authorization".to_string(),
            auth.to_header_value()
        );
    }
    
    // Modify methods to accept auth
    pub fn get_with_auth(&self, url: &str, auth: AuthOption) -> Result<HttpResponse> {
        let mut headers = HashMap::new();
        self.apply_auth(&mut headers, auth);
        
        // Use existing get with headers
        let parsed_url = Url::parse(url)?;
        self.runtime.block_on(async {
            let mut request = self.client.get(parsed_url.as_str());
            for (key, value) in headers {
                request = request.header(key, value);
            }
            let response = request.send().await?;
            HttpResponse::from_reqwest(response).await.map_err(Into::into)
        })
    }
}
```

#### Step 5: Expose to Lua (3-4 hours)

**File:** `src/modules/builtins/http/lua_bindings.rs`

```rust
// Register new functions
pub fn create_http_module(lua: &Lua) -> mlua::Result<Table> {
    let http_table = lua.create_table()?;
    let client = Arc::new(HttpClient::new().map_err(|e| mlua::Error::external(e))?);

    // Existing functions...
    register_get(lua, &http_table, client.clone())?;
    register_post(lua, &http_table, client.clone())?;
    // ...
    
    // NEW functions
    register_post_form(lua, &http_table, client.clone())?;
    register_upload_file(lua, &http_table, client.clone())?;
    
    Ok(http_table)
}

fn register_post_form(lua: &Lua, table: &Table, client: Arc<HttpClient>) -> mlua::Result<()> {
    let post_form_fn = lua.create_function(move |lua, (url, fields): (String, Table)| {
        let mut field_map = HashMap::new();
        for pair in fields.pairs::<String, String>() {
            let (key, value) = pair?;
            field_map.insert(key, value);
        }
        
        let response = client.post_form(&url, field_map)
            .map_err(|e| mlua::Error::external(e))?;
        create_response_table(lua, response)
    })?;
    table.set("postForm", post_form_fn)?;
    Ok(())
}

fn register_upload_file(lua: &Lua, table: &Table, client: Arc<HttpClient>) -> mlua::Result<()> {
    let upload_fn = lua.create_function(move |lua, (url, options): (String, Table)| {
        // Parse options table
        let file_table: Table = options.get("file")?;
        let filename: String = file_table.get("filename")?;
        let content: String = file_table.get("content")?;
        let content_type: String = file_table.get("contentType")
            .unwrap_or_else(|_| "application/octet-stream".to_string());
        
        let file = FileField {
            field_name: "file".to_string(),
            filename,
            content: content.into_bytes(),
            content_type,
        };
        
        // Parse additional fields
        let mut fields = HashMap::new();
        if let Ok(metadata) = options.get::<_, String>("metadata") {
            fields.insert("metadata".to_string(), metadata);
        }
        
        let response = client.upload_file(&url, fields, file)
            .map_err(|e| mlua::Error::external(e))?;
        create_response_table(lua, response)
    })?;
    table.set("uploadFile", upload_fn)?;
    Ok(())
}

// Modify existing functions to support options
fn register_get(lua: &Lua, table: &Table, client: Arc<HttpClient>) -> mlua::Result<()> {
    let get_fn = lua.create_function(move |lua, (url, options): (String, Option<Table>)| {
        if let Some(opts) = options {
            let mut req_options = RequestOptions::default();
            
            // Parse proxy
            if let Ok(proxy) = opts.get::<_, String>("proxy") {
                req_options.proxy = Some(proxy);
            }
            
            // Parse auth
            if let Ok(auth_table) = opts.get::<_, Table>("auth") {
                let username: String = auth_table.get("username")?;
                let password: String = auth_table.get("password")?;
                req_options.auth = Some(AuthOption::Basic { username, password });
            } else if let Ok(token) = opts.get::<_, String>("authToken") {
                req_options.auth = Some(AuthOption::Bearer(token));
            }
            
            let response = client.get_with_options(&url, req_options)
                .map_err(|e| mlua::Error::external(e))?;
            create_response_table(lua, response)
        } else {
            // Backward compatible: no options
            let response = client.get(&url)
                .map_err(|e| mlua::Error::external(e))?;
            create_response_table(lua, response)
        }
    })?;
    table.set("get", get_fn)?;
    Ok(())
}
```

#### Step 6: Testing (3-4 hours)

**File:** `tests/http_advanced_features_test.rs` (NEW)

```rust
#[cfg(test)]
mod tests {
    use hype_rs::modules::builtins::http::HttpClient;
    use hype_rs::modules::builtins::http::auth::AuthOption;
    
    #[test]
    #[ignore] // Network test
    fn test_basic_auth() {
        let client = HttpClient::new().unwrap();
        let auth = AuthOption::Basic {
            username: "user".to_string(),
            password: "pass".to_string(),
        };
        
        let res = client.get_with_auth("https://httpbin.org/basic-auth/user/pass", auth);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().status, 200);
    }
    
    #[test]
    #[ignore] // Network test
    fn test_bearer_token() {
        let client = HttpClient::new().unwrap();
        let auth = AuthOption::Bearer("test_token".to_string());
        
        let res = client.get_with_auth("https://httpbin.org/bearer", auth);
        assert!(res.is_ok());
    }
    
    #[test]
    fn test_form_encoding() {
        let mut fields = HashMap::new();
        fields.insert("field1".to_string(), "value1".to_string());
        fields.insert("field2".to_string(), "value with spaces".to_string());
        
        let encoded = forms::encode_form_urlencoded(fields).unwrap();
        assert!(encoded.contains("field1=value1"));
        assert!(encoded.contains("field2=value"));
    }
}
```

**File:** `tests/lua_scripts/test_http_advanced.lua` (NEW)

```lua
local http = require("http")

print("=== Testing HTTP Advanced Features ===\n")

-- Test 1: Form submission
print("1. Testing postForm()...")
local res1 = http.postForm("https://httpbin.org/post", {
    username = "alice",
    password = "secret123"
})
if res1.status == 200 and res1.body:find("alice") then
    print("✓ Form submission works")
else
    print("✗ Form submission failed")
    os.exit(1)
end

-- Test 2: File upload
print("\n2. Testing uploadFile()...")
local file_content = "This is a test file content"
local res2 = http.uploadFile("https://httpbin.org/post", {
    file = {
        filename = "test.txt",
        content = file_content,
        contentType = "text/plain"
    },
    metadata = "Test upload"
})
if res2.status == 200 and res2.body:find("test.txt") then
    print("✓ File upload works")
else
    print("✗ File upload failed")
    os.exit(1)
end

-- Test 3: Basic Auth
print("\n3. Testing Basic Auth...")
local res3 = http.get("https://httpbin.org/basic-auth/user/pass", {
    auth = {
        username = "user",
        password = "pass"
    }
})
if res3.status == 200 then
    print("✓ Basic Auth works")
else
    print("✗ Basic Auth failed: status=" .. res3.status)
    os.exit(1)
end

-- Test 4: Bearer Token
print("\n4. Testing Bearer Token...")
local res4 = http.get("https://httpbin.org/bearer", {
    authToken = "test_token_123"
})
if res4.status == 200 and res4.body:find("test_token_123") then
    print("✓ Bearer token works")
else
    print("✗ Bearer token failed")
    os.exit(1)
end

-- Test 5: Proxy (skip if not available)
print("\n5. Testing Proxy...")
local ok, res5 = pcall(http.get, "https://httpbin.org/get", {
    proxy = "http://localhost:8888"
})
if ok then
    print("✓ Proxy configuration accepted")
else
    print("? Proxy test skipped (no local proxy)")
end

print("\n=== All Tests Passed ===")
```

#### Step 7: Documentation (1-2 hours)

**File:** `CHANGELOG.md`

```markdown
## [0.2.0] - 2025-10-28

### Added
- HTTP module now supports proxy configuration (PRP-012)
  - HTTP and HTTPS proxy via `{proxy = "http://proxy:8080"}`
  - Works with all HTTP methods
  - Simple string URL format for ease of use
  
- Form data helpers for easier form submission (PRP-012)
  - `http.postForm(url, fields)` for URL-encoded forms
  - `http.uploadFile(url, options)` for file uploads with multipart/form-data
  - Automatic Content-Type header setting
  - Automatic encoding and boundary management
  
- Authentication helpers (PRP-012)
  - Basic Auth: `{auth = {username = "u", password = "p"}}`
  - Bearer Token: `{authToken = "token"}`
  - Automatic Base64 encoding for Basic Auth
  - Automatic Authorization header construction

### Changed
- All HTTP methods now accept optional options table for proxy and auth
- Added `base64` and `serde_urlencoded` dependencies
- Enabled `multipart` feature in reqwest dependency

### Technical Details
- Base64 encoding using standard `base64` crate (RFC 4648 compliant)
- Form encoding using `serde_urlencoded` (RFC 1738 compliant)
- Multipart forms using reqwest's built-in multipart support (RFC 7578 compliant)
```

### 7.2 Phase 2: Testing and Refinement (2-3 hours)

1. Run all tests (unit + integration)
2. Test with real proxies (corporate network if available)
3. Test with various auth APIs
4. Verify backward compatibility
5. Performance benchmarks
6. Security review (no credential logging)

---

## 8. Success Criteria

### 8.1 Functional Criteria

✅ **Must Have:**
1. HTTP/HTTPS proxy configuration works
2. Proxy can be set per-request via options
3. `postForm()` correctly encodes form data
4. `uploadFile()` uploads files with multipart/form-data
5. Basic Auth helper encodes credentials correctly
6. Bearer token helper sets Authorization header
7. All existing HTTP tests pass (backward compatibility)
8. New features work with cookies (v0.1.4)

✅ **Should Have:**
9. Proxy works with all HTTP methods (GET, POST, PUT, DELETE, PATCH, HEAD, fetch)
10. Form encoding handles special characters correctly
11. File upload supports custom content types
12. Auth works with all HTTP methods

### 8.2 Non-Functional Criteria

✅ **Performance:**
- Proxy overhead < 5ms per request
- Form encoding < 10ms for typical forms (10-20 fields)
- Base64 encoding < 1ms

✅ **Security:**
- Proxy credentials NOT logged
- Auth credentials NOT logged
- HTTPS proxy validates certificates
- Base64 implementation is standard-compliant

✅ **Quality:**
- Unit test coverage > 80%
- Integration tests cover all new features
- No regressions in existing functionality
- Clear error messages for invalid input

✅ **Documentation:**
- CHANGELOG updated
- Examples in integration tests
- Clear API documentation

### 8.3 Acceptance Tests

**Test 1: Proxy configuration**
```lua
local http = require("http")
local res = http.get("https://httpbin.org/get", {
    proxy = "http://proxy.example.com:8080"
})
assert(res.status == 200)
```
**Expected:** Pass ✅ (or fail gracefully if proxy unavailable)

**Test 2: Form submission**
```lua
local res = http.postForm("https://httpbin.org/post", {
    username = "alice",
    password = "p@ss w0rd!",  -- Special chars
    remember = "true"
})
assert(res.status == 200)
assert(res.body:find("alice"))
assert(res.body:find("p%40ss"))  -- URL encoded
```
**Expected:** Pass ✅

**Test 3: File upload**
```lua
local res = http.uploadFile("https://httpbin.org/post", {
    file = {
        filename = "test.txt",
        content = "Hello World",
        contentType = "text/plain"
    }
})
assert(res.status == 200)
assert(res.body:find("test.txt"))
```
**Expected:** Pass ✅

**Test 4: Basic Auth**
```lua
local res = http.get("https://httpbin.org/basic-auth/user/pass", {
    auth = {username = "user", password = "pass"}
})
assert(res.status == 200)
```
**Expected:** Pass ✅

**Test 5: Bearer Token**
```lua
local res = http.get("https://httpbin.org/bearer", {
    authToken = "abc123"
})
assert(res.status == 200)
```
**Expected:** Pass ✅

**Test 6: Backward Compatibility**
```lua
-- Old code without options should still work
local res = http.get("https://httpbin.org/get")
assert(res.status == 200)
```
**Expected:** Pass ✅

---

## 9. Risk Assessment

### 9.1 Technical Risks

| Risk | Severity | Mitigation |
|------|----------|------------|
| Proxy authentication format varies | 🟡 Medium | Use standard URL format: `http://user:pass@proxy:8080` |
| Form encoding edge cases | 🟡 Medium | Use battle-tested `serde_urlencoded` crate |
| Multipart boundary conflicts | 🟢 Low | Reqwest handles boundary generation |
| Base64 implementation bugs | 🟢 Low | Use standard `base64` crate (widely used) |
| Breaking backward compatibility | 🔴 High | All new features opt-in via options parameter |
| Proxy SSL certificate issues | 🟡 Medium | Use reqwest's cert validation (same as existing) |

### 9.2 User Impact Risks

| Risk | Severity | Mitigation |
|------|----------|------------|
| Users confused by new options | 🟢 Low | Keep options simple, provide clear examples |
| Proxy misconfiguration | 🟡 Medium | Clear error messages for invalid proxy URLs |
| Credentials logged accidentally | 🔴 High | Never log proxy/auth credentials, add tests |
| Special chars in forms break | 🟡 Medium | Comprehensive test suite for edge cases |

### 9.3 Security Risks

| Risk | Severity | Mitigation |
|------|----------|------------|
| Proxy credentials in logs | 🔴 HIGH | Filter all logging of proxy URLs and auth |
| Auth headers logged | 🔴 HIGH | Never log Authorization header values |
| Insecure proxy (HTTP) leaks data | 🟡 Medium | Document that HTTPS proxy recommended |
| Base64 timing attacks | 🟢 Low | Use constant-time comparison (base64 crate handles) |

### 9.4 Mitigation Plan

1. **Credential Protection**
   - Add test to verify credentials never appear in logs
   - Sanitize all error messages containing URLs
   - Use `[REDACTED]` for proxy auth in debug output

2. **Thorough Testing**
   - Test with httpbin.org for auth validation
   - Test form encoding with Unicode, spaces, special chars
   - Test file uploads with various content types

3. **Clear Documentation**
   - Provide examples for each new feature
   - Document security considerations
   - Warn about HTTP vs HTTPS proxy implications

4. **Error Handling**
   - Validate proxy URL format before passing to reqwest
   - Validate auth options have required fields
   - Clear error messages for common mistakes

---

## 10. Future Enhancements

### 10.1 Phase 2 (v0.3.0)

**Advanced Proxy Features**
```lua
http.get(url, {
    proxy = {
        url = "http://proxy:8080",
        auth = {username = "user", password = "pass"},
        bypass = {"localhost", "*.internal.com"},
        type = "socks5"  -- Support SOCKS
    }
})
```

**Multiple File Upload**
```lua
http.uploadFiles(url, {
    files = {
        {filename = "doc1.pdf", content = data1},
        {filename = "doc2.pdf", content = data2}
    },
    metadata = "Batch upload"
})
```

**Custom Auth Schemes**
```lua
http.get(url, {
    auth = {
        type = "custom",
        scheme = "AWS4-HMAC-SHA256",
        value = computed_signature
    }
})
```

### 10.2 Phase 3 (v0.4.0)

**Per-Client Configuration**
```lua
local client = http.newClient({
    proxy = "http://proxy:8080",
    auth = {type = "bearer", token = "global_token"},
    timeout = 60000
})

client.get(url)  -- Uses client defaults
client.get(url, {proxy = nil})  -- Override: no proxy
```

**Environment Variable Support**
```lua
-- Automatically read HTTP_PROXY, HTTPS_PROXY, NO_PROXY
http.get(url, {proxy = "env"})
```

### 10.3 Long-term (v1.0+)

- OAuth 2.0 helpers
- Digest authentication
- NTLM authentication (Windows)
- Form validation helpers
- Progress callbacks for uploads

---

## Appendices

### Appendix A: API Reference

#### Proxy Configuration

```lua
-- Simple proxy
http.get(url, {proxy = "http://proxy:8080"})

-- Proxy with auth (URL format)
http.get(url, {proxy = "http://user:pass@proxy:8080"})

-- HTTPS proxy
http.get(url, {proxy = "https://secure-proxy:8443"})

-- Works with all methods
http.post(url, {body = data, proxy = "http://proxy:8080"})
http.fetch(url, {method = "PUT", proxy = "http://proxy:8080"})
```

#### Form Data

```lua
-- URL-encoded form
http.postForm(url, {
    field1 = "value1",
    field2 = "value with spaces",
    field3 = "special!@#$%chars"
})

-- File upload
http.uploadFile(url, {
    file = {
        filename = "document.pdf",
        content = file_data,        -- String or bytes
        contentType = "application/pdf"
    },
    description = "Q4 Report",      -- Additional fields
    category = "financial"
})
```

#### Authentication

```lua
-- Basic Auth
http.get(url, {
    auth = {
        username = "alice",
        password = "secret123"
    }
})

-- Bearer Token
http.get(url, {
    authToken = "eyJhbGciOiJIUzI1NiIs..."
})

-- Works with all methods
http.post(url, {
    body = data,
    auth = {username = "user", password = "pass"}
})
```

#### Combined Usage

```lua
-- Proxy + Auth + Form
http.postForm(url, {field1 = "value"}, {
    proxy = "http://proxy:8080",
    auth = {username = "user", password = "pass"}
})

-- All features together
http.uploadFile(url, {
    file = {filename = "doc.pdf", content = data}
}, {
    proxy = "http://proxy:8080",
    authToken = "bearer_token"
})
```

### Appendix B: Error Handling

```lua
-- Invalid proxy URL
local ok, err = pcall(http.get, url, {proxy = "invalid url"})
-- err: "Invalid proxy: invalid url format"

-- Missing auth fields
local ok, err = pcall(http.get, url, {auth = {username = "alice"}})
-- err: "Auth error: password required for basic auth"

-- File upload missing required field
local ok, err = pcall(http.uploadFile, url, {file = {content = "data"}})
-- err: "Upload error: filename required"
```

### Appendix C: Testing Checklist

- [ ] Proxy with HTTP URL
- [ ] Proxy with HTTPS URL
- [ ] Proxy with auth in URL (user:pass@host)
- [ ] Proxy with GET request
- [ ] Proxy with POST request
- [ ] Proxy with all HTTP methods
- [ ] Form with simple fields
- [ ] Form with special characters
- [ ] Form with Unicode characters
- [ ] File upload with text file
- [ ] File upload with binary file
- [ ] File upload with custom content type
- [ ] File upload with additional fields
- [ ] Basic Auth with valid credentials
- [ ] Basic Auth with invalid credentials
- [ ] Bearer token auth
- [ ] Auth with GET request
- [ ] Auth with POST request
- [ ] Backward compatibility (no options)
- [ ] Combined: proxy + auth
- [ ] Combined: proxy + form
- [ ] Combined: auth + form upload
- [ ] Error: invalid proxy URL
- [ ] Error: missing auth fields
- [ ] Error: missing file fields
- [ ] No credential logging test

---

**End of PRP-012**
