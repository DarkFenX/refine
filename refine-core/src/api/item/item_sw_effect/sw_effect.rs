use crate::{ItemCommon, api::ItemSealed, sol::SolCore, ud::UItemId};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct SwEffect<'sol> {
    pub(super) sol: &'sol mut SolCore,
    pub(super) uid: UItemId,
}
impl<'sol> SwEffect<'sol> {
    pub fn get_state(&self) -> bool {
        self.sol
            .u_data
            .items
            .get(self.uid)
            .dc_sw_effect()
            .unwrap()
            .get_sw_effect_state()
    }
}
impl<'sol> ItemCommon for SwEffect<'sol> {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'sol> SwEffect<'sol> {
    pub(in crate::api) fn new(sol: &'sol mut SolCore, uid: UItemId) -> Self {
        Self { sol, uid }
    }
}
impl<'sol> ItemSealed for SwEffect<'sol> {
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
