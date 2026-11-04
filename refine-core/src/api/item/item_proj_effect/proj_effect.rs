use crate::{ItemCommon, api::ItemSealed, sol::CoreInner, ud::UItemId};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct ProjEffect<'s> {
    pub(super) core: &'s mut CoreInner,
    pub(super) uid: UItemId,
}
impl<'s> ProjEffect<'s> {
    pub fn get_state(&self) -> bool {
        self.core
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
    pub(in crate::api) fn new(core: &'s mut CoreInner, uid: UItemId) -> Self {
        Self { core, uid }
    }
}
impl<'s> ItemSealed for ProjEffect<'s> {
    fn get_uid(&self) -> UItemId {
        self.uid
    }
    fn get_core(&self) -> &CoreInner {
        self.core
    }
    fn get_core_mut(&mut self) -> &mut CoreInner {
        self.core
    }
}
