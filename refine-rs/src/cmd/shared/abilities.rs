#[cfg_attr(
    feature = "serde",
    cfg_eval,
    serde_with::serde_as,
    derive(serde::Deserialize),
    serde(transparent)
)]
#[derive(Default)]
pub(in crate::cmd) struct Abilities {
    #[cfg_attr(feature = "serde", serde_as(as = "serde_with::Map<_, _>"))]
    data: Vec<(rc::AbilityId, bool)>,
}
impl Abilities {
    pub(in crate::cmd) const fn new() -> Self {
        Self { data: Vec::new() }
    }
    pub(in crate::cmd) fn clear(&mut self) {
        self.data.clear();
    }
    pub(in crate::cmd) fn extend(&mut self, effect_modes: impl Iterator<Item = (rc::AbilityId, bool)>) {
        self.data.extend(effect_modes);
    }
    pub(in crate::cmd) fn apply(&self, core_fighter: &mut rc::FighterMut) {
        // Apply state changes only to existing abilities, ignore the rest
        for (abil_id, new_state) in self.data.iter() {
            if let Ok(mut core_ability) = core_fighter.get_ability_mut(abil_id) {
                core_ability.set_state(*new_state);
            }
        }
    }
}
