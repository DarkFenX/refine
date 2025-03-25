use full::HChargeInfoFull;
use id::HChargeInfoId;
use partial::HChargeInfoPartial;

use crate::info::HItemInfoMode;

mod full;
mod id;
mod partial;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HChargeInfo {
    Id(HChargeInfoId),
    Partial(HChargeInfoPartial),
    Full(HChargeInfoFull),
}
impl HChargeInfo {
    pub(crate) fn mk_info(
        core_sol: &mut rc::SolarSystem,
        core_charge_info: &rc::ChargeInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(core_charge_info.into()),
            HItemInfoMode::Partial => Self::Partial(core_charge_info.into()),
            HItemInfoMode::Full => Self::Full(HChargeInfoFull::mk_info(core_sol, core_charge_info)),
        }
    }
}
