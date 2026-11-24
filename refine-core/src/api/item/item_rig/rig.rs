use crate::{
    FitId, ItemCommon,
    api::ItemSealed,
    sol::SolCore,
    ud::{UItemId, URig},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Rig<'sol> {
    pub(super) sol: &'sol mut SolCore,
    pub(super) uid: UItemId,
}
impl<'sol> Rig<'sol> {
    pub fn get_fit_id(&self) -> FitId {
        let fit_uid = get_u_rig(self.sol, self.uid).get_fit_uid();
        self.sol.u_data.fits.ext_id_by_int_id(fit_uid)
    }
    pub fn get_state(&self) -> bool {
        get_u_rig(self.sol, self.uid).get_rig_state()
    }
}
impl<'sol> ItemCommon for Rig<'sol> {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'sol> Rig<'sol> {
    pub(in crate::api) fn new(sol: &'sol mut SolCore, uid: UItemId) -> Self {
        Self { sol, uid }
    }
}
impl<'sol> ItemSealed for Rig<'sol> {
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

fn get_u_rig(sol: &SolCore, rig_uid: UItemId) -> &URig {
    sol.u_data.items.get(rig_uid).dc_rig().unwrap()
}
