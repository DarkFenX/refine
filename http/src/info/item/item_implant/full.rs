use serde::Serialize;

use super::partial::HImplantInfoPartial;
use crate::info::item::extended::HItemExtendedInfo;

#[derive(Serialize)]
pub(crate) struct HImplantInfoFull {
    #[serde(flatten)]
    partial_info: HImplantInfoPartial,
    #[serde(flatten)]
    extended_info: HItemExtendedInfo,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HImplantInfoFull {
    pub(super) fn from_core(core_implant: &mut rc::ImplantMut) -> Self {
        Self {
            partial_info: HImplantInfoPartial::from_core(core_implant),
            extended_info: HItemExtendedInfo::from_core(core_implant),
        }
    }
}
