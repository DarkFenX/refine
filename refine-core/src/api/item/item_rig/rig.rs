use crate::{
    FitId, ItemCommon,
    api::ItemSealed,
    sol::CoreInner,
    ud::{UItemId, URig},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Rig<'s> {
    pub(super) core: &'s mut CoreInner,
    pub(super) uid: UItemId,
}
impl<'s> Rig<'s> {
    pub fn get_fit_id(&self) -> FitId {
        let fit_uid = get_u_rig(self.core, self.uid).get_fit_uid();
        self.core.u_data.fits.ext_id_by_int_id(fit_uid)
    }
    pub fn get_state(&self) -> bool {
        get_u_rig(self.core, self.uid).get_rig_state()
    }
}
impl<'s> ItemCommon for Rig<'s> {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Rig<'s> {
    pub(in crate::api) fn new(core: &'s mut CoreInner, uid: UItemId) -> Self {
        Self { core, uid }
    }
}
impl<'s> ItemSealed for Rig<'s> {
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

fn get_u_rig(core: &CoreInner, rig_uid: UItemId) -> &URig {
    core.u_data.items.get(rig_uid).dc_rig().unwrap()
}
