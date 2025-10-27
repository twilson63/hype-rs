# PRP-010: HTTP Module URL Encoding Bug Fix

**Status:** Proposed  
**Priority:** Medium  
**Created:** 2025-01-26  
**Author:** AI Assistant

## Project Overview

### Problem Statement

The HTTP module in hype-rs currently does not properly handle the tilde (`~`) character and other special characters in URL paths. While basic testing shows that tildes are passed through to `reqwest`, there's a potential issue with URL encoding standards and consistency with other special characters.

According to RFC 3986, the tilde character (`~`) is part of the "unreserved" character set and should NOT be percent-encoded in URLs. However, other special characters like spaces, `#`, `?`, `&`, etc., have specific meanings or encoding requirements.

### Current Behavior

Testing reveals:
- ✅ Tilde (`~`) - Currently works: `https://httpbin.org/anything/~test` → Server receives correctly
- ✅ Space (` `) - Currently works: `https://httpbin.org/anything/ test` → Server receives correctly  
- ⚠️ Hash (`#`) - Fails: Treated as URL fragment, request doesn't reach path
- ⚠️ Other special chars - Behavior untested/inconsistent

The root issue is that hype-rs passes URLs directly to `reqwest::Client` without any URL validation or encoding, relying entirely on reqwest's internal handling.

### Impact

**Severity:** Medium

**Affected Users:**
- Users making HTTP requests to URLs with special characters in paths
- Users working with legacy systems that use `~username` path conventions
- Users integrating with APIs that have special characters in endpoints

**Current Workaround:** 
- Manually percent-encode URLs in Lua before passing to http module
- Avoid using URLs with special characters

## Technical Requirements

### Functional Requirements

1. **FR-1:** HTTP module must correctly handle all RFC 3986 unreserved characters in URL paths without encoding:
   - Letters (A-Z, a-z)
   - Digits (0-9)
   - Hyphen (`-`), period (`.`), underscore (`_`), tilde (`~`)

2. **FR-2:** HTTP module must properly percent-encode reserved characters when used in path segments:
   - Space (` `) → `%20`
   - Hash (`#`) → `%23` (when in path, not fragment)
   - Other reserved chars per RFC 3986

3. **FR-3:** HTTP module must preserve properly encoded URLs (don't double-encode)

4. **FR-4:** HTTP module must provide clear error messages for invalid URLs

### Non-Functional Requirements

1. **NFR-1:** Performance impact should be minimal (< 1ms overhead per request)
2. **NFR-2:** Solution must maintain backward compatibility with existing valid URLs
3. **NFR-3:** Implementation should follow Rust HTTP client best practices
4. **NFR-4:** Code should include comprehensive tests for URL edge cases

### Dependencies

- `reqwest` v0.12 (already in use)
- Potential new dependency: `url` crate for proper URL parsing/encoding

## Proposed Solutions

### Solution 1: Use `url` Crate for Proper URL Parsing and Validation

**Description:**  
Add the `url` crate as a dependency and use it to parse and validate URLs before passing them to reqwest. The `url` crate is the de-facto standard for URL handling in Rust and provides RFC 3986-compliant parsing.

**Implementation:**
```rust
use url::Url;

pub fn get(&self, url: &str) -> Result<HttpResponse> {
    // Parse and validate URL
    let parsed_url = Url::parse(url)
        .map_err(|e| HttpError::RequestError(format!("Invalid URL: {}", e)))?;
    
    self.runtime.block_on(async {
        let response = self.client.get(parsed_url.as_str()).send().await?;
        HttpResponse::from_reqwest(response).await.map_err(Into::into)
    })
}
```

**Pros:**
- ✅ Industry-standard URL handling (used by reqwest internally)
- ✅ Automatic RFC 3986-compliant encoding
- ✅ Comprehensive URL validation with clear error messages
- ✅ Handles all edge cases (fragments, query params, special chars)
- ✅ No risk of double-encoding (already encoded URLs pass through)
- ✅ Minimal performance overhead (~10-50µs per URL parse)
- ✅ Provides additional features: path manipulation, query building, etc.

**Cons:**
- ❌ Adds external dependency (~100KB compiled size)
- ❌ Slight API change if we want to expose URL manipulation to Lua
- ❌ May change behavior for currently "working" but technically invalid URLs

**Complexity:** Low  
**Risk:** Low

---

### Solution 2: Manual URL Encoding with Custom Logic

**Description:**  
Implement custom URL encoding logic that identifies path segments and encodes only the necessary characters based on RFC 3986.

**Implementation:**
```rust
fn encode_url_path(url: &str) -> Result<String> {
    // Split URL into components
    let (base, path_and_query) = if let Some(pos) = url.find("://") {
        let protocol_end = pos + 3;
        let after_protocol = &url[protocol_end..];
        if let Some(slash_pos) = after_protocol.find('/') {
            let base = &url[..protocol_end + slash_pos];
            let rest = &url[protocol_end + slash_pos..];
            (base, rest)
        } else {
            return Ok(url.to_string());
        }
    } else {
        return Err(HttpError::RequestError("Invalid URL".to_string()));
    };

    // Encode only path portion, preserve query and fragment
    let encoded_path = path_and_query
        .chars()
        .map(|c| match c {
            ' ' => "%20".to_string(),
            '#' => "%23".to_string(),
            // Add other chars as needed
            c if c.is_ascii_alphanumeric() || "-._~".contains(c) => c.to_string(),
            c => format!("%{:02X}", c as u8),
        })
        .collect::<String>();

    Ok(format!("{}{}", base, encoded_path))
}
```

**Pros:**
- ✅ No external dependencies
- ✅ Full control over encoding behavior
- ✅ Can be tailored to hype-rs specific needs
- ✅ Potentially slightly faster than parsing (though negligible)

**Cons:**
- ❌ High complexity - reimplementing RFC 3986 is error-prone
- ❌ Must handle: fragments, query params, IPv6 addresses, international domains, etc.
- ❌ Risk of bugs and edge cases
- ❌ Maintenance burden (keeping up with RFC changes/clarifications)
- ❌ Likely incomplete compared to battle-tested libraries
- ❌ Doesn't handle URL validation (malformed URLs)

**Complexity:** High  
**Risk:** High

---

### Solution 3: Rely on Reqwest with Validation Layer

**Description:**  
Keep using reqwest as-is but add a lightweight validation layer to catch obviously invalid URLs before they reach reqwest. Document that users should follow RFC 3986 or use percent-encoding when needed.

**Implementation:**
```rust
fn validate_url(url: &str) -> Result<()> {
    // Basic checks
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(HttpError::RequestError("URL must start with http:// or https://".to_string()));
    }
    
    // Check for problematic unencoded characters
    if url.contains(" ") || url.contains("\t") || url.contains("\n") {
        return Err(HttpError::RequestError(
            "URL contains unencoded whitespace. Use %20 for spaces.".to_string()
        ));
    }
    
    Ok(())
}

pub fn get(&self, url: &str) -> Result<HttpResponse> {
    validate_url(url)?;
    self.runtime.block_on(async {
        let response = self.client.get(url).send().await?;
        HttpResponse::from_reqwest(response).await.map_err(Into::into)
    })
}
```

**Pros:**
- ✅ Minimal code changes
- ✅ No new dependencies
- ✅ Zero performance impact (basic string checks)
- ✅ Maintains current behavior for valid URLs
- ✅ Provides better error messages for common mistakes

**Cons:**
- ❌ Doesn't actually fix encoding issues
- ❌ Puts burden on Lua developers to encode URLs manually
- ❌ Still allows many invalid URL patterns through
- ❌ Inconsistent with expectations (most HTTP libraries auto-encode)
- ❌ Doesn't solve the stated problem - just documents it

**Complexity:** Very Low  
**Risk:** Low (minimal change)

---

## Recommended Solution

**Solution 1: Use `url` Crate** ✅

### Justification

1. **Industry Standard:** The `url` crate is the standard URL handling library in Rust, used by reqwest itself and virtually all HTTP-related crates
2. **Correctness:** Provides RFC 3986-compliant parsing and encoding, handling all edge cases correctly
3. **Maintainability:** Offloads complex URL logic to a well-tested, actively maintained library
4. **Error Handling:** Provides clear, actionable error messages for malformed URLs
5. **Future-Proof:** Enables future features like URL building, query manipulation, etc.
6. **Low Risk:** Battle-tested library with millions of downloads/month
7. **Minimal Cost:** ~100KB binary size increase is negligible, performance overhead is < 50µs per request

### Trade-offs Accepted

- **Binary size:** +~100KB (acceptable for the value provided)
- **Dependency:** One additional well-maintained, widely-used crate
- **Behavior change:** Some technically invalid but currently "working" URLs may now fail with clear errors (this is actually a positive - fail fast)

## Implementation Plan

### Phase 1: Add Dependency and Basic Integration (1-2 hours)

**Tasks:**
1. Add `url = "2.5"` to `Cargo.toml` dependencies
2. Update `HttpClient` methods to use `Url::parse()` before requests
3. Map URL parsing errors to `HttpError::RequestError` with descriptive messages
4. Update all HTTP methods: `get`, `post`, `put`, `delete`, `fetch`, etc.

**Files to Modify:**
- `Cargo.toml`
- `src/modules/builtins/http/client.rs`
- `src/modules/builtins/http/error.rs` (if new error variant needed)

**Example:**
```rust
use url::Url;

#[cfg(feature = "http")]
pub fn get(&self, url: &str) -> Result<HttpResponse> {
    let parsed_url = Url::parse(url)
        .map_err(|e| HttpError::RequestError(format!("Invalid URL '{}': {}", url, e)))?;
    
    self.runtime.block_on(async {
        let response = self.client.get(parsed_url.as_str()).send().await?;
        HttpResponse::from_reqwest(response).await.map_err(Into::into)
    })
}
```

### Phase 2: Comprehensive Testing (1-2 hours)

**Tasks:**
1. Create unit tests for URL edge cases:
   - Tilde in path: `https://example.com/~user/data`
   - Spaces: `https://example.com/path with spaces`
   - Hash in path: `https://example.com/path%23hash` (encoded)
   - Hash as fragment: `https://example.com/path#fragment` (valid)
   - Already encoded URLs: `https://example.com/path%20space` (should not double-encode)
   - Special chars: `& ? = + @ !`
   - International domains: `https://münchen.de/path`
   - IPv6: `http://[::1]:8080/path`
2. Create integration tests with real HTTP requests (using httpbin.org)
3. Add error case tests (malformed URLs)

**Files to Create/Modify:**
- `src/modules/builtins/http/client.rs` (add tests section)
- `tests/http_url_encoding_test.rs` (new integration test file)

**Test Cases:**
```rust
#[test]
fn test_url_with_tilde() {
    let client = HttpClient::new().unwrap();
    // Should not encode tilde
    let result = client.get("https://httpbin.org/anything/~user/data");
    assert!(result.is_ok());
}

#[test]
fn test_url_with_space() {
    let client = HttpClient::new().unwrap();
    // Should encode space as %20
    let result = client.get("https://httpbin.org/anything/path with space");
    assert!(result.is_ok());
}

#[test]
fn test_invalid_url() {
    let client = HttpClient::new().unwrap();
    let result = client.get("not a valid url");
    assert!(result.is_err());
    if let Err(HttpError::RequestError(msg)) = result {
        assert!(msg.contains("Invalid URL"));
    }
}

#[test]
fn test_already_encoded_url() {
    let client = HttpClient::new().unwrap();
    // Should not double-encode
    let result = client.get("https://httpbin.org/anything/path%20with%20encoded%20spaces");
    assert!(result.is_ok());
}
```

### Phase 3: Documentation and Examples (30 minutes)

**Tasks:**
1. Update agent documentation (`src/cli/agent/generator.rs`) to mention URL encoding behavior
2. Add examples to README showing special character handling
3. Update CHANGELOG.md with bug fix note
4. Add note to HTTP module docs about automatic URL encoding

**Documentation Updates:**
```markdown
## URL Encoding

The HTTP module automatically handles URL encoding according to RFC 3986:

- **Unreserved characters** (a-z, A-Z, 0-9, `-`, `.`, `_`, `~`) are never encoded
- **Reserved characters** (` `, `#`, `?`, `&`, etc.) are automatically percent-encoded when in path segments
- **Already-encoded URLs** are preserved without double-encoding

Examples:
```lua
local http = require("http")

-- Tilde is preserved
http.get("https://example.com/~username/profile")

-- Space is automatically encoded to %20
http.get("https://example.com/path with spaces")

-- Already encoded URLs work fine
http.get("https://example.com/path%20already%20encoded")
```
```

### Phase 4: Validation and Release (30 minutes)

**Tasks:**
1. Run full test suite: `cargo test`
2. Run benchmarks to verify performance: `cargo bench`
3. Build release binary: `cargo build --release`
4. Manual testing with various URL patterns
5. Update version in `Cargo.toml` (patch version bump)
6. Commit changes with descriptive message

**Commit Message:**
```
fix(http): Add proper URL encoding using url crate

- Add `url` crate dependency for RFC 3986-compliant URL handling
- Fix tilde (~) and other special characters in URL paths
- Prevent invalid URLs with clear error messages
- Add comprehensive tests for URL edge cases
- Update documentation with URL encoding examples

Fixes: PRP-010
```

## Success Criteria

### Acceptance Criteria

1. ✅ All RFC 3986 unreserved characters work in URL paths without encoding
   - Test: `https://example.com/~user-name_file.txt` succeeds
   
2. ✅ Reserved characters are properly encoded in path segments
   - Test: `https://example.com/path with spaces` → `https://example.com/path%20with%20spaces`
   
3. ✅ Already-encoded URLs are not double-encoded
   - Test: `https://example.com/path%20space` remains `https://example.com/path%20space`
   
4. ✅ Invalid URLs produce clear error messages
   - Test: `not a url` → Error with message "Invalid URL 'not a url': relative URL without a base"
   
5. ✅ URL fragments and query parameters work correctly
   - Test: `https://example.com/path?query=1#fragment` succeeds
   
6. ✅ All existing tests continue to pass (backward compatibility)

### Performance Benchmarks

- URL parsing overhead: < 50 microseconds per request
- Memory overhead: < 1KB per request
- Binary size increase: < 200KB

### Test Coverage

- Unit tests: 100% coverage of new URL parsing code
- Integration tests: Minimum 10 test cases covering edge cases
- All tests pass on all supported platforms (macOS, Linux)

## Risks and Mitigation

### Risk 1: Breaking Changes
**Probability:** Low  
**Impact:** Medium  
**Mitigation:** The `url` crate is very permissive and handles most reasonable URL inputs. Users with truly invalid URLs will get clear errors instead of silent failures.

### Risk 2: Performance Regression
**Probability:** Very Low  
**Impact:** Low  
**Mitigation:** URL parsing is extremely fast (~10-50µs). Network latency (10-500ms) dwarfs any parsing overhead by 3-4 orders of magnitude.

### Risk 3: Dependency Bloat
**Probability:** Low  
**Impact:** Very Low  
**Mitigation:** The `url` crate is already an indirect dependency through reqwest. Adding it as a direct dependency has minimal impact (~100KB).

## Future Enhancements

1. **URL Builder API:** Expose URL building functionality to Lua for safer URL construction
   ```lua
   local http = require("http")
   local url = http.url("https://api.example.com")
     :path("/users")
     :query("limit", 10)
     :query("offset", 0)
     :build()  -- "https://api.example.com/users?limit=10&offset=0"
   ```

2. **URL Validation Function:** Add `http.validate_url(url)` for pre-flight checking

3. **Base URL Support:** Allow setting a base URL for relative paths
   ```lua
   local http = require("http")
   http.set_base_url("https://api.example.com")
   http.get("/users")  -- Resolves to https://api.example.com/users
   ```

## References

- [RFC 3986: Uniform Resource Identifier (URI): Generic Syntax](https://datatracker.ietf.org/doc/html/rfc3986)
- [`url` crate documentation](https://docs.rs/url/latest/url/)
- [`reqwest` URL handling](https://docs.rs/reqwest/latest/reqwest/#making-a-get-request)
- [Percent-encoding (Wikipedia)](https://en.wikipedia.org/wiki/Percent-encoding)

## Appendix: URL Encoding Reference

### Characters that MUST NOT be encoded in path segments (RFC 3986 unreserved):
- `A-Z a-z 0-9`
- `-` (hyphen)
- `.` (period)
- `_` (underscore)  
- `~` (tilde) ← **The bug reported**

### Characters that MUST be encoded in path segments:
- ` ` (space) → `%20`
- `#` (hash) → `%23` (when in path, not fragment)
- `?` (question) → `%3F` (when in path, not query)
- `%` (percent) → `%25` (when not part of existing encoding)
- And other control/special characters

### Characters with special meaning (context-dependent):
- `#` - Fragment identifier (after path/query)
- `?` - Query string separator
- `&` - Query parameter separator
- `=` - Query parameter assignment
- `/` - Path separator
