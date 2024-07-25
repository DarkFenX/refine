use full::HAutoChargeInfoFull;
use id::HAutoChargeInfoId;
use partial::HAutoChargeInfoPartial;

use crate::info::HItemInfoMode;

mod full;
mod id;
mod partial;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HAutoChargeInfo {
    Id(HAutoChargeInfoId),
    Partial(HAutoChargeInfoPartial),
    Full(HAutoChargeInfoFull),
}
impl HAutoChargeInfo {
    pub(crate) fn mk_info(
        core_sol: &mut rc::SolarSystem,
        core_auto_charge_info: &rc::SolAutoChargeInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(core_auto_charge_info.into()),
            HItemInfoMode::Partial => Self::Partial(core_auto_charge_info.into()),
            HItemInfoMode::Full => Self::Full(HAutoChargeInfoFull::mk_info(core_sol, core_auto_charge_info)),
        }
    }
}
