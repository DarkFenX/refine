use crate::{
    api::{Fit, FitMut, ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed},
    sol::SolarSystem,
    ud::{UItemId, URig},
};

pub struct Rig<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) key: UItemId,
}
impl<'a> Rig<'a> {
    pub(in crate::api) fn new(sol: &'a SolarSystem, key: UItemId) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.key)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key)
    }
}
impl<'a> ItemSealed for Rig<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> UItemId {
        self.key
    }
}
impl<'a> ItemCommon for Rig<'a> {}

pub struct RigMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) key: UItemId,
}
impl<'a> RigMut<'a> {
    pub(in crate::api) fn new(sol: &'a mut SolarSystem, key: UItemId) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.key)
    }
    pub fn get_fit_mut(&mut self) -> FitMut<'_> {
        let fit_key = get_u_rig(self.sol, self.key).get_fit_key();
        FitMut::new(self.sol, fit_key)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key)
    }
}
impl<'a> ItemSealed for RigMut<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> UItemId {
        self.key
    }
}
impl<'a> ItemMutSealed for RigMut<'a> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'a> ItemCommon for RigMut<'a> {}
impl<'a> ItemMutCommon for RigMut<'a> {}

fn get_fit(sol: &SolarSystem, rig_key: UItemId) -> Fit<'_> {
    let fit_key = get_u_rig(sol, rig_key).get_fit_key();
    Fit::new(sol, fit_key)
}
fn get_state(sol: &SolarSystem, rig_key: UItemId) -> bool {
    get_u_rig(sol, rig_key).get_rig_state()
}
fn get_u_rig(sol: &SolarSystem, rig_key: UItemId) -> &URig {
    sol.u_data.items.get(rig_key).dc_rig().unwrap()
}
