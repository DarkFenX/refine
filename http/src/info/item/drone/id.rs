use rc::ItemCommon;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HDroneInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::ItemId,
}
impl From<&mut rc::DroneMut<'_>> for HDroneInfoId {
    fn from(core_drone: &mut rc::DroneMut) -> Self {
        Self {
            id: core_drone.get_item_id(),
        }
    }
}
