use crate::{
    FitId, ItemCommon,
    api::ItemSealed,
    sol::CoreInner,
    ud::{UItemId, UStance},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Stance<'s> {
    pub(super) sol: &'s mut CoreInner,
    pub(super) uid: UItemId,
}
impl<'s> Stance<'s> {
    pub fn get_fit_id(&self) -> FitId {
        let fit_uid = get_u_stance(self.sol, self.uid).get_fit_uid();
        self.sol.u_data.fits.ext_id_by_int_id(fit_uid)
    }
    pub fn get_state(&self) -> bool {
        get_u_stance(self.sol, self.uid).get_stance_state()
    }
}
impl<'s> ItemCommon for Stance<'s> {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Stance<'s> {
    pub(in crate::api) fn new(sol: &'s mut CoreInner, uid: UItemId) -> Self {
        Self { sol, uid }
    }
}
impl<'s> ItemSealed for Stance<'s> {
    fn get_uid(&self) -> UItemId {
        self.uid
    }
    fn get_sol(&self) -> &CoreInner {
        self.sol
    }
    fn get_sol_mut(&mut self) -> &mut CoreInner {
        self.sol
    }
}

fn get_u_stance(sol: &CoreInner, stance_uid: UItemId) -> &UStance {
    sol.u_data.items.get(stance_uid).dc_stance().unwrap()
}
