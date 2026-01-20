use serde::Serialize;

use super::partial::HBoosterInfoPartial;
use crate::info::item::extended::HItemExtendedInfo;

#[derive(Serialize)]
pub(crate) struct HBoosterInfoFull {
    #[serde(flatten)]
    partial_info: HBoosterInfoPartial,
    #[serde(flatten)]
    extended_info: HItemExtendedInfo,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HBoosterInfoFull {
    pub(super) fn from_core(core_booster: &mut rc::BoosterMut) -> Self {
        Self {
            partial_info: HBoosterInfoPartial::from_core(core_booster),
            extended_info: HItemExtendedInfo::from_core(core_booster),
        }
    }
}
