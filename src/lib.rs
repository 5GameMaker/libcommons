#![allow(incomplete_features)]
#![cfg_attr(
    all(feature = "matrix", feature = "nightly"),
    feature(
        generic_const_exprs,
        generic_arg_infer,
        maybe_uninit_array_assume_init,
        array_try_map,
    )
)]

#[cfg(any(feature = "dirs", feature = "lock"))]
pub mod fs;
#[cfg(feature = "io")]
pub mod io;
#[cfg(all(feature = "matrix", feature = "nightly"))]
pub mod matrix;
#[cfg(any(feature = "dirs", feature = "lock"))]
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
    #[cfg(feature = "str")]
    pub use crate::str::AsUtf8;
    #[cfg(feature = "extra_traits")]
    pub use crate::util::Fun;
    #[cfg(feature = "result")]
    pub use crate::util::{K, Result};
}
