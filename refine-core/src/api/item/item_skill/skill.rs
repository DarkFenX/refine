use crate::{
    api::{Fit, FitMut, ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed},
    num::SkillLevel,
    sol::SolarSystem,
    ud::{UItemId, USkill},
};

pub struct Skill<'s> {
    pub(in crate::api) sol: &'s SolarSystem,
    pub(in crate::api) uid: UItemId,
}
impl<'s> Skill<'s> {
    pub(in crate::api) fn new(sol: &'s SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.uid)
    }
    pub fn get_level(&self) -> SkillLevel {
        get_level(self.sol, self.uid)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.uid)
    }
}
impl<'s> ItemSealed for Skill<'s> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_uid(&self) -> UItemId {
        self.uid
    }
}
impl<'s> ItemCommon for Skill<'s> {}

pub struct SkillMut<'s> {
    pub(in crate::api) sol: &'s mut SolarSystem,
    pub(in crate::api) uid: UItemId,
}
impl<'s> SkillMut<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.uid)
    }
    pub fn get_fit_mut(&mut self) -> FitMut<'_> {
        let fit_uid = get_u_skill(self.sol, self.uid).get_fit_uid();
        FitMut::new(self.sol, fit_uid)
    }
    pub fn get_level(&self) -> SkillLevel {
        get_level(self.sol, self.uid)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.uid)
    }
}
impl<'s> ItemSealed for SkillMut<'s> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_uid(&self) -> UItemId {
        self.uid
    }
}
impl<'s> ItemMutSealed for SkillMut<'s> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'s> ItemCommon for SkillMut<'s> {}
impl<'s> ItemMutCommon for SkillMut<'s> {}

fn get_fit(sol: &SolarSystem, skill_uid: UItemId) -> Fit<'_> {
    let fit_uid = get_u_skill(sol, skill_uid).get_fit_uid();
    Fit::new(sol, fit_uid)
}
fn get_level(sol: &SolarSystem, skill_uid: UItemId) -> SkillLevel {
    get_u_skill(sol, skill_uid).get_level()
}
fn get_state(sol: &SolarSystem, skill_uid: UItemId) -> bool {
    get_u_skill(sol, skill_uid).get_skill_state()
}
fn get_u_skill(sol: &SolarSystem, skill_uid: UItemId) -> &USkill {
    sol.u_data.items.get(skill_uid).dc_skill().unwrap()
}
