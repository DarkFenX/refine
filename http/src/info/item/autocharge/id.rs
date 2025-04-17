use rc::ItemCommon;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HAutochargeInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::ItemId,
}
impl From<&mut rc::AutochargeMut<'_>> for HAutochargeInfoId {
    fn from(core_autocharge: &mut rc::AutochargeMut) -> Self {
        Self {
            id: core_autocharge.get_item_id(),
        }
    }
}
