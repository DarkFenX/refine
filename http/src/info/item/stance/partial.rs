use rc::ItemCommon;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HStanceInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::ItemId,
    pub(crate) kind: &'static str,
    pub(crate) type_id: rc::ItemTypeId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::FitId,
    pub(crate) enabled: bool,
}
impl From<&mut rc::StanceMut<'_>> for HStanceInfoPartial {
    fn from(core_stance: &mut rc::StanceMut) -> Self {
        Self {
            id: core_stance.get_item_id(),
            kind: "stance",
            type_id: core_stance.get_type_id(),
            fit_id: core_stance.get_fit().get_fit_id(),
            enabled: core_stance.get_state(),
        }
    }
}
