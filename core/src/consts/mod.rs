//! Contains enums and constants used throughout the crate.

pub(crate) use eve::{attrs, effcats, effects, get_abil_effect, itemcats, itemgrps, units};
pub use ree::{
    ModAfeeFilter, ModAggrMode, ModBuildStatus, ModDomain, ModOp, ModRack, ModSrq, OrdAddMode, OrdRmMode, TgtMode,
};

mod eve;
mod ree;
