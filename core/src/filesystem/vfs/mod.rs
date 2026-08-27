pub mod walkdir;

use std::future;
use std::os::fd::FromRawFd;
use std::sync::OnceLock;

use thiserror::Error;
use tokio::fs::File;

/// The virtual filesystem instance
static VFS: OnceLock<Box<dyn VirtualFilesystem>> = OnceLock::new();

#[derive(Error, Clone, Debug)]
pub enum VirtualFilesystemError {
    #[error("Filesystem already set")]
    AlreadySet,
    #[error("Filesystem not set")]
    NotSet,
    #[error("File not found")]
    FileNotFound,
    #[error("File already exists")]
    FileAlreadyExists,
    #[error("Permission denied")]
    PermissionDenied,
    #[error("Invalid path")]
    InvalidPath,
    #[error("Failed to create file")]
    FileCreateError,
}

#[derive(Debug, Clone)]
pub struct VirtualMetadata {
    pub is_dir: bool,
    pub name: String,
    pub size: u64,
    pub file_count: u64,
}

impl VirtualMetadata {
    pub fn is_file(&self) -> bool {
        !self.is_dir
    }

    pub fn is_dir(&self) -> bool {
        self.is_dir
    }
}

#[derive(Debug, Clone)]
pub struct VirtualEntry {
    pub is_dir: bool,
    pub path: String,
    pub name: String,
}

impl VirtualEntry {
    pub fn from_metadata(path: String, metadata: VirtualMetadata) -> Self {
        VirtualEntry { is_dir: metadata.is_dir, path, name: metadata.name }
    }
}

type Result<T> = std::result::Result<T, VirtualFilesystemError>;

#[async_trait::async_trait]
pub trait VirtualFilesystem: Send + Sync {
    /// Returns a `VirtualMetadata` struct for the given path. If the path is a
    /// directory, it should not return file count and size. If it's a file, it
    /// should return the size and 0 file count.
    async fn metadata(&self, path: String) -> Result<VirtualMetadata>;
    /// Reads a directory and returns a list of its entries. Each entry is
    /// represented as a `VirtualMetadata` struct. If is_dir is true, it should
    /// have calculated the size of the directory by summing the sizes of all
    /// its contents and the total number of children. If is_dir is false, it
    /// should return the size of the file and 1 file_count
    async fn read_dir(&self, path: String) -> Result<Vec<VirtualMetadata>>;
    /// A faster listing of only the top directory entries
    async fn list_dir(&self, path: String) -> Result<Vec<VirtualEntry>>;
    /// Create a directory in the path using the relative path.
    async fn create_dir(&self, path: String, folder: String) -> Result<String>;

    /// Opens a file and returns a file descriptor
    async fn open_file(&self, path: String) -> Result<i32>;
    /// Creates a file in the path using the relative path and returns a file
    /// descriptor
    async fn create_file(&self, path: String, file: String) -> Result<i32>;
}

pub fn set_virtual_filesystem(vfs: Box<dyn VirtualFilesystem>) -> Result<()> {
    VFS.set(vfs).map_err(|_| VirtualFilesystemError::AlreadySet)
}

pub fn metadata(path: &str) -> impl Future<Output = Result<VirtualMetadata>> {
    match VFS.get() {
        Some(vfs) => vfs.metadata(path.to_string()),
        None => Box::pin(future::ready(Err(VirtualFilesystemError::NotSet))),
    }
}

pub fn read_dir(path: &str) -> impl Future<Output = Result<Vec<VirtualMetadata>>> {
    match VFS.get() {
        Some(vfs) => vfs.read_dir(path.to_string()),
        None => Box::pin(future::ready(Err(VirtualFilesystemError::NotSet))),
    }
}

pub fn list_dir(path: &str) -> impl Future<Output = Result<Vec<VirtualEntry>>> {
    match VFS.get() {
        Some(vfs) => vfs.list_dir(path.to_string()),
        None => Box::pin(future::ready(Err(VirtualFilesystemError::NotSet))),
    }
}

pub fn create_dir(path: &str, folder: &str) -> impl Future<Output = Result<String>> {
    match VFS.get() {
        Some(vfs) => vfs.create_dir(path.to_string(), folder.to_string()),
        None => Box::pin(future::ready(Err(VirtualFilesystemError::NotSet))),
    }
}

pub async fn open_file(path: &str) -> Result<File> {
    let fd = VFS.get().ok_or(VirtualFilesystemError::NotSet)?.open_file(path.to_string()).await?;
    Ok(unsafe { File::from_raw_fd(fd) })
}

pub async fn create_file(path: &str, file: &str) -> Result<File> {
    let fd = VFS
        .get()
        .ok_or(VirtualFilesystemError::NotSet)?
        .create_file(path.to_string(), file.to_string())
        .await?;
    Ok(unsafe { File::from_raw_fd(fd) })
}
