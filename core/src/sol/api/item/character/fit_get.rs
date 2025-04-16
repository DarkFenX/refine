use crate::sol::{
    FitKey, SolarSystem,
    api::{Character, CharacterMut, Fit, FitMut},
};

impl<'a> Fit<'a> {
    pub fn get_character(&'a self) -> Option<Character<'a>> {
        get_character(self.sol, self.key)
    }
}

impl<'a> FitMut<'a> {
    pub fn get_character(&'a self) -> Option<Character<'a>> {
        get_character(self.sol, self.key)
    }
    pub fn get_character_mut(&'a mut self) -> Option<CharacterMut<'a>> {
        self.sol
            .uad
            .fits
            .get(self.key)
            .character
            .map(|item_key| CharacterMut::new(self.sol, item_key))
    }
}

fn get_character(sol: &SolarSystem, fit_key: FitKey) -> Option<Character> {
    sol.uad
        .fits
        .get(fit_key)
        .character
        .map(|item_key| Character::new(sol, item_key))
}
