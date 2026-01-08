use lender::{Lender, Lending};

use crate::{
    api::{RangedProj, RangedProjMut, iter_projectee_uids},
    sol::SolarSystem,
    ud::UItemId,
};

// Lending iterator for ranged projections
pub struct RangedProjIter<'iter> {
    sol: &'iter mut SolarSystem,
    item_uid: UItemId,
    projectee_uids: Vec<UItemId>,
    index: usize,
}
impl<'iter> RangedProjIter<'iter> {
    pub(in crate::api) fn new(sol: &'iter mut SolarSystem, item_uid: UItemId) -> Self {
        let projectee_uids = iter_projectee_uids(sol, item_uid).collect();
        Self {
            sol,
            item_uid,
            projectee_uids,
            index: 0,
        }
    }
}
impl<'iter, 'lend> Lending<'lend> for RangedProjIter<'iter> {
    type Lend = RangedProjMut<'lend>;
}
impl<'iter> Lender for RangedProjIter<'iter> {
    fn next(&mut self) -> Option<RangedProjMut<'_>> {
        let projectee_uid = *self.projectee_uids.get(self.index)?;
        self.index += 1;
        Some(RangedProjMut::new(self.sol, self.item_uid, projectee_uid))
    }
}

pub(in crate::api) fn iter_ranged_projs(
    sol: &SolarSystem,
    item_uid: UItemId,
) -> impl ExactSizeIterator<Item = RangedProj<'_>> {
    iter_projectee_uids(sol, item_uid).map(move |projectee_uid| RangedProj::new(sol, item_uid, projectee_uid))
}
