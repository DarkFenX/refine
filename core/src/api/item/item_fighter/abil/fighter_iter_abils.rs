use lender::{Lender, Lending};

use crate::{
    ad::AAbilId,
    api::{Ability, AbilityMut, Fighter, FighterMut},
    sol::SolarSystem,
    ud::UItemId,
};

// Lending iterator for fighter abilities
pub struct AbilityIter<'iter> {
    sol: &'iter mut SolarSystem,
    fighter_key: UItemId,
    abil_ids: Vec<AAbilId>,
    index: usize,
}
impl<'iter> AbilityIter<'iter> {
    fn new(sol: &'iter mut SolarSystem, fighter_key: UItemId, abil_ids: Vec<AAbilId>) -> Self {
        Self {
            sol,
            fighter_key,
            abil_ids,
            index: 0,
        }
    }
}
impl<'iter, 'lend> Lending<'lend> for AbilityIter<'iter> {
    type Lend = AbilityMut<'lend>;
}
impl<'iter> Lender for AbilityIter<'iter> {
    fn next(&mut self) -> Option<AbilityMut<'_>> {
        let abil_id = *self.abil_ids.get(self.index)?;
        self.index += 1;
        Some(AbilityMut::new(self.sol, self.fighter_key, abil_id))
    }
}

impl<'a> Fighter<'a> {
    /// Iterates over fighter's abilities.
    pub fn iter_abilities(&self) -> impl Iterator<Item = Ability<'_>> {
        iter_abils(self.sol, self.key)
    }
}

impl<'a> FighterMut<'a> {
    /// Iterates over fighter's abilities.
    pub fn iter_abilities(&self) -> impl Iterator<Item = Ability<'_>> {
        iter_abils(self.sol, self.key)
    }
    /// Iterates over fighter's abilities.
    pub fn iter_abilities_mut(&mut self) -> AbilityIter<'_> {
        let abil_ids = get_abil_ids(self.sol, self.key);
        AbilityIter::new(self.sol, self.key, abil_ids)
    }
}

fn iter_abils(sol: &SolarSystem, fighter_key: UItemId) -> impl Iterator<Item = Ability<'_>> {
    get_abil_ids(sol, fighter_key)
        .into_iter()
        .map(move |abil_id| Ability::new(sol, fighter_key, abil_id))
}

fn get_abil_ids(sol: &SolarSystem, fighter_key: UItemId) -> Vec<AAbilId> {
    match sol.u_data.items.get(fighter_key).dc_fighter().unwrap().get_abils() {
        Some(abil_keys) => abil_keys.clone(),
        None => Vec::new(),
    }
}
