use full::HAutochargeInfoFull;
use id::HAutochargeInfoId;
use partial::HAutochargeInfoPartial;

use crate::info::HItemInfoMode;

mod full;
mod id;
mod partial;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HAutochargeInfo {
    Id(HAutochargeInfoId),
    Partial(HAutochargeInfoPartial),
    Full(HAutochargeInfoFull),
}
impl HAutochargeInfo {
    pub(crate) fn mk_info(
        core_sol: &mut rc::SolarSystem,
        core_autocharge_info: &rc::AutochargeInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(core_autocharge_info.into()),
            HItemInfoMode::Partial => Self::Partial(core_autocharge_info.into()),
            HItemInfoMode::Full => Self::Full(HAutochargeInfoFull::mk_info(core_sol, core_autocharge_info)),
        }
    }
}
