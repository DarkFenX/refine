use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem, info::ServiceInfo},
};

impl SolarSystem {
    pub fn get_service(&self, item_id: &ItemId) -> Result<ServiceInfo, GetServiceError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.get_service_internal(item_key)?)
    }
    pub(in crate::sol) fn get_service_internal(&self, item_key: ItemKey) -> Result<ServiceInfo, ItemKindMatchError> {
        let service = self.uad.items.get(item_key).get_service()?;
        Ok(ServiceInfo::from_service(&self.uad, service))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetServiceError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotService(#[from] ItemKindMatchError),
}
