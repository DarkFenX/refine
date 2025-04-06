use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, SolarSystem, info::ServiceInfo},
};

impl SolarSystem {
    pub fn get_service(&self, item_id: &ItemId) -> Result<ServiceInfo, GetServiceError> {
        let service = self.uad.items.get_by_id(item_id)?.get_service()?;
        Ok(ServiceInfo::from(service))
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
