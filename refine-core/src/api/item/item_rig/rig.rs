use crate::{
    api::{Fit, ItemCommon, ItemSealed},
    sol::SolarSystem,
    ud::{UItemId, URig},
};

pub struct Rig<'s> {
    pub(super) sol: &'s mut SolarSystem,
    pub(super) uid: UItemId,
}
impl<'s> Rig<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fit(&mut self) -> Fit<'_> {
        let fit_uid = get_u_rig(self.sol, self.uid).get_fit_uid();
        Fit::new(self.sol, fit_uid)
    }
    pub fn get_state(&self) -> bool {
        get_u_rig(self.sol, self.uid).get_rig_state()
    }
}
impl<'s> ItemSealed for Rig<'s> {
    fn get_uid(&self) -> UItemId {
        self.uid
    }
    fn get_sol_ref(&self) -> &SolarSystem {
        self.sol
    }
}
impl<'s> ItemCommon for Rig<'s> {
    fn get_sol(&mut self) -> &mut SolarSystem {
        self.sol
    }
}

fn get_u_rig(sol: &SolarSystem, rig_uid: UItemId) -> &URig {
    sol.u_data.items.get(rig_uid).dc_rig().unwrap()
}
