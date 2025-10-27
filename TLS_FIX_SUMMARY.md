# TLS Connection Issue Fix - Summary

## Problem Description

Users reported that `hype-rs` HTTP module failed to connect to certain HTTPS endpoints (specifically `push-1.forward.computer` and `push-5.forward.computer`) with error:

```
error sending request for url (https://push-5.forward.computer/)
Error: hyper_util::client::legacy::Error(Connect, Error { code: -9824, message: "handshake failure" })
```

Meanwhile, the same URLs worked fine with `curl` and other HTTP clients, and a different server (`dev-router.forward.computer`) worked correctly with hype-rs.

## Root Cause

The issue was caused by incompatible TLS cipher suite negotiation:

1. **hype-rs was using `native-tls`** (macOS Security framework)
2. **push-1 and push-5 servers** had limited TLS 1.2 cipher suite support
3. **macOS native TLS** couldn't negotiate a compatible TLS 1.2 cipher suite with these servers
4. The servers supported TLS 1.3 but native-tls wasn't using it properly

### Technical Details

TLS handshake testing revealed:
- `push-5`: TLS 1.2 negotiation **failed** (Cipher: NONE), TLS 1.3 worked
- `dev-router`: TLS 1.2 worked (ECDHE-ECDSA-AES256-GCM-SHA384), TLS 1.3 worked
- Error code -9824 = TLS handshake failure from macOS Security framework

## Solution

Switched from `native-tls` to `rustls` for TLS connections:

### Changes Made

**File: `Cargo.toml`**
```diff
-reqwest = { version = "0.12", features = ["json", "blocking"], optional = true }
+reqwest = { version = "0.12", features = ["json", "blocking", "rustls-tls"], default-features = false, optional = true }
```

**Result:**
- All servers now connect successfully (dev-router, push-1, push-5)
- Better TLS 1.3 support and cipher suite negotiation
- More predictable cross-platform behavior
- No changes to user-facing API

## Testing

### Before Fix
```lua
local http = require("http")
local res = http.get("https://push-5.forward.computer/")
-- Error: handshake failure (error code -9824)
```

### After Fix
```lua
local http = require("http")
local res = http.get("https://push-5.forward.computer/~hyperbuddy@1.0/metrics")
print("Status: " .. res.status)  -- 200
print("Size: " .. #res.body)     -- 56452534 bytes
-- ✓ Works!
```

### Verification

Tested all three servers:
- ✅ `https://dev-router.forward.computer/` - Success (200 OK)
- ✅ `https://push-1.forward.computer/` - Success (200 OK)  
- ✅ `https://push-5.forward.computer/` - Success (200 OK)

All HTTP tests pass:
```
cargo test http --lib
test result: ok. 19 passed; 0 failed
```

## Impact

**Benefits:**
- Fixes connection failures to servers with limited TLS 1.2 cipher suites
- Better TLS 1.3 support
- More consistent behavior across different platforms
- Improved security with modern TLS implementation

**Breaking Changes:**
- None - API remains identical

**Binary Size:**
- Increased slightly (~400KB) due to rustls
- Before: 3.6M (with native-tls)
- After: 4.0M (with rustls)
- Acceptable tradeoff for improved compatibility

## Version

This fix is included in **v0.1.3** of hype-rs.

## Related Files

- `Cargo.toml` - Updated reqwest dependency
- `CHANGELOG.md` - Documented the fix
- `src/modules/builtins/http/client.rs` - HTTP client (no changes needed)

## Conclusion

The TLS handshake failure was caused by incompatibility between macOS's native TLS implementation and specific server TLS configurations. Switching to rustls provides better TLS 1.3 support and more predictable cipher suite negotiation, fixing the connection issues without requiring any changes to user code.
