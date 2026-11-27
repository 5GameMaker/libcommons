//! OS-specific APIs.

#[cfg(unix)]
pub mod unix;
#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(windows)]
pub mod windows;
