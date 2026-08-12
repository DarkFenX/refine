#[cfg(feature = "sde-fs")]
pub use handler_fs::{SdeFsEdh, SdeFsEdhError};
#[cfg(feature = "sde-http")]
pub use handler_http::{SdeHttpEdh, SdeHttpEdhError, SdeHttpEdhInitError};

#[cfg(any(feature = "sde-fs", feature = "sde-http"))]
mod data;
#[cfg(feature = "sde-fs")]
mod handler_fs;
#[cfg(feature = "sde-http")]
mod handler_http;
#[cfg(any(feature = "sde-fs", feature = "sde-http"))]
mod parsing;
#[cfg(any(feature = "sde-fs", feature = "sde-http"))]
mod serde_custom;
