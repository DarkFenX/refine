use crate::{
    FitId, ItemCommon, ItemId,
    api::ItemSealed,
    sol::SolCore,
    ud::{UCharge, UItemId},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Charge<'sol> {
    pub(super) sol: &'sol mut SolCore,
    pub(super) uid: UItemId,
}
impl<'sol> Charge<'sol> {
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
impl<'sol> ItemCommon for Charge<'sol> {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'sol> Charge<'sol> {
    pub(in crate::api) fn new(sol: &'sol mut SolCore, uid: UItemId) -> Self {
        Self { sol, uid }
    }
}
impl<'sol> ItemSealed for Charge<'sol> {
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

fn get_u_charge(sol: &SolCore, charge_uid: UItemId) -> &UCharge {
    sol.u_data.items.get(charge_uid).dc_charge().unwrap()
}
