use crate::{
    api::{Fit, FitMut, Item, ItemCommon, ItemMut, ItemMutCommon, ItemMutSealed, ItemSealed},
    sol::SolarSystem,
    ud::{UCharge, UItemId},
};

// Charges expose no projection info, since it fully matches projections of the parent item
pub struct Charge<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) key: UItemId,
}
impl<'a> Charge<'a> {
    pub(in crate::api) fn new(sol: &'a SolarSystem, key: UItemId) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.key)
    }
    pub fn get_cont_item(&self) -> Item<'_> {
        get_cont_item(self.sol, self.key)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key)
    }
}
impl<'a> ItemSealed for Charge<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> UItemId {
        self.key
    }
}
impl<'a> ItemCommon for Charge<'a> {}

pub struct ChargeMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) key: UItemId,
}
impl<'a> ChargeMut<'a> {
    pub(in crate::api) fn new(sol: &'a mut SolarSystem, key: UItemId) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.key)
    }
    pub fn get_fit_mut(&mut self) -> FitMut<'_> {
        let fit_key = get_u_charge(self.sol, self.key).get_fit_key();
        FitMut::new(self.sol, fit_key)
    }
    pub fn get_cont_item(&self) -> Item<'_> {
        get_cont_item(self.sol, self.key)
    }
    pub fn get_cont_item_mut(&mut self) -> ItemMut<'_> {
        let cont_key = get_u_charge(self.sol, self.key).get_cont_item_key();
        ItemMut::new(self.sol, cont_key)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key)
    }
}
impl<'a> ItemSealed for ChargeMut<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> UItemId {
        self.key
    }
}
impl<'a> ItemMutSealed for ChargeMut<'a> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'a> ItemCommon for ChargeMut<'a> {}
impl<'a> ItemMutCommon for ChargeMut<'a> {}

fn get_fit(sol: &SolarSystem, charge_key: UItemId) -> Fit<'_> {
    let fit_key = get_u_charge(sol, charge_key).get_fit_key();
    Fit::new(sol, fit_key)
}
fn get_cont_item(sol: &SolarSystem, charge_key: UItemId) -> Item<'_> {
    let cont_key = get_u_charge(sol, charge_key).get_cont_item_key();
    Item::new(sol, cont_key)
}
fn get_state(sol: &SolarSystem, charge_key: UItemId) -> bool {
    !get_u_charge(sol, charge_key).get_force_disabled()
}
fn get_u_charge(sol: &SolarSystem, charge_key: UItemId) -> &UCharge {
    sol.u_data.items.get(charge_key).dc_charge().unwrap()
}
