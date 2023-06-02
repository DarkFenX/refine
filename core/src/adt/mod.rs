//! EVE reefast types.
//!
//! EVE reefast types are built from the EVE data provided by an EVE data handler and are optimized
//! for ease of use by various reefast components. The EVE reefast types are assumed to be read-only
//! by the components, anything mutable is built on top of them.

pub use attr::AAttr;
pub use buff::{ABuff, ABuffAttrMod};
pub use effect::{AAttrMod, AEffect};
pub use item::{AItem, AItemEffData};
pub use muta::{AMuta, AMutaAttrRange};

mod attr;
mod buff;
mod effect;
mod item;
mod muta;
