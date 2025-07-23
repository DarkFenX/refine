use crate::handler_json::data::{CAttrVal, CCount, CItemId};

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json) struct CItemEffectData {
    autocharge: Option<CItemId>,
    cd: Option<CAttrVal>,
    charge_count: Option<CCount>,
    charge_reload_time: Option<CAttrVal>,
}
impl From<&rc::ad::AItemEffectData> for CItemEffectData {
    fn from(a_item_effect_data: &rc::ad::AItemEffectData) -> Self {
        Self {
            autocharge: a_item_effect_data.autocharge,
            cd: a_item_effect_data.cd,
            charge_count: a_item_effect_data.charge_count,
            charge_reload_time: a_item_effect_data.charge_reload_time,
        }
    }
}
impl From<&CItemEffectData> for rc::ad::AItemEffectData {
    fn from(c_item_effect_data: &CItemEffectData) -> Self {
        Self {
            autocharge: c_item_effect_data.autocharge,
            cd: c_item_effect_data.cd,
            charge_count: c_item_effect_data.charge_count,
            charge_reload_time: c_item_effect_data.charge_reload_time,
        }
    }
}
