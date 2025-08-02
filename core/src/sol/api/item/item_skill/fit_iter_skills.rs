use crate::{
    sol::{
        SolarSystem,
        api::{Fit, FitMut, MutIter, Skill, SkillMut},
    },
    ud::UFitKey,
};

impl<'a> Fit<'a> {
    pub fn iter_skills(&self) -> impl ExactSizeIterator<Item = Skill<'_>> {
        iter_skills(self.sol, self.key)
    }
}

impl<'a> FitMut<'a> {
    pub fn iter_skills(&self) -> impl ExactSizeIterator<Item = Skill<'_>> {
        iter_skills(self.sol, self.key)
    }
    pub fn iter_skills_mut(&mut self) -> MutIter<'_, SkillMut<'_>> {
        let fit_skills = self.sol.u_data.fits.get(self.key).skills.values();
        let skill_keys = fit_skills.map(|fit_skill| fit_skill.item_key).collect();
        MutIter::new(self.sol, skill_keys)
    }
}

fn iter_skills(sol: &SolarSystem, fit_key: UFitKey) -> impl ExactSizeIterator<Item = Skill<'_>> {
    let fit_skills = sol.u_data.fits.get(fit_key).skills.values();
    fit_skills.map(|fit_skill| Skill::new(sol, fit_skill.item_key))
}
