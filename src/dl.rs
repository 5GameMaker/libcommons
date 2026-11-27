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
    pub fn open<S: AsRef<Path>>(path: S) -> io::Result<Self> {
        #[cfg(unix)]
        { Handle::open(path).map(Self) }
        #[cfg(not(unix))]
        {
            _ = path;
            Err(io::Error::new(io::ErrorKind::Unsupported, "not supported on this platform"))
        }
    }

    /// Load a symbol.
    ///
    /// ## Platform-specific behavior
    ///
    /// - On Unix uses `dlsym`.
    pub fn load(&self, sym: &str) -> *mut c_void {
        #[cfg(unix)]
        {
            use std::{ffi::CStr, ptr::null_mut};
            use crate::{ffi::str::str_to_c_str, os::unix::path::PATH_MAX};

            // Not impossible, but highly doubtful that your symbol is that long.
            let mut tmp = [0; PATH_MAX];
            let Some(cstr) = str_to_c_str(sym, &mut tmp) else { return null_mut() };
            let cstr = cstr.len();
            let cstr = CStr::from_bytes_with_nul(&tmp[..cstr]).unwrap();
            self.0.get(cstr)
        }
        #[cfg(not(unix))]
        {
            _ = sym;
            todo!()
        }
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
        { crate::os::unix::dl::LibraryFile::open(path).map(Self) }
        #[cfg(not(unix))]
        {
            _ = path;
            Err(io::Error::new(io::ErrorKind::Unsupported, "not supported on this platform"))
        }
    }

    pub fn section_names(&self) -> SectionNamesIter<'_> {
        #[cfg(unix)]
        return self.0.sections();

        #[cfg(not(unix))]
        todo!()
    }

    pub fn section_data_by_name<S: AsRef<[u8]>>(&self, name: S) -> Option<&[u8]> {
        #[cfg(unix)]
        return self.0.section_data_by_name(name);

        #[cfg(not(unix))]
        {
            _ = name;
            todo!()
        }
    }
}
