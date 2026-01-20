use serde::Serialize;

use super::partial::HProjEffectInfoPartial;
use crate::info::item::extended::HItemExtendedInfo;

#[derive(Serialize)]
pub(crate) struct HProjEffectInfoFull {
    #[serde(flatten)]
    partial_info: HProjEffectInfoPartial,
    #[serde(flatten)]
    extended_info: HItemExtendedInfo,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HProjEffectInfoFull {
    pub(super) fn from_core(core_proj_effect: &mut rc::ProjEffectMut) -> Self {
        Self {
            partial_info: HProjEffectInfoPartial::from_core(core_proj_effect),
            extended_info: HItemExtendedInfo::from_core(core_proj_effect),
        }
    }
}
