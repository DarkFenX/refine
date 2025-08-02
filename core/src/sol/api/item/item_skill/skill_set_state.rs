use crate::{
    sol::{SolarSystem, api::SkillMut},
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_skill_state(
        &mut self,
        item_key: UItemKey,
        state: bool,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_skill = self.u_data.items.get_mut(item_key).get_skill_mut().unwrap();
        let old_a_state = u_skill.get_state();
        u_skill.set_skill_state(state);
        let new_a_state = u_skill.get_state();
        u_skill.update_reffs(reuse_eupdates, &self.u_data.src);
        SolarSystem::util_switch_item_state(
            &self.u_data,
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
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_set_skill_state(self.key, state, &mut reuse_eupdates)
    }
}
