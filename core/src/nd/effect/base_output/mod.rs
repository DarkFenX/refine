pub(crate) use base_output::NBaseOutputGetter;
pub(crate) use dmg_breacher::NBaseBreacherDmgGetter;
pub(crate) use dmg_normal::NBaseNormalDmgGetter;
pub(crate) use mining::{NBaseMiningGetter, NMiningXargs};
pub(crate) use remote_cap::NBaseRemoteCapGetter;

mod base_output;
mod dmg_breacher;
mod dmg_normal;
mod mining;
mod remote_cap;
mod shared;
