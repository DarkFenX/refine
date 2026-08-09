//! Adapted data cachers for `refine-core`. They implement persistence for adapted data.
//!
//! ## Feature flags
//!
//! - `json-zfs`: Enables cacher which writes compressed JSON cache to disk.
//! - `postcard-zfs`: Enables cacher which writes compressed postcard cache to disk.

#[cfg(feature = "json-zfs")]
pub use json_zfs::JsonZfsAdc;
#[cfg(feature = "postcard-zfs")]
pub use postcard_zfs::PostcardZfsAdc;

#[cfg(feature = "json-zfs")]
mod json_zfs;
#[cfg(feature = "postcard-zfs")]
mod postcard_zfs;

pub mod err {
    #[cfg(feature = "json-zfs")]
    pub use crate::json_zfs::{JsonZfsAdcDataReadError, JsonZfsAdcFpReadError, JsonZfsAdcWriteError};
    #[cfg(feature = "postcard-zfs")]
    pub use crate::postcard_zfs::{PostcardZfsAdcDataReadError, PostcardZfsAdcFpReadError, PostcardZfsAdcWriteError};
}

const VERSION: &str = env!("CARGO_PKG_VERSION");
