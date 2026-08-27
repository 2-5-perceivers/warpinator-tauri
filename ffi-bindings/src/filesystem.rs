#[cfg(feature = "virtual_filesystem")]
mod vfs {
    use thiserror::Error;

    #[derive(uniffi::Error, Error, Debug)]
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

    impl From<VirtualFilesystemError> for warpinator_lib::filesystem::vfs::VirtualFilesystemError {
        fn from(value: VirtualFilesystemError) -> Self {
            match value {
                VirtualFilesystemError::AlreadySet => {
                    warpinator_lib::filesystem::vfs::VirtualFilesystemError::AlreadySet
                }
                VirtualFilesystemError::NotSet => {
                    warpinator_lib::filesystem::vfs::VirtualFilesystemError::NotSet
                }
                VirtualFilesystemError::FileNotFound => {
                    warpinator_lib::filesystem::vfs::VirtualFilesystemError::FileNotFound
                }
                VirtualFilesystemError::FileAlreadyExists => {
                    warpinator_lib::filesystem::vfs::VirtualFilesystemError::FileAlreadyExists
                }
                VirtualFilesystemError::PermissionDenied => {
                    warpinator_lib::filesystem::vfs::VirtualFilesystemError::PermissionDenied
                }
                VirtualFilesystemError::InvalidPath => {
                    warpinator_lib::filesystem::vfs::VirtualFilesystemError::InvalidPath
                }
                VirtualFilesystemError::FileCreateError => {
                    warpinator_lib::filesystem::vfs::VirtualFilesystemError::FileCreateError
                }
            }
        }
    }

    #[derive(uniffi::Record)]
    pub struct VirtualMetadata {
        pub is_dir: bool,
        pub name: String,
        pub size: u64,
        pub file_count: u64,
    }

    impl From<VirtualMetadata> for warpinator_lib::filesystem::vfs::VirtualMetadata {
        fn from(value: VirtualMetadata) -> Self {
            Self {
                is_dir: value.is_dir,
                name: value.name,
                size: value.size,
                file_count: value.file_count,
            }
        }
    }

    #[derive(uniffi::Record)]
    pub struct VirtualEntry {
        pub is_dir: bool,
        pub path: String,
        pub name: String,
    }

    impl From<VirtualEntry> for warpinator_lib::filesystem::vfs::VirtualEntry {
        fn from(value: VirtualEntry) -> Self {
            Self {
                is_dir: value.is_dir,
                path: value.path,
                name: value.name,
            }
        }
    }

    pub type Result<T> = std::result::Result<T, VirtualFilesystemError>;

    #[uniffi::export(callback_interface)]
    #[async_trait::async_trait]
    pub trait VirtualFilesystem: Send + Sync {
        async fn metadata(&self, path: String) -> Result<VirtualMetadata>;
        async fn read_dir(&self, path: String) -> Result<Vec<VirtualMetadata>>;
        async fn list_dir(&self, path: String) -> Result<Vec<VirtualEntry>>;
        async fn create_dir(&self, path: String, folder: String) -> Result<String>;
        async fn open_file(&self, path: String) -> Result<i32>;
        async fn create_file(&self, path: String, file: String) -> Result<i32>;
    }

    struct VirtualFilesystemWrapper {
        inner: Box<dyn VirtualFilesystem>,
    }
    type InnerResult<T> =
        std::result::Result<T, warpinator_lib::filesystem::vfs::VirtualFilesystemError>;

    #[async_trait::async_trait]
    impl warpinator_lib::filesystem::vfs::VirtualFilesystem for VirtualFilesystemWrapper {
        async fn metadata(
            &self,
            path: String,
        ) -> InnerResult<warpinator_lib::filesystem::vfs::VirtualMetadata> {
            let meta = self.inner.metadata(path).await?;

            Ok(warpinator_lib::filesystem::vfs::VirtualMetadata {
                is_dir: meta.is_dir,
                name: meta.name,
                size: meta.size,
                file_count: meta.file_count,
            })
        }

        async fn read_dir(
            &self,
            path: String,
        ) -> InnerResult<Vec<warpinator_lib::filesystem::vfs::VirtualMetadata>> {
            let entries = self.inner.read_dir(path).await?;

            Ok(entries.into_iter().map(|meta| meta.into()).collect())
        }

        async fn list_dir(
            &self,
            path: String,
        ) -> InnerResult<Vec<warpinator_lib::filesystem::vfs::VirtualEntry>> {
            let entries = self.inner.list_dir(path).await?;
            Ok(entries.into_iter().map(|meta| meta.into()).collect())
        }

        async fn create_dir(&self, path: String, folder: String) -> InnerResult<String> {
            self.inner
                .create_dir(path, folder)
                .await
                .map_err(|e| e.into())
        }

        async fn open_file(&self, path: String) -> InnerResult<i32> {
            self.inner.open_file(path).await.map_err(|e| e.into())
        }

        async fn create_file(&self, path: String, file: String) -> InnerResult<i32> {
            self.inner
                .create_file(path, file)
                .await
                .map_err(|e| e.into())
        }
    }

    #[uniffi::export]
    pub fn set_virtual_filesystem(vfs: Box<dyn VirtualFilesystem>) -> Result<()> {
        warpinator_lib::filesystem::vfs::set_virtual_filesystem(Box::new(
            VirtualFilesystemWrapper { inner: vfs },
        ))
        .map_err(|_| VirtualFilesystemError::AlreadySet)
    }
}
