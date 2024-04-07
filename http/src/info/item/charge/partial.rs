#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HChargeInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SsItemId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::SsFitId,
    pub(crate) type_id: rc::EItemId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) cont_id: rc::SsItemId,
}
impl From<&rc::SsChargeInfo> for HChargeInfoPartial {
    fn from(core_charge_info: &rc::SsChargeInfo) -> Self {
        Self {
            id: core_charge_info.id,
            fit_id: core_charge_info.fit_id,
            type_id: core_charge_info.a_item_id,
            cont_id: core_charge_info.cont_id,
        }
    }
}
