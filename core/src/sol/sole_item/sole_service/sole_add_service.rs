use crate::{
    err::basic::FitFoundError,
    sol::{
        FitId, FitKey, ItemKey, ItemTypeId, SolarSystem,
        info::ServiceInfo,
        uad::item::{ServiceState, UadItem, UadService},
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
        Ok(self.get_service_info_internal(item_key).unwrap())
    }
    pub(in crate::sol) fn add_service_internal(
        &mut self,
        fit_key: FitKey,
        type_id: ItemTypeId,
        state: ServiceState,
    ) -> ItemKey {
        let item_id = self.uad.items.alloc_id();
        let service = UadService::new(&self.uad.src, item_id, type_id, fit_key, state);
        let item = UadItem::Service(service);
        let item_key = self.uad.items.add(item);
        let fit = self.uad.fits.get_mut(fit_key);
        fit.services.insert(item_key);
        self.add_item_key_to_svc(item_key);
        item_key
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AddServiceError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
}
