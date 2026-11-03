use crate::{
    Coordinates, CountNz, FighterCountInfo, FitId, ItemCommon, MinionState, Movement, RearmMinion, SolarSystem,
    api::ItemSealed,
    ud::{UFighter, UItemId},
};

pub struct Fighter<'s> {
    pub(super) sol: &'s mut SolarSystem,
    pub(super) uid: UItemId,
}
impl<'s> Fighter<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fit_id(&self) -> FitId {
        let fit_uid = get_u_fighter(self.sol, self.uid).get_fit_uid();
        self.sol.u_data.fits.ext_id_by_int_id(fit_uid)
    }
    pub fn get_state(&self) -> MinionState {
        get_u_fighter(self.sol, self.uid).get_fighter_state()
    }
    pub fn get_count(&self) -> Option<FighterCountInfo> {
        get_u_fighter(self.sol, self.uid).get_count_info()
    }
    pub fn get_count_override(&self) -> Option<CountNz> {
        get_u_fighter(self.sol, self.uid).get_count_override()
    }
    pub fn get_coordinates(&self) -> Coordinates {
        Coordinates::from_xyz(get_u_fighter(self.sol, self.uid).get_physics().coordinates)
    }
    pub fn get_movement(&self) -> Movement {
        Movement::from_u_physics(get_u_fighter(self.sol, self.uid).get_physics())
    }
    pub fn get_rearm_minion(&self) -> RearmMinion {
        self.sol.u_data.get_item_rearm_minion(self.uid, None)
    }
    pub fn get_rearm_minion_override(&self) -> Option<RearmMinion> {
        get_u_fighter(self.sol, self.uid).get_rearm_minion_override()
    }
}
impl<'s> ItemSealed for Fighter<'s> {
    fn get_uid(&self) -> UItemId {
        self.uid
    }
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'s> ItemCommon for Fighter<'s> {}

fn get_u_fighter(sol: &SolarSystem, fighter_uid: UItemId) -> &UFighter {
    sol.u_data.items.get(fighter_uid).dc_fighter().unwrap()
}
