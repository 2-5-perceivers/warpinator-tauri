#[cfg(all(feature = "virtual_filesystem", feature = "real_filesystem"))]
compile_error!(
    "The `virtual_filesystem` and `real_filesystem` features are mutually exclusive. Please \
     choose one or the other."
);

#[cfg(not(any(feature = "virtual_filesystem", feature = "real_filesystem")))]
compile_error!(
    "Either the `virtual_filesystem` or `real_filesystem` feature must be enabled. Please enable \
     one of these features."
);

#[cfg(all(target_os = "android", feature = "real_filesystem"))]
compile_error!(
    "The `real_filesystem` feature is not supported on Android. Please use the \
     `virtual_filesystem` feature instead."
);

#[cfg(all(feature = "virtual_filesystem", not(target_family = "unix")))]
compile_error!(
    "The `virtual_filesystem` feature is only supported on Unix systems. Please disable the \
     `virtual_filesystem` feature or use a Unix system."
);

#[cfg(feature = "virtual_filesystem")]
/// An opinionated abstraction of the filesystem to be used on Unix systems with
/// locked down permissions, such as Android. It allows us to implement a custom
/// filesystem that can be used to access files and directories without relying
/// on the underlying OS filesystem. This is especially useful for Android,
/// where we can use the Storage Access Framework (SAF) to access files and
/// directories that are not directly accessible through the OS filesystem.
pub mod vfs;
