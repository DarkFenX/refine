use rc::ItemCommon;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HBoosterInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::ItemId,
}
impl From<&mut rc::BoosterMut<'_>> for HBoosterInfoId {
    fn from(core_booster: &mut rc::BoosterMut) -> Self {
        Self {
            id: core_booster.get_item_id(),
        }
    }
}
