use lender::{Lender, Lending};

use crate::{
    api::{Proj, ProjEffect, ProjEffectMut, ProjMut, iter_projectee_uids},
    sol::SolarSystem,
    ud::UItemId,
};

// Lending iterator for non-ranged projections
pub struct ProjIter<'iter> {
    sol: &'iter mut SolarSystem,
    item_uid: UItemId,
    projectee_uids: Vec<UItemId>,
    index: usize,
}
impl<'iter> ProjIter<'iter> {
    fn new(sol: &'iter mut SolarSystem, item_uid: UItemId, projectee_uids: Vec<UItemId>) -> Self {
        Self {
            sol,
            item_uid,
            projectee_uids,
            index: 0,
        }
    }
}
impl<'iter, 'lend> Lending<'lend> for ProjIter<'iter> {
    type Lend = ProjMut<'lend>;
}
impl<'iter> Lender for ProjIter<'iter> {
    fn next(&mut self) -> Option<ProjMut<'_>> {
        let projectee_uid = *self.projectee_uids.get(self.index)?;
        self.index += 1;
        Some(ProjMut::new(self.sol, self.item_uid, projectee_uid))
    }
}

impl<'a> ProjEffect<'a> {
    /// Iterates over projected effect's projections.
    pub fn iter_projs(&self) -> impl ExactSizeIterator<Item = Proj<'_>> {
        iter_projs(self.sol, self.uid)
    }
}

impl<'a> ProjEffectMut<'a> {
    /// Iterates over projected effect's projections.
    pub fn iter_projs(&self) -> impl ExactSizeIterator<Item = Proj<'_>> {
        iter_projs(self.sol, self.uid)
    }
    /// Iterates over projected effect's projections.
    pub fn iter_projs_mut(&mut self) -> ProjIter<'_> {
        let projectee_uids = iter_projectee_uids(self.sol, self.uid).collect();
        ProjIter::new(self.sol, self.uid, projectee_uids)
    }
}

fn iter_projs(sol: &SolarSystem, proj_effect_uid: UItemId) -> impl ExactSizeIterator<Item = Proj<'_>> {
    iter_projectee_uids(sol, proj_effect_uid).map(move |projectee_uid| Proj::new(sol, projectee_uid))
}
