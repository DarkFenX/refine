use crate::{
    ad::{AAttrVal, ACount, AItemEffectData, AItemId, AItemListId},
    def::OF,
    rd::RItemListId,
    util::RMap,
};

#[derive(Copy, Clone)]
pub(crate) struct RItemEffectData {
    pub(crate) autocharge: Option<AItemId>,
    pub(crate) cooldown_s: AAttrVal,
    pub(crate) charge_count: Option<ACount>,
    pub(crate) charge_reload_time_s: AAttrVal,
    pub(crate) projectee_filter: Option<RItemListId>,
}
impl RItemEffectData {
    pub(in crate::rd::data::item) fn from_a_effect_data(
        a_effect_data: &AItemEffectData,
        item_list_id_key_map: &RMap<AItemListId, RItemListId>,
    ) -> Self {
        Self {
            autocharge: a_effect_data.autocharge,
            cooldown_s: a_effect_data.cooldown.unwrap_or(OF(0.0)).max(OF(0.0)),
            charge_count: a_effect_data.charge_count,
            charge_reload_time_s: a_effect_data.charge_reload_time.unwrap_or(OF(0.0)).max(OF(0.0)),
            projectee_filter: a_effect_data
                .projectee_filter
                .as_ref()
                .and_then(|v| item_list_id_key_map.get(v).copied()),
        }
    }
}
