#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HFwEffectInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolItemId,
}
impl From<&rc::FwEffectInfo> for HFwEffectInfoId {
    fn from(core_fw_effect_info: &rc::FwEffectInfo) -> Self {
        Self {
            id: core_fw_effect_info.id,
        }
    }
}
