use crate::modules::error::ModuleError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use tempfile::NamedTempFile;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledPackage {
    pub name: String,
    pub version: String,
    pub install_date: String,
    pub location: PathBuf,
    pub bin: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RegistryData {
    packages: HashMap<String, InstalledPackage>,
    bin_commands: HashMap<String, String>,
}

#[derive(Debug)]
pub struct GlobalPackageRegistry {
    root_dir: PathBuf,
    packages_dir: PathBuf,
    bin_dir: PathBuf,
    registry_file: PathBuf,
    packages: HashMap<String, InstalledPackage>,
    bin_commands: HashMap<String, String>,
}

impl GlobalPackageRegistry {
    pub fn new() -> Result<Self, ModuleError> {
        let root_dir = Self::get_root_dir()?;
        let packages_dir = root_dir.join("packages");
        let bin_dir = root_dir.join("bin");
        let registry_file = root_dir.join("registry.json");

        fs::create_dir_all(&root_dir).map_err(|e| {
            ModuleError::RegistryError(format!("Failed to create root directory: {}", e))
        })?;

        fs::create_dir_all(&packages_dir).map_err(|e| {
            ModuleError::RegistryError(format!("Failed to create packages directory: {}", e))
        })?;

        fs::create_dir_all(&bin_dir).map_err(|e| {
            ModuleError::RegistryError(format!("Failed to create bin directory: {}", e))
        })?;

        Ok(Self {
            root_dir,
            packages_dir,
            bin_dir,
            registry_file,
            packages: HashMap::new(),
            bin_commands: HashMap::new(),
        })
    }

    pub fn load() -> Result<Self, ModuleError> {
        let root_dir = Self::get_root_dir()?;
        let packages_dir = root_dir.join("packages");
        let bin_dir = root_dir.join("bin");
        let registry_file = root_dir.join("registry.json");

        if !registry_file.exists() {
            return Self::new();
        }

        let content = fs::read_to_string(&registry_file).map_err(|e| {
            ModuleError::RegistryError(format!("Failed to read registry file: {}", e))
        })?;

        let data: RegistryData = serde_json::from_str(&content).map_err(|e| {
            ModuleError::RegistryError(format!("Failed to parse registry JSON: {}", e))
        })?;

        Ok(Self {
            root_dir,
            packages_dir,
            bin_dir,
            registry_file,
            packages: data.packages,
            bin_commands: data.bin_commands,
        })
    }

    pub fn save(&self) -> Result<(), ModuleError> {
        let data = RegistryData {
            packages: self.packages.clone(),
            bin_commands: self.bin_commands.clone(),
        };

        let json = serde_json::to_string_pretty(&data).map_err(|e| {
            ModuleError::RegistryError(format!("Failed to serialize registry: {}", e))
        })?;

        let parent = self
            .registry_file
            .parent()
            .ok_or_else(|| ModuleError::RegistryError("Invalid registry path".to_string()))?;

        let mut temp_file = NamedTempFile::new_in(parent).map_err(|e| {
            ModuleError::RegistryError(format!("Failed to create temp file: {}", e))
        })?;

        temp_file.write_all(json.as_bytes()).map_err(|e| {
            ModuleError::RegistryError(format!("Failed to write to temp file: {}", e))
        })?;

        temp_file.persist(&self.registry_file).map_err(|e| {
            ModuleError::RegistryError(format!("Failed to persist registry file: {}", e))
        })?;

        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&InstalledPackage> {
        self.packages.get(name)
    }

    pub fn list(&self) -> Vec<&InstalledPackage> {
        self.packages.values().collect()
    }

    pub fn check_bin_conflict(
        &self,
        bin_map: &HashMap<String, String>,
    ) -> Result<Vec<String>, ModuleError> {
        let mut conflicts = Vec::new();

        for cmd in bin_map.keys() {
            if self.bin_commands.contains_key(cmd) {
                conflicts.push(cmd.clone());
            }
        }

        Ok(conflicts)
    }

    pub fn add_package(&mut self, pkg: InstalledPackage) -> Result<(), ModuleError> {
        let conflicts = self.check_bin_conflict(&pkg.bin)?;
        if !conflicts.is_empty() {
            return Err(ModuleError::RegistryError(format!(
                "Binary command conflicts: {}",
                conflicts.join(", ")
            )));
        }

        let pkg_version = format!("{}@{}", pkg.name, pkg.version);
        for cmd in pkg.bin.keys() {
            self.bin_commands.insert(cmd.clone(), pkg_version.clone());
        }

        self.packages.insert(pkg.name.clone(), pkg);
        self.save()?;

        Ok(())
    }

    pub fn remove_package(&mut self, name: &str) -> Result<InstalledPackage, ModuleError> {
        let pkg = self
            .packages
            .remove(name)
            .ok_or_else(|| ModuleError::ModuleNotFound(name.to_string()))?;

        for cmd in pkg.bin.keys() {
            self.bin_commands.remove(cmd);
        }

        self.save()?;

        Ok(pkg)
    }

    pub fn which_command(&self, cmd: &str) -> Option<String> {
        self.bin_commands.get(cmd).cloned()
    }

    fn get_root_dir() -> Result<PathBuf, ModuleError> {
        if let Ok(hype_home) = env::var("HYPE_HOME") {
            return Ok(PathBuf::from(hype_home));
        }

        let home = env::var("HOME").map_err(|_| {
            ModuleError::RegistryError("Could not determine home directory".to_string())
        })?;

        Ok(PathBuf::from(home).join(".hype"))
    }

    pub fn root_dir(&self) -> &PathBuf {
        &self.root_dir
    }

    pub fn packages_dir(&self) -> &PathBuf {
        &self.packages_dir
    }

    pub fn bin_dir(&self) -> &PathBuf {
        &self.bin_dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::sync::Mutex;

    static TEST_MUTEX: Mutex<()> = Mutex::new(());

    fn setup_test_env() -> tempfile::TempDir {
        let temp_dir = tempfile::tempdir().unwrap();
        env::set_var("HYPE_HOME", temp_dir.path());
        temp_dir
    }

    #[test]
    fn test_new_registry_creates_directories() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let _temp = setup_test_env();
        let registry = GlobalPackageRegistry::new().unwrap();

        assert!(registry.root_dir.exists());
        assert!(registry.packages_dir.exists());
        assert!(registry.bin_dir.exists());
        assert_eq!(registry.packages.len(), 0);
        assert_eq!(registry.bin_commands.len(), 0);
    }

    #[test]
    fn test_load_missing_registry_file() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let _temp = setup_test_env();
        let registry = GlobalPackageRegistry::load().unwrap();

        assert_eq!(registry.packages.len(), 0);
        assert_eq!(registry.bin_commands.len(), 0);
    }

    #[test]
    fn test_save_and_load_registry() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let _temp = setup_test_env();
        let mut registry = GlobalPackageRegistry::new().unwrap();

        let mut bin_map = HashMap::new();
        bin_map.insert("testcmd".to_string(), "bin/test.lua".to_string());

        let pkg = InstalledPackage {
            name: "test-pkg".to_string(),
            version: "1.0.0".to_string(),
            install_date: "2025-10-26T12:00:00Z".to_string(),
            location: PathBuf::from("/test/path"),
            bin: bin_map,
        };

        registry.add_package(pkg).unwrap();

        let loaded = GlobalPackageRegistry::load().unwrap();
        assert_eq!(loaded.packages.len(), 1);
        assert!(loaded.packages.contains_key("test-pkg"));
        assert_eq!(loaded.bin_commands.len(), 1);
        assert_eq!(
            loaded.bin_commands.get("testcmd"),
            Some(&"test-pkg@1.0.0".to_string())
        );
    }

    #[test]
    fn test_add_package() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let _temp = setup_test_env();
        let mut registry = GlobalPackageRegistry::new().unwrap();

        let mut bin_map = HashMap::new();
        bin_map.insert("mycmd".to_string(), "bin/cli.lua".to_string());

        let pkg = InstalledPackage {
            name: "my-package".to_string(),
            version: "2.1.0".to_string(),
            install_date: "2025-10-26T12:00:00Z".to_string(),
            location: PathBuf::from("/some/path"),
            bin: bin_map,
        };

        registry.add_package(pkg).unwrap();

        assert_eq!(registry.packages.len(), 1);
        assert!(registry.get("my-package").is_some());
        assert_eq!(
            registry.which_command("mycmd"),
            Some("my-package@2.1.0".to_string())
        );
    }

    #[test]
    fn test_remove_package() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let _temp = setup_test_env();
        let mut registry = GlobalPackageRegistry::new().unwrap();

        let mut bin_map = HashMap::new();
        bin_map.insert("rmcmd".to_string(), "bin/rm.lua".to_string());

        let pkg = InstalledPackage {
            name: "rm-pkg".to_string(),
            version: "1.0.0".to_string(),
            install_date: "2025-10-26T12:00:00Z".to_string(),
            location: PathBuf::from("/rm/path"),
            bin: bin_map,
        };

        registry.add_package(pkg).unwrap();
        assert_eq!(registry.packages.len(), 1);

        let removed = registry.remove_package("rm-pkg").unwrap();
        assert_eq!(removed.name, "rm-pkg");
        assert_eq!(registry.packages.len(), 0);
        assert_eq!(registry.bin_commands.len(), 0);
        assert!(registry.which_command("rmcmd").is_none());
    }

    #[test]
    fn test_remove_nonexistent_package() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let _temp = setup_test_env();
        let mut registry = GlobalPackageRegistry::new().unwrap();

        let result = registry.remove_package("nonexistent");
        assert!(result.is_err());
        assert!(matches!(result, Err(ModuleError::ModuleNotFound(_))));
    }

    #[test]
    fn test_check_bin_conflict() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let _temp = setup_test_env();
        let mut registry = GlobalPackageRegistry::new().unwrap();

        let mut bin_map1 = HashMap::new();
        bin_map1.insert("conflict".to_string(), "bin/test1.lua".to_string());

        let pkg1 = InstalledPackage {
            name: "pkg1".to_string(),
            version: "1.0.0".to_string(),
            install_date: "2025-10-26T12:00:00Z".to_string(),
            location: PathBuf::from("/path1"),
            bin: bin_map1,
        };

        registry.add_package(pkg1).unwrap();

        let mut bin_map2 = HashMap::new();
        bin_map2.insert("conflict".to_string(), "bin/test2.lua".to_string());

        let conflicts = registry.check_bin_conflict(&bin_map2).unwrap();
        assert_eq!(conflicts.len(), 1);
        assert_eq!(conflicts[0], "conflict");
    }

    #[test]
    fn test_add_package_with_conflict() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let _temp = setup_test_env();
        let mut registry = GlobalPackageRegistry::new().unwrap();

        let mut bin_map1 = HashMap::new();
        bin_map1.insert("samecmd".to_string(), "bin/first.lua".to_string());

        let pkg1 = InstalledPackage {
            name: "first-pkg".to_string(),
            version: "1.0.0".to_string(),
            install_date: "2025-10-26T12:00:00Z".to_string(),
            location: PathBuf::from("/first"),
            bin: bin_map1,
        };

        registry.add_package(pkg1).unwrap();

        let mut bin_map2 = HashMap::new();
        bin_map2.insert("samecmd".to_string(), "bin/second.lua".to_string());

        let pkg2 = InstalledPackage {
            name: "second-pkg".to_string(),
            version: "1.0.0".to_string(),
            install_date: "2025-10-26T12:00:00Z".to_string(),
            location: PathBuf::from("/second"),
            bin: bin_map2,
        };

        let result = registry.add_package(pkg2);
        assert!(result.is_err());
        assert_eq!(registry.packages.len(), 1);
    }

    #[test]
    fn test_list_packages() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let _temp = setup_test_env();
        let mut registry = GlobalPackageRegistry::new().unwrap();

        let pkg1 = InstalledPackage {
            name: "pkg-a".to_string(),
            version: "1.0.0".to_string(),
            install_date: "2025-10-26T12:00:00Z".to_string(),
            location: PathBuf::from("/a"),
            bin: HashMap::new(),
        };

        let pkg2 = InstalledPackage {
            name: "pkg-b".to_string(),
            version: "2.0.0".to_string(),
            install_date: "2025-10-26T12:00:00Z".to_string(),
            location: PathBuf::from("/b"),
            bin: HashMap::new(),
        };

        registry.add_package(pkg1).unwrap();
        registry.add_package(pkg2).unwrap();

        let list = registry.list();
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_which_command() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let _temp = setup_test_env();
        let mut registry = GlobalPackageRegistry::new().unwrap();

        let mut bin_map = HashMap::new();
        bin_map.insert("special".to_string(), "bin/special.lua".to_string());

        let pkg = InstalledPackage {
            name: "special-pkg".to_string(),
            version: "3.2.1".to_string(),
            install_date: "2025-10-26T12:00:00Z".to_string(),
            location: PathBuf::from("/special"),
            bin: bin_map,
        };

        registry.add_package(pkg).unwrap();

        assert_eq!(
            registry.which_command("special"),
            Some("special-pkg@3.2.1".to_string())
        );
        assert_eq!(registry.which_command("nonexistent"), None);
    }

    #[test]
    fn test_atomic_save() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let _temp = setup_test_env();
        let mut registry = GlobalPackageRegistry::new().unwrap();

        let pkg = InstalledPackage {
            name: "atomic-pkg".to_string(),
            version: "1.0.0".to_string(),
            install_date: "2025-10-26T12:00:00Z".to_string(),
            location: PathBuf::from("/atomic"),
            bin: HashMap::new(),
        };

        registry.add_package(pkg).unwrap();

        assert!(registry.registry_file.exists());

        let loaded = GlobalPackageRegistry::load().unwrap();
        assert!(loaded.get("atomic-pkg").is_some());
    }

    #[test]
    fn test_multiple_bin_commands() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let _temp = setup_test_env();
        let mut registry = GlobalPackageRegistry::new().unwrap();

        let mut bin_map = HashMap::new();
        bin_map.insert("cmd1".to_string(), "bin/cmd1.lua".to_string());
        bin_map.insert("cmd2".to_string(), "bin/cmd2.lua".to_string());
        bin_map.insert("cmd3".to_string(), "bin/cmd3.lua".to_string());

        let pkg = InstalledPackage {
            name: "multi-bin".to_string(),
            version: "1.0.0".to_string(),
            install_date: "2025-10-26T12:00:00Z".to_string(),
            location: PathBuf::from("/multi"),
            bin: bin_map,
        };

        registry.add_package(pkg).unwrap();

        assert_eq!(registry.bin_commands.len(), 3);
        assert!(registry.which_command("cmd1").is_some());
        assert!(registry.which_command("cmd2").is_some());
        assert!(registry.which_command("cmd3").is_some());
    }
}
