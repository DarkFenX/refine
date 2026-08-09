#[cfg(feature = "phb-fs")]
pub use handler_fs::{PhbFsEdh, PhbFsEdhError};
#[cfg(feature = "phb-http")]
pub use handler_http::{PhbHttpEdh, PhbHttpEdhError, PhbHttpEdhInitError};

#[cfg(any(feature = "phb-fs", feature = "phb-http"))]
mod data;
#[cfg(feature = "phb-fs")]
mod handler_fs;
#[cfg(feature = "phb-http")]
mod handler_http;
#[cfg(any(feature = "phb-fs", feature = "phb-http"))]
mod parsing;
#[cfg(any(feature = "phb-fs", feature = "phb-http"))]
mod serde_custom;
