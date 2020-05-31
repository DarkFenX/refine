//! Contains enums and constants used throughout the crate.

pub(crate) use eve::{attrs, get_abil_effect, itemcats, itemgrps, units};
pub use ree::{ItemType, ModAfeeFilter, ModAggrMode, ModDomain, ModOp, State, TgtMode};

mod eve;
mod ree;
