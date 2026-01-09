use rc::ItemCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HSubsystemInfoPartial {
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
impl From<&mut rc::SubsystemMut<'_>> for HSubsystemInfoPartial {
    fn from(core_subsystem: &mut rc::SubsystemMut) -> Self {
        Self {
            id: core_subsystem.get_item_id(),
            kind: "subsystem",
            type_id: core_subsystem.get_type_id().into_i32(),
            fit_id: core_subsystem.get_fit().get_fit_id(),
            slot: core_subsystem.get_slot().map(|v| v.into_i32()),
            enabled: core_subsystem.get_state(),
        }
    }
}
