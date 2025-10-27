use std::fmt;

#[derive(Debug)]
pub enum OsError {
    HostnameError(String),
    HomeDirNotFound,
    TempDirNotFound,
    SystemInfoError(String),
    NetworkInterfacesError(String),
    UserInfoError(String),
}

impl fmt::Display for OsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OsError::HostnameError(err) => write!(f, "Failed to get hostname: {}", err),
            OsError::HomeDirNotFound => write!(f, "Home directory not found"),
            OsError::TempDirNotFound => write!(f, "Temp directory not found"),
            OsError::SystemInfoError(err) => write!(f, "Failed to get system info: {}", err),
            OsError::NetworkInterfacesError(err) => {
                write!(f, "Failed to get network interfaces: {}", err)
            }
            OsError::UserInfoError(err) => write!(f, "Failed to get user info: {}", err),
        }
    }
}

impl std::error::Error for OsError {}

impl From<OsError> for crate::error::HypeError {
    fn from(err: OsError) -> Self {
        crate::error::HypeError::Execution(err.to_string())
    }
}
