use crate::sol::{
    FitId, ItemId, ItemTypeId,
    uad::item::{Service, ServiceState},
};

pub struct ServiceInfo {
    pub id: ItemId,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub state: ServiceState,
}
impl ServiceInfo {
    pub(in crate::sol) fn from_service(service: &Service) -> Self {
        Self {
            id: service.get_item_id(),
            type_id: service.get_a_item_id(),
            fit_id: service.get_fit_id(),
            state: service.get_service_state(),
        }
    }
}
