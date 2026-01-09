use full::HFwEffectInfoFull;
use id::HFwEffectInfoId;
use partial::HFwEffectInfoPartial;
use serde::Serialize;

use crate::info::HItemInfoMode;

mod full;
mod id;
mod partial;

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum HFwEffectInfo {
    Id(HFwEffectInfoId),
    Partial(HFwEffectInfoPartial),
    Full(HFwEffectInfoFull),
}
impl HFwEffectInfo {
    pub(in crate::info::item) fn mk_info(core_fw_effect: &mut rc::FwEffectMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(core_fw_effect.into()),
            HItemInfoMode::Partial => Self::Partial(core_fw_effect.into()),
            HItemInfoMode::Full => Self::Full(core_fw_effect.into()),
        }
    }
}
