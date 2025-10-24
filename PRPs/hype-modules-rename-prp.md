# Project Request Protocol: Rename node_modules to hype_modules

**Project Name**: Hype-RS Module Directory Rename  
**Project ID**: HYPE-REFACTOR-002  
**Priority**: ⭐⭐⭐⭐ HIGH  
**Estimated Duration**: 3-5 days  
**Target Completion**: Sprint 8  

---

## 1. Project Overview

### 1.1 Executive Summary

Rename the module resolution directory from `node_modules` to `hype_modules` throughout the Hype-RS codebase to establish a distinct identity for the Hype-RS ecosystem. This change differentiates Hype-RS from Node.js while maintaining familiarity with module resolution patterns. The rename affects the module resolver, documentation, tests, and examples.

### 1.2 Business Case

| Aspect | Current State | After Implementation |
|--------|---------------|----------------------|
| Brand Identity | Uses Node.js convention `node_modules` | Distinct Hype-RS identity with `hype_modules` |
| Ecosystem Clarity | Confusion with npm packages | Clear separation from npm ecosystem |
| Community Perception | "Another Node.js clone" | Independent Lua runtime with own conventions |
| Package Manager Integration | Ambiguous compatibility | Clear that Hype-RS has its own package system |
| Search Engine Optimization | Conflicts with npm results | Unique searchable terms for Hype-RS |

### 1.3 Motivation

**Why rename?**
1. **Brand Differentiation**: Establish Hype-RS as a distinct ecosystem, not a Node.js alternative
2. **Prevent Confusion**: Users might expect npm compatibility if they see `node_modules`
3. **Ecosystem Independence**: Signal that Hype-RS has its own package management approach
4. **Consistency**: Align directory naming with project name (hype-rs → hype_modules)
5. **Future-Proofing**: Avoid legal/trademark concerns with Node.js branding

**Why use underscore instead of hyphen?**
- `hype_modules` is more filesystem-friendly (some shells treat hyphens specially)
- Matches Rust naming convention (snake_case)
- Easier to type and autocomplete
- Consistent with hidden directories like `.hype`

### 1.4 Success Metrics

- ✅ All module resolution uses `hype_modules` directory
- ✅ Zero references to `node_modules` in production code
- ✅ All tests pass with new directory structure
- ✅ Documentation updated with new conventions
- ✅ Examples demonstrate `hype_modules` usage
- ✅ Backward compatibility maintained (optional)
- ✅ Migration guide provided for existing users

### 1.5 Scope

**In Scope**:
- Update `ModuleResolver` to search `hype_modules` instead of `node_modules`
- Update all test fixtures and test code
- Update documentation (README, docs/, PRPs)
- Update example projects
- Add migration notes to CHANGELOG
- Update error messages and logging

**Out of Scope**:
- Package manager implementation (future work)
- Automatic migration tool for existing projects (optional nice-to-have)
- Support for both `node_modules` and `hype_modules` simultaneously (decided per solution)
- Changes to `~/.hype/modules` (already uses hype branding)

---

## 2. Technical Requirements

### 2.1 Functional Requirements

#### F1: Module Resolution
```
Given: A module identifier "my-module"
When: require("my-module") is called
Then: Search in ./hype_modules/my-module/
And: Search in ../hype_modules/my-module/ (walk up)
And: Search in ~/.hype/modules/my-module/ (unchanged)
And: Fallback to built-in modules
```

#### F2: Resolution Priority Order
```
1. Built-in modules (fs, path, events, util, table)
2. hype_modules directories (walk up from current directory)
   - ./hype_modules/module_id
   - ../hype_modules/module_id
   - ../../hype_modules/module_id
   - ... (continue to root)
3. Home directory modules (~/.hype/modules/module_id)
4. Return ModuleNotFound error
```

#### F3: Path Handling
```
- Support cross-platform paths (Unix and Windows)
- Handle spaces in directory names
- Support nested module directories
- Resolve symlinks appropriately
```

#### F4: Error Messages
```
Old: "Module not found in node_modules"
New: "Module not found in hype_modules"

Old: "Searching node_modules directories..."
New: "Searching hype_modules directories..."
```

### 2.2 Non-Functional Requirements

#### NF1: Performance
- No performance degradation in module resolution
- Cache behavior unchanged
- Resolution time < 1ms for cached modules
- Resolution time < 50ms for uncached modules

#### NF2: Compatibility
- Examples must work out of the box
- Existing test fixtures continue to work
- Clear upgrade path for existing users

#### NF3: Documentation
- All documentation reflects new directory name
- Migration guide for existing projects
- Clear rationale in changelog

#### NF4: Code Quality
- No breaking changes to public APIs
- All existing tests pass
- Code coverage maintained at 97%+

---

## 3. Proposed Solutions

### Solution 1: Direct Rename (Recommended)

**Description**: Replace all occurrences of `node_modules` with `hype_modules` throughout the codebase. No backward compatibility.

**Implementation**:
```rust
// src/modules/resolver.rs
// OLD:
let candidate = search_path.join("node_modules").join(module_id);

// NEW:
let candidate = search_path.join("hype_modules").join(module_id);
```

**Changes Required**:
- `src/modules/resolver.rs`: Update all `join("node_modules")` calls
- `docs/modules/require-api.md`: Update documentation
- `README.md`: Update examples
- All test files: Update directory creation and assertions
- `examples/`: Rename directories in example projects

**Pros**:
✅ Clean, simple implementation  
✅ No code complexity added  
✅ Clear direction for ecosystem  
✅ No maintenance burden of dual support  
✅ Forces ecosystem to standardize early  
✅ Fast implementation (1-2 days)  
✅ Zero performance impact  

**Cons**:
❌ Breaking change for early adopters (if any)  
❌ Requires communication and migration guide  
❌ No grace period for transition  

**Risk Assessment**: **LOW**
- Project is pre-1.0, breaking changes are acceptable
- Small user base (if any) can be notified directly
- Clear migration path (just rename directory)

---

### Solution 2: Support Both Directories with Deprecation Warning

**Description**: Check both `hype_modules` and `node_modules`, preferring `hype_modules`. Log deprecation warning when `node_modules` is used.

**Implementation**:
```rust
// src/modules/resolver.rs
pub fn resolve(&self, module_id: &str) -> Result<PathBuf, HypeError> {
    if self.is_builtin(module_id) {
        return Ok(self.get_builtin_path(module_id));
    }

    for search_path in &self.search_paths {
        // Check hype_modules first (preferred)
        let candidate = search_path.join("hype_modules").join(module_id);
        if candidate.exists() {
            return Ok(candidate);
        }
        
        // Check node_modules (deprecated)
        let legacy_candidate = search_path.join("node_modules").join(module_id);
        if legacy_candidate.exists() {
            eprintln!("WARNING: Using deprecated 'node_modules' directory. Please rename to 'hype_modules'.");
            return Ok(legacy_candidate);
        }
    }

    // ... rest of resolution logic
}
```

**Changes Required**:
- `src/modules/resolver.rs`: Add dual-check logic with deprecation warning
- Update documentation to recommend `hype_modules`
- Add deprecation notice to CHANGELOG
- Update examples to use `hype_modules`
- Keep test coverage for both directory names

**Pros**:
✅ Backward compatible  
✅ Smooth transition for existing users  
✅ Gradual migration path  
✅ Users can migrate on their schedule  
✅ Clear deprecation signal  

**Cons**:
❌ Added code complexity  
❌ Two code paths to maintain  
❌ Potential confusion about which to use  
❌ Small performance penalty (checks two directories)  
❌ Technical debt accumulates  
❌ Need to plan deprecation removal timeline  
❌ Longer implementation (2-3 days)  

**Risk Assessment**: **MEDIUM**
- Adds maintenance burden
- Must plan future removal of `node_modules` support
- Potential for users to stay on deprecated path

---

### Solution 3: Configuration-Based Directory Name

**Description**: Allow users to configure the module directory name via `hype.json` or environment variable. Default to `hype_modules`.

**Implementation**:
```rust
// src/modules/resolver.rs
pub struct ModuleResolver {
    root_dir: PathBuf,
    search_paths: Vec<PathBuf>,
    module_dir_name: String, // New field
}

impl ModuleResolver {
    pub fn new(root_dir: PathBuf) -> Self {
        let module_dir_name = env::var("HYPE_MODULE_DIR")
            .unwrap_or_else(|_| "hype_modules".to_string());
        
        // ... initialization
    }
    
    pub fn resolve(&self, module_id: &str) -> Result<PathBuf, HypeError> {
        // ...
        let candidate = search_path
            .join(&self.module_dir_name)
            .join(module_id);
        // ...
    }
}
```

**Changes Required**:
- `src/modules/resolver.rs`: Add `module_dir_name` field and configuration loading
- `src/config/mod.rs`: Add configuration option
- Documentation for configuration option
- Examples use default `hype_modules`
- Tests for both default and custom names

**Pros**:
✅ Ultimate flexibility for users  
✅ Can support migration scenarios  
✅ Power users can customize  
✅ Interoperability with existing ecosystems (if desired)  

**Cons**:
❌ High complexity added  
❌ Fragments ecosystem (everyone uses different names)  
❌ Harder to document and support  
❌ Configuration is additional surface area for bugs  
❌ Not recommended for package ecosystems (needs standardization)  
❌ Longest implementation time (3-4 days)  
❌ Against principle of convention over configuration  

**Risk Assessment**: **HIGH**
- Ecosystem fragmentation
- Increased support burden
- Goes against goal of establishing conventions
- Over-engineering for the use case

---

## 4. Solution Comparison Matrix

| Criteria | Solution 1: Direct Rename | Solution 2: Dual Support | Solution 3: Configurable |
|----------|--------------------------|-------------------------|-------------------------|
| **Implementation Complexity** | ⭐⭐⭐⭐⭐ Lowest | ⭐⭐⭐ Medium | ⭐ Highest |
| **Code Maintainability** | ⭐⭐⭐⭐⭐ Clean | ⭐⭐⭐ Two paths | ⭐⭐ Complex |
| **Performance** | ⭐⭐⭐⭐⭐ No impact | ⭐⭐⭐⭐ Slight overhead | ⭐⭐⭐⭐ Config overhead |
| **Backward Compatibility** | ⭐ Breaking | ⭐⭐⭐⭐⭐ Full | ⭐⭐⭐⭐ Optional |
| **Ecosystem Clarity** | ⭐⭐⭐⭐⭐ Clear standard | ⭐⭐⭐ Transitional | ⭐ Fragmented |
| **Documentation Burden** | ⭐⭐⭐⭐⭐ Simple | ⭐⭐⭐ Dual examples | ⭐⭐ Complex |
| **Long-term Viability** | ⭐⭐⭐⭐⭐ Sustainable | ⭐⭐⭐ Tech debt | ⭐⭐ Over-engineered |
| **User Experience** | ⭐⭐⭐⭐ Clear | ⭐⭐⭐⭐ Smooth migration | ⭐⭐⭐ Flexible but confusing |
| **Implementation Time** | 1-2 days | 2-3 days | 3-4 days |
| **Risk Level** | LOW | MEDIUM | HIGH |

**Scoring (out of 5 stars):**
- **Solution 1**: 37/40 stars (92.5%)
- **Solution 2**: 28/40 stars (70%)
- **Solution 3**: 24/40 stars (60%)

---

## 5. Recommended Solution: Solution 1 (Direct Rename)

### 5.1 Rationale

**Solution 1: Direct Rename** is the clear winner because:

1. **Project Maturity**: Hype-RS is pre-1.0, making this the ideal time for breaking changes
2. **Ecosystem Health**: Establishing clear conventions early prevents fragmentation
3. **Simplicity**: Clean code is easier to maintain, debug, and extend
4. **Performance**: Zero overhead compared to conditional checking
5. **User Clarity**: One way to do things = less confusion
6. **Maintenance**: No technical debt from supporting deprecated features

### 5.2 Why Not Solution 2 or 3?

**Solution 2** (Dual Support) seems appealing for backward compatibility, but:
- Pre-1.0 projects should embrace breaking changes
- Technical debt accumulates quickly
- Users need to eventually migrate anyway
- The warning noise pollutes logs

**Solution 3** (Configurable) is over-engineering:
- Package ecosystems need standardization, not flexibility
- Configuration complexity isn't worth the cost
- Fragments the community
- Makes documentation and support harder

### 5.3 Risk Mitigation

**Risk**: Early adopters need to migrate  
**Mitigation**:
- Clear migration guide in CHANGELOG
- Document in upgrade notes
- Simple migration (just rename directory)
- Communication via release notes

**Risk**: Confusion about the change  
**Mitigation**:
- Explain rationale in documentation
- Prominent note in README
- Add to FAQ

---

## 6. Implementation Plan

### 6.1 Phase 1: Core Module Resolver (Day 1)

**File**: `src/modules/resolver.rs`

```rust
// Line 14: Update comment
- /// 2. node_modules directories (walk up from current directory)
+ /// 2. hype_modules directories (walk up from current directory)

// Line 52: Update comment  
- /// 2. Walk up directory tree looking for node_modules/module_id
+ /// 2. Walk up directory tree looking for hype_modules/module_id

// Lines 74, 113: Update path join
- let candidate = search_path.join("node_modules").join(module_id);
+ let candidate = search_path.join("hype_modules").join(module_id);

- let candidate = current.join("node_modules").join(module_id);
+ let candidate = current.join("hype_modules").join(module_id);

// Line 177: Update comment
- /// Search paths are checked before the standard node_modules walk.
+ /// Search paths are checked before the standard hype_modules walk.
```

**Tests to Update**: `tests` module in `src/modules/resolver.rs`
- `test_resolve_node_modules` → `test_resolve_hype_modules`
- Update directory creation in test fixtures

### 6.2 Phase 2: Unit Tests (Day 1-2)

**Files**:
- `tests/advanced_unit_tests.rs` (18 occurrences)
- `tests/edge_cases_test.rs` (21 occurrences)
- `tests/stress_test.rs` (5 occurrences)
- `tests/module_system_integration_test.rs` (if exists)
- `benches/module_benchmarks.rs` (3 occurrences)

**Changes**:
```rust
// OLD:
let node_modules = temp_path.join("node_modules");
fs::create_dir_all(&node_modules).unwrap();

// NEW:
let hype_modules = temp_path.join("hype_modules");
fs::create_dir_all(&hype_modules).unwrap();
```

### 6.3 Phase 3: Documentation (Day 2)

**Files to Update**:

1. **`docs/modules/require-api.md`**:
   - Line 66-69: Update resolution order description
   - All examples showing `node_modules` → `hype_modules`
   
2. **`README.md`**:
   - Module resolution section
   - Any examples or diagrams
   
3. **`docs/modules/getting-started.md`**:
   - Installation examples
   - Module structure examples
   
4. **`docs/modules/builtin-modules.md`**:
   - Update any references to directory structure

5. **`PRPs/module-system-prp.md`**:
   - Update technical specifications (line 76)

### 6.4 Phase 4: Examples (Day 2-3)

**Files**:
- `examples/package-app/`: Rename any `node_modules` directories
- Update example instructions in README files
- Update `examples/module-*.lua` if they reference directory structure

### 6.5 Phase 5: Error Messages & Logging (Day 3)

Search for user-facing strings mentioning `node_modules`:
```bash
rg "node_modules" --type rust -g '!tests' -g '!benches'
```

Update error messages in:
- `src/error/mod.rs` (if any)
- `src/modules/error.rs`
- Any logging statements

### 6.6 Phase 6: Validation & Testing (Day 3)

1. Run full test suite: `cargo test`
2. Run benchmarks: `cargo bench`
3. Run clippy: `cargo clippy -- -D warnings`
4. Run examples manually
5. Check documentation renders correctly
6. Verify error messages are clear

### 6.7 Phase 7: Documentation & Communication (Day 3-4)

1. **CHANGELOG.md**: Add breaking change notice
   ```markdown
   ## [Unreleased]
   
   ### BREAKING CHANGES
   
   - Module directory renamed from `node_modules` to `hype_modules`
     - Establishes Hype-RS as distinct ecosystem
     - Migration: Rename your `node_modules/` directory to `hype_modules/`
     - Rationale: See PRPs/hype-modules-rename-prp.md
   ```

2. **Migration Guide** (add to `docs/`):
   ```markdown
   # Migrating to hype_modules
   
   ## Quick Migration
   
   If you have an existing Hype-RS project:
   
   ```bash
   mv node_modules hype_modules
   ```
   
   That's it! Your project will now use the new directory structure.
   ```

3. **Update README.md** with prominent notice

### 6.8 Phase 8: Final Review (Day 4-5)

- Code review
- QA testing
- Documentation review
- Prepare release notes

---

## 7. Testing Strategy

### 7.1 Unit Tests

**Existing Tests** (verify they pass):
- `test_resolve_hype_modules` (renamed from `test_resolve_node_modules`)
- `test_resolve_walking_up_directories`
- All resolver tests in `src/modules/resolver.rs`

**New Tests** (if needed):
- Test that `node_modules` is NOT searched (negative test)
- Test error messages contain "hype_modules"

### 7.2 Integration Tests

**Files**: `tests/module_system_integration_test.rs`
- Test full module loading with `hype_modules`
- Test built-in modules still work
- Test module caching
- Test circular dependency detection

### 7.3 End-to-End Tests

**Manual Testing**:
1. Create fresh project with `hype_modules/`
2. Run `require()` from Lua scripts
3. Verify error messages
4. Test examples work

### 7.4 Regression Tests

- All existing tests must pass
- No performance degradation
- Code coverage remains at 97%+

---

## 8. Success Criteria

### 8.1 Functional Criteria

✅ Module resolution searches `hype_modules` directories  
✅ All built-in modules load correctly  
✅ Module caching works as expected  
✅ Circular dependency detection functions properly  
✅ Error messages reference `hype_modules`  
✅ Examples work with new directory structure  

### 8.2 Quality Criteria

✅ All tests pass (265 tests)  
✅ Code coverage ≥ 97%  
✅ Zero clippy warnings  
✅ Documentation is accurate and complete  
✅ Migration guide is clear  
✅ CHANGELOG updated  

### 8.3 Performance Criteria

✅ No performance regression (< 5% variance)  
✅ Benchmark results within acceptable range  
✅ Module resolution time unchanged  

---

## 9. Rollout Plan

### 9.1 Pre-Release

1. Implement changes on feature branch
2. Full test suite passes
3. Internal review and QA
4. Update documentation

### 9.2 Release

1. Merge to main
2. Tag release (breaking change version bump)
3. Publish release notes with migration guide
4. Update website/documentation (if exists)
5. Announce on community channels (if any)

### 9.3 Post-Release

1. Monitor for issues
2. Respond to migration questions
3. Update FAQ with common questions
4. Consider blog post explaining rationale

---

## 10. Risks & Mitigation

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| **Users miss the migration notice** | HIGH | MEDIUM | Prominent CHANGELOG entry, README notice, clear error messages if old directory exists |
| **Confusion about rationale** | MEDIUM | MEDIUM | Clear documentation explaining "why", FAQ entry |
| **Performance regression** | HIGH | LOW | Benchmark testing, performance validation |
| **Tests fail** | HIGH | LOW | Comprehensive testing before merge, CI/CD validation |
| **Incomplete rename** | MEDIUM | MEDIUM | grep/rg search for all occurrences, code review |

---

## 11. Dependencies & Blockers

### 11.1 Dependencies

- None (no external dependencies)

### 11.2 Blockers

- None identified

---

## 12. Timeline

| Phase | Duration | Start | End |
|-------|----------|-------|-----|
| Phase 1: Core Resolver | 0.5 days | Day 1 AM | Day 1 PM |
| Phase 2: Unit Tests | 1 day | Day 1 PM | Day 2 PM |
| Phase 3: Documentation | 0.5 days | Day 2 PM | Day 3 AM |
| Phase 4: Examples | 0.5 days | Day 3 AM | Day 3 PM |
| Phase 5: Error Messages | 0.5 days | Day 3 PM | Day 4 AM |
| Phase 6: Validation | 0.5 days | Day 4 AM | Day 4 PM |
| Phase 7: Communication | 0.5 days | Day 4 PM | Day 5 AM |
| Phase 8: Final Review | 0.5 days | Day 5 AM | Day 5 PM |
| **Total** | **4-5 days** | | |

---

## 13. Appendix

### 13.1 File Impact Analysis

**Total Files to Modify**: ~30-35 files

**Breakdown**:
- Core implementation: 2 files (`resolver.rs`, possibly `mod.rs`)
- Unit tests: 5-6 files
- Documentation: 4-5 files
- Examples: 2-3 files
- PRPs: 1-2 files
- Benchmarks: 1 file
- README/CHANGELOG: 2 files

### 13.2 Search Patterns for Implementation

```bash
# Find all occurrences
rg "node_modules" --type rust --type md

# Find in source code only
rg "node_modules" --type rust -g '!tests' -g '!benches'

# Find in tests
rg "node_modules" tests/

# Find in documentation
rg "node_modules" --type md docs/
```

### 13.3 Example Migration Command

For users with existing projects:
```bash
# Unix/Linux/macOS
mv node_modules hype_modules

# Windows (PowerShell)
Move-Item node_modules hype_modules

# Windows (CMD)
move node_modules hype_modules
```

### 13.4 FAQ Additions

**Q: Why did you rename node_modules to hype_modules?**  
A: To establish Hype-RS as an independent ecosystem with its own conventions. Using `node_modules` could create confusion about npm compatibility and dilute the Hype-RS brand identity.

**Q: Will there be a compatibility layer for node_modules?**  
A: No. Hype-RS is pre-1.0, and this is the right time to establish conventions. Migration is simple: just rename the directory.

**Q: Can I use both node_modules and hype_modules?**  
A: No. Hype-RS only searches for `hype_modules`. This ensures ecosystem consistency.

**Q: What about ~/.hype/modules?**  
A: This directory is unchanged and continues to work as before.

---

**Document Version**: 1.0  
**Last Updated**: October 2025  
**Author**: Hype-RS Team  
**Status**: APPROVED - Ready for Implementation
