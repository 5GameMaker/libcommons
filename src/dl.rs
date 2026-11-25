#[cfg(unix)]
use std::{ffi::c_void, io, path::Path};

// TODO: Windows

#[cfg(unix)]
pub type Handle = crate::os::unix::dl::Handle;
#[cfg(not(unix))]
pub type Handle = !;

#[cfg(unix)]
pub type SectionNamesIter<'a> = crate::os::unix::dl::SectionNamesIter<'a>;
#[cfg(not(unix))]
pub type SectionNamesIter<'a> = std::slice::Iter<'a, &'a std::ffi::OsStr>;

/// A dlopen'd library.
///
/// May not be implemented for all platforms.
///
/// ## Platform-specific behavior
///
/// - On Unix-like systems (i.e. Linux) uses libc `dlopen`;
/// - On Windows: not yet implemented.
#[repr(transparent)]
pub struct Library(Handle);
impl Library {
    /// Open a dynamic library.
    ///
    /// ## Platform-specific behavior
    ///
    /// - On Unix uses `dlopen`.
    #[cfg(unix)]
    pub fn open<S: AsRef<Path>>(path: S) -> io::Result<Self> {
        Handle::open(path).map(|x| Self(x))
    }

    /// Load a symbol.
    ///
    /// ## Platform-specific behavior
    ///
    /// - On Unix uses `dlsym`.
    #[cfg(unix)]
    pub fn load(&self, sym: &str) -> *mut c_void {
        use std::{ffi::CStr, ptr::null_mut};
        use crate::{ffi::str::str_to_c_str, os::unix::path::PATH_MAX};

        // Not impossible, but highly doubtful that your symbol is that long.
        let mut tmp = [0; PATH_MAX];
        let Some(cstr) = str_to_c_str(sym, &mut tmp) else { return null_mut() };
        let cstr = cstr.len();
        let cstr = CStr::from_bytes_with_nul(&tmp[..cstr]).unwrap();
        self.0.get(cstr)
    }
    
    pub fn into_raw_handle(self) -> Handle {
        self.0
    }
}

pub struct LibraryFile(
    #[cfg(unix)]
    crate::os::unix::dl::LibraryFile,
);
impl LibraryFile {
    pub fn open<S: AsRef<Path>>(path: S) -> io::Result<Self> {
        #[cfg(unix)]
        return crate::os::unix::dl::LibraryFile::open(path).map(|x| Self(x));

        #[allow(unreachable_code)]
        Err(io::Error::new(io::ErrorKind::Unsupported, "not supported on this platform"))
    }

    pub fn section_names(&self) -> SectionNamesIter<'_> {
        #[cfg(unix)]
        return self.0.sections();

        #[cfg(not(unix))]
        {
            static O: &[&std::ffi::OsStr] = &[];
            O.iter()
        }
    }

    pub fn section_data_by_name<S: AsRef<[u8]>>(&self, name: S) -> Option<&[u8]> {
        #[cfg(unix)]
        return self.0.section_data_by_name(name);

        #[allow(unreachable_code)]
        None
    }
}
