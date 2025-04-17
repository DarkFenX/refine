use std::collections::VecDeque;

use lender::{Lender, Lending};

use crate::sol::{
    ItemKey, SolarSystem,
    api::{Autocharge, AutochargeMut, Fighter, FighterMut},
};

// Lending iterator for autocharges
pub struct AutochargeIter<'this> {
    sol: &'this mut SolarSystem,
    autocharge_keys: VecDeque<ItemKey>,
}
impl<'this, 'lend> Lending<'lend> for AutochargeIter<'this> {
    type Lend = AutochargeMut<'lend>;
}
impl<'this> Lender for AutochargeIter<'this> {
    fn next(&mut self) -> Option<AutochargeMut> {
        let autocharge_key = self.autocharge_keys.pop_front()?;
        Some(AutochargeMut::new(self.sol, autocharge_key))
    }
}

impl<'a> Fighter<'a> {
    pub fn iter_autocharges(&self) -> impl Iterator<Item = Autocharge> {
        iter_autocharges(self.sol, self.key)
    }
}

impl<'a> FighterMut<'a> {
    pub fn iter_autocharges(&self) -> impl Iterator<Item = Autocharge> {
        iter_autocharges(self.sol, self.key)
    }
    pub fn iter_autocharges_mut(&mut self) -> AutochargeIter {
        let autocharge_keys = self
            .sol
            .uad
            .items
            .get(self.key)
            .get_fighter()
            .unwrap()
            .get_autocharges()
            .values()
            .copied()
            .collect();
        AutochargeIter {
            sol: self.sol,
            autocharge_keys,
        }
    }
}

fn iter_autocharges(sol: &SolarSystem, fighter_key: ItemKey) -> impl Iterator<Item = Autocharge> {
    sol.uad
        .items
        .get(fighter_key)
        .get_fighter()
        .unwrap()
        .get_autocharges()
        .values()
        .map(|&autocharge_key| Autocharge::new(sol, autocharge_key))
}
