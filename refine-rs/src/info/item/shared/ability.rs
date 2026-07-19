use crate::Count;

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
pub struct AbilityInfo {
    pub state: bool,
    pub charge_count: Option<Count>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AbilityInfo {
    pub(in crate::info::item) fn from_core(core_ability: rc::Ability) -> Self {
        Self {
            state: core_ability.get_state(),
            charge_count: core_ability.get_charge_count(),
        }
    }
}
