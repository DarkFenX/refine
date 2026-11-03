use crate::{OptionalReload, SolarSystem, sol::UndoStep};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolarSystem {
    pub fn set_default_optional_reloads(&mut self, optional_reloads: OptionalReload) {
        let old_optional_reloads = std::mem::replace(&mut self.u_data.default_optional_reloads, optional_reloads);
        if old_optional_reloads != optional_reloads && self.undo.is_recording() {
            self.undo.record(UndoStep::DefaultOptionalReloads(old_optional_reloads));
        }
    }
}
