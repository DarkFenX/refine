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
    fighter_uid: UItemId,
    abil_aids: Vec<AAbilId>,
    index: usize,
}
impl<'iter> AbilityIter<'iter> {
    fn new(sol: &'iter mut SolarSystem, fighter_uid: UItemId, abil_aids: Vec<AAbilId>) -> Self {
        Self {
            sol,
            fighter_uid,
            abil_aids,
            index: 0,
        }
    }
}
impl<'iter, 'lend> Lending<'lend> for AbilityIter<'iter> {
    type Lend = AbilityMut<'lend>;
}
impl<'iter> Lender for AbilityIter<'iter> {
    fn next(&mut self) -> Option<AbilityMut<'_>> {
        let abil_aid = *self.abil_aids.get(self.index)?;
        self.index += 1;
        Some(AbilityMut::new(self.sol, self.fighter_uid, abil_aid))
    }
}

impl<'a> Fighter<'a> {
    /// Iterates over fighter's abilities.
    pub fn iter_abilities(&self) -> impl Iterator<Item = Ability<'_>> {
        iter_abils(self.sol, self.uid)
    }
}

impl<'a> FighterMut<'a> {
    /// Iterates over fighter's abilities.
    pub fn iter_abilities(&self) -> impl Iterator<Item = Ability<'_>> {
        iter_abils(self.sol, self.uid)
    }
    /// Iterates over fighter's abilities.
    pub fn iter_abilities_mut(&mut self) -> AbilityIter<'_> {
        let abil_aids = get_abil_aids(self.sol, self.uid);
        AbilityIter::new(self.sol, self.uid, abil_aids)
    }
}

fn iter_abils(sol: &SolarSystem, fighter_uid: UItemId) -> impl Iterator<Item = Ability<'_>> {
    get_abil_aids(sol, fighter_uid)
        .into_iter()
        .map(move |abil_aid| Ability::new(sol, fighter_uid, abil_aid))
}

fn get_abil_aids(sol: &SolarSystem, fighter_uid: UItemId) -> Vec<AAbilId> {
    match sol.u_data.items.get(fighter_uid).dc_fighter().unwrap().get_abils() {
        Some(abil_aids) => abil_aids.clone(),
        None => Vec::new(),
    }
}
