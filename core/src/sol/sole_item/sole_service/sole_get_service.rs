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
        Ok(ServiceInfo::from_service(service))
    }
}

#[derive(Debug)]
pub enum GetServiceError {
    ItemNotFound(ItemFoundError),
    ItemIsNotService(ItemKindMatchError),
}
impl std::error::Error for GetServiceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotService(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotService(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for GetServiceError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for GetServiceError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotService(error)
    }
}
