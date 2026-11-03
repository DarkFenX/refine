use crate::{
    api::{Fit, ItemCommon, ItemSealed},
    sol::SolarSystem,
    ud::{UCharacter, UItemId},
};

pub struct Character<'s> {
    pub(super) sol: &'s mut SolarSystem,
    pub(super) uid: UItemId,
}
impl<'s> Character<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fit(&mut self) -> Fit<'_> {
        let fit_uid = get_u_character(self.sol, self.uid).get_fit_uid();
        Fit::new(self.sol, fit_uid)
    }
    pub fn get_state(&self) -> bool {
        get_u_character(self.sol, self.uid).get_character_state()
    }
}
impl<'s> ItemSealed for Character<'s> {
    fn get_uid(&self) -> UItemId {
        self.uid
    }
    fn get_sol_ref(&self) -> &SolarSystem {
        self.sol
    }
}
impl<'s> ItemCommon for Character<'s> {
    fn get_sol(&mut self) -> &mut SolarSystem {
        self.sol
    }
}

fn get_u_character(sol: &SolarSystem, character_uid: UItemId) -> &UCharacter {
    sol.u_data.items.get(character_uid).dc_character().unwrap()
}
