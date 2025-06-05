use rc::ItemCommon;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HSwEffectInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::ItemId,
    kind: &'static str,
    type_id: rc::ItemTypeId,
    enabled: bool,
}
impl From<&mut rc::SwEffectMut<'_>> for HSwEffectInfoPartial {
    fn from(core_sw_effect: &mut rc::SwEffectMut) -> Self {
        Self {
            id: core_sw_effect.get_item_id(),
            kind: "sw_effect",
            type_id: core_sw_effect.get_type_id(),
            enabled: core_sw_effect.get_state(),
        }
    }
}
