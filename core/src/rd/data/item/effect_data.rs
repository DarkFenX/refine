use crate::{
    ad::{AItemEffectData, AItemId, AItemListId},
    misc::{Count, PValue},
    rd::RItemListId,
    util::RMap,
};

#[derive(Copy, Clone)]
pub(crate) struct RItemEffectData {
    pub(crate) autocharge: Option<AItemId>,
    pub(crate) cooldown_s: PValue,
    pub(crate) charge_count: Option<Count>,
    pub(crate) charge_reload_time_s: PValue,
    pub(crate) projectee_filter: Option<RItemListId>,
}
impl RItemEffectData {
    pub(in crate::rd::data::item) fn from_a_effect_data(
        a_effect_data: &AItemEffectData,
        item_list_id_key_map: &RMap<AItemListId, RItemListId>,
    ) -> Self {
        Self {
            autocharge: a_effect_data.autocharge,
            cooldown_s: a_effect_data
                .cooldown
                .map(Into::into)
                .unwrap_or(PValue::from_f64_unchecked(0.0)),
            charge_count: a_effect_data.charge_count.map(Into::into),
            charge_reload_time_s: a_effect_data
                .charge_reload_time
                .map(Into::into)
                .unwrap_or(PValue::from_f64_unchecked(0.0)),
            projectee_filter: a_effect_data
                .projectee_filter
                .as_ref()
                .and_then(|item_list_aid| item_list_id_key_map.get(item_list_aid).copied()),
        }
    }
}
