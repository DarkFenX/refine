#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HAbilityInfo {
    state: bool,
    charge_count: Option<rc::Count>,
}
impl From<rc::Ability<'_>> for HAbilityInfo {
    fn from(core_ability: rc::Ability) -> Self {
        Self {
            state: core_ability.get_state(),
            charge_count: core_ability.get_charge_count(),
        }
    }
}
