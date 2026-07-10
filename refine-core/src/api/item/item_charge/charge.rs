use crate::{
    api::{Fit, FitMut, Item, ItemCommon, ItemMut, ItemMutCommon, ItemMutSealed, ItemSealed},
    sol::SolarSystem,
    ud::{UCharge, UItemId},
};

// Charges expose no projection info, since it fully matches projections of the parent item
pub struct Charge<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) uid: UItemId,
}
impl<'a> Charge<'a> {
    pub(in crate::api) fn new(sol: &'a SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.uid)
    }
    pub fn get_cont_item(&self) -> Item<'_> {
        get_cont_item(self.sol, self.uid)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.uid)
    }
}
impl<'a> ItemSealed for Charge<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_uid(&self) -> UItemId {
        self.uid
    }
}
impl<'a> ItemCommon for Charge<'a> {}

pub struct ChargeMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) uid: UItemId,
}
impl<'a> ChargeMut<'a> {
    pub(in crate::api) fn new(sol: &'a mut SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.uid)
    }
    pub fn get_fit_mut(&mut self) -> FitMut<'_> {
        let fit_uid = get_u_charge(self.sol, self.uid).get_fit_uid();
        FitMut::new(self.sol, fit_uid)
    }
    pub fn get_cont_item(&self) -> Item<'_> {
        get_cont_item(self.sol, self.uid)
    }
    pub fn get_cont_item_mut(&mut self) -> ItemMut<'_> {
        let cont_uid = get_u_charge(self.sol, self.uid).get_cont_item_uid();
        ItemMut::new(self.sol, cont_uid)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.uid)
    }
}
impl<'a> ItemSealed for ChargeMut<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_uid(&self) -> UItemId {
        self.uid
    }
}
impl<'a> ItemMutSealed for ChargeMut<'a> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'a> ItemCommon for ChargeMut<'a> {}
impl<'a> ItemMutCommon for ChargeMut<'a> {}

fn get_fit(sol: &SolarSystem, charge_uid: UItemId) -> Fit<'_> {
    let fit_uid = get_u_charge(sol, charge_uid).get_fit_uid();
    Fit::new(sol, fit_uid)
}
fn get_cont_item(sol: &SolarSystem, charge_uid: UItemId) -> Item<'_> {
    let cont_uid = get_u_charge(sol, charge_uid).get_cont_item_uid();
    Item::new(sol, cont_uid)
}
fn get_state(sol: &SolarSystem, charge_uid: UItemId) -> bool {
    !get_u_charge(sol, charge_uid).get_force_disabled()
}
fn get_u_charge(sol: &SolarSystem, charge_uid: UItemId) -> &UCharge {
    sol.u_data.items.get(charge_uid).dc_charge().unwrap()
}
