#[derive(serde::Serialize)]
pub(crate) struct HSwEffectInfoPartial {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) id: rc::SsItemId,
    pub(crate) type_id: rc::ItemId,
    pub(crate) enabled: bool,
}
impl From<&rc::SsSwEffectInfo> for HSwEffectInfoPartial {
    fn from(core_sw_effect_info: &rc::SsSwEffectInfo) -> Self {
        Self {
            id: core_sw_effect_info.id,
            type_id: core_sw_effect_info.a_item_id,
            enabled: core_sw_effect_info.enabled,
        }
    }
}
