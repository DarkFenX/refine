pub use item::SrcItem;
pub use main::Src;
pub use origin::{SrcOrigin, SrcOriginGeneratedReason};
pub use warnings::SrcWarnings;

pub mod err;
mod item;
mod main;
mod origin;
mod warnings;
