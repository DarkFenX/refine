use serde::Serialize;

use super::HFwEffectInfoPartial;
use crate::info::item::extended::HItemExtendedInfo;

#[derive(Serialize)]
pub(crate) struct HFwEffectInfoFull {
    #[serde(flatten)]
    partial_info: HFwEffectInfoPartial,
    #[serde(flatten)]
    extended_info: HItemExtendedInfo,
}
impl From<&mut rc::FwEffectMut<'_>> for HFwEffectInfoFull {
    fn from(core_fw_effect: &mut rc::FwEffectMut) -> Self {
        Self {
            partial_info: core_fw_effect.into(),
            extended_info: core_fw_effect.into(),
        }
    }
}
