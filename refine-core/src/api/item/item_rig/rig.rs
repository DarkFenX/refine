use crate::{
    api::{Fit, FitMut, ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed},
    sol::SolarSystem,
    ud::{UItemId, URig},
};

pub struct Rig<'s> {
    pub(in crate::api) sol: &'s SolarSystem,
    pub(in crate::api) uid: UItemId,
}
impl<'s> Rig<'s> {
    pub(in crate::api) fn new(sol: &'s SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.uid)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.uid)
    }
}
impl<'s> ItemSealed for Rig<'s> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_uid(&self) -> UItemId {
        self.uid
    }
}
impl<'s> ItemCommon for Rig<'s> {}

pub struct RigMut<'s> {
    pub(in crate::api) sol: &'s mut SolarSystem,
    pub(in crate::api) uid: UItemId,
}
impl<'s> RigMut<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.uid)
    }
    pub fn get_fit_mut(&mut self) -> FitMut<'_> {
        let fit_uid = get_u_rig(self.sol, self.uid).get_fit_uid();
        FitMut::new(self.sol, fit_uid)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.uid)
    }
}
impl<'s> ItemSealed for RigMut<'s> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_uid(&self) -> UItemId {
        self.uid
    }
}
impl<'s> ItemMutSealed for RigMut<'s> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'s> ItemCommon for RigMut<'s> {}
impl<'s> ItemMutCommon for RigMut<'s> {}

fn get_fit(sol: &SolarSystem, rig_uid: UItemId) -> Fit<'_> {
    let fit_uid = get_u_rig(sol, rig_uid).get_fit_uid();
    Fit::new(sol, fit_uid)
}
fn get_state(sol: &SolarSystem, rig_uid: UItemId) -> bool {
    get_u_rig(sol, rig_uid).get_rig_state()
}
fn get_u_rig(sol: &SolarSystem, rig_uid: UItemId) -> &URig {
    sol.u_data.items.get(rig_uid).dc_rig().unwrap()
}
