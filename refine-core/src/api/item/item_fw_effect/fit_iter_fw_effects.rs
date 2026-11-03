use crate::{EntityIter, Fit, FwEffect};

impl<'s> Fit<'s> {
    pub fn iter_fw_effects(&mut self) -> EntityIter<'_, FwEffect<'_>> {
        let fw_effect_uids = self.sol.u_data.fits.get(self.uid).fw_effects.iter().copied().collect();
        EntityIter::new(self.sol, fw_effect_uids)
    }
}
