pub(crate) use breacher::NEffectBreacherOutputGetter;
pub(crate) use dmg::NEffectDmgOutputGetter;
pub(crate) use ecm::NEffectEcmOutputGetter;
pub(crate) use general::NEffectGeneralOutputGetter;
pub(crate) use mining::{NEffectMiningOutputGetter, NEffectMiningXargs};
pub(crate) use output_getter::NEffectOutputGetter;

mod breacher;
mod dmg;
mod ecm;
mod general;
mod mining;
mod output_getter;
