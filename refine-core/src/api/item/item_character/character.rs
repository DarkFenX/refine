use crate::{
    api::{Fit, FitMut, ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed},
    sol::SolarSystem,
    ud::{UCharacter, UItemId},
};

pub struct Character<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) uid: UItemId,
}
impl<'a> Character<'a> {
    pub(in crate::api) fn new(sol: &'a SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.uid)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.uid)
    }
}
impl<'a> ItemSealed for Character<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_uid(&self) -> UItemId {
        self.uid
    }
}
impl<'a> ItemCommon for Character<'a> {}

pub struct CharacterMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) uid: UItemId,
}
impl<'a> CharacterMut<'a> {
    pub(in crate::api) fn new(sol: &'a mut SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.uid)
    }
    pub fn get_fit_mut(&mut self) -> FitMut<'_> {
        let fit_uid = get_u_character(self.sol, self.uid).get_fit_uid();
        FitMut::new(self.sol, fit_uid)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.uid)
    }
}
impl<'a> ItemSealed for CharacterMut<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_uid(&self) -> UItemId {
        self.uid
    }
}
impl<'a> ItemMutSealed for CharacterMut<'a> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'a> ItemCommon for CharacterMut<'a> {}
impl<'a> ItemMutCommon for CharacterMut<'a> {}

fn get_fit(sol: &SolarSystem, character_uid: UItemId) -> Fit<'_> {
    let fit_uid = get_u_character(sol, character_uid).get_fit_uid();
    Fit::new(sol, fit_uid)
}
fn get_state(sol: &SolarSystem, character_uid: UItemId) -> bool {
    get_u_character(sol, character_uid).get_character_state()
}
fn get_u_character(sol: &SolarSystem, character_uid: UItemId) -> &UCharacter {
    sol.u_data.items.get(character_uid).dc_character().unwrap()
}
