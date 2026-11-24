use crate::{
    FitId, ItemCommon, SlotIndex,
    api::ItemSealed,
    sol::SolCore,
    ud::{UItemId, USubsystem},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Subsystem<'sol> {
    pub(super) sol: &'sol mut SolCore,
    pub(super) uid: UItemId,
}
impl<'sol> Subsystem<'sol> {
    pub fn get_fit_id(&self) -> FitId {
        let fit_uid = get_u_subsystem(self.sol, self.uid).get_fit_uid();
        self.sol.u_data.fits.ext_id_by_int_id(fit_uid)
    }
    pub fn get_slot(&self) -> Option<SlotIndex> {
        get_u_subsystem(self.sol, self.uid).get_r_item_base()?.subsystem_slot
    }
    pub fn get_state(&self) -> bool {
        get_u_subsystem(self.sol, self.uid).get_subsystem_state()
    }
}
impl<'sol> ItemCommon for Subsystem<'sol> {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'sol> Subsystem<'sol> {
    pub(in crate::api) fn new(sol: &'sol mut SolCore, uid: UItemId) -> Self {
        Self { sol, uid }
    }
}
impl<'sol> ItemSealed for Subsystem<'sol> {
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

fn get_u_subsystem(sol: &SolCore, subsystem_uid: UItemId) -> &USubsystem {
    sol.u_data.items.get(subsystem_uid).dc_subsystem().unwrap()
}
