use serde::Serialize;

use super::partial::HAutochargeInfoPartial;
use crate::info::item::extended::HItemExtendedInfo;

#[derive(Serialize)]
pub(crate) struct HAutochargeInfoFull {
    #[serde(flatten)]
    partial_info: HAutochargeInfoPartial,
    #[serde(flatten)]
    extended_info: HItemExtendedInfo,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HAutochargeInfoFull {
    pub(super) fn from_core(core_autocharge: &mut rc::AutochargeMut) -> Self {
        Self {
            partial_info: HAutochargeInfoPartial::from_core(core_autocharge),
            extended_info: HItemExtendedInfo::from_core(core_autocharge),
        }
    }
}
