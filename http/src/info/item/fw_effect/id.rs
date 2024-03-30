#[derive(serde::Serialize)]
pub(crate) struct HFwEffectInfoId {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) id: rc::SsItemId,
}
impl From<&rc::SsFwEffectInfo> for HFwEffectInfoId {
    fn from(core_fw_effect_info: &rc::SsFwEffectInfo) -> Self {
        Self {
            id: core_fw_effect_info.id,
        }
    }
}
