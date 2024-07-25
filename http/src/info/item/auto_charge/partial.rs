#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HAutoChargeInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolItemId,
    pub(crate) kind: &'static str,
    pub(crate) type_id: rc::EItemId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::SolFitId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) cont_id: rc::SolItemId,
}
impl From<&rc::SolAutoChargeInfo> for HAutoChargeInfoPartial {
    fn from(core_auto_charge_info: &rc::SolAutoChargeInfo) -> Self {
        Self {
            id: core_auto_charge_info.id,
            kind: "autocharge",
            type_id: core_auto_charge_info.a_item_id,
            fit_id: core_auto_charge_info.fit_id,
            cont_id: core_auto_charge_info.cont_id,
        }
    }
}
