use rc::ItemCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HStanceInfoPartial {
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
impl HStanceInfoPartial {
    pub(super) fn from_core(core_stance: &mut rc::StanceMut) -> Self {
        Self {
            id: core_stance.get_item_id(),
            kind: "stance",
            type_id: core_stance.get_type_id().into_i32(),
            fit_id: core_stance.get_fit().get_fit_id(),
            enabled: core_stance.get_state(),
        }
    }
}
