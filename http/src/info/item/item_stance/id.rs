use rc::ItemCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HStanceInfoId {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::ItemId,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HStanceInfoId {
    pub(super) fn from_core(core_stance: &mut rc::StanceMut) -> Self {
        Self {
            id: core_stance.get_item_id(),
        }
    }
}
