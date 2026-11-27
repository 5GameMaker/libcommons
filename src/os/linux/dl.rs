use std::{ffi::{CStr, OsStr, c_void}, fs, io, os::unix::ffi::OsStrExt, path::Path, slice};

use libc::{Elf32_Ehdr, Elf32_Shdr, Elf64_Ehdr, Elf64_Shdr};

use crate::{ffi::str::StrKind, os::unix::path::PathExt, util::{Fun, SliceExt}};

/// A handle for an opened dynamic library.
#[repr(transparent)]
pub struct Handle(*mut c_void);
impl Handle {
    pub fn as_raw_ptr(&self) -> *mut c_void { self.0 }
    pub fn open<S: AsRef<Path>>(path: S) -> io::Result<Self> { unsafe {
        let Some(path) = path.as_ref().as_c_str_byte_array() else {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "path is too long"));
        };
        let ptr = libc::dlopen(path.as_ptr(), libc::RTLD_NOW);
        if ptr.is_null() {
            return Err(io::Error::last_os_error());
        }
        Ok(Self(ptr))
    } }
    pub fn get(&self, sym: &CStr) -> *mut c_void { unsafe {
        libc::dlsym(self.0, sym.as_ptr())
    } }

    /// Obtain a versioned symbol.
    ///
    /// ## Platform-specific behavior
    ///
    /// This only ever calls to anything if [dlvsym][libc::dlvsym] is available.
    /// Otherwise returns `null`.
    pub fn get_versioned(&self, sym: &CStr, ver: &CStr) -> *mut c_void {
        #[cfg(target_env = "gnu")]
        unsafe {
            return libc::dlvsym(self.0, sym.as_ptr(), ver.as_ptr())
        }
        #[cfg(not(target_env = "gnu"))]
        {
            use std::ptr::null_mut;

            _ = (sym, ver);
            #[allow(unreachable_code)]
            null_mut()
        }
    }
}
impl Drop for Handle {
    fn drop(&mut self) { unsafe {
        libc::dlclose(self.0);
    } }
}

pub struct ShdrWithData64<'a> {
    pub shdr: &'a Elf64_Shdr,
    pub data: &'a [u8],
}

pub struct Library64 {
    backing: Vec<u8>,
}
impl Library64 {
    pub fn data(&self) -> &[u8] { &self.backing }

    pub fn ehdr(&self) -> &Elf64_Ehdr { unsafe {
        self.backing.as_ptr().cast::<Elf64_Ehdr>().as_ref().unwrap_unchecked()
    } }

    pub fn shdrs(&self) -> &[Elf64_Shdr] { unsafe {
        let ehdr = self.ehdr();
        slice::from_raw_parts(
            self.backing[ehdr.e_shoff as usize..].as_ptr().cast(),
            ehdr.e_shnum as usize)
    } }

    pub fn shdr_data(&self, idx: u16) -> Option<&[u8]> {
        let shdr = self.shdrs().get(idx as usize)?;
        Some(&self.backing[shdr.sh_offset as usize..][..shdr.sh_size as usize])
    }

    pub fn shdr_name(&self, idx: u16) -> Option<&CStr> { unsafe {
        let shdr = self.shdrs().get(idx as usize)?;
        Some(CStr::from_ptr(self.strtable()[shdr.sh_name as usize..].as_ptr().cast()))
    } }

    pub fn shdr_by_name<S: AsRef<[u8]>>(&self, name: S) -> Option<ShdrWithData64<'_>> {
        let name = name.as_ref();
        self.shdrs().iter()
            .enumerate()
            .find(|x| unsafe {
                self.shdr_name(x.0 as u16).unwrap_unchecked().as_bytes() == name
            })
            .map(|x| ShdrWithData64 {
                shdr: x.1,
                data: unsafe { self.shdr_data(x.0 as u16).unwrap_unchecked() },
            })
    }

    pub fn strtable(&self) -> &[u8] { unsafe {
        // Safety: e_shstrndx is always valid.
        self.shdr_data(self.ehdr().e_shstrndx).unwrap_unchecked()
    } }

    pub(self) fn new(backing: Vec<u8>) -> Self { Self { backing } }
}

pub struct ShdrWithData32<'a> {
    pub shdr: &'a Elf32_Shdr,
    pub data: &'a [u8],
}

pub struct Library32 {
    backing: Vec<u8>,
}
impl Library32 {
    pub fn data(&self) -> &[u8] { &self.backing }

    pub fn ehdr(&self) -> &Elf32_Ehdr { unsafe {
        self.backing.as_ptr().cast::<Elf32_Ehdr>().as_ref().unwrap_unchecked()
    } }

    pub fn shdrs(&self) -> &[Elf32_Shdr] { unsafe {
        let ehdr = self.ehdr();
        slice::from_raw_parts(
            self.backing[ehdr.e_shoff as usize..].as_ptr().cast(),
            ehdr.e_shnum as usize)
    } }

    pub fn shdr_data(&self, idx: u16) -> Option<&[u8]> {
        let shdr = self.shdrs().get(idx as usize)?;
        Some(&self.backing[shdr.sh_offset as usize..][..shdr.sh_size as usize])
    }

    pub fn shdr_name(&self, idx: u16) -> Option<&CStr> { unsafe {
        let shdr = self.shdrs().get(idx as usize)?;
        Some(CStr::from_ptr(self.strtable()[shdr.sh_name as usize..].as_ptr().cast()))
    } }

    pub fn shdr_by_name<S: AsRef<[u8]>>(&self, name: S) -> Option<ShdrWithData32<'_>> {
        let name = name.as_ref();
        self.shdrs().iter()
            .enumerate()
            .find(|x| unsafe {
                self.shdr_name(x.0 as u16).unwrap_unchecked().as_bytes() == name
            })
            .map(|x| ShdrWithData32 {
                shdr: x.1,
                data: unsafe { self.shdr_data(x.0 as u16).unwrap_unchecked() },
            })
    }

    pub fn strtable(&self) -> &[u8] { unsafe {
        // Safety: e_shstrndx is always valid.
        self.shdr_data(self.ehdr().e_shstrndx).unwrap_unchecked()
    } }

    pub(self) fn new(backing: Vec<u8>) -> Self { Self { backing } }
}

enum SectionNamesIterI<'a> {
    E32 {
        lib: &'a Library32,
        idx: usize,
    },
    E64 {
        lib: &'a Library64,
        idx: usize,
    },
}
pub struct SectionNamesIter<'a>(SectionNamesIterI<'a>);
impl<'a> Iterator for SectionNamesIter<'a> {
    type Item = &'a OsStr;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            SectionNamesIterI::E32 { lib, idx } => {
                if *idx == lib.shdrs().len() { return None }
                let name = lib.shdr_name(*idx as u16);
                *idx += 1;
                name.map(|x| OsStr::from_bytes(x.to_bytes()))
            }
            SectionNamesIterI::E64 { lib, idx } => {
                if *idx == lib.shdrs().len() { return None }
                let name = lib.shdr_name(*idx as u16);
                *idx += 1;
                name.map(|x| OsStr::from_bytes(x.to_bytes()))
            }
        }
    }
}

pub enum LibraryFile {
    E32(Library32),
    E64(Library64),
}
impl LibraryFile {
    pub fn open<S: AsRef<Path>>(path: S) -> io::Result<Self> { unsafe {
        let data = fs::read(path)?;

        macro_rules! notelf {
            () => {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "not an elf file"))
            };
        }

        if !data.starts_with(b"\x7fELF") { notelf!() }
        if data.len() < size_of::<Elf64_Ehdr>() { notelf!() }

        let ehdr = data.as_ptr().cast::<Elf64_Ehdr>().as_ref().unwrap_unchecked();
        match ehdr.e_ident[libc::EI_CLASS] {
            libc::ELFCLASS32 => {
                if data.len() < size_of::<Elf32_Ehdr>() { notelf!() }

                let lib = Library32::new(data);

                if lib.ehdr().e_shentsize as usize != size_of::<Elf32_Shdr>() { notelf!() }
                let Ok(shoff) = TryInto::<usize>::try_into(lib.ehdr().e_shoff) else { notelf!() };
                if shoff >= lib.data().len() { notelf!() }
                if !lib.ehdr().e_shoff.is_multiple_of(align_of::<Elf32_Shdr>() as u32) { notelf!() }
                let shdrs = lib.shdrs();
                let shdrs_bytes = shdrs.as_u8_slice();
                if lib.data().len() < lib.shdrs().as_ptr().addr() - (lib.ehdr() as *const Elf32_Ehdr).addr()
                    + shdrs_bytes.len() { notelf!() }
                if lib.ehdr().e_shstrndx >= lib.ehdr().e_shnum { notelf!() }

                for shdr in lib.shdrs() {
                    let Ok(off) = TryInto::<usize>::try_into(shdr.sh_offset) else { notelf!() };
                    let Ok(len) = TryInto::<usize>::try_into(shdr.sh_size) else { notelf!() };

                    let Some(end) = off.checked_add(len) else { notelf!() };

                    if lib.data().len() < end { notelf!() }
                }

                let strtable = lib.strtable();

                for shdr in lib.shdrs() {
                    let idx = shdr.sh_name as usize;
                    let Some(end) = strtable[idx..].iter().enumerate()
                        .filter(|x| *x.1 == 0)
                        .map(|x| x.0.checked_add(idx))
                        .next()
                        .flatten() else { notelf!() };
                    if end >= strtable.len() { notelf!() }
                }

                Self::E32(lib).instead_map(Ok)
            }
            libc::ELFCLASS64 => {
                let lib = Library64::new(data);

                if lib.ehdr().e_shentsize as usize != size_of::<Elf64_Shdr>() { notelf!() }
                let Ok(shoff) = TryInto::<usize>::try_into(lib.ehdr().e_shoff) else { notelf!() };
                if shoff >= lib.data().len() { notelf!() }
                if !lib.ehdr().e_shoff.is_multiple_of(align_of::<Elf64_Shdr>() as u64) { notelf!() }
                let shdrs = lib.shdrs();
                let shdrs_bytes = shdrs.as_u8_slice();
                if lib.data().len() < lib.shdrs().as_ptr().addr() - (lib.ehdr() as *const Elf64_Ehdr).addr()
                    + shdrs_bytes.len() { notelf!() }
                if lib.ehdr().e_shstrndx >= lib.ehdr().e_shnum { notelf!() }

                for shdr in lib.shdrs() {
                    let Ok(off) = TryInto::<usize>::try_into(shdr.sh_offset) else { notelf!() };
                    let Ok(len) = TryInto::<usize>::try_into(shdr.sh_size) else { notelf!() };

                    let Some(end) = off.checked_add(len) else { notelf!() };

                    if lib.data().len() < end { notelf!() }
                }

                let strtable = lib.strtable();

                for shdr in lib.shdrs() {
                    let idx = shdr.sh_name as usize;
                    let Some(end) = strtable[idx..].iter().enumerate()
                        .filter(|x| *x.1 == 0)
                        .map(|x| x.0.checked_add(idx))
                        .next()
                        .flatten() else { notelf!() };
                    if end >= strtable.len() { notelf!() }
                }

                Self::E64(lib).instead_map(Ok)
            }
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "invalid class")),
        }
    } }
    
    pub fn sections(&self) -> SectionNamesIter<'_> {
        match self {
            LibraryFile::E32(x) => SectionNamesIter(SectionNamesIterI::E32 { lib: x, idx: 0 }),
            LibraryFile::E64(x) => SectionNamesIter(SectionNamesIterI::E64 { lib: x, idx: 0 }),
        }
    }

    pub fn section_data_by_name<S: AsRef<[u8]>>(&self, name: S) -> Option<&[u8]> {
        match self {
            LibraryFile::E32(x) => x.shdr_by_name(name).map(|x| x.data),
            LibraryFile::E64(x) => x.shdr_by_name(name).map(|x| x.data),
        }
    }
}
