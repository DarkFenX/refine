use crate::{
    api::{EffectId, Fit, FitMut, Item, ItemCommon, ItemMut, ItemMutCommon, ItemMutSealed, ItemSealed},
    sol::SolarSystem,
    ud::{UAutocharge, UItemId},
};

// Autocharges expose no projection info, since it fully matches projections of the parent item
pub struct Autocharge<'s> {
    pub(in crate::api) sol: &'s SolarSystem,
    pub(in crate::api) uid: UItemId,
}
impl<'s> Autocharge<'s> {
    pub(in crate::api) fn new(sol: &'s SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.uid)
    }
    pub fn get_cont_item(&self) -> Item<'_> {
        get_cont_item(self.sol, self.uid)
    }
    pub fn get_cont_effect_id(&self) -> EffectId {
        get_cont_effect_id(self.sol, self.uid)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.uid)
    }
}
impl<'s> ItemSealed for Autocharge<'s> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_uid(&self) -> UItemId {
        self.uid
    }
}
impl<'s> ItemCommon for Autocharge<'s> {}

pub struct AutochargeMut<'s> {
    pub(in crate::api) sol: &'s mut SolarSystem,
    pub(in crate::api) uid: UItemId,
}
impl<'s> AutochargeMut<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.uid)
    }
    pub fn get_fit_mut(&mut self) -> FitMut<'_> {
        let fit_uid = get_u_autocharge(self.sol, self.uid).get_fit_uid();
        FitMut::new(self.sol, fit_uid)
    }
    pub fn get_cont_item(&self) -> Item<'_> {
        get_cont_item(self.sol, self.uid)
    }
    pub fn get_cont_item_mut(&mut self) -> ItemMut<'_> {
        let cont_uid = get_u_autocharge(self.sol, self.uid).get_cont_item_uid();
        ItemMut::new(self.sol, cont_uid)
    }
    pub fn get_cont_effect_id(&self) -> EffectId {
        get_cont_effect_id(self.sol, self.uid)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.uid)
    }
}
impl<'s> ItemSealed for AutochargeMut<'s> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_uid(&self) -> UItemId {
        self.uid
    }
}
impl<'s> ItemMutSealed for AutochargeMut<'s> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'s> ItemCommon for AutochargeMut<'s> {}
impl<'s> ItemMutCommon for AutochargeMut<'s> {}

fn get_fit(sol: &SolarSystem, autocharge_uid: UItemId) -> Fit<'_> {
    let fit_uid = get_u_autocharge(sol, autocharge_uid).get_fit_uid();
    Fit::new(sol, fit_uid)
}
fn get_cont_item(sol: &SolarSystem, autocharge_uid: UItemId) -> Item<'_> {
    let cont_uid = get_u_autocharge(sol, autocharge_uid).get_cont_item_uid();
    Item::new(sol, cont_uid)
}
fn get_cont_effect_id(sol: &SolarSystem, autocharge_uid: UItemId) -> EffectId {
    let cont_effect_rid = get_u_autocharge(sol, autocharge_uid).get_cont_effect_rid();
    let effect_aid = sol.u_data.r_data.get_effect_by_rid(cont_effect_rid).aid;
    EffectId::from_aid(effect_aid)
}
fn get_state(sol: &SolarSystem, autocharge_uid: UItemId) -> bool {
    !get_u_autocharge(sol, autocharge_uid).get_force_disabled()
}
fn get_u_autocharge(sol: &SolarSystem, autocharge_uid: UItemId) -> &UAutocharge {
    sol.u_data.items.get(autocharge_uid).dc_autocharge().unwrap()
}
