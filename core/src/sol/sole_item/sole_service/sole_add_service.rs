use crate::{
    err::basic::FitFoundError,
    sol::{
        FitId, ItemId, ItemTypeId, SolarSystem,
        info::ServiceInfo,
        uad::item::{Item, Service, ServiceState},
    },
};

impl SolarSystem {
    pub fn add_service(
        &mut self,
        fit_id: FitId,
        type_id: ItemTypeId,
        state: ServiceState,
    ) -> Result<ServiceInfo, AddServiceError> {
        let item_id = self.add_service_internal(fit_id, type_id, state)?;
        Ok(self.get_service(&item_id).unwrap())
    }
    pub(in crate::sol) fn add_service_internal(
        &mut self,
        fit_id: FitId,
        type_id: ItemTypeId,
        state: ServiceState,
    ) -> Result<ItemId, AddServiceError> {
        let item_id = self.uad.items.alloc_item_id();
        let service = Service::new(&self.uad.src, item_id, type_id, fit_id, state);
        let item = Item::Service(service);
        let fit = self.uad.fits.get_fit_mut(&fit_id)?;
        fit.services.insert(item_id);
        self.uad.items.add_item(item);
        self.add_item_id_to_svc(&item_id);
        Ok(item_id)
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
