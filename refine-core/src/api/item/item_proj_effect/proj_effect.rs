use crate::{ItemCommon, SolarSystem, api::ItemSealed, ud::UItemId};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct ProjEffect<'s> {
    pub(super) sol: &'s mut SolarSystem,
    pub(super) uid: UItemId,
}
impl<'s> ProjEffect<'s> {
    pub fn get_state(&self) -> bool {
        self.sol
            .u_data
            .items
            .get(self.uid)
            .dc_proj_effect()
            .unwrap()
            .get_proj_effect_state()
    }
}
impl<'s> ItemCommon for ProjEffect<'s> {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> ProjEffect<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
}
impl<'s> ItemSealed for ProjEffect<'s> {
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
