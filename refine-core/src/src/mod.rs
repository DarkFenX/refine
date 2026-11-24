pub use item::SrcItem;
pub use origin::{SrcOrigin, SrcOriginGeneratedReason};
pub use warnings::SrcWarnings;

mod item;
mod origin;
#[expect(clippy::module_inception)]
pub(crate) mod src;
mod src_get_mutated_attr_roll;
mod src_new;
mod warnings;

pub mod err {
    pub use crate::src::src_new::SrcInitError;
}
