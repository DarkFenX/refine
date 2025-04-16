#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HImplantInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::ItemId,
}
impl HImplantInfoId {
    pub(super) fn from_item_id(implant_id: &rc::ItemId) -> Self {
        Self { id: *implant_id }
    }
}
