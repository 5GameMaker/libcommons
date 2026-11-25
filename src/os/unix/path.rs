use std::{ffi::c_char, path::Path};

use crate::ffi::str::{ByteToChar, str_to_c_str};

pub const PATH_MAX: usize = libc::PATH_MAX as usize;

/// Extensions for [Path].
///
/// Used for conversion between Rust's [Path] and C's `const char*`.
pub trait PathExt {
    /// Write this [Path] to a byte buffer.
    ///
    /// The buffer must have enough space for the contents of `self`
    /// as well as for a trailing NULL byte.
    #[must_use]
    fn as_c_str<'a, 'b>(&'a self, buf: &'b mut [u8]) -> Option<&'b [c_char]>;

    /// Return a new byte buffer containing the path.
    ///
    /// The path plus a trailing NULL byte must not be longer than [PATH_MAX].
    #[must_use]
    fn as_c_str_byte_array(&self) -> Option<[c_char; PATH_MAX]> {
        let mut buf = [0; PATH_MAX];
        if self.as_c_str(&mut buf).is_some() {
            Some(buf.cast())
        } else {
            None
        }
    }
}
impl PathExt for &Path {
    fn as_c_str<'a, 'b>(&'a self, buf: &'b mut [u8]) -> Option<&'b [c_char]> {
        str_to_c_str(self.as_os_str(), buf)
    }
}
