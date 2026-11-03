use crate::{
    Service, ServiceState,
    sol::{CoreInner, UndoStep},
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Service<'s> {
    pub fn set_state(&mut self, state: ServiceState) {
        let u_service = self.sol.u_data.items.get(self.uid).dc_service().unwrap();
        if state == u_service.get_service_state() {
            return;
        }
        self.sol.internal_set_service_state(self.uid, state)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CoreInner {
    pub(in crate::api) fn internal_set_service_state(&mut self, service_uid: UItemId, state: ServiceState) {
        let u_service = self.u_data.items.get_mut(service_uid).dc_service_mut().unwrap();
        if self.undo.is_recording() {
            self.undo
                .record(UndoStep::ServiceState(service_uid, u_service.get_service_state()));
        }
        let old_a_state = u_service.get_state();
        u_service.set_service_state(state);
        let new_a_state = u_service.get_state();
        u_service.update_reffs(&mut self.reuse.eupdates, &self.u_data.r_data);
        CoreInner::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            service_uid,
            old_a_state,
            new_a_state,
            &self.reuse.eupdates,
        );
    }
}
