use crate::api::{EntityIter, Fit, Rig};

impl<'s> Fit<'s> {
    pub fn iter_rigs(&mut self) -> EntityIter<'_, Rig<'_>> {
        let rig_uids = self.sol.u_data.fits.get(self.uid).rigs.iter().copied().collect();
        EntityIter::new(self.sol, rig_uids)
    }
}
