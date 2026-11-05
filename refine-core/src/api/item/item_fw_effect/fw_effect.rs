use crate::{
    FitId, ItemCommon,
    api::ItemSealed,
    sol::SolCore,
    ud::{UFwEffect, UItemId},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct FwEffect<'s> {
    pub(super) sol: &'s mut SolCore,
    pub(super) uid: UItemId,
}
impl<'s> FwEffect<'s> {
    pub fn get_fit_id(&self) -> FitId {
        let fit_uid = get_u_fw_effect(self.sol, self.uid).get_fit_uid();
        self.sol.u_data.fits.ext_id_by_int_id(fit_uid)
    }
    pub fn get_state(&self) -> bool {
        get_u_fw_effect(self.sol, self.uid).get_fw_effect_state()
    }
}
impl<'s> ItemCommon for FwEffect<'s> {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> FwEffect<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolCore, uid: UItemId) -> Self {
        Self { sol, uid }
    }
}
impl<'s> ItemSealed for FwEffect<'s> {
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

fn get_u_fw_effect(sol: &SolCore, fw_effect_uid: UItemId) -> &UFwEffect {
    sol.u_data.items.get(fw_effect_uid).dc_fw_effect().unwrap()
}
