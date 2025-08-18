use lender::{Lender, Lending};

use crate::{
    sol::{
        SolarSystem,
        api::{Proj, ProjEffect, ProjEffectMut, ProjMut, iter_projectee_keys},
    },
    ud::UItemKey,
};

// Lending iterator for non-ranged projections
pub struct ProjIter<'iter> {
    sol: &'iter mut SolarSystem,
    key: UItemKey,
    projectee_keys: Vec<UItemKey>,
    index: usize,
}
impl<'iter> ProjIter<'iter> {
    fn new(sol: &'iter mut SolarSystem, key: UItemKey, projectee_keys: Vec<UItemKey>) -> Self {
        Self {
            sol,
            key,
            projectee_keys,
            index: 0,
        }
    }
}
impl<'iter, 'lend> Lending<'lend> for ProjIter<'iter> {
    type Lend = ProjMut<'lend>;
}
impl<'iter> Lender for ProjIter<'iter> {
    fn next(&mut self) -> Option<ProjMut<'_>> {
        let projectee_key = *self.projectee_keys.get(self.index)?;
        self.index += 1;
        Some(ProjMut::new(self.sol, self.key, projectee_key))
    }
}

impl<'a> ProjEffect<'a> {
    /// Iterates over projected effect's projections.
    pub fn iter_projs(&self) -> impl ExactSizeIterator<Item = Proj<'_>> {
        iter_projs(self.sol, self.key)
    }
}

impl<'a> ProjEffectMut<'a> {
    /// Iterates over projected effect's projections.
    pub fn iter_projs(&self) -> impl ExactSizeIterator<Item = Proj<'_>> {
        iter_projs(self.sol, self.key)
    }
    /// Iterates over projected effect's projections.
    pub fn iter_projs_mut(&mut self) -> ProjIter<'_> {
        let projectee_keys = iter_projectee_keys(self.sol, self.key).collect();
        ProjIter::new(self.sol, self.key, projectee_keys)
    }
}

fn iter_projs(sol: &SolarSystem, proj_effect_key: UItemKey) -> impl ExactSizeIterator<Item = Proj<'_>> {
    iter_projectee_keys(sol, proj_effect_key).map(move |projectee_key| Proj::new(sol, projectee_key))
}
