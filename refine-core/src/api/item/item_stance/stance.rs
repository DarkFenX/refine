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
    pub(super) core: &'s mut CoreInner,
    pub(super) uid: UItemId,
}
impl<'s> Stance<'s> {
    pub fn get_fit_id(&self) -> FitId {
        let fit_uid = get_u_stance(self.core, self.uid).get_fit_uid();
        self.core.u_data.fits.ext_id_by_int_id(fit_uid)
    }
    pub fn get_state(&self) -> bool {
        get_u_stance(self.core, self.uid).get_stance_state()
    }
}
impl<'s> ItemCommon for Stance<'s> {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Stance<'s> {
    pub(in crate::api) fn new(core: &'s mut CoreInner, uid: UItemId) -> Self {
        Self { core, uid }
    }
}
impl<'s> ItemSealed for Stance<'s> {
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

fn get_u_stance(core: &CoreInner, stance_uid: UItemId) -> &UStance {
    core.u_data.items.get(stance_uid).dc_stance().unwrap()
}
