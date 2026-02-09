use rc::ItemCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HRigInfoPartial {
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
impl HRigInfoPartial {
    pub(super) fn from_core(core_rig: &mut rc::RigMut) -> Self {
        Self {
            id: core_rig.get_item_id(),
            kind: "rig",
            type_id: core_rig.get_type_id().into_i32(),
            fit_id: core_rig.get_fit().get_fit_id(),
            enabled: core_rig.get_state(),
        }
    }
}
