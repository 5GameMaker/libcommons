//! # Libcommons
//!
//! Utilities I don't want to write again.

#![allow(incomplete_features)]
#![cfg_attr(
    all(feature = "nightly", feature = "matrix", not(libcommons_rustc_min_1 = "100")),
    feature(generic_const_items, maybe_uninit_array_assume_init)
)]
#![cfg_attr(
    all(feature = "ffi", libcommons_rustc_nightly = "y"),
    feature(fn_traits, unboxed_closures)
)]

mod r#gen {
    include!(concat!(env!("OUT_DIR"), "/gen.rs"));
}

#[cfg(feature = "ffi")]
pub mod ffi;
#[cfg(feature = "dirs")]
pub mod fs;
#[cfg(feature = "io")]
pub mod io;
#[cfg(feature = "iter")]
pub mod iter;
pub mod os;
#[cfg(feature = "str")]
pub mod str;
#[cfg(feature = "meta")]
pub mod meta;
pub mod util;
#[cfg(feature = "validity")]
pub mod validity;
#[cfg(feature = "dl")]
pub mod dl;

// All the cfg's for matrix, a feature that doesn't even work anymore.
#[cfg(feature = "matrix")]
#[cfg(not(libcommons_rustc_min_1 = "100"))]
#[cfg(not(feature = "nightly"))]
compile_error!("'matrix' is a nigthly feature and requires 'nightly' to be enabled");
#[cfg(feature = "matrix")]
#[cfg(not(libcommons_rustc_min_1 = "100"))]
#[cfg(feature = "nightly")]
pub mod matrix;
#[cfg(feature = "matrix")]
#[cfg(libcommons_rustc_min_1 = "100")]
compile_error!("'matrix' feature is broken on Rust >=1.100 due to new trait solver not supporting 'generic_const_items' feature");

#[cfg(all(feature = "nightly", not(libcommons_rustc_nightly = "y")))]
compile_error!("'nightly' feature can only be enabled when compiling on nightly");

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
