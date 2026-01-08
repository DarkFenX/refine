use crate::{
    api::{Character, CharacterMut, Fit, FitMut},
    sol::SolarSystem,
    ud::UFitId,
};

impl<'a> Fit<'a> {
    pub fn get_character(&self) -> Option<Character<'_>> {
        get_character(self.sol, self.uid)
    }
}

impl<'a> FitMut<'a> {
    pub fn get_character(&self) -> Option<Character<'_>> {
        get_character(self.sol, self.uid)
    }
    pub fn get_character_mut(&mut self) -> Option<CharacterMut<'_>> {
        self.sol
            .u_data
            .fits
            .get(self.uid)
            .character
            .map(|character_uid| CharacterMut::new(self.sol, character_uid))
    }
}

fn get_character(sol: &SolarSystem, fit_uid: UFitId) -> Option<Character<'_>> {
    sol.u_data
        .fits
        .get(fit_uid)
        .character
        .map(|character_uid| Character::new(sol, character_uid))
}
