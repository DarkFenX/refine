#[derive(serde::Serialize)]
pub(crate) struct HChargeInfoPartial {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) id: rc::ReeId,
    #[serde(with = "crate::util::serde_string")]
    pub(crate) fit_id: rc::ReeId,
    pub(crate) type_id: rc::ReeInt,
    #[serde(with = "crate::util::serde_string")]
    pub(crate) cont_id: rc::ReeId,
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
