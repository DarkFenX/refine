use full::HFwEffectInfoFull;
use id::HFwEffectInfoId;
use partial::HFwEffectInfoPartial;

use crate::info::HItemInfoMode;

mod full;
mod id;
mod partial;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HFwEffectInfo {
    Id(HFwEffectInfoId),
    Partial(HFwEffectInfoPartial),
    Full(HFwEffectInfoFull),
}
impl HFwEffectInfo {
    pub(crate) fn mk_info(
        core_ss: &mut rc::SolarSystem,
        core_fw_effect_info: &rc::SsFwEffectInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(core_fw_effect_info.into()),
            HItemInfoMode::Partial => Self::Partial(core_fw_effect_info.into()),
            HItemInfoMode::Full => Self::Full(HFwEffectInfoFull::mk_info(core_ss, core_fw_effect_info)),
        }
    }
}
