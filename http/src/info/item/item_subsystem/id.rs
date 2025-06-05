use rc::ItemCommon;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HSubsystemInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::ItemId,
}
impl From<&mut rc::SubsystemMut<'_>> for HSubsystemInfoId {
    fn from(core_subsystem: &mut rc::SubsystemMut) -> Self {
        Self {
            id: core_subsystem.get_item_id(),
        }
    }
}
