pub use info::{SrcInfo, SrcOrigin, SrcOriginGeneratedReason, SrcWarnings};
pub use main::Src;

pub(crate) mod error;
mod info;
mod main;
