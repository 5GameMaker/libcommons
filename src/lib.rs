//! # Libcommons
//!
//! Utilities I don't want to write again.

// #![allow(incomplete_features)]
// #![cfg_attr(
//     all(feature = "nightly", feature = "matrix"),
//     feature(generic_const_items, maybe_uninit_array_assume_init, gca_const_items, gca_min_const_items)
// )]

#[cfg(feature = "ffi")]
pub mod ffi;
#[cfg(feature = "dirs")]
pub mod fs;
#[cfg(feature = "io")]
pub mod io;
#[cfg(feature = "iter")]
pub mod iter;
// #[cfg(feature = "matrix")]
// pub mod matrix;
#[cfg(feature = "dirs")]
pub mod os;
#[cfg(feature = "str")]
pub mod str;
#[cfg(any(feature = "extra_traits", feature = "result"))]
pub mod util;

pub mod prelude {
    #[cfg(feature = "io")]
    pub use crate::io::ReadExt;
    #[cfg(feature = "iter")]
    pub use crate::iter::IterExt;
    #[cfg(feature = "str")]
    pub use crate::str::AsUtf8;
    #[cfg(feature = "extra_traits")]
    pub use crate::util::{Fun, ResultExt};
    #[cfg(feature = "result")]
    pub use crate::util::{K, Result};
}
