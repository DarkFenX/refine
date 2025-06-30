use crate::{
    def::{ItemKey, SlotIndex},
    sol::{
        SolarSystem,
        api::{Fit, FitMut, ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed},
    },
    uad::UadImplant,
};

pub struct Implant<'a> {
    pub(in crate::sol::api) sol: &'a SolarSystem,
    pub(in crate::sol::api) key: ItemKey,
}
impl<'a> Implant<'a> {
    pub(in crate::sol::api) fn new(sol: &'a SolarSystem, key: ItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.key)
    }
    pub fn get_slot(&self) -> Option<SlotIndex> {
        get_slot(self.sol, self.key)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key)
    }
}
impl<'a> ItemSealed for Implant<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> ItemKey {
        self.key
    }
}
impl<'a> ItemCommon for Implant<'a> {}

pub struct ImplantMut<'a> {
    pub(in crate::sol::api) sol: &'a mut SolarSystem,
    pub(in crate::sol::api) key: ItemKey,
}
impl<'a> ImplantMut<'a> {
    pub(in crate::sol::api) fn new(sol: &'a mut SolarSystem, key: ItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.key)
    }
    pub fn get_fit_mut(&mut self) -> FitMut<'_> {
        let fit_key = get_uad_implant(self.sol, self.key).get_fit_key();
        FitMut::new(self.sol, fit_key)
    }
    pub fn get_slot(&self) -> Option<SlotIndex> {
        get_slot(self.sol, self.key)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key)
    }
}
impl<'a> ItemSealed for ImplantMut<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> ItemKey {
        self.key
    }
}
impl<'a> ItemMutSealed for ImplantMut<'a> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'a> ItemCommon for ImplantMut<'a> {}
impl<'a> ItemMutCommon for ImplantMut<'a> {}

fn get_fit(sol: &SolarSystem, item_key: ItemKey) -> Fit<'_> {
    let fit_key = get_uad_implant(sol, item_key).get_fit_key();
    Fit::new(sol, fit_key)
}
fn get_slot(sol: &SolarSystem, item_key: ItemKey) -> Option<SlotIndex> {
    get_uad_implant(sol, item_key).get_a_slot()
}
fn get_state(sol: &SolarSystem, item_key: ItemKey) -> bool {
    get_uad_implant(sol, item_key).get_implant_state()
}
fn get_uad_implant(sol: &SolarSystem, item_key: ItemKey) -> &UadImplant {
    sol.uad.items.get(item_key).get_implant().unwrap()
}
