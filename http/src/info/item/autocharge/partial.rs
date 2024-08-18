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
    pub(crate) enabled: bool,
}
impl From<&rc::SolAutoChargeInfo> for HAutoChargeInfoPartial {
    fn from(core_autocharge_info: &rc::SolAutoChargeInfo) -> Self {
        Self {
            id: core_autocharge_info.id,
            kind: "autocharge",
            type_id: core_autocharge_info.type_id,
            fit_id: core_autocharge_info.fit_id,
            cont_id: core_autocharge_info.cont_id,
            enabled: core_autocharge_info.enabled,
        }
    }
}
