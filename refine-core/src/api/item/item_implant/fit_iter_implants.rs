use crate::{
    api::{Fit, FitMut, Implant, ImplantMut, MutIter},
    sol::SolarSystem,
    ud::UFitId,
};

impl<'a> Fit<'a> {
    pub fn iter_implants(&self) -> impl ExactSizeIterator<Item = Implant<'_>> {
        iter_implants(self.sol, self.uid)
    }
}

impl<'a> FitMut<'a> {
    pub fn iter_implants(&self) -> impl ExactSizeIterator<Item = Implant<'_>> {
        iter_implants(self.sol, self.uid)
    }
    pub fn iter_implants_mut(&mut self) -> MutIter<'_, ImplantMut<'_>> {
        let implant_uids = self.sol.u_data.fits.get(self.uid).implants.iter().copied().collect();
        MutIter::new(self.sol, implant_uids)
    }
}

fn iter_implants(sol: &SolarSystem, fit_uid: UFitId) -> impl ExactSizeIterator<Item = Implant<'_>> {
    let implant_uids = sol.u_data.fits.get(fit_uid).implants.iter();
    implant_uids.map(|implant_uid| Implant::new(sol, *implant_uid))
}
