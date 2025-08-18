use crate::{
    sol::{
        SolarSystem,
        api::{Fit, FitMut, ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed},
    },
    ud::{UCharacter, UItemKey},
};

pub struct Character<'a> {
    pub(in crate::sol::api) sol: &'a SolarSystem,
    pub(in crate::sol::api) key: UItemKey,
}
impl<'a> Character<'a> {
    pub(in crate::sol::api) fn new(sol: &'a SolarSystem, key: UItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.key)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key)
    }
}
impl<'a> ItemSealed for Character<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> UItemKey {
        self.key
    }
}
impl<'a> ItemCommon for Character<'a> {}

pub struct CharacterMut<'a> {
    pub(in crate::sol::api) sol: &'a mut SolarSystem,
    pub(in crate::sol::api) key: UItemKey,
}
impl<'a> CharacterMut<'a> {
    pub(in crate::sol::api) fn new(sol: &'a mut SolarSystem, key: UItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.key)
    }
    pub fn get_fit_mut(&mut self) -> FitMut<'_> {
        let fit_key = get_u_character(self.sol, self.key).get_fit_key();
        FitMut::new(self.sol, fit_key)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key)
    }
}
impl<'a> ItemSealed for CharacterMut<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> UItemKey {
        self.key
    }
}
impl<'a> ItemMutSealed for CharacterMut<'a> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'a> ItemCommon for CharacterMut<'a> {}
impl<'a> ItemMutCommon for CharacterMut<'a> {}

fn get_fit(sol: &SolarSystem, character_key: UItemKey) -> Fit<'_> {
    let fit_key = get_u_character(sol, character_key).get_fit_key();
    Fit::new(sol, fit_key)
}
fn get_state(sol: &SolarSystem, character_key: UItemKey) -> bool {
    get_u_character(sol, character_key).get_character_state()
}
fn get_u_character(sol: &SolarSystem, character_key: UItemKey) -> &UCharacter {
    sol.u_data.items.get(character_key).get_character().unwrap()
}
