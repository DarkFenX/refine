use full::HStanceInfoFull;
use id::HStanceInfoId;
use partial::HStanceInfoPartial;

use crate::info::HItemInfoMode;

mod full;
mod id;
mod partial;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HStanceInfo {
    Id(HStanceInfoId),
    Partial(HStanceInfoPartial),
    Full(HStanceInfoFull),
}
impl HStanceInfo {
    pub(crate) fn mk_info(
        core_sol: &mut rc::SolarSystem,
        core_stance_info: &rc::SolStanceInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(core_stance_info.into()),
            HItemInfoMode::Partial => Self::Partial(core_stance_info.into()),
            HItemInfoMode::Full => Self::Full(HStanceInfoFull::mk_info(core_sol, core_stance_info)),
        }
    }
}
