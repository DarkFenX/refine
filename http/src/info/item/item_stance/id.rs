use rc::ItemCommon;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HStanceInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::ItemId,
}
impl From<&mut rc::StanceMut<'_>> for HStanceInfoId {
    fn from(core_stance: &mut rc::StanceMut) -> Self {
        Self {
            id: core_stance.get_item_id(),
        }
    }
}
