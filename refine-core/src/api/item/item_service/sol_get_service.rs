use crate::{
    ItemId, Service, Sol,
    err::{ItemKindMatchError, basic::ItemFoundError},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sol {
    pub fn get_service(&mut self, item_id: &ItemId) -> Result<Service<'_>, ServiceGetError> {
        let core = self.inner_mut();
        let service_uid = core.u_data.items.int_id_by_ext_id_err(item_id)?;
        core.u_data.items.get(service_uid).dc_service()?;
        Ok(Service::new(core, service_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ServiceGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ItemIsNotService(#[from] ItemKindMatchError),
}
