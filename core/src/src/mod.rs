//! Data source management.
//!
//! Data sources ease management of EVE static data versions.

pub use mgr::SrcMgr;
pub(crate) use src::Src;

mod mgr;
mod src;
