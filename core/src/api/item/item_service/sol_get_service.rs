use crate::{
    api::{Service, ServiceMut},
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
    ud::ItemId,
};

impl SolarSystem {
    pub fn get_service(&self, item_id: &ItemId) -> Result<Service<'_>, GetServiceError> {
        let service_uid = self.u_data.items.iid_by_xid_err(item_id)?;
        self.u_data.items.get(service_uid).dc_service()?;
        Ok(Service::new(self, service_uid))
    }
    pub fn get_service_mut(&mut self, item_id: &ItemId) -> Result<ServiceMut<'_>, GetServiceError> {
        let service_uid = self.u_data.items.iid_by_xid_err(item_id)?;
        self.u_data.items.get(service_uid).dc_service()?;
        Ok(ServiceMut::new(self, service_uid))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetServiceError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotService(#[from] ItemKindMatchError),
}
