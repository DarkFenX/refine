use crate::{
    api::{Fit, ItemCommon, ItemSealed},
    sol::SolarSystem,
    ud::{UItemId, UStance},
};

pub struct Stance<'s> {
    pub(super) sol: &'s mut SolarSystem,
    pub(super) uid: UItemId,
}
impl<'s> Stance<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fit(&mut self) -> Fit<'_> {
        let fit_uid = get_u_stance(self.sol, self.uid).get_fit_uid();
        Fit::new(self.sol, fit_uid)
    }
    pub fn get_state(&self) -> bool {
        get_u_stance(self.sol, self.uid).get_stance_state()
    }
}
impl<'s> ItemSealed for Stance<'s> {
    fn get_uid(&self) -> UItemId {
        self.uid
    }
    fn get_sol_ref(&self) -> &SolarSystem {
        self.sol
    }
}
impl<'s> ItemCommon for Stance<'s> {
    fn get_sol(&mut self) -> &mut SolarSystem {
        self.sol
    }
}

fn get_u_stance(sol: &SolarSystem, stance_uid: UItemId) -> &UStance {
    sol.u_data.items.get(stance_uid).dc_stance().unwrap()
}
