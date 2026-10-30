use crate::{
    api::ModuleMut,
    misc::OptionalReload,
    sol::{SolarSystem, UndoStep},
    ud::UItemId,
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_module_optional_reload_override(
        &mut self,
        module_uid: UItemId,
        optional_reload_override: Option<OptionalReload>,
    ) {
        let u_module = self.u_data.items.get_mut(module_uid).dc_module_mut().unwrap();
        if self.undo.is_recording() {
            let old_optional_reload_override = u_module.get_optional_reload_override();
            self.undo.record(UndoStep::ModuleOptionalReloadOverride(
                module_uid,
                old_optional_reload_override,
            ));
        }
        u_module.set_optional_reload_override(optional_reload_override);
    }
}

impl<'s> ModuleMut<'s> {
    /// Force module to use specific optional reload behavior.
    ///
    /// Solar system's default is used when override is not set.
    pub fn set_optional_reload_override(&mut self, optional_reload_override: Option<OptionalReload>) {
        self.sol
            .internal_set_module_optional_reload_override(self.uid, optional_reload_override);
    }
}
