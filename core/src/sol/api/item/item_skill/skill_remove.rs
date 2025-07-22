use crate::{
    sol::{SolarSystem, api::SkillMut},
    uad::{UadEffectUpdates, UadItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_skill(
        &mut self,
        item_key: UadItemKey,
        reuse_eupdates: &mut UadEffectUpdates,
    ) {
        let uad_item = self.uad.items.get(item_key);
        let uad_skill = uad_item.get_skill().unwrap();
        SolarSystem::util_remove_skill(&self.uad, &mut self.svc, item_key, uad_item, reuse_eupdates);
        let uad_fit = self.uad.fits.get_mut(uad_skill.get_fit_key());
        uad_fit.skills.remove(&uad_skill.get_a_item_id());
        self.uad.items.remove(item_key);
    }
}

impl<'a> SkillMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UadEffectUpdates::new();
        self.sol.internal_remove_skill(self.key, &mut reuse_eupdates);
    }
}
