use crate::{EntityIter, Fit, Fleet};

impl<'s> Fleet<'s> {
    pub fn iter_fits(&mut self) -> EntityIter<'_, Fit<'_>> {
        let fit_uids = self.sol.u_data.fleets.get(self.uid).iter_fits().collect();
        EntityIter::new(self.sol, fit_uids)
    }
}
