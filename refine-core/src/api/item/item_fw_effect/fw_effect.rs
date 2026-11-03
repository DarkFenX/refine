use crate::{
    FitId, ItemCommon, SolarSystem,
    api::ItemSealed,
    ud::{UFwEffect, UItemId},
};

pub struct FwEffect<'s> {
    pub(super) sol: &'s mut SolarSystem,
    pub(super) uid: UItemId,
}
impl<'s> FwEffect<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fit_id(&self) -> FitId {
        let fit_uid = get_u_fw_effect(self.sol, self.uid).get_fit_uid();
        self.sol.u_data.fits.ext_id_by_int_id(fit_uid)
    }
    pub fn get_state(&self) -> bool {
        get_u_fw_effect(self.sol, self.uid).get_fw_effect_state()
    }
}
impl<'s> ItemSealed for FwEffect<'s> {
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
impl<'s> ItemCommon for FwEffect<'s> {}

fn get_u_fw_effect(sol: &SolarSystem, fw_effect_uid: UItemId) -> &UFwEffect {
    sol.u_data.items.get(fw_effect_uid).dc_fw_effect().unwrap()
}
