use crate::{
    api::{Fit, ItemCommon, ItemSealed},
    num::SlotIndex,
    sol::SolarSystem,
    ud::{UItemId, USubsystem},
};

pub struct Subsystem<'s> {
    pub(super) sol: &'s mut SolarSystem,
    pub(super) uid: UItemId,
}
impl<'s> Subsystem<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fit(&mut self) -> Fit<'_> {
        let fit_uid = get_u_subsystem(self.sol, self.uid).get_fit_uid();
        Fit::new(self.sol, fit_uid)
    }
    pub fn get_slot(&self) -> Option<SlotIndex> {
        get_u_subsystem(self.sol, self.uid).get_r_item_base()?.subsystem_slot
    }
    pub fn get_state(&self) -> bool {
        get_u_subsystem(self.sol, self.uid).get_subsystem_state()
    }
}
impl<'s> ItemSealed for Subsystem<'s> {
    fn get_uid(&self) -> UItemId {
        self.uid
    }
    fn get_sol_ref(&self) -> &SolarSystem {
        self.sol
    }
}
impl<'s> ItemCommon for Subsystem<'s> {
    fn get_sol(&mut self) -> &mut SolarSystem {
        self.sol
    }
}

fn get_u_subsystem(sol: &SolarSystem, subsystem_uid: UItemId) -> &USubsystem {
    sol.u_data.items.get(subsystem_uid).dc_subsystem().unwrap()
}
