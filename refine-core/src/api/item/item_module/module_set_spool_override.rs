use crate::{Module, SolarSystem, Spool, sol::UndoStep, ud::UItemId};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Module<'s> {
    /// Force module to use specific spool setting.
    ///
    /// Solar system's default is used when override is not set.
    pub fn set_spool_override(&mut self, spool_override: Option<Spool>) {
        self.sol.internal_set_module_spool_override(self.uid, spool_override);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolarSystem {
    pub(in crate::api) fn internal_set_module_spool_override(
        &mut self,
        module_uid: UItemId,
        spool_override: Option<Spool>,
    ) {
        let u_module = self.u_data.items.get_mut(module_uid).dc_module_mut().unwrap();
        let old_spool_override = u_module.get_spool_override();
        if old_spool_override == spool_override {
            return;
        }
        if self.undo.is_recording() {
            self.undo
                .record(UndoStep::ModuleSpoolOverride(module_uid, old_spool_override));
        }
        u_module.set_spool_override(spool_override);
    }
}
