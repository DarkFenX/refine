use lender::{ExactSizeLender, Lender, Lending, check_covariance};

use crate::{RangedProj, SolarSystem, api::item::shared::proj::iter_projectee_uids, ud::UItemId};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Lending iterator for ranged projections
pub struct RangedProjIter<'iter> {
    sol: &'iter mut SolarSystem,
    item_uid: UItemId,
    projectee_uids: Vec<UItemId>,
    index: usize,
}
impl<'iter, 'lend> Lending<'lend> for RangedProjIter<'iter> {
    type Lend = RangedProj<'lend>;
}
impl<'iter> Lender for RangedProjIter<'iter> {
    check_covariance!();

    fn next(&mut self) -> Option<RangedProj<'_>> {
        let projectee_uid = *self.projectee_uids.get(self.index)?;
        self.index += 1;
        Some(RangedProj::new(self.sol, self.item_uid, projectee_uid))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.projectee_uids.len() - self.index;
        (remaining, Some(remaining))
    }
}
impl<'iter> ExactSizeLender for RangedProjIter<'iter> {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
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
