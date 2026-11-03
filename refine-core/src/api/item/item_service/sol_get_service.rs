use crate::{
    api::Service,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
    ud::ItemId,
};

impl SolarSystem {
    pub fn get_service(&mut self, item_id: &ItemId) -> Result<Service<'_>, ServiceGetError> {
        let service_uid = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        self.u_data.items.get(service_uid).dc_service()?;
        Ok(Service::new(self, service_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ServiceGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ItemIsNotService(#[from] ItemKindMatchError),
}
