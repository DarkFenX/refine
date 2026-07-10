use rc::ItemCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::info::item::proj::HProjInfo;

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HProjEffectInfoPartial {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::ItemId,
    kind: &'static str,
    type_id: i32,
    state: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    proj_item_ids: Vec<HProjInfo>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HProjEffectInfoPartial {
    pub(super) fn from_core(core_proj_effect: &mut rc::ProjEffectMut) -> Self {
        Self {
            id: core_proj_effect.get_item_id(),
            kind: "proj_effect",
            type_id: core_proj_effect.get_type_id().into_i32(),
            state: core_proj_effect.get_state(),
            proj_item_ids: core_proj_effect.iter_projs().map(HProjInfo::from_core).collect(),
        }
    }
}
