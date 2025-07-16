use crate::{
    def::ItemKey,
    sol::{SolarSystem, api::SkillMut},
    uad::UadEffectUpdates,
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_skill_state(
        &mut self,
        item_key: ItemKey,
        state: bool,
        reuse_eupdates: &mut UadEffectUpdates,
    ) {
        let uad_skill = self.uad.items.get_mut(item_key).get_skill_mut().unwrap();
        let old_a_state = uad_skill.get_a_state();
        uad_skill.set_skill_state(state, reuse_eupdates, &self.uad.src);
        let new_a_state = uad_skill.get_a_state();
        SolarSystem::util_switch_item_state(
            &self.uad,
            &mut self.svc,
            item_key,
            old_a_state,
            new_a_state,
            reuse_eupdates,
        );
    }
}

impl<'a> SkillMut<'a> {
    pub fn set_state(&mut self, state: bool) {
        let mut reuse_eupdates = UadEffectUpdates::new();
        self.sol.internal_set_skill_state(self.key, state, &mut reuse_eupdates)
    }
}
