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
impl From<&Service> for ServiceInfo {
    fn from(sol_service: &Service) -> Self {
        ServiceInfo {
            id: sol_service.get_item_id(),
            type_id: sol_service.get_a_item_id(),
            fit_id: sol_service.get_fit_id(),
            state: sol_service.get_service_state(),
        }
    }
}
