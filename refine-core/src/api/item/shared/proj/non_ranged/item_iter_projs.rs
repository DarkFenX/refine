use lender::{ExactSizeLender, Lender, Lending, check_covariance};

use crate::{Proj, ProjMut, SolarSystem, api::item::shared::proj::iter_projectee_uids, ud::UItemId};

// Lending iterator for non-ranged projections
pub struct ProjIter<'iter> {
    sol: &'iter mut SolarSystem,
    item_uid: UItemId,
    projectee_uids: Vec<UItemId>,
    index: usize,
}
impl<'iter> ProjIter<'iter> {
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
impl<'iter, 'lend> Lending<'lend> for ProjIter<'iter> {
    type Lend = ProjMut<'lend>;
}
impl<'iter> Lender for ProjIter<'iter> {
    check_covariance!();

    fn next(&mut self) -> Option<ProjMut<'_>> {
        let projectee_uid = *self.projectee_uids.get(self.index)?;
        self.index += 1;
        Some(ProjMut::new(self.sol, self.item_uid, projectee_uid))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.projectee_uids.len() - self.index;
        (remaining, Some(remaining))
    }
}
impl<'iter> ExactSizeLender for ProjIter<'iter> {}

pub(in crate::api) fn iter_projs(sol: &SolarSystem, item_uid: UItemId) -> impl ExactSizeIterator<Item = Proj<'_>> {
    iter_projectee_uids(sol, item_uid).map(move |projectee_uid| Proj::new(sol, projectee_uid))
}
