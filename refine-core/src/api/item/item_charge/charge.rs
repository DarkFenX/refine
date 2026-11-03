use crate::{
    FitId, ItemCommon, ItemId, SolarSystem,
    api::ItemSealed,
    ud::{UCharge, UItemId},
};

pub struct Charge<'s> {
    pub(super) sol: &'s mut SolarSystem,
    pub(super) uid: UItemId,
}
impl<'s> Charge<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fit_id(&self) -> FitId {
        let fit_uid = get_u_charge(self.sol, self.uid).get_fit_uid();
        self.sol.u_data.fits.ext_id_by_int_id(fit_uid)
    }
    pub fn get_cont_item_id(&self) -> ItemId {
        let cont_uid = get_u_charge(self.sol, self.uid).get_cont_item_uid();
        self.sol.u_data.items.ext_id_by_int_id(cont_uid)
    }
    pub fn get_state(&self) -> bool {
        !get_u_charge(self.sol, self.uid).get_force_disabled()
    }
}
impl<'s> ItemSealed for Charge<'s> {
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
impl<'s> ItemCommon for Charge<'s> {}

fn get_u_charge(sol: &SolarSystem, charge_uid: UItemId) -> &UCharge {
    sol.u_data.items.get(charge_uid).dc_charge().unwrap()
}
