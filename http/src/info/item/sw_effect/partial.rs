#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HSwEffectInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::ItemId,
    pub(crate) kind: &'static str,
    pub(crate) type_id: rc::ItemTypeId,
    pub(crate) enabled: bool,
}
impl From<&rc::SwEffectInfo> for HSwEffectInfoPartial {
    fn from(core_sw_effect_info: &rc::SwEffectInfo) -> Self {
        Self {
            id: core_sw_effect_info.id,
            kind: "sw_effect",
            type_id: core_sw_effect_info.type_id,
            enabled: core_sw_effect_info.enabled,
        }
    }
}
