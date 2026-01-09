use rc::ItemCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HSubsystemInfoId {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::ItemId,
}
impl From<&mut rc::SubsystemMut<'_>> for HSubsystemInfoId {
    fn from(core_subsystem: &mut rc::SubsystemMut) -> Self {
        Self {
            id: core_subsystem.get_item_id(),
        }
    }
}
