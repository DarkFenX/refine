use crate::{
    def::ItemKey,
    misc::SkillLevel,
    sol::{SolarSystem, api::SkillMut},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_skill_level(&mut self, item_key: ItemKey, level: SkillLevel) {
        let uad_skill = self.uad.items.get_mut(item_key).get_skill_mut().unwrap();
        if uad_skill.get_a_level() == level {
            return;
        }
        uad_skill.set_a_level(level.into());
        self.uad
            .fits
            .get_mut(uad_skill.get_fit_key())
            .skills
            .get_mut(&uad_skill.get_a_item_id())
            .unwrap()
            .level = level;
        let uad_skill = self.uad.items.get(item_key).get_skill().unwrap();
        self.svc.notify_skill_level_changed(&self.uad, item_key, uad_skill);
    }
}

impl<'a> SkillMut<'a> {
    pub fn set_level(&mut self, level: SkillLevel) {
        self.sol.internal_set_skill_level(self.key, level);
    }
}
