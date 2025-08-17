//! # Libcommons
//!
//! Utilities I don't want to write again.

#![allow(incomplete_features)]
#![cfg_attr(
    feature = "nightly",
    feature(generic_const_exprs, maybe_uninit_array_assume_init, array_try_map)
)]

#[cfg(feature = "ffi")]
pub mod ffi;
#[cfg(feature = "dirs")]
pub mod fs;
#[cfg(feature = "io")]
pub mod io;
#[cfg(feature = "iter")]
pub mod iter;
#[cfg(all(feature = "matrix", feature = "nightly"))]
pub mod matrix;
#[cfg(feature = "dirs")]
pub mod os;
#[cfg(feature = "str")]
pub mod str;
#[cfg(any(feature = "extra_traits", feature = "result"))]
pub mod util;

#[cfg(all(feature = "matrix", not(feature = "nightly")))]
compile_error!("'matrix' feature requires 'nightly'!");

pub mod prelude {
    #[cfg(feature = "io")]
    pub use crate::io::ReadExt;
    #[cfg(feature = "iter")]
    pub use crate::iter::IterExt;
    #[cfg(feature = "str")]
    pub use crate::str::AsUtf8;
    #[cfg(feature = "extra_traits")]
    pub use crate::util::Fun;
    #[cfg(feature = "result")]
    pub use crate::util::{K, Result};
}
