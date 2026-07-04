pub use info::{SrcInfo, SrcOrigin, SrcOriginGeneratedReason, SrcWarnings};
pub use src::Src;

pub(crate) mod error;
mod info;
mod src;
