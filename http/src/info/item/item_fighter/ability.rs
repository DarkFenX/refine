use serde_tuple::Serialize_tuple;

#[derive(Serialize_tuple)]
pub(in crate::info::item::item_fighter) struct HAbilityInfo {
    state: bool,
    charge_count: Option<u32>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HAbilityInfo {
    pub(super) fn from_core(core_ability: rc::Ability) -> Self {
        Self {
            state: core_ability.get_state(),
            charge_count: core_ability.get_charge_count().map(|v| v.into_u32()),
        }
    }
}
