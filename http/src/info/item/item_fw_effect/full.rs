use serde::Serialize;

use super::partial::HFwEffectInfoPartial;
use crate::info::item::extended::HItemExtendedInfo;

#[derive(Serialize)]
pub(crate) struct HFwEffectInfoFull {
    #[serde(flatten)]
    partial_info: HFwEffectInfoPartial,
    #[serde(flatten)]
    extended_info: HItemExtendedInfo,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HFwEffectInfoFull {
    pub(super) fn from_core(core_fw_effect: &mut rc::FwEffectMut) -> Self {
        Self {
            partial_info: HFwEffectInfoPartial::from_core(core_fw_effect),
            extended_info: HItemExtendedInfo::from_core(core_fw_effect),
        }
    }
}
