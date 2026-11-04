use crate::{EntityIter, Fit, Subsystem};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Fit<'s> {
    pub fn iter_subsystems(&mut self) -> EntityIter<'_, Subsystem<'_>> {
        let subsystem_uids = self.core.u_data.fits.get(self.uid).subsystems.iter().copied().collect();
        EntityIter::new(self.core, subsystem_uids)
    }
}
