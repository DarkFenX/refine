//! Contains enums and constants used throughout the crate.

pub(crate) use eve::{attrs, effcats, effects, get_abil_effect, itemcats, itemgrps, units};
pub use ree::{ErrorKind, ItemType, ModAfeeFilter, ModAggrMode, ModBuildStatus, ModDomain, ModOp, State, TgtMode};

mod eve;
mod ree;
