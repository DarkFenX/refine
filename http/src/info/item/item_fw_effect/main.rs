use serde::Serialize;

use super::{full::HFwEffectInfoFull, id::HFwEffectInfoId, partial::HFwEffectInfoPartial};
use crate::info::HItemInfoMode;

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
