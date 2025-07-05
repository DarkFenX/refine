use lender::{Lender, Lending};

use crate::{
    def::ItemKey,
    sol::{
        SolarSystem,
        api::{RangedProj, RangedProjMut, iter_projectee_keys},
    },
};

// Lending iterator for ranged projections
pub struct RangedProjIter<'iter> {
    sol: &'iter mut SolarSystem,
    key: ItemKey,
    projectee_keys: Vec<ItemKey>,
    index: usize,
}
impl<'iter> RangedProjIter<'iter> {
    pub(in crate::sol::api) fn new(sol: &'iter mut SolarSystem, key: ItemKey) -> Self {
        let projectee_keys = iter_projectee_keys(sol, key).collect();
        Self {
            sol,
            key,
            projectee_keys,
            index: 0,
        }
    }
}
impl<'iter, 'lend> Lending<'lend> for RangedProjIter<'iter> {
    type Lend = RangedProjMut<'lend>;
}
impl<'iter> Lender for RangedProjIter<'iter> {
    fn next(&mut self) -> Option<RangedProjMut<'_>> {
        let projectee_key = *self.projectee_keys.get(self.index)?;
        self.index += 1;
        Some(RangedProjMut::new(self.sol, self.key, projectee_key))
    }
}

pub(in crate::sol::api) fn iter_ranged_projs(
    sol: &SolarSystem,
    item_key: ItemKey,
) -> impl ExactSizeIterator<Item = RangedProj<'_>> {
    iter_projectee_keys(sol, item_key).map(move |projectee_key| RangedProj::new(sol, item_key, projectee_key))
}
