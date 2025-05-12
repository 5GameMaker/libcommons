use std::{
    ffi::c_int,
    fs::File,
    io::{self, Error},
    mem::forget,
    os::fd::AsRawFd,
    path::Path,
    ptr::drop_in_place,
};

pub struct PathLock(File);
impl PathLock {
    pub fn unlock(mut self) -> io::Result<()> {
        unsafe {
            if flock(self.0.as_raw_fd(), LOCK_UN) == -1 {
                return Err(Error::last_os_error());
            }
            drop_in_place(&raw mut self.0);
            forget(self);
            Ok(())
        }
    }
}
impl Drop for PathLock {
    fn drop(&mut self) {
        unsafe {
            if flock(self.0.as_raw_fd(), LOCK_UN) == -1 {
                panic!("Failed to clear lock: {:#}", Error::last_os_error());
            }
        }
    }
}
unsafe impl Send for PathLock {}

/// Exclusive lock.
const LOCK_EX: c_int = 2;
/// Unlock.
const LOCK_UN: c_int = 8;

unsafe extern "C" {
    /// Apply or remove an advisory lock, according to OPERATION,
    /// on the file FD refers to.
    fn flock(fd: c_int, operation: c_int) -> c_int;
}

/// Obtains a lock on path.
///
/// Blocks until a lock is removed.
pub fn lock(path: &Path) -> io::Result<PathLock> {
    unsafe {
        let file = File::create(path)?;
        if flock(file.as_raw_fd(), LOCK_EX) == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(PathLock(file))
        }
    }
}
