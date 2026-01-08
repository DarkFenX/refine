use crate::{
    api::{Fit, FitMut, ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed},
    misc::SkillLevel,
    sol::SolarSystem,
    ud::{UItemId, USkill},
};

pub struct Skill<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) uid: UItemId,
}
impl<'a> Skill<'a> {
    pub(in crate::api) fn new(sol: &'a SolarSystem, uid: UItemId) -> Self {
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
impl<'a> ItemSealed for Skill<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_uid(&self) -> UItemId {
        self.uid
    }
}
impl<'a> ItemCommon for Skill<'a> {}

pub struct SkillMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) uid: UItemId,
}
impl<'a> SkillMut<'a> {
    pub(in crate::api) fn new(sol: &'a mut SolarSystem, uid: UItemId) -> Self {
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
impl<'a> ItemSealed for SkillMut<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_uid(&self) -> UItemId {
        self.uid
    }
}
impl<'a> ItemMutSealed for SkillMut<'a> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'a> ItemCommon for SkillMut<'a> {}
impl<'a> ItemMutCommon for SkillMut<'a> {}

fn get_fit(sol: &SolarSystem, skill_uid: UItemId) -> Fit<'_> {
    let fit_uid = get_u_skill(sol, skill_uid).get_fit_uid();
    Fit::new(sol, fit_uid)
}
fn get_level(sol: &SolarSystem, skill_uid: UItemId) -> SkillLevel {
    get_u_skill(sol, skill_uid).get_level().into()
}
fn get_state(sol: &SolarSystem, skill_uid: UItemId) -> bool {
    get_u_skill(sol, skill_uid).get_skill_state()
}
fn get_u_skill(sol: &SolarSystem, skill_uid: UItemId) -> &USkill {
    sol.u_data.items.get(skill_uid).dc_skill().unwrap()
}
