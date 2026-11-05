use crate::{
    FitId, ItemCommon,
    api::ItemSealed,
    sol::SolCore,
    ud::{UItemId, URig},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Rig<'s> {
    pub(super) sol: &'s mut SolCore,
    pub(super) uid: UItemId,
}
impl<'s> Rig<'s> {
    pub fn get_fit_id(&self) -> FitId {
        let fit_uid = get_u_rig(self.sol, self.uid).get_fit_uid();
        self.sol.u_data.fits.ext_id_by_int_id(fit_uid)
    }
    pub fn get_state(&self) -> bool {
        get_u_rig(self.sol, self.uid).get_rig_state()
    }
}
impl<'s> ItemCommon for Rig<'s> {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Rig<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolCore, uid: UItemId) -> Self {
        Self { sol, uid }
    }
}
impl<'s> ItemSealed for Rig<'s> {
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
