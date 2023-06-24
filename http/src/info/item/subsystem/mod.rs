use full::HSubsystemInfoFull;
use id::HSubsystemInfoId;
use partial::HSubsystemInfoPartial;

use crate::info::HItemInfoMode;

mod full;
mod id;
mod partial;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HSubsystemInfo {
    Id(HSubsystemInfoId),
    Partial(HSubsystemInfoPartial),
    Full(HSubsystemInfoFull),
}
impl HSubsystemInfo {
    pub(crate) fn mk_info(
        core_ss: &mut rc::SolarSystem,
        core_subsystem_info: &rc::SsSubsystemInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(core_subsystem_info.into()),
            HItemInfoMode::Partial => Self::Partial(core_subsystem_info.into()),
            HItemInfoMode::Full => Self::Full(HSubsystemInfoFull::mk_info(core_ss, core_subsystem_info)),
        }
    }
}
