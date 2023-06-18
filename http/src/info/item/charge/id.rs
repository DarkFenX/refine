#[derive(serde::Serialize)]
pub(crate) struct HChargeInfoId {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) id: rc::ReeId,
}
impl From<&rc::SsChargeInfo> for HChargeInfoId {
    fn from(core_charge_info: &rc::SsChargeInfo) -> Self {
        Self {
            id: core_charge_info.id,
        }
    }
}
