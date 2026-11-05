use crate::{
    Coordinates, FitId, ItemCommon, MinionState, Movement, NpcProp,
    api::ItemSealed,
    sol::SolCore,
    ud::{UDrone, UItemId},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Drone<'s> {
    pub(super) sol: &'s mut SolCore,
    pub(super) uid: UItemId,
}
impl<'s> Drone<'s> {
    pub fn get_fit_id(&self) -> FitId {
        let fit_uid = get_u_drone(self.sol, self.uid).get_fit_uid();
        self.sol.u_data.fits.ext_id_by_int_id(fit_uid)
    }
    pub fn get_state(&self) -> MinionState {
        get_u_drone(self.sol, self.uid).get_drone_state()
    }
    pub fn get_npc_prop(&self) -> NpcProp {
        let u_item = self.sol.u_data.items.get(self.uid);
        self.sol.u_data.get_item_npc_prop(u_item).unwrap()
    }
    pub fn get_npc_prop_override(&self) -> Option<NpcProp> {
        get_u_drone(self.sol, self.uid).get_npc_prop_override()
    }
    pub fn get_coordinates(&self) -> Coordinates {
        Coordinates::from_xyz(get_u_drone(self.sol, self.uid).get_physics().coordinates)
    }
    pub fn get_movement(&self) -> Movement {
        Movement::from_u_physics(get_u_drone(self.sol, self.uid).get_physics())
    }
}
impl<'s> ItemCommon for Drone<'s> {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Drone<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolCore, uid: UItemId) -> Self {
        Self { sol, uid }
    }
}
impl<'s> ItemSealed for Drone<'s> {
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

fn get_u_drone(sol: &SolCore, drone_uid: UItemId) -> &UDrone {
    sol.u_data.items.get(drone_uid).dc_drone().unwrap()
}
