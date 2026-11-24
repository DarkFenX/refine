pub use item::SrcItem;
pub use origin::{SrcOrigin, SrcOriginGeneratedReason};
pub use warnings::SrcWarnings;

pub mod err;
mod item;
pub(crate) mod main;
mod origin;
mod warnings;
