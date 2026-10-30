use crate::{ItemId, api::ModuleState, sol::SolarSystem};

/// Data needed to revert a single change done to a solar system.
///
/// A step can be applied only to the state right after its change, so steps of multiple changes
/// have to be applied in reverse order. Items are referenced by public IDs, since restoring removed
/// items is allowed to change their internal IDs.
pub(in crate::api) enum UndoStep {
    ModuleState(ItemId, ModuleState),
}

impl SolarSystem {
    pub(in crate::api) fn internal_undo(&mut self, step: UndoStep) {
        match step {
            UndoStep::ModuleState(module_id, state) => {
                let module_uid = self.u_data.items.int_id_by_ext_id(&module_id).unwrap();
                self.internal_set_module_state(module_uid, state);
            }
        }
    }
}
