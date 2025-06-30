use crate::{
    def::ItemKey,
    sol::{
        SolarSystem,
        api::{Fit, FitMut, Item, ItemCommon, ItemMut, ItemMutCommon, ItemMutSealed, ItemSealed},
    },
    uad::UadCharge,
};

// Charges expose no projection info, since it fully matches projections of the parent item
pub struct Charge<'a> {
    pub(in crate::sol::api) sol: &'a SolarSystem,
    pub(in crate::sol::api) key: ItemKey,
}
impl<'a> Charge<'a> {
    pub(in crate::sol::api) fn new(sol: &'a SolarSystem, key: ItemKey) -> Self {
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
    fn get_key(&self) -> ItemKey {
        self.key
    }
}
impl<'a> ItemCommon for Charge<'a> {}

pub struct ChargeMut<'a> {
    pub(in crate::sol::api) sol: &'a mut SolarSystem,
    pub(in crate::sol::api) key: ItemKey,
}
impl<'a> ChargeMut<'a> {
    pub(in crate::sol::api) fn new(sol: &'a mut SolarSystem, key: ItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.key)
    }
    pub fn get_fit_mut(&mut self) -> FitMut<'_> {
        let fit_key = get_uad_charge(self.sol, self.key).get_fit_key();
        FitMut::new(self.sol, fit_key)
    }
    pub fn get_cont_item(&self) -> Item<'_> {
        get_cont_item(self.sol, self.key)
    }
    pub fn get_cont_item_mut(&mut self) -> ItemMut<'_> {
        let cont_item_key = get_uad_charge(self.sol, self.key).get_cont_item_key();
        ItemMut::new(self.sol, cont_item_key)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key)
    }
}
impl<'a> ItemSealed for ChargeMut<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> ItemKey {
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

fn get_fit(sol: &SolarSystem, item_key: ItemKey) -> Fit<'_> {
    let fit_key = get_uad_charge(sol, item_key).get_fit_key();
    Fit::new(sol, fit_key)
}
fn get_cont_item(sol: &SolarSystem, item_key: ItemKey) -> Item<'_> {
    let cont_item_key = get_uad_charge(sol, item_key).get_cont_item_key();
    Item::new(sol, cont_item_key)
}
fn get_state(sol: &SolarSystem, item_key: ItemKey) -> bool {
    !get_uad_charge(sol, item_key).get_force_disable()
}
fn get_uad_charge(sol: &SolarSystem, item_key: ItemKey) -> &UadCharge {
    sol.uad.items.get(item_key).get_charge().unwrap()
}
