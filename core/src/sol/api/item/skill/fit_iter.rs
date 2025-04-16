use crate::sol::{
    FitKey, SolarSystem,
    api::{Fit, FitMut, Skill},
};

impl<'a> Fit<'a> {
    pub fn iter_skills(&self) -> impl ExactSizeIterator<Item = Skill> {
        iter_skills(self.sol, self.key)
    }
}

impl<'a> FitMut<'a> {
    pub fn iter_skills(&self) -> impl ExactSizeIterator<Item = Skill> {
        iter_skills(self.sol, self.key)
    }
}

fn iter_skills(sol: &SolarSystem, fit_key: FitKey) -> impl ExactSizeIterator<Item = Skill> {
    sol.uad
        .fits
        .get(fit_key)
        .skills
        .values()
        .map(|fit_skill| Skill::new(sol, fit_skill.item_key))
}
