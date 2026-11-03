use crate::{
    FitId, ItemCommon, SlotIndex, SolarSystem,
    api::ItemSealed,
    ud::{UImplant, UItemId},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Implant<'s> {
    pub(super) sol: &'s mut SolarSystem,
    pub(super) uid: UItemId,
}
impl<'s> Implant<'s> {
    pub fn get_fit_id(&self) -> FitId {
        let fit_uid = get_u_implant(self.sol, self.uid).get_fit_uid();
        self.sol.u_data.fits.ext_id_by_int_id(fit_uid)
    }
    pub fn get_slot(&self) -> Option<SlotIndex> {
        get_u_implant(self.sol, self.uid).get_r_item_base()?.implant_slot
    }
    pub fn get_state(&self) -> bool {
        get_u_implant(self.sol, self.uid).get_implant_state()
    }
}
impl<'s> ItemCommon for Implant<'s> {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Implant<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
}
impl<'s> ItemSealed for Implant<'s> {
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

fn get_u_implant(sol: &SolarSystem, implant_uid: UItemId) -> &UImplant {
    sol.u_data.items.get(implant_uid).dc_implant().unwrap()
}
