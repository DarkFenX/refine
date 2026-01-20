use serde::Serialize;

use super::partial::HRigInfoPartial;
use crate::info::item::extended::HItemExtendedInfo;

#[derive(Serialize)]
pub(crate) struct HRigInfoFull {
    #[serde(flatten)]
    partial_info: HRigInfoPartial,
    #[serde(flatten)]
    extended_info: HItemExtendedInfo,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HRigInfoFull {
    pub(super) fn from_core(core_rig: &mut rc::RigMut) -> Self {
        Self {
            partial_info: HRigInfoPartial::from_core(core_rig),
            extended_info: HItemExtendedInfo::from_core(core_rig),
        }
    }
}
