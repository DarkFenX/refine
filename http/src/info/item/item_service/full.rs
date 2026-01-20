use serde::Serialize;

use super::partial::HServiceInfoPartial;
use crate::info::item::extended::HItemExtendedInfo;

#[derive(Serialize)]
pub(crate) struct HServiceInfoFull {
    #[serde(flatten)]
    partial_info: HServiceInfoPartial,
    #[serde(flatten)]
    extended_info: HItemExtendedInfo,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HServiceInfoFull {
    pub(super) fn from_core(core_service: &mut rc::ServiceMut) -> Self {
        Self {
            partial_info: HServiceInfoPartial::from_core(core_service),
            extended_info: HItemExtendedInfo::from_core(core_service),
        }
    }
}
