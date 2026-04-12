use crate::{
    api::{Coordinates, Fit, FitMut, ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed, MinionState, Movement},
    misc::{FighterCountInfo, ItemRearmMinionInfo},
    sol::SolarSystem,
    ud::{UFighter, UItemId},
};

pub struct Fighter<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) uid: UItemId,
}
impl<'a> Fighter<'a> {
    pub(in crate::api) fn new(sol: &'a SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.uid)
    }
    pub fn get_state(&self) -> MinionState {
        get_state(self.sol, self.uid)
    }
    pub fn get_count(&self) -> Option<FighterCountInfo> {
        get_count(self.sol, self.uid)
    }
    pub fn get_coordinates(&self) -> Coordinates {
        get_coordinates(self.sol, self.uid)
    }
    pub fn get_movement(&self) -> Movement {
        get_movement(self.sol, self.uid)
    }
    pub fn get_rearm_minion(&self) -> ItemRearmMinionInfo {
        get_rearm_minion(self.sol, self.uid)
    }
}
impl<'a> ItemSealed for Fighter<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_uid(&self) -> UItemId {
        self.uid
    }
}
impl<'a> ItemCommon for Fighter<'a> {}

pub struct FighterMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) uid: UItemId,
}
impl<'a> FighterMut<'a> {
    pub(in crate::api) fn new(sol: &'a mut SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.uid)
    }
    pub fn get_fit_mut(&mut self) -> FitMut<'_> {
        let fit_uid = get_u_fighter(self.sol, self.uid).get_fit_uid();
        FitMut::new(self.sol, fit_uid)
    }
    pub fn get_state(&self) -> MinionState {
        get_state(self.sol, self.uid)
    }
    pub fn get_count(&self) -> Option<FighterCountInfo> {
        get_count(self.sol, self.uid)
    }
    pub fn get_coordinates(&self) -> Coordinates {
        get_coordinates(self.sol, self.uid)
    }
    pub fn get_movement(&self) -> Movement {
        get_movement(self.sol, self.uid)
    }
    pub fn get_rearm_minion(&self) -> ItemRearmMinionInfo {
        get_rearm_minion(self.sol, self.uid)
    }
}
impl<'a> ItemSealed for FighterMut<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_uid(&self) -> UItemId {
        self.uid
    }
}
impl<'a> ItemMutSealed for FighterMut<'a> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'a> ItemCommon for FighterMut<'a> {}
impl<'a> ItemMutCommon for FighterMut<'a> {}

fn get_fit(sol: &SolarSystem, fighter_uid: UItemId) -> Fit<'_> {
    let fit_uid = get_u_fighter(sol, fighter_uid).get_fit_uid();
    Fit::new(sol, fit_uid)
}
fn get_state(sol: &SolarSystem, fighter_uid: UItemId) -> MinionState {
    get_u_fighter(sol, fighter_uid).get_fighter_state()
}
fn get_count(sol: &SolarSystem, fighter_uid: UItemId) -> Option<FighterCountInfo> {
    get_u_fighter(sol, fighter_uid).get_count_info()
}
fn get_coordinates(sol: &SolarSystem, fighter_uid: UItemId) -> Coordinates {
    Coordinates::from_xyz(get_u_fighter(sol, fighter_uid).get_physics().coordinates)
}
fn get_movement(sol: &SolarSystem, fighter_uid: UItemId) -> Movement {
    Movement::from_u_physics(get_u_fighter(sol, fighter_uid).get_physics())
}
fn get_rearm_minion(sol: &SolarSystem, fighter_uid: UItemId) -> ItemRearmMinionInfo {
    let value = sol.u_data.get_item_rearm_minion(fighter_uid, None);
    let overridden = get_u_fighter(sol, fighter_uid).get_rearm_minion().is_some();
    ItemRearmMinionInfo { value, overridden }
}
fn get_u_fighter(sol: &SolarSystem, fighter_uid: UItemId) -> &UFighter {
    sol.u_data.items.get(fighter_uid).dc_fighter().unwrap()
}
