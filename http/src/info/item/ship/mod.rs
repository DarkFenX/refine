use full::HShipInfoFull;
use id::HShipInfoId;
use partial::HShipInfoPartial;

use crate::info::HItemInfoMode;

mod full;
mod id;
mod partial;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HShipInfo {
    Id(HShipInfoId),
    Partial(HShipInfoPartial),
    Full(HShipInfoFull),
}
impl HShipInfo {
    pub(crate) fn mk_info(
        core_ss: &mut rc::SolarSystem,
        core_ship_info: &rc::SsShipInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(core_ship_info.into()),
            HItemInfoMode::Partial => Self::Partial(core_ship_info.into()),
            HItemInfoMode::Full => Self::Full(HShipInfoFull::mk_info(core_ss, core_ship_info)),
        }
    }
}
