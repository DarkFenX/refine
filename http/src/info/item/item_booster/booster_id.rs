use rc::ItemCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HBoosterInfoId {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::ItemId,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HBoosterInfoId {
    pub(super) fn from_core(core_booster: &mut rc::BoosterMut) -> Self {
        Self {
            id: core_booster.get_item_id(),
        }
    }
}
