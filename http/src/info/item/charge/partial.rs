#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HChargeInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolItemId,
    pub(crate) kind: &'static str,
    pub(crate) type_id: rc::EItemId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::SolFitId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) cont_id: rc::SolItemId,
}
impl From<&rc::SolChargeInfo> for HChargeInfoPartial {
    fn from(core_charge_info: &rc::SolChargeInfo) -> Self {
        Self {
            id: core_charge_info.id,
            kind: "charge",
            type_id: core_charge_info.type_id,
            fit_id: core_charge_info.fit_id,
            cont_id: core_charge_info.cont_id,
        }
    }
}
