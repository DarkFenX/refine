use crate::{
    api::{Fit, ItemCommon, ItemSealed},
    num::SlotIndex,
    sol::SolarSystem,
    ud::{UImplant, UItemId},
};

pub struct Implant<'s> {
    pub(super) sol: &'s mut SolarSystem,
    pub(super) uid: UItemId,
}
impl<'s> Implant<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fit(&mut self) -> Fit<'_> {
        let fit_uid = get_u_implant(self.sol, self.uid).get_fit_uid();
        Fit::new(self.sol, fit_uid)
    }
    pub fn get_slot(&self) -> Option<SlotIndex> {
        get_u_implant(self.sol, self.uid).get_r_item_base()?.implant_slot
    }
    pub fn get_state(&self) -> bool {
        get_u_implant(self.sol, self.uid).get_implant_state()
    }
}
impl<'s> ItemSealed for Implant<'s> {
    fn get_uid(&self) -> UItemId {
        self.uid
    }
    fn get_sol_ref(&self) -> &SolarSystem {
        self.sol
    }
}
impl<'s> ItemCommon for Implant<'s> {
    fn get_sol(&mut self) -> &mut SolarSystem {
        self.sol
    }
}

fn get_u_implant(sol: &SolarSystem, implant_uid: UItemId) -> &UImplant {
    sol.u_data.items.get(implant_uid).dc_implant().unwrap()
}
