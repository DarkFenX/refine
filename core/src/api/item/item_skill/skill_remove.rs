use crate::{
    api::SkillMut,
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_skill(&mut self, skill_key: UItemId, reuse_eupdates: &mut UEffectUpdates) {
        SolarSystem::util_remove_skill(&mut self.u_data, &mut self.svc, skill_key, reuse_eupdates);
        let u_skill = self.u_data.items.get(skill_key).dc_skill().unwrap();
        let u_fit = self.u_data.fits.get_mut(u_skill.get_fit_key());
        u_fit.skills.remove(&u_skill.get_type_id());
        self.u_data.items.remove(skill_key);
    }
}

impl<'a> SkillMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_remove_skill(self.key, &mut reuse_eupdates);
    }
}
