#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HChargeInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolItemId,
}
impl From<&rc::SolChargeInfo> for HChargeInfoId {
    fn from(core_charge_info: &rc::SolChargeInfo) -> Self {
        Self {
            id: core_charge_info.id,
        }
    }
}
