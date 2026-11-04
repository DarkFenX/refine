use crate::{
    FitId, ItemCommon,
    api::ItemSealed,
    sol::CoreInner,
    ud::{UFwEffect, UItemId},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct FwEffect<'s> {
    pub(super) core: &'s mut CoreInner,
    pub(super) uid: UItemId,
}
impl<'s> FwEffect<'s> {
    pub fn get_fit_id(&self) -> FitId {
        let fit_uid = get_u_fw_effect(self.core, self.uid).get_fit_uid();
        self.core.u_data.fits.ext_id_by_int_id(fit_uid)
    }
    pub fn get_state(&self) -> bool {
        get_u_fw_effect(self.core, self.uid).get_fw_effect_state()
    }
}
impl<'s> ItemCommon for FwEffect<'s> {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> FwEffect<'s> {
    pub(in crate::api) fn new(core: &'s mut CoreInner, uid: UItemId) -> Self {
        Self { core, uid }
    }
}
impl<'s> ItemSealed for FwEffect<'s> {
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

fn get_u_fw_effect(core: &CoreInner, fw_effect_uid: UItemId) -> &UFwEffect {
    core.u_data.items.get(fw_effect_uid).dc_fw_effect().unwrap()
}
