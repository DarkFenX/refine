use crate::{
    Count, PValue,
    ad::{AAttrId, AItemEffectData},
    rd::RAttrId,
    util::RMap,
};

#[derive(Copy, Clone)]
pub(crate) struct RItemEffectData {
    pub(crate) cooldown_s: PValue,
    pub(crate) charge_count: Option<Count>,
    pub(crate) charge_reload_duration: PValue,
    pub(crate) autocharge_attr_rid: Option<RAttrId>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RItemEffectData {
    pub(super) fn from_a_effect_data(
        a_effect_data: &AItemEffectData,
        attr_aid_rid_map: &RMap<AAttrId, RAttrId>,
    ) -> Self {
        Self {
            cooldown_s: a_effect_data
                .cooldown
                .map(PValue::from_a_value_clamped)
                .unwrap_or(PValue::ZERO),
            charge_count: a_effect_data.charge_count.map(Count::from_a_count),
            charge_reload_duration: a_effect_data
                .charge_reload_duration
                .map(PValue::from_a_value_clamped)
                .unwrap_or(PValue::ZERO),
            autocharge_attr_rid: a_effect_data
                .autocharge_attr_id
                .as_ref()
                .and_then(|attr_aid| attr_aid_rid_map.get(attr_aid).copied()),
        }
    }
}
