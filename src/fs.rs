use std::path::PathBuf;

#[cfg(feature = "dirs")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirRel {
    /// User-specific dir.
    User,
    /// System-wide dir.
    System,
}
#[cfg(feature = "dirs")]
impl DirRel {
    pub fn dir(self, ty: DirType) -> Option<PathBuf> {
        #[cfg(unix)]
        {
            crate::os::unix::dirs::dir(self, ty)
        }
    }
}

#[cfg(feature = "dirs")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirType {
    /// Home directory.
    ///
    /// - [DirRel::User] will return user's home directory.
    /// - [DirRel::System] will return system installation directory.
    Home,
    /// Session information directory.
    Runtime,
    /// Resources directory.
    Share,
    /// Temporary application data directory.
    Cache,
    /// Permanent application data directory.
    State,
    /// Application configuration directory.
    Config,
    /// Executable files directory.
    ///
    /// ## OS-specific
    /// Will return [None] on Windows.
    Bin,
    /// Library files directory.
    ///
    /// ## OS-specific
    /// Will return [None] on Windows.
    Lib,
}
#[cfg(feature = "dirs")]
impl DirType {
    pub fn dir(self, rel: DirRel) -> Option<PathBuf> {
        #[cfg(unix)]
        {
            crate::os::unix::dirs::dir(rel, self)
        }
    }
}

#[cfg(all(unix, feature = "lock"))]
pub use crate::os::unix::lock::PathLock;
#[cfg(all(unix, feature = "lock"))]
pub use crate::os::unix::lock::lock;
