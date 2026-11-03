use crate::api::{EntityIter, Fit, Implant};

impl<'s> Fit<'s> {
    pub fn iter_implants(&mut self) -> EntityIter<'_, Implant<'_>> {
        let implant_uids = self.sol.u_data.fits.get(self.uid).implants.iter().copied().collect();
        EntityIter::new(self.sol, implant_uids)
    }
}
