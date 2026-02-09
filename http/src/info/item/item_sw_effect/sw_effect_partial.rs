use rc::ItemCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HSwEffectInfoPartial {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::ItemId,
    kind: &'static str,
    type_id: i32,
    enabled: bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HSwEffectInfoPartial {
    pub(super) fn from_core(core_sw_effect: &mut rc::SwEffectMut) -> Self {
        Self {
            id: core_sw_effect.get_item_id(),
            kind: "sw_effect",
            type_id: core_sw_effect.get_type_id().into_i32(),
            enabled: core_sw_effect.get_state(),
        }
    }
}
