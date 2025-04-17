use rc::ItemCommon;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HImplantInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::ItemId,
}
impl From<&mut rc::ImplantMut<'_>> for HImplantInfoId {
    fn from(core_implant: &mut rc::ImplantMut) -> Self {
        Self {
            id: core_implant.get_item_id(),
        }
    }
}
