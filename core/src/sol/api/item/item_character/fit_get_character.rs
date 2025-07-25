use crate::{
    sol::{
        SolarSystem,
        api::{Character, CharacterMut, Fit, FitMut},
    },
    ud::UFitKey,
};

impl<'a> Fit<'a> {
    pub fn get_character(&self) -> Option<Character<'_>> {
        get_character(self.sol, self.key)
    }
}

impl<'a> FitMut<'a> {
    pub fn get_character(&self) -> Option<Character<'_>> {
        get_character(self.sol, self.key)
    }
    pub fn get_character_mut(&mut self) -> Option<CharacterMut<'_>> {
        self.sol
            .u_data
            .fits
            .get(self.key)
            .character
            .map(|item_key| CharacterMut::new(self.sol, item_key))
    }
}

fn get_character(sol: &SolarSystem, fit_key: UFitKey) -> Option<Character<'_>> {
    sol.u_data
        .fits
        .get(fit_key)
        .character
        .map(|item_key| Character::new(sol, item_key))
}
