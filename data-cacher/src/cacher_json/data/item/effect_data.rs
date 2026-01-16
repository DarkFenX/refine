#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(super) struct CItemEffectData {
    autocharge: Option<i32>,
    cooldown: Option<f64>,
    charge_count: Option<u32>,
    charge_reload_duration: Option<f64>,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>", no_default)]
    projectee_filter: Option<rc::ad::AItemListId>,
}
impl CItemEffectData {
    pub(super) fn from_adapted(a_item_effect_data: &rc::ad::AItemEffectData) -> Self {
        Self {
            autocharge: a_item_effect_data.autocharge.map(|v| v.into_i32()),
            cooldown: a_item_effect_data.cooldown.map(|v| v.into_f64()),
            charge_count: a_item_effect_data.charge_count.map(|v| v.into_u32()),
            charge_reload_duration: a_item_effect_data.charge_reload_duration.map(|v| v.into_f64()),
            projectee_filter: a_item_effect_data.projectee_filter,
        }
    }
    pub(super) fn into_adapted(self) -> rc::ad::AItemEffectData {
        rc::ad::AItemEffectData {
            autocharge: self.autocharge.map(|v| rc::ad::AItemId::from_i32(v)),
            cooldown: self.cooldown.map(|v| rc::ad::AValue::from_f64(v)),
            charge_count: self.charge_count.map(|v| rc::ad::ACount::from_u32(v)),
            charge_reload_duration: self.charge_reload_duration.map(|v| rc::ad::AValue::from_f64(v)),
            projectee_filter: self.projectee_filter,
        }
    }
}
