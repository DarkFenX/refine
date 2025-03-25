use full::HRigInfoFull;
use id::HRigInfoId;
use partial::HRigInfoPartial;

use crate::info::HItemInfoMode;

mod full;
mod id;
mod partial;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HRigInfo {
    Id(HRigInfoId),
    Partial(HRigInfoPartial),
    Full(HRigInfoFull),
}
impl HRigInfo {
    pub(crate) fn mk_info(
        core_sol: &mut rc::SolarSystem,
        core_rig_info: &rc::RigInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(core_rig_info.into()),
            HItemInfoMode::Partial => Self::Partial(core_rig_info.into()),
            HItemInfoMode::Full => Self::Full(HRigInfoFull::mk_info(core_sol, core_rig_info)),
        }
    }
}
