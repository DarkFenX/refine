#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HFwEffectInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolItemId,
    pub(crate) kind: &'static str,
    pub(crate) type_id: rc::EItemId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::SolFitId,
    pub(crate) enabled: bool,
}
impl From<&rc::SolFwEffectInfo> for HFwEffectInfoPartial {
    fn from(core_fw_effect_info: &rc::SolFwEffectInfo) -> Self {
        Self {
            id: core_fw_effect_info.id,
            kind: "fw_effect",
            type_id: core_fw_effect_info.a_item_id,
            fit_id: core_fw_effect_info.fit_id,
            enabled: core_fw_effect_info.enabled,
        }
    }
}
