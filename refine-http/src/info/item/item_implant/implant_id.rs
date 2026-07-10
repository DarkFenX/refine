use rc::ItemCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HImplantInfoId {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::ItemId,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HImplantInfoId {
    pub(super) fn from_core(core_implant: &mut rc::ImplantMut) -> Self {
        Self {
            id: core_implant.get_item_id(),
        }
    }
}
