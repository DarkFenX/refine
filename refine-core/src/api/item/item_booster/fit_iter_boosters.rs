use crate::{Booster, EntityIter, Fit};

impl<'s> Fit<'s> {
    pub fn iter_boosters(&mut self) -> EntityIter<'_, Booster<'_>> {
        let booster_uids = self.sol.u_data.fits.get(self.uid).boosters.iter().copied().collect();
        EntityIter::new(self.sol, booster_uids)
    }
}
