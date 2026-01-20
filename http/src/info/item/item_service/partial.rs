use rc::ItemCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::shared::HServiceState;

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HServiceInfoPartial {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::ItemId,
    kind: &'static str,
    type_id: i32,
    #[serde_as(as = "DisplayFromStr")]
    fit_id: rc::FitId,
    state: HServiceState,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HServiceInfoPartial {
    pub(super) fn from_core(core_service: &mut rc::ServiceMut) -> Self {
        Self {
            id: core_service.get_item_id(),
            kind: "service",
            type_id: core_service.get_type_id().into_i32(),
            fit_id: core_service.get_fit().get_fit_id(),
            state: HServiceState::from_core(core_service.get_state()),
        }
    }
}
