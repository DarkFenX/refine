use crate::{FitId, SolarSystem, ud::UFitId};

pub struct Fit<'s> {
    pub(in crate::api) sol: &'s mut SolarSystem,
    pub(in crate::api) uid: UFitId,
}
impl<'s> Fit<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolarSystem, uid: UFitId) -> Self {
        Self { sol, uid }
    }
    pub fn get_sol(&mut self) -> &mut SolarSystem {
        self.sol
    }
    pub fn get_fit_id(&self) -> FitId {
        self.sol.u_data.fits.ext_id_by_int_id(self.uid)
    }
}
