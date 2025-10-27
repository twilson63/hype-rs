use super::error::OsError;
use std::env;
use sysinfo::System;

pub fn platform() -> &'static str {
    if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "freebsd") {
        "freebsd"
    } else if cfg!(target_os = "openbsd") {
        "openbsd"
    } else {
        "unknown"
    }
}

pub fn arch() -> &'static str {
    if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else if cfg!(target_arch = "aarch64") {
        "aarch64"
    } else if cfg!(target_arch = "arm") {
        "arm"
    } else if cfg!(target_arch = "x86") {
        "x86"
    } else {
        "unknown"
    }
}

pub fn hostname() -> Result<String, OsError> {
    System::host_name().ok_or_else(|| OsError::HostnameError("Unable to get hostname".to_string()))
}

pub fn homedir() -> Result<String, OsError> {
    env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .map_err(|_| OsError::HomeDirNotFound)
}

pub fn tmpdir() -> Result<String, OsError> {
    env::temp_dir()
        .to_str()
        .map(|s| s.to_string())
        .ok_or(OsError::TempDirNotFound)
}

pub struct CpuInfo {
    pub model: String,
    pub speed: u64,
}

pub fn cpus() -> Result<Vec<CpuInfo>, OsError> {
    let mut sys = System::new();
    sys.refresh_cpu();

    let cpus: Vec<CpuInfo> = sys
        .cpus()
        .iter()
        .map(|cpu| CpuInfo {
            model: cpu.brand().to_string(),
            speed: cpu.frequency(),
        })
        .collect();

    if cpus.is_empty() {
        return Err(OsError::SystemInfoError(
            "No CPU information available".to_string(),
        ));
    }

    Ok(cpus)
}

pub fn totalmem() -> Result<u64, OsError> {
    let mut sys = System::new();
    sys.refresh_memory();
    Ok(sys.total_memory())
}

pub fn freemem() -> Result<u64, OsError> {
    let mut sys = System::new();
    sys.refresh_memory();
    Ok(sys.available_memory())
}

pub fn uptime() -> Result<u64, OsError> {
    Ok(System::uptime())
}

pub fn loadavg() -> Result<(f64, f64, f64), OsError> {
    let load_avg = System::load_average();
    Ok((load_avg.one, load_avg.five, load_avg.fifteen))
}

pub struct NetworkInterface {
    pub name: String,
    pub mac_address: String,
}

pub fn network_interfaces() -> Result<Vec<NetworkInterface>, OsError> {
    let networks = sysinfo::Networks::new_with_refreshed_list();

    let interfaces: Vec<NetworkInterface> = networks
        .iter()
        .map(|(name, data)| NetworkInterface {
            name: name.clone(),
            mac_address: data.mac_address().to_string(),
        })
        .collect();

    Ok(interfaces)
}

pub struct UserInfo {
    pub username: String,
    pub uid: Option<u32>,
    pub gid: Option<u32>,
    pub shell: Option<String>,
    pub homedir: String,
}

pub fn user_info() -> Result<UserInfo, OsError> {
    let username = env::var("USER")
        .or_else(|_| env::var("USERNAME"))
        .unwrap_or_else(|_| "unknown".to_string());

    let homedir = homedir()?;

    let shell = env::var("SHELL").ok();

    #[cfg(unix)]
    let (uid, gid) = {
        use std::os::unix::fs::MetadataExt;
        let metadata = std::fs::metadata(&homedir).ok();
        (
            metadata.as_ref().map(|m| m.uid()),
            metadata.as_ref().map(|m| m.gid()),
        )
    };

    #[cfg(not(unix))]
    let (uid, gid) = (None, None);

    Ok(UserInfo {
        username,
        uid,
        gid,
        shell,
        homedir,
    })
}

pub fn eol() -> &'static str {
    if cfg!(target_os = "windows") {
        "\r\n"
    } else {
        "\n"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform() {
        let p = platform();
        assert!(["linux", "macos", "windows", "freebsd", "openbsd", "unknown"].contains(&p));
    }

    #[test]
    fn test_arch() {
        let a = arch();
        assert!(["x86_64", "aarch64", "arm", "x86", "unknown"].contains(&a));
    }

    #[test]
    fn test_hostname() {
        let result = hostname();
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }

    #[test]
    fn test_homedir() {
        let result = homedir();
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }

    #[test]
    fn test_tmpdir() {
        let result = tmpdir();
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }

    #[test]
    fn test_cpus() {
        let result = cpus();
        assert!(result.is_ok());
        let cpu_list = result.unwrap();
        assert!(!cpu_list.is_empty());
        assert!(!cpu_list[0].model.is_empty());
    }

    #[test]
    fn test_totalmem() {
        let result = totalmem();
        assert!(result.is_ok());
        assert!(result.unwrap() > 0);
    }

    #[test]
    fn test_freemem() {
        let result = freemem();
        assert!(result.is_ok());
    }

    #[test]
    fn test_uptime() {
        let result = uptime();
        assert!(result.is_ok());
        assert!(result.unwrap() > 0);
    }

    #[test]
    fn test_loadavg() {
        let result = loadavg();
        assert!(result.is_ok());
        let (one, five, fifteen) = result.unwrap();
        assert!(one >= 0.0);
        assert!(five >= 0.0);
        assert!(fifteen >= 0.0);
    }

    #[test]
    fn test_network_interfaces() {
        let result = network_interfaces();
        assert!(result.is_ok());
    }

    #[test]
    fn test_user_info() {
        let result = user_info();
        assert!(result.is_ok());
        let info = result.unwrap();
        assert!(!info.username.is_empty());
        assert!(!info.homedir.is_empty());
    }

    #[test]
    fn test_eol() {
        let e = eol();
        #[cfg(target_os = "windows")]
        assert_eq!(e, "\r\n");
        #[cfg(not(target_os = "windows"))]
        assert_eq!(e, "\n");
    }
}
