use crate::{
    ad::{AAttrVal, ACount, AItemEffectData, AItemId, AItemListId},
    rd::RItemListKey,
    util::RMap,
};

#[derive(Copy, Clone)]
pub(crate) struct RItemEffectData {
    pub(crate) autocharge: Option<AItemId>,
    pub(crate) cooldown: Option<AAttrVal>,
    pub(crate) charge_count: Option<ACount>,
    pub(crate) charge_reload_time: Option<AAttrVal>,
    pub(crate) projectee_filter: Option<RItemListKey>,
}
impl RItemEffectData {
    pub(in crate::rd::data::item) fn from_a_effect_data(
        a_effect_data: &AItemEffectData,
        item_list_id_key_map: &RMap<AItemListId, RItemListKey>,
    ) -> Self {
        Self {
            autocharge: a_effect_data.autocharge,
            cooldown: a_effect_data.cooldown,
            charge_count: a_effect_data.charge_count,
            charge_reload_time: a_effect_data.charge_reload_time,
            projectee_filter: a_effect_data
                .projectee_filter
                .as_ref()
                .and_then(|v| item_list_id_key_map.get(v).copied()),
        }
    }
}
