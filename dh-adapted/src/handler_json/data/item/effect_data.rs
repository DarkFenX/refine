#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json) struct CItemEffectData {
    cd: Option<rc::AttrVal>,
    charge_count: Option<rc::Count>,
    charge_reload_time: Option<rc::AttrVal>,
}
impl From<&rc::ad::AItemEffectData> for CItemEffectData {
    fn from(a_item_effect_data: &rc::ad::AItemEffectData) -> Self {
        Self {
            cd: a_item_effect_data.cd,
            charge_count: a_item_effect_data.charge_count,
            charge_reload_time: a_item_effect_data.charge_reload_time,
        }
    }
}
impl From<&CItemEffectData> for rc::ad::AItemEffectData {
    fn from(c_item_effect_data: &CItemEffectData) -> Self {
        Self {
            cd: c_item_effect_data.cd,
            charge_count: c_item_effect_data.charge_count,
            charge_reload_time: c_item_effect_data.charge_reload_time,
        }
    }
}
