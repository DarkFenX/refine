//! Adapted data cachers for `refine-core`. They implement persistence for adapted data.
//!
//! ## Feature flags
//!
//! - `json-zfile`: Enables cacher which writes compressed JSON cache to disk.
//! - `postcard-zfile`: Enables cacher which writes compressed postcard cache to disk.

#[cfg(feature = "json-zfile")]
pub use json_zfile::JsonZfileAdc;
#[cfg(feature = "postcard-zfile")]
pub use postcard_zfile::PostcardZfileAdc;

#[cfg(feature = "json-zfile")]
mod json_zfile;
#[cfg(feature = "postcard-zfile")]
mod postcard_zfile;

pub mod err {
    #[cfg(feature = "json-zfile")]
    pub use crate::json_zfile::{JsonZfileAdcDataReadError, JsonZfileAdcFpReadError, JsonZfileAdcWriteError};
    #[cfg(feature = "postcard-zfile")]
    pub use crate::postcard_zfile::{
        PostcardZfileAdcDataReadError, PostcardZfileAdcFpReadError, PostcardZfileAdcWriteError,
    };
}

const VERSION: &str = env!("CARGO_PKG_VERSION");
