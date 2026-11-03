use crate::{SolarSystem, Spool, sol::UndoStep};

impl SolarSystem {
    pub fn set_default_spool(&mut self, spool: Spool) {
        let old_spool = std::mem::replace(&mut self.u_data.default_spool, spool);
        if old_spool != spool && self.undo.is_recording() {
            self.undo.record(UndoStep::DefaultSpool(old_spool));
        }
    }
}
