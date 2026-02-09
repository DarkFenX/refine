use rc::ItemCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HFwEffectInfoPartial {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::ItemId,
    kind: &'static str,
    type_id: i32,
    #[serde_as(as = "DisplayFromStr")]
    fit_id: rc::FitId,
    enabled: bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HFwEffectInfoPartial {
    pub(super) fn from_core(core_fw_effect: &mut rc::FwEffectMut) -> Self {
        Self {
            id: core_fw_effect.get_item_id(),
            kind: "fw_effect",
            type_id: core_fw_effect.get_type_id().into_i32(),
            fit_id: core_fw_effect.get_fit().get_fit_id(),
            enabled: core_fw_effect.get_state(),
        }
    }
}
