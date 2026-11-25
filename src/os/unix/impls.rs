#[cfg(feature = "dl")]
use crate::util::Zeroable;

#[cfg(feature = "dl")]
macro_rules! impl_zeroable {
    ($($ty:ty),* $(,)?) => {$(
        unsafe impl Zeroable for $ty {}
    )*};
}
#[cfg(feature = "dl")]
impl_zeroable! {
    libc::Elf64_Ehdr,
    libc::Elf64_Chdr,
    libc::Elf64_Phdr,
    libc::Elf64_Shdr,
}
