#[cfg(any(feature = "dirs", feature = "lock"))]
pub mod fs;
#[cfg(feature = "io")]
pub mod io;
#[cfg(any(feature = "dirs", feature = "lock"))]
pub mod os;
#[cfg(feature = "str")]
pub mod str;
#[cfg(any(feature = "extra_traits", feature = "result"))]
pub mod util;

pub mod prelude {
    #[cfg(feature = "io")]
    pub use crate::io::ReadExt;
    #[cfg(feature = "str")]
    pub use crate::str::AsUtf8;
    #[cfg(feature = "extra_traits")]
    pub use crate::util::Fun;
}
