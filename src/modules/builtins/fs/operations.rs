use super::error::FsError;
use std::fs;
use std::path::Path;

pub type Result<T> = std::result::Result<T, FsError>;

pub fn read_file_sync(path: &str) -> Result<String> {
    fs::read_to_string(path).map_err(Into::into)
}

pub fn write_file_sync(path: &str, data: &str) -> Result<()> {
    fs::write(path, data).map_err(Into::into)
}

pub fn exists_sync(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn stat_sync(path: &str) -> Result<FileStat> {
    let metadata = fs::metadata(path)?;

    Ok(FileStat {
        size: metadata.len(),
        is_file: metadata.is_file(),
        is_directory: metadata.is_dir(),
        is_symlink: metadata.file_type().is_symlink(),
        mtime: metadata
            .modified()?
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0),
    })
}

pub fn readdir_sync(path: &str) -> Result<Vec<String>> {
    let entries = fs::read_dir(path)?;
    let mut names = Vec::new();

    for entry in entries {
        let entry = entry?;
        if let Some(name) = entry.file_name().to_str() {
            names.push(name.to_string());
        }
    }

    names.sort();
    Ok(names)
}

pub fn unlink_sync(path: &str) -> Result<()> {
    fs::remove_file(path).map_err(Into::into)
}

pub fn mkdir_sync(path: &str) -> Result<()> {
    fs::create_dir_all(path).map_err(Into::into)
}

pub fn rmdir_sync(path: &str) -> Result<()> {
    let metadata = fs::metadata(path)?;

    if !metadata.is_dir() {
        return Err(FsError::InvalidOperation(
            "Path is not a directory".to_string(),
        ));
    }

    fs::remove_dir(path).map_err(Into::into)
}

pub struct FileStat {
    pub size: u64,
    pub is_file: bool,
    pub is_directory: bool,
    pub is_symlink: bool,
    pub mtime: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_read_write_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let file_path_str = file_path.to_str().unwrap();

        write_file_sync(file_path_str, "Hello World").unwrap();
        let content = read_file_sync(file_path_str).unwrap();
        assert_eq!(content, "Hello World");
    }

    #[test]
    fn test_exists() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        assert!(!exists_sync(file_path.to_str().unwrap()));

        fs::write(&file_path, "test").unwrap();
        assert!(exists_sync(file_path.to_str().unwrap()));
    }

    #[test]
    fn test_mkdir_rmdir() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path().join("test_dir");
        let dir_path_str = dir_path.to_str().unwrap();

        mkdir_sync(dir_path_str).unwrap();
        assert!(exists_sync(dir_path_str));

        rmdir_sync(dir_path_str).unwrap();
        assert!(!exists_sync(dir_path_str));
    }

    #[test]
    fn test_readdir() {
        let temp_dir = TempDir::new().unwrap();

        fs::write(temp_dir.path().join("file1.txt"), "").unwrap();
        fs::write(temp_dir.path().join("file2.txt"), "").unwrap();

        let files = readdir_sync(temp_dir.path().to_str().unwrap()).unwrap();
        assert_eq!(files.len(), 2);
        assert!(files.contains(&"file1.txt".to_string()));
        assert!(files.contains(&"file2.txt".to_string()));
    }

    #[test]
    fn test_stat() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "Hello").unwrap();

        let stat = stat_sync(file_path.to_str().unwrap()).unwrap();
        assert_eq!(stat.size, 5);
        assert!(stat.is_file);
        assert!(!stat.is_directory);
    }

    #[test]
    fn test_mkdir_recursive() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path().join("a").join("b").join("c");

        mkdir_sync(dir_path.to_str().unwrap()).unwrap();
        assert!(exists_sync(dir_path.to_str().unwrap()));
    }

    #[test]
    fn test_utf8_content() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("utf8.txt");
        let file_path_str = file_path.to_str().unwrap();

        let content = "Hello 世界 🌍";
        write_file_sync(file_path_str, content).unwrap();

        let read_content = read_file_sync(file_path_str).unwrap();
        assert_eq!(read_content, content);
    }
}
