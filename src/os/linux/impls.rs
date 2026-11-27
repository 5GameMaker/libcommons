#[cfg(all(feature = "dl", target_os = "linux"))]
use crate::util::Zeroable;

#[cfg(all(feature = "dl", target_os = "linux"))]
macro_rules! impl_zeroable {
    ($($ty:ty),* $(,)?) => {$(
        unsafe impl Zeroable for $ty {}
    )*};
}
#[cfg(all(feature = "dl", target_os = "linux"))]
impl_zeroable! {
    libc::Elf64_Ehdr,
    libc::Elf64_Chdr,
    libc::Elf64_Phdr,
    libc::Elf64_Shdr,
}
