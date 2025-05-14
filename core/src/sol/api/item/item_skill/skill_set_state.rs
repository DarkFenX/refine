use crate::sol::{ItemKey, SolarSystem, api::SkillMut};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_skill_state(&mut self, item_key: ItemKey, state: bool) {
        let uad_skill = self.uad.items.get_mut(item_key).get_skill_mut().unwrap();
        let old_a_state = uad_skill.get_a_state();
        uad_skill.set_skill_state(state);
        let new_a_state = uad_skill.get_a_state();
        self.internal_change_item_key_state_in_svc(item_key, old_a_state, new_a_state);
    }
}

impl<'a> SkillMut<'a> {
    pub fn set_state(&mut self, state: bool) {
        self.sol.internal_set_skill_state(self.key, state)
    }
}
