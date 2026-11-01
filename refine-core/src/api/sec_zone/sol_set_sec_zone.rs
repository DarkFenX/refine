use crate::{SecZone, SolarSystem, sol::UndoStep};

impl SolarSystem {
    pub fn set_sec_zone(&mut self, sec_zone: SecZone) {
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
