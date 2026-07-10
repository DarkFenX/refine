use rc::ItemCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HSwEffectInfoId {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::ItemId,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HSwEffectInfoId {
    pub(super) fn from_core(core_sw_effect: &mut rc::SwEffectMut) -> Self {
        Self {
            id: core_sw_effect.get_item_id(),
        }
    }
}
