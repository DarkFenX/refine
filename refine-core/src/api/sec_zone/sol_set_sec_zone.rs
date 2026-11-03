use crate::{
    SecZone, Sol,
    sol::{CoreInner, UndoStep},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sol {
    pub fn set_sec_zone(&mut self, sec_zone: SecZone) {
        self.inner_mut().internal_set_sec_zone(sec_zone);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CoreInner {
    pub(in crate::api) fn internal_set_sec_zone(&mut self, sec_zone: SecZone) {
        let old_sec_zone = std::mem::replace(&mut self.u_data.sec_zone, sec_zone);
        if old_sec_zone == sec_zone {
            return;
        }
        if self.undo.is_recording() {
            self.undo.record(UndoStep::SecZone(old_sec_zone));
        }
        self.svc.notify_sol_sec_zone_changed(&self.u_data);
    }
}
