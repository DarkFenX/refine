pub(in crate::cmd) struct SideEffects {
    data: Vec<(rc::EffectId, bool)>,
}
impl SideEffects {
    pub(in crate::cmd) const fn new() -> Self {
        Self { data: Vec::new() }
    }
    pub(in crate::cmd) fn clear(&mut self) {
        self.data.clear();
    }
    pub(in crate::cmd) fn extend(&mut self, effect_modes: impl Iterator<Item = (rc::EffectId, bool)>) {
        self.data.extend(effect_modes);
    }
    pub(in crate::cmd) fn apply(&self, core_booster: &mut rc::BoosterMut) {
        for (effect_id, status) in self.data.iter() {
            if let Ok(mut core_side_effect) = core_booster.get_side_effect_mut(effect_id) {
                core_side_effect.set_state(*status);
            }
        }
    }
}
