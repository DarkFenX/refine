pub(crate) use breacher::NBreacherOutputGetter;
pub(crate) use dmg::NDmgOutputGetter;
pub(crate) use ecm::NEcmOutputGetter;
pub(crate) use general::NGeneralOutputGetter;
pub(crate) use mining::{NMiningOutputGetter, NMiningXargs};
pub(crate) use output_getter::NOutputGetter;

mod breacher;
mod dmg;
mod ecm;
mod general;
mod mining;
mod output_getter;
