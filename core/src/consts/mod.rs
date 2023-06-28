//! Contains enums and constants used throughout the crate.

pub(crate) use eve::{attrs, effcats, effects, get_abil_effect, itemcats, itemgrps, units};
pub(crate) use ree::DEFAULT_EFFECT_MODE;
pub use ree::{
    EffectMode, ItemType, ModAfeeFilter, ModAggrMode, ModBuildStatus, ModDomain, ModOp, ModRack, OrdAddMode, OrdRmMode,
    State, TgtMode,
};

mod eve;
mod ree;
