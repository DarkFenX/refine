use rc::ItemCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HRigInfoId {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::ItemId,
}
impl From<&mut rc::RigMut<'_>> for HRigInfoId {
    fn from(core_rig: &mut rc::RigMut) -> Self {
        Self {
            id: core_rig.get_item_id(),
        }
    }
}
