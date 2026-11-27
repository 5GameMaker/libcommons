#[cfg(target_os = "linux")]
pub mod elf;

#[cfg(target_os = "linux")]
pub type Handle = elf::Handle;
#[cfg(not(target_os = "linux"))]
pub type Handle = !;

#[cfg(target_os = "linux")]
pub type ShdrWithData64<'a> = elf::ShdrWithData64<'a>;
#[cfg(not(target_os = "linux"))]
pub type ShdrWithData64<'a> = &'a !;

#[cfg(target_os = "linux")]
pub type ShdrWithData32<'a> = elf::ShdrWithData32<'a>;
#[cfg(not(target_os = "linux"))]
pub type ShdrWithData32<'a> = &'a !;

#[cfg(target_os = "linux")]
pub type Library64 = elf::Library64;
#[cfg(not(target_os = "linux"))]
pub type Library64 = !;

#[cfg(target_os = "linux")]
pub type Library32 = elf::Library32;
#[cfg(not(target_os = "linux"))]
pub type Library32 = !;

#[cfg(target_os = "linux")]
pub type LibraryFile = elf::LibraryFile;
#[cfg(not(target_os = "linux"))]
pub type LibraryFile = !;

#[cfg(target_os = "linux")]
pub type SectionNamesIter<'a> = elf::SectionNamesIter<'a>;
#[cfg(not(target_os = "linux"))]
pub type SectionNamesIter<'a> = &'a !;
