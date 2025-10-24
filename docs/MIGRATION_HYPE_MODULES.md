# Migration Guide: node_modules → hype_modules

**Effective Version**: 0.2.0  
**Last Updated**: October 2025  
**Migration Difficulty**: ⭐ Easy (5 minutes)

---

## Overview

Starting with version 0.2.0, Hype-RS has renamed the module directory from `node_modules` to `hype_modules`. This change establishes Hype-RS as an independent Lua runtime with its own conventions and ecosystem identity.

### Why This Change?

1. **Brand Differentiation**: Establishes Hype-RS as distinct from Node.js
2. **Prevent Confusion**: Avoids expectations of npm compatibility
3. **Ecosystem Independence**: Signals Hype-RS has its own package system
4. **Consistency**: Aligns with project naming (hype-rs → hype_modules)
5. **Future-Proofing**: Avoids potential trademark concerns

---

## Quick Migration

### For Project Maintainers

If you have an existing Hype-RS project with modules:

**Unix/Linux/macOS:**
```bash
mv node_modules hype_modules
```

**Windows (PowerShell):**
```powershell
Move-Item node_modules hype_modules
```

**Windows (Command Prompt):**
```cmd
move node_modules hype_modules
```

That's it! Your project will now work with Hype-RS 0.2.0+.

---

## Detailed Migration Steps

### Step 1: Check Your Current Structure

Before migrating, verify if you have a `node_modules` directory:

```bash
# Check if node_modules exists
ls -la | grep node_modules

# Or on Windows
dir | findstr node_modules
```

If you don't have `node_modules`, you don't need to migrate anything.

### Step 2: Rename the Directory

Choose the command for your operating system from the "Quick Migration" section above.

### Step 3: Verify the Migration

After renaming, verify the new structure:

```bash
# Unix/Linux/macOS
ls -la hype_modules/

# Windows
dir hype_modules
```

You should see all your modules in the new `hype_modules` directory.

### Step 4: Test Your Application

Run your application to ensure everything works:

```bash
hype your-script.lua
```

If you encounter any issues, see the "Troubleshooting" section below.

---

## What Changed

### Module Resolution Order

**Before (< 0.2.0):**
```
1. Built-in modules (fs, path, events, util, table)
2. node_modules directories (walk up)
3. ~/.hype/modules
4. Error if not found
```

**After (>= 0.2.0):**
```
1. Built-in modules (fs, path, events, util, table)
2. hype_modules directories (walk up)
3. ~/.hype/modules
4. Error if not found
```

### Directory Structure

**Before:**
```
my-project/
├── node_modules/
│   ├── my-lib/
│   └── another-lib/
├── app.lua
└── hype.json
```

**After:**
```
my-project/
├── hype_modules/
│   ├── my-lib/
│   └── another-lib/
├── app.lua
└── hype.json
```

### Code Changes

**No code changes required!** Your `require()` calls remain exactly the same:

```lua
-- This works before and after migration
local fs = require("fs")
local myLib = require("my-lib")
```

The change is only in the filesystem directory name, not in the API.

---

## Compatibility

### Version Compatibility

| Hype-RS Version | Module Directory | Status |
|-----------------|------------------|--------|
| < 0.2.0 | `node_modules` | Legacy |
| >= 0.2.0 | `hype_modules` | Current |

### Backward Compatibility

**Important**: Hype-RS 0.2.0+ does **NOT** support `node_modules`. You must rename the directory.

This is intentional to:
- Force early adoption of the new standard
- Avoid confusion and fragmentation
- Keep the codebase simple and maintainable

---

## Special Cases

### Multiple Projects

If you have multiple Hype-RS projects:

```bash
# Create a simple script to migrate all projects
for dir in ~/projects/*/; do
  if [ -d "$dir/node_modules" ]; then
    echo "Migrating $dir"
    cd "$dir"
    mv node_modules hype_modules
  fi
done
```

### Shared Modules

If you share modules between projects via symlinks:

1. Rename the shared directory once
2. Update all symlinks to point to the new location

Example:
```bash
# Rename shared modules
mv ~/shared/node_modules ~/shared/hype_modules

# Update symlinks in projects
cd ~/projects/my-app
rm node_modules
ln -s ~/shared/hype_modules hype_modules
```

### Version Control

Don't forget to update your `.gitignore`:

**Before:**
```gitignore
node_modules/
```

**After:**
```gitignore
hype_modules/
```

### CI/CD Pipelines

Update your CI/CD scripts if they reference `node_modules`:

**Before:**
```yaml
- name: Cache modules
  uses: actions/cache@v3
  with:
    path: node_modules
```

**After:**
```yaml
- name: Cache modules
  uses: actions/cache@v3
  with:
    path: hype_modules
```

---

## Troubleshooting

### Issue: "Module not found" errors after migration

**Cause**: Directory not renamed correctly or modules missing

**Solution**:
1. Verify `hype_modules` exists: `ls -la hype_modules/`
2. Check module is in directory: `ls hype_modules/module-name/`
3. Ensure directory has correct permissions

### Issue: Old version of Hype-RS still looking for node_modules

**Cause**: Using Hype-RS version < 0.2.0

**Solution**:
```bash
# Update Hype-RS
cargo install hype-rs --force

# Verify version
hype --version  # Should be >= 0.2.0
```

### Issue: Git shows many file changes

**Cause**: Git tracks the directory rename as deletions + additions

**Solution**:
```bash
# Configure Git to detect renames
git config merge.renameLimit 999999

# Stage the changes
git add -A

# Commit with clear message
git commit -m "Migrate from node_modules to hype_modules"
```

Git will automatically detect this as a rename in the commit history.

### Issue: Scripts or tools reference node_modules

**Cause**: Custom scripts hardcode `node_modules` path

**Solution**: Update scripts to use `hype_modules`:

```bash
# Find all references in scripts
grep -r "node_modules" scripts/

# Use find/replace in your editor to update them
```

---

## FAQ

### Q: Why not support both node_modules and hype_modules?

**A**: Supporting both would:
- Add unnecessary complexity
- Create confusion about which to use
- Accumulate technical debt
- Fragment the ecosystem

Hype-RS is pre-1.0, making this the ideal time for breaking changes.

### Q: Will node_modules support be added back?

**A**: No. The decision is final. `hype_modules` is the standard going forward.

### Q: Can I configure the directory name?

**A**: No. Standardization is key for a healthy ecosystem. Everyone uses `hype_modules`.

### Q: What about ~/.hype/modules?

**A**: This directory is unchanged and continues to work. The rename only affects project-local module directories.

### Q: Do I need to update my module manifests (hype.json)?

**A**: No. Module manifests don't reference the directory name.

### Q: Will this affect npm packages?

**A**: No. Hype-RS modules are separate from npm. This change has no effect on npm.

### Q: What if I'm using a module manager/tool?

**A**: Update your tool to version 0.2.0+ which supports `hype_modules`.

---

## Getting Help

If you encounter issues during migration:

1. **Check this guide** for common issues
2. **Search closed issues** on GitHub for similar problems
3. **Open a new issue** with:
   - Your Hype-RS version (`hype --version`)
   - Your operating system
   - Steps to reproduce the issue
   - Error messages

---

## Rollback (Emergency Only)

If you must temporarily rollback to an older version:

**Step 1**: Rename back to node_modules
```bash
mv hype_modules node_modules
```

**Step 2**: Downgrade Hype-RS
```bash
cargo install hype-rs --version 0.1.0
```

**Note**: This is NOT recommended. Please report issues instead so we can help you complete the migration.

---

## Summary Checklist

Before considering migration complete:

- [ ] Renamed `node_modules` to `hype_modules`
- [ ] Tested application runs correctly
- [ ] Updated `.gitignore` if needed
- [ ] Updated CI/CD scripts if needed
- [ ] Updated documentation if you have any
- [ ] Updated custom tools/scripts if any
- [ ] Committed changes with clear message

---

## Additional Resources

- [Hype-RS Module Documentation](./modules/README.md)
- [require() API Reference](./modules/require-api.md)
- [Getting Started with Modules](./modules/getting-started.md)
- [PRP: hype_modules Rename](../PRPs/hype-modules-rename-prp.md)

---

**Migration completed?** Welcome to Hype-RS 0.2.0! 🎉

If you found this guide helpful, consider contributing to the project or sharing your experience with the community.
