use crate::{
    api::{Fit, FitMut, MutIter, Skill, SkillMut},
    sol::SolarSystem,
    ud::UFitId,
};

impl<'a> Fit<'a> {
    pub fn iter_skills(&self) -> impl ExactSizeIterator<Item = Skill<'_>> {
        iter_skills(self.sol, self.uid)
    }
}

impl<'a> FitMut<'a> {
    pub fn iter_skills(&self) -> impl ExactSizeIterator<Item = Skill<'_>> {
        iter_skills(self.sol, self.uid)
    }
    pub fn iter_skills_mut(&mut self) -> MutIter<'_, SkillMut<'_>> {
        let fit_skills = self.sol.u_data.fits.get(self.uid).skills.values();
        let skill_uids = fit_skills.map(|fit_skill| fit_skill.skill_uid).collect();
        MutIter::new(self.sol, skill_uids)
    }
}

fn iter_skills(sol: &SolarSystem, fit_uid: UFitId) -> impl ExactSizeIterator<Item = Skill<'_>> {
    let fit_skills = sol.u_data.fits.get(fit_uid).skills.values();
    fit_skills.map(|fit_skill| Skill::new(sol, fit_skill.skill_uid))
}
