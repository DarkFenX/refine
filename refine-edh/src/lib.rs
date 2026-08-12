#![cfg_attr(
    any(feature = "phb-fs", feature = "phb-http", feature = "sde-fs", feature = "sde-http"),
    feature(integer_casts)
)]
#![cfg_attr(any(feature = "sde-fs", feature = "sde-http"), feature(default_field_values))]

//! EVE data handlers which use static data export and convert it into EVE data model as defined in
//! `refine-core`.
//!
//! ## Feature flags
//!
//! This library provides different data handlers, with their availability controlled via feature
//! flags.
//!
//! - `phb-fs`: Enables handler which reads Phobos data export from filesystem.
//! - `phb-http`: Enables handler which fetches Phobos data export over HTTP.
//! - `sde-fs`: Enables handler which reads FC-produced SDE from filesystem.
//! - `sde-http`: Enables handler which fetches FC-produced SDE over HTTP.

#[cfg(feature = "phb-fs")]
pub use phb::PhbFsEdh;
#[cfg(feature = "phb-http")]
pub use phb::PhbHttpEdh;
#[cfg(feature = "sde-fs")]
pub use sde::SdeFsEdh;
#[cfg(feature = "sde-http")]
pub use sde::SdeHttpEdh;

#[cfg(any(feature = "phb-fs", feature = "phb-http"))]
mod phb;
#[cfg(any(feature = "sde-fs", feature = "sde-http"))]
mod sde;
#[cfg(any(feature = "phb-fs", feature = "phb-http", feature = "sde-fs", feature = "sde-http"))]
mod util;

pub mod err {
    #[cfg(feature = "phb-fs")]
    pub use crate::phb::PhbFsEdhError;
    #[cfg(feature = "phb-http")]
    pub use crate::phb::{PhbHttpEdhError, PhbHttpEdhInitError};
    #[cfg(feature = "sde-fs")]
    pub use crate::sde::SdeFsEdhError;
    #[cfg(feature = "sde-http")]
    pub use crate::sde::{SdeHttpEdhError, SdeHttpEdhInitError};
}
