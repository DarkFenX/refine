use crate::{
    api::{EffectId, Fit, FitMut, Item, ItemCommon, ItemMut, ItemMutCommon, ItemMutSealed, ItemSealed},
    sol::SolarSystem,
    ud::{UAutocharge, UItemId},
};

// Autocharges expose no projection info, since it fully matches projections of the parent item
pub struct Autocharge<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) key: UItemId,
}
impl<'a> Autocharge<'a> {
    pub(in crate::api) fn new(sol: &'a SolarSystem, key: UItemId) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.key)
    }
    pub fn get_cont_item(&self) -> Item<'_> {
        get_cont_item(self.sol, self.key)
    }
    pub fn get_cont_effect_id(&self) -> EffectId {
        get_cont_effect_id(self.sol, self.key)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key)
    }
}
impl<'a> ItemSealed for Autocharge<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> UItemId {
        self.key
    }
}
impl<'a> ItemCommon for Autocharge<'a> {}

pub struct AutochargeMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) key: UItemId,
}
impl<'a> AutochargeMut<'a> {
    pub(in crate::api) fn new(sol: &'a mut SolarSystem, key: UItemId) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.key)
    }
    pub fn get_fit_mut(&mut self) -> FitMut<'_> {
        let fit_key = get_u_autocharge(self.sol, self.key).get_fit_key();
        FitMut::new(self.sol, fit_key)
    }
    pub fn get_cont_item(&self) -> Item<'_> {
        get_cont_item(self.sol, self.key)
    }
    pub fn get_cont_item_mut(&mut self) -> ItemMut<'_> {
        let cont_key = get_u_autocharge(self.sol, self.key).get_cont_item_key();
        ItemMut::new(self.sol, cont_key)
    }
    pub fn get_cont_effect_id(&self) -> EffectId {
        get_cont_effect_id(self.sol, self.key)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key)
    }
}
impl<'a> ItemSealed for AutochargeMut<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> UItemId {
        self.key
    }
}
impl<'a> ItemMutSealed for AutochargeMut<'a> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'a> ItemCommon for AutochargeMut<'a> {}
impl<'a> ItemMutCommon for AutochargeMut<'a> {}

fn get_fit(sol: &SolarSystem, autocharge_key: UItemId) -> Fit<'_> {
    let fit_key = get_u_autocharge(sol, autocharge_key).get_fit_key();
    Fit::new(sol, fit_key)
}
fn get_cont_item(sol: &SolarSystem, autocharge_key: UItemId) -> Item<'_> {
    let cont_key = get_u_autocharge(sol, autocharge_key).get_cont_item_key();
    Item::new(sol, cont_key)
}
fn get_cont_effect_id(sol: &SolarSystem, autocharge_key: UItemId) -> EffectId {
    let cont_effect_key = get_u_autocharge(sol, autocharge_key).get_cont_effect_key();
    sol.u_data.src.get_effect(cont_effect_key).id.into()
}
fn get_state(sol: &SolarSystem, autocharge_key: UItemId) -> bool {
    !get_u_autocharge(sol, autocharge_key).get_force_disabled()
}
fn get_u_autocharge(sol: &SolarSystem, autocharge_key: UItemId) -> &UAutocharge {
    sol.u_data.items.get(autocharge_key).dc_autocharge().unwrap()
}
