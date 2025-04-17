use rc::ItemCommon;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HServiceInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::ItemId,
}
impl From<&mut rc::ServiceMut<'_>> for HServiceInfoId {
    fn from(core_service: &mut rc::ServiceMut) -> Self {
        Self {
            id: core_service.get_item_id(),
        }
    }
}
