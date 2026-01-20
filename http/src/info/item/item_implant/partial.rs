use rc::ItemCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HImplantInfoPartial {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::ItemId,
    kind: &'static str,
    type_id: i32,
    #[serde_as(as = "DisplayFromStr")]
    fit_id: rc::FitId,
    #[serde(skip_serializing_if = "Option::is_none")]
    slot: Option<i32>,
    enabled: bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HImplantInfoPartial {
    pub(super) fn from_core(core_implant: &mut rc::ImplantMut) -> Self {
        Self {
            id: core_implant.get_item_id(),
            kind: "implant",
            type_id: core_implant.get_type_id().into_i32(),
            fit_id: core_implant.get_fit().get_fit_id(),
            slot: core_implant.get_slot().map(|v| v.into_i32()),
            enabled: core_implant.get_state(),
        }
    }
}
