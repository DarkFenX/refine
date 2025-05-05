use crate::sol::{ItemKey, SolarSystem, api::SkillMut};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_skill(&mut self, item_key: ItemKey) {
        let uad_item = self.uad.items.get(item_key);
        let uad_skill = uad_item.get_skill().unwrap();
        self.svc.remove_item(&self.uad, item_key, uad_item);
        let uad_fit = self.uad.fits.get_mut(uad_skill.get_fit_key());
        uad_fit.skills.remove(&uad_skill.get_a_item_id());
        self.uad.items.remove(item_key);
    }
}

impl<'a> SkillMut<'a> {
    pub fn remove(self) {
        self.sol.internal_remove_skill(self.key);
    }
}
