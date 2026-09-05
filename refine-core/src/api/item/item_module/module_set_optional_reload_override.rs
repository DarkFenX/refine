use crate::{api::ModuleMut, misc::OptionalReload, sol::SolarSystem, ud::UItemId};

impl SolarSystem {
    fn internal_set_module_optional_reload_override(
        &mut self,
        module_uid: UItemId,
        optional_reload_override: Option<OptionalReload>,
    ) {
        let u_module = self.u_data.items.get_mut(module_uid).dc_module_mut().unwrap();
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
