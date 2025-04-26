use lender::{Lender, Lending};

use crate::sol::{ItemKey, SolarSystem, api::RangedProjMut};

// Lending iterator for ranged projections
pub struct RangedProjIter<'iter> {
    sol: &'iter mut SolarSystem,
    key: ItemKey,
    projectee_keys: Vec<ItemKey>,
    index: usize,
}
impl<'iter> RangedProjIter<'iter> {
    pub(in crate::sol::api) fn new(sol: &'iter mut SolarSystem, key: ItemKey, projectee_keys: Vec<ItemKey>) -> Self {
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
    fn next(&mut self) -> Option<RangedProjMut> {
        let projectee_item_key = *self.projectee_keys.get(self.index)?;
        self.index += 1;
        Some(RangedProjMut::new(self.sol, self.key, projectee_item_key))
    }
}
