use serde::Serialize;

use super::partial::HSwEffectInfoPartial;
use crate::info::item::extended::HItemExtendedInfo;

#[derive(Serialize)]
pub(crate) struct HSwEffectInfoFull {
    #[serde(flatten)]
    partial_info: HSwEffectInfoPartial,
    #[serde(flatten)]
    extended_info: HItemExtendedInfo,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HSwEffectInfoFull {
    pub(super) fn from_core(core_sw_effect: &mut rc::SwEffectMut) -> Self {
        Self {
            partial_info: HSwEffectInfoPartial::from_core(core_sw_effect),
            extended_info: HItemExtendedInfo::from_core(core_sw_effect),
        }
    }
}
