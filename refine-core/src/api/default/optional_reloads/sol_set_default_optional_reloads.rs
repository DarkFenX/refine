use crate::{
    OptionalReload, Sol,
    sol::{SolCore, UndoStep},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sol {
    pub fn set_default_optional_reloads(&mut self, optional_reloads: OptionalReload) {
        self.inner_mut().internal_set_default_optional_reloads(optional_reloads);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolCore {
    pub(in crate::api) fn internal_set_default_optional_reloads(&mut self, optional_reloads: OptionalReload) {
        let old_optional_reloads = std::mem::replace(&mut self.u_data.default_optional_reloads, optional_reloads);
        if old_optional_reloads != optional_reloads && self.undo.is_recording() {
            self.undo.record(UndoStep::DefaultOptionalReloads(old_optional_reloads));
        }
    }
}
