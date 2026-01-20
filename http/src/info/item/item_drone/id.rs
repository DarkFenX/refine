use rc::ItemCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HDroneInfoId {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::ItemId,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HDroneInfoId {
    pub(super) fn from_core(core_drone: &mut rc::DroneMut) -> Self {
        Self {
            id: core_drone.get_item_id(),
        }
    }
}
