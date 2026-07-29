use crate::{
    Count, PValue,
    ad::{AAttrId, AItemEffectData},
    rd::RAttrId,
    util::RMap,
};

// Item-specific attribute-independent effect data
#[derive(Copy, Clone)]
pub(crate) struct RItemEffectData {
    pub(crate) ability_cooldown: Option<PValue>,
    pub(crate) ability_charge_count: Option<Count>,
    pub(crate) ability_charge_reload_duration: PValue,
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
            ability_cooldown: a_effect_data
                .ability_cooldown
                .map(PValue::from_a_value_clamped)
                .and_then(|v| match v {
                    PValue::ZERO => None,
                    _ => Some(v),
                }),
            ability_charge_count: a_effect_data.ability_charge_count.map(Count::from_a_count),
            ability_charge_reload_duration: a_effect_data
                .ability_charge_reload_duration
                .map(PValue::from_a_value_clamped)
                .unwrap_or(PValue::ZERO),
            autocharge_attr_rid: a_effect_data
                .autocharge_attr_id
                .as_ref()
                .and_then(|attr_aid| attr_aid_rid_map.get(attr_aid).copied()),
        }
    }
}
