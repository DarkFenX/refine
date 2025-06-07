use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{
        ItemId, SolarSystem,
        api::{Service, ServiceMut},
    },
};

impl SolarSystem {
    pub fn get_service(&self, item_id: &ItemId) -> Result<Service<'_>, GetServiceError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        self.uad.items.get(item_key).get_service()?;
        Ok(Service::new(self, item_key))
    }
    pub fn get_service_mut(&mut self, item_id: &ItemId) -> Result<ServiceMut<'_>, GetServiceError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        self.uad.items.get(item_key).get_service()?;
        Ok(ServiceMut::new(self, item_key))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetServiceError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotService(#[from] ItemKindMatchError),
}
