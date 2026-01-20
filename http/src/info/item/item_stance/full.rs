use serde::Serialize;

use super::partial::HStanceInfoPartial;
use crate::info::item::extended::HItemExtendedInfo;

#[derive(Serialize)]
pub(crate) struct HStanceInfoFull {
    #[serde(flatten)]
    partial_info: HStanceInfoPartial,
    #[serde(flatten)]
    extended_info: HItemExtendedInfo,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HStanceInfoFull {
    pub(super) fn from_core(core_stance: &mut rc::StanceMut) -> Self {
        Self {
            partial_info: HStanceInfoPartial::from_core(core_stance),
            extended_info: HItemExtendedInfo::from_core(core_stance),
        }
    }
}
