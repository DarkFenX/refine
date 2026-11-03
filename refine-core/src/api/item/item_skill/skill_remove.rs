use crate::{Skill, SolarSystem, sol::UndoStep, ud::UItemId};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Skill<'s> {
    pub fn remove(self) {
        self.sol.internal_remove_skill(self.uid);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolarSystem {
    pub(in crate::api) fn internal_remove_skill(&mut self, skill_uid: UItemId) {
        SolarSystem::util_remove_skill(&mut self.u_data, &mut self.svc, skill_uid, &mut self.reuse.eupdates);
        let u_skill = self.u_data.items.get(skill_uid).dc_skill().unwrap();
        let u_fit = self.u_data.fits.get_mut(u_skill.get_fit_uid());
        u_fit.skills.remove(&u_skill.get_type_aid());
        self.internal_discard_item(skill_uid, UndoStep::SkillRemoved);
    }
}
