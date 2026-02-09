use rc::ItemCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HFwEffectInfoId {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::ItemId,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HFwEffectInfoId {
    pub(super) fn from_core(core_fw_effect: &mut rc::FwEffectMut) -> Self {
        Self {
            id: core_fw_effect.get_item_id(),
        }
    }
}
