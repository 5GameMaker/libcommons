pub use crate::r#gen::RUSTC_VERSION_STRING;
pub use crate::r#gen::RUSTC_VERSION_WITH_TAG;
pub use crate::r#gen::RUSTC_VERSION_TAG;
pub use crate::r#gen::RUSTC_VERSION_ONLY;
pub use crate::r#gen::RUSTC_VERSION_MAJOR;
pub use crate::r#gen::RUSTC_VERSION_MINOR;
pub use crate::r#gen::RUSTC_VERSION_PATCH;
use crate::util::same_bytes;

/// Check if running on nightly.
pub const fn is_nightly() -> bool {
    same_bytes(RUSTC_VERSION_TAG.as_bytes(), b"nightly")
}
