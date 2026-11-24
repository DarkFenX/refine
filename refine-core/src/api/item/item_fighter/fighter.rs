use crate::{
    Coordinates, CountNz, FighterCountInfo, FitId, ItemCommon, MinionState, Movement, RearmMinion,
    api::ItemSealed,
    sol::SolCore,
    ud::{UFighter, UItemId},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Fighter<'sol> {
    pub(super) sol: &'sol mut SolCore,
    pub(super) uid: UItemId,
}
impl<'sol> Fighter<'sol> {
    pub fn get_fit_id(&self) -> FitId {
        let fit_uid = get_u_fighter(self.sol, self.uid).get_fit_uid();
        self.sol.u_data.fits.ext_id_by_int_id(fit_uid)
    }
    pub fn get_state(&self) -> MinionState {
        get_u_fighter(self.sol, self.uid).get_fighter_state()
    }
    pub fn get_count_override(&self) -> Option<CountNz> {
        get_u_fighter(self.sol, self.uid).get_count_override()
    }
    pub fn get_count_info_effective(&self) -> Option<FighterCountInfo> {
        get_u_fighter(self.sol, self.uid).get_count_info_effective()
    }
    pub fn get_coordinates(&self) -> Coordinates {
        Coordinates::from_xyz(get_u_fighter(self.sol, self.uid).get_physics().coordinates)
    }
    pub fn get_movement(&self) -> Movement {
        Movement::from_u_physics(get_u_fighter(self.sol, self.uid).get_physics())
    }
    pub fn get_rearm_minion_override(&self) -> Option<RearmMinion> {
        get_u_fighter(self.sol, self.uid).get_rearm_minion_override()
    }
    pub fn get_rearm_minion_effective(&self) -> Option<RearmMinion> {
        match get_u_fighter(self.sol, self.uid).get_r_item_base()?.has_rearm {
            true => Some(self.sol.u_data.get_item_rearm_minion_resolved(self.uid, None)),
            false => None,
        }
    }
}
impl<'sol> ItemCommon for Fighter<'sol> {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'sol> Fighter<'sol> {
    pub(in crate::api) fn new(sol: &'sol mut SolCore, uid: UItemId) -> Self {
        Self { sol, uid }
    }
}
impl<'sol> ItemSealed for Fighter<'sol> {
    fn get_uid(&self) -> UItemId {
        self.uid
    }
    fn get_sol(&self) -> &SolCore {
        self.sol
    }
    fn get_sol_mut(&mut self) -> &mut SolCore {
        self.sol
    }
}

fn get_u_fighter(sol: &SolCore, fighter_uid: UItemId) -> &UFighter {
    sol.u_data.items.get(fighter_uid).dc_fighter().unwrap()
}
