use rc::ItemCommon;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HChargeInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::ItemId,
    pub(crate) kind: &'static str,
    pub(crate) type_id: rc::ItemTypeId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::FitId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) cont_item_id: rc::ItemId,
    pub(crate) enabled: bool,
}
impl From<&mut rc::ChargeMut<'_>> for HChargeInfoPartial {
    fn from(core_charge: &mut rc::ChargeMut) -> Self {
        Self {
            id: core_charge.get_item_id(),
            kind: "charge",
            type_id: core_charge.get_type_id(),
            fit_id: core_charge.get_fit().get_fit_id(),
            cont_item_id: core_charge.get_cont_item().get_item_id(),
            enabled: core_charge.get_state(),
        }
    }
}
