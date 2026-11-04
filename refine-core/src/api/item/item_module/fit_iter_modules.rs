use crate::{EntityIter, Fit, ModRack, Module};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Fit<'s> {
    /// Iterates over modules of a rack, ordered by position.
    pub fn iter_modules(&mut self, rack: ModRack) -> EntityIter<'_, Module<'_>> {
        let u_fit = self.core.u_data.fits.get(self.uid);
        let u_module_vec = match rack {
            ModRack::High => &u_fit.mods_high,
            ModRack::Mid => &u_fit.mods_mid,
            ModRack::Low => &u_fit.mods_low,
        };
        let module_uids = u_module_vec.iter_uids().collect();
        EntityIter::new(self.core, module_uids)
    }
}
