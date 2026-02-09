use rc::ItemCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HRigInfoId {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::ItemId,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HRigInfoId {
    pub(super) fn from_core(core_rig: &mut rc::RigMut) -> Self {
        Self {
            id: core_rig.get_item_id(),
        }
    }
}
