use rc::ItemCommon;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HRigInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::ItemId,
}
impl From<&mut rc::RigMut<'_>> for HRigInfoId {
    fn from(core_rig: &mut rc::RigMut) -> Self {
        Self {
            id: core_rig.get_item_id(),
        }
    }
}
