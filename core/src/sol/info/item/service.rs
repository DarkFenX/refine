use crate::sol::{
    FitId, ItemId, ItemTypeId,
    uad::{
        Uad,
        item::{Service, ServiceState},
    },
};

pub struct ServiceInfo {
    pub id: ItemId,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub state: ServiceState,
}
impl ServiceInfo {
    pub(in crate::sol) fn from_service(uad: &Uad, service: &Service) -> Self {
        Self {
            id: service.get_item_id(),
            type_id: service.get_a_item_id(),
            fit_id: uad.fits.id_by_key(service.get_fit_key()),
            state: service.get_service_state(),
        }
    }
}
