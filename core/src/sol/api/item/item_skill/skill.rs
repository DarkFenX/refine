use crate::{
    misc::SkillLevel,
    sol::{
        SolarSystem,
        api::{Fit, FitMut, ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed},
    },
    ud::{UItemKey, USkill},
};

pub struct Skill<'a> {
    pub(in crate::sol::api) sol: &'a SolarSystem,
    pub(in crate::sol::api) key: UItemKey,
}
impl<'a> Skill<'a> {
    pub(in crate::sol::api) fn new(sol: &'a SolarSystem, key: UItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.key)
    }
    pub fn get_level(&self) -> SkillLevel {
        get_level(self.sol, self.key)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key)
    }
}
impl<'a> ItemSealed for Skill<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> UItemKey {
        self.key
    }
}
impl<'a> ItemCommon for Skill<'a> {}

pub struct SkillMut<'a> {
    pub(in crate::sol::api) sol: &'a mut SolarSystem,
    pub(in crate::sol::api) key: UItemKey,
}
impl<'a> SkillMut<'a> {
    pub(in crate::sol::api) fn new(sol: &'a mut SolarSystem, key: UItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.key)
    }
    pub fn get_fit_mut(&mut self) -> FitMut<'_> {
        let fit_key = get_u_skill(self.sol, self.key).get_fit_key();
        FitMut::new(self.sol, fit_key)
    }
    pub fn get_level(&self) -> SkillLevel {
        get_level(self.sol, self.key)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key)
    }
}
impl<'a> ItemSealed for SkillMut<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> UItemKey {
        self.key
    }
}
impl<'a> ItemMutSealed for SkillMut<'a> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'a> ItemCommon for SkillMut<'a> {}
impl<'a> ItemMutCommon for SkillMut<'a> {}

fn get_fit(sol: &SolarSystem, item_key: UItemKey) -> Fit<'_> {
    let fit_key = get_u_skill(sol, item_key).get_fit_key();
    Fit::new(sol, fit_key)
}
fn get_level(sol: &SolarSystem, item_key: UItemKey) -> SkillLevel {
    get_u_skill(sol, item_key).get_a_level().into()
}
fn get_state(sol: &SolarSystem, item_key: UItemKey) -> bool {
    get_u_skill(sol, item_key).get_skill_state()
}
fn get_u_skill(sol: &SolarSystem, item_key: UItemKey) -> &USkill {
    sol.u_data.items.get(item_key).get_skill().unwrap()
}
