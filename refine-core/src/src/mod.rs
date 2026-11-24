pub use info::{SrcInfo, SrcOrigin, SrcOriginGeneratedReason, SrcWarnings};
pub use item::SrcItem;
pub use main::Src;

pub mod err;
mod info;
mod item;
mod main;
