use crate::{
    FitId, ItemCommon, ServiceState,
    api::ItemSealed,
    sol::CoreInner,
    ud::{UItemId, UService},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Service<'s> {
    pub(super) core: &'s mut CoreInner,
    pub(super) uid: UItemId,
}
impl<'s> Service<'s> {
    pub fn get_fit_id(&self) -> FitId {
        let fit_uid = get_u_service(self.core, self.uid).get_fit_uid();
        self.core.u_data.fits.ext_id_by_int_id(fit_uid)
    }
    pub fn get_state(&self) -> ServiceState {
        get_u_service(self.core, self.uid).get_service_state()
    }
}
impl<'s> ItemCommon for Service<'s> {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Service<'s> {
    pub(in crate::api) fn new(core: &'s mut CoreInner, uid: UItemId) -> Self {
        Self { core, uid }
    }
}
impl<'s> ItemSealed for Service<'s> {
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

fn get_u_service(core: &CoreInner, service_uid: UItemId) -> &UService {
    core.u_data.items.get(service_uid).dc_service().unwrap()
}
