#[cfg(feature = "dirs")]
use std::path::PathBuf;

#[cfg(feature = "dirs")]
#[deprecated = "I decided that the API for this is meh and will be redoing it"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirRel {
    /// User-specific dir.
    User,
    /// System-wide dir.
    System,
}
#[cfg(feature = "dirs")]
#[allow(deprecated)]
impl DirRel {
    pub fn dir(self, ty: DirType) -> Option<PathBuf> {
        #[cfg(unix)]
        { crate::os::unix::dirs::dir(self, ty) }
        #[cfg(not(unix))]
        {
            _ = ty;
            None
        }
    }
}

#[cfg(feature = "dirs")]
#[deprecated = "I decided that the API for this is meh and will be redoing it"]
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
    ///
    /// User share directory is also used for storing user data.
    Share,
    /// Temporary application data directory.
    Cache,
    /// Permanent application data directory.
    State,
    /// Application configuration directory.
    Config,
    /// Executable files directory.
    Bin,
    /// Library files directory.
    Lib,
}
#[cfg(feature = "dirs")]
#[allow(deprecated)]
impl DirType {
    pub fn dir(self, rel: DirRel) -> Option<PathBuf> {
        #[cfg(unix)]
        { crate::os::unix::dirs::dir(rel, self) }
        #[cfg(not(unix))]
        {
            _ = rel;
            None
        }
    }
}
