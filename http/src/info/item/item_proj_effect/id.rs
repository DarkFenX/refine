use rc::ItemCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HProjEffectInfoId {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::ItemId,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HProjEffectInfoId {
    pub(super) fn from_core(core_proj_effect: &mut rc::ProjEffectMut) -> Self {
        Self {
            id: core_proj_effect.get_item_id(),
        }
    }
}
