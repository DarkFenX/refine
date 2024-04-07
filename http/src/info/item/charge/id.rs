#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HChargeInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SsItemId,
}
impl From<&rc::SsChargeInfo> for HChargeInfoId {
    fn from(core_charge_info: &rc::SsChargeInfo) -> Self {
        Self {
            id: core_charge_info.id,
        }
    }
}
