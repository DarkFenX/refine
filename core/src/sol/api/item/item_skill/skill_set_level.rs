use crate::{
    misc::SkillLevel,
    sol::{SolarSystem, api::SkillMut},
    ud::UItemKey,
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_skill_level(&mut self, item_key: UItemKey, level: SkillLevel) {
        let u_skill = self.u_data.items.get_mut(item_key).get_skill_mut().unwrap();
        if u_skill.get_a_level() == level {
            return;
        }
        u_skill.set_a_level(level.into());
        self.u_data
            .fits
            .get_mut(u_skill.get_fit_key())
            .skills
            .get_mut(&u_skill.get_a_item_id())
            .unwrap()
            .level = level;
        let u_skill = self.u_data.items.get(item_key).get_skill().unwrap();
        self.svc.notify_skill_level_changed(&self.u_data, item_key, u_skill);
    }
}

impl<'a> SkillMut<'a> {
    pub fn set_level(&mut self, level: SkillLevel) {
        self.sol.internal_set_skill_level(self.key, level);
    }
}
