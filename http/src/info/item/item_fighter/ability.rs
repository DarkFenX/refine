#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HAbilityInfo {
    status: bool,
    charge_count: Option<rc::Count>,
}
impl From<rc::Ability<'_>> for HAbilityInfo {
    fn from(core_ability: rc::Ability) -> Self {
        Self {
            status: core_ability.get_state(),
            charge_count: core_ability.get_charge_count(),
        }
    }
}
