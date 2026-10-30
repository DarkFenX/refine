use crate::{
    api::ModuleMut,
    misc::Spool,
    sol::{SolarSystem, UndoStep},
    ud::UItemId,
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_module_spool_override(
        &mut self,
        module_uid: UItemId,
        spool_override: Option<Spool>,
    ) {
        let u_module = self.u_data.items.get_mut(module_uid).dc_module_mut().unwrap();
        if self.undo.is_recording() {
            self.undo
                .record(UndoStep::ModuleSpoolOverride(module_uid, u_module.get_spool_override()));
        }
        u_module.set_spool_override(spool_override);
    }
}

impl<'s> ModuleMut<'s> {
    /// Force module to use specific spool setting.
    ///
    /// Solar system's default is used when override is not set.
    pub fn set_spool_override(&mut self, spool_override: Option<Spool>) {
        self.sol.internal_set_module_spool_override(self.uid, spool_override);
    }
}
