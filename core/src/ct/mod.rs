//! Cached types.
//!
//! Cached types are built from the EVE data provided by a data handler and are optimized for ease
//! of use by various REEFAST components. The cached types are assumed to be read-only by the
//! components, anything mutable is built on top of them.

pub use abil::FighterAbil;
pub use attr::Attr;
pub use buff::{Buff, BuffAttrMod};
pub use effect::{AttrMod, Effect};
pub use item::{FighterAbilData, Item};
pub use muta::{Muta, MutaAttrRange};

mod abil;
mod attr;
mod buff;
mod effect;
mod item;
mod muta;
