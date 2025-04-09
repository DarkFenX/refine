use crate::{
    err::basic::FitFoundError,
    sol::{
        FitId, FitKey, ItemKey, ItemTypeId, SolarSystem,
        info::ServiceInfo,
        uad::item::{Item, Service, ServiceState},
    },
};

impl SolarSystem {
    pub fn add_service(
        &mut self,
        fit_id: &FitId,
        type_id: ItemTypeId,
        state: ServiceState,
    ) -> Result<ServiceInfo, AddServiceError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        let item_key = self.add_service_internal(fit_key, type_id, state);
        Ok(self.get_service_internal(item_key).unwrap())
    }
    pub(in crate::sol) fn add_service_internal(
        &mut self,
        fit_key: FitKey,
        type_id: ItemTypeId,
        state: ServiceState,
    ) -> ItemKey {
        let item_id = self.uad.items.alloc_id();
        let service = Service::new(&self.uad.src, item_id, type_id, fit_key, state);
        let item = Item::Service(service);
        let item_key = self.uad.items.add(item);
        let fit = self.uad.fits.get_mut(fit_key);
        fit.services.insert(item_key);
        self.add_item_key_to_svc(item_key);
        item_key
    }
}

#[derive(Debug)]
pub enum AddServiceError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for AddServiceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AddServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for AddServiceError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
