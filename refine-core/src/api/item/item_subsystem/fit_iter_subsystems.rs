use crate::api::{EntityIter, Fit, Subsystem};

impl<'s> Fit<'s> {
    pub fn iter_subsystems(&mut self) -> EntityIter<'_, Subsystem<'_>> {
        let subsystem_uids = self.sol.u_data.fits.get(self.uid).subsystems.iter().copied().collect();
        EntityIter::new(self.sol, subsystem_uids)
    }
}
