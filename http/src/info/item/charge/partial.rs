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
impl From<&rc::ChargeInfo> for HChargeInfoPartial {
    fn from(core_charge_info: &rc::ChargeInfo) -> Self {
        Self {
            id: core_charge_info.id,
            kind: "charge",
            type_id: core_charge_info.type_id,
            fit_id: core_charge_info.fit_id,
            cont_item_id: core_charge_info.cont_item_id,
            enabled: core_charge_info.enabled,
        }
    }
}
