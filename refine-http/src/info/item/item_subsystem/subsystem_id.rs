use rc::ItemCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HSubsystemInfoId {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::ItemId,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HSubsystemInfoId {
    pub(super) fn from_core(core_subsystem: &mut rc::SubsystemMut) -> Self {
        Self {
            id: core_subsystem.get_item_id(),
        }
    }
}
