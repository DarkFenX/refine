#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HAutoChargeInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolItemId,
}
impl From<&rc::SolAutoChargeInfo> for HAutoChargeInfoId {
    fn from(core_autocharge_info: &rc::SolAutoChargeInfo) -> Self {
        Self {
            id: core_autocharge_info.id,
        }
    }
}
