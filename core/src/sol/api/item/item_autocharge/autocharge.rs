use crate::{
    misc::EffectId,
    sol::{
        SolarSystem,
        api::{Fit, FitMut, Item, ItemCommon, ItemMut, ItemMutCommon, ItemMutSealed, ItemSealed},
    },
    uad::{UadAutocharge, UadItemKey},
};

// Autocharges expose no projection info, since it fully matches projections of the parent item
pub struct Autocharge<'a> {
    pub(in crate::sol::api) sol: &'a SolarSystem,
    pub(in crate::sol::api) key: UadItemKey,
}
impl<'a> Autocharge<'a> {
    pub(in crate::sol::api) fn new(sol: &'a SolarSystem, key: UadItemKey) -> Self {
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
    fn get_key(&self) -> UadItemKey {
        self.key
    }
}
impl<'a> ItemCommon for Autocharge<'a> {}

pub struct AutochargeMut<'a> {
    pub(in crate::sol::api) sol: &'a mut SolarSystem,
    pub(in crate::sol::api) key: UadItemKey,
}
impl<'a> AutochargeMut<'a> {
    pub(in crate::sol::api) fn new(sol: &'a mut SolarSystem, key: UadItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.key)
    }
    pub fn get_fit_mut(&mut self) -> FitMut<'_> {
        let fit_key = get_uad_autocharge(self.sol, self.key).get_fit_key();
        FitMut::new(self.sol, fit_key)
    }
    pub fn get_cont_item(&self) -> Item<'_> {
        get_cont_item(self.sol, self.key)
    }
    pub fn get_cont_item_mut(&mut self) -> ItemMut<'_> {
        let cont_key = get_uad_autocharge(self.sol, self.key).get_cont_key();
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
    fn get_key(&self) -> UadItemKey {
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

fn get_fit(sol: &SolarSystem, item_key: UadItemKey) -> Fit<'_> {
    let fit_key = get_uad_autocharge(sol, item_key).get_fit_key();
    Fit::new(sol, fit_key)
}
fn get_cont_item(sol: &SolarSystem, item_key: UadItemKey) -> Item<'_> {
    let cont_key = get_uad_autocharge(sol, item_key).get_cont_key();
    Item::new(sol, cont_key)
}
fn get_cont_effect_id(sol: &SolarSystem, item_key: UadItemKey) -> EffectId {
    get_uad_autocharge(sol, item_key).get_cont_effect_id().into()
}
fn get_state(sol: &SolarSystem, item_key: UadItemKey) -> bool {
    !get_uad_autocharge(sol, item_key).get_force_disable()
}
fn get_uad_autocharge(sol: &SolarSystem, item_key: UadItemKey) -> &UadAutocharge {
    sol.uad.items.get(item_key).get_autocharge().unwrap()
}
