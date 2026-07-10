use crate::{api::ModuleMut, misc::OptionalReload};

impl<'a> ModuleMut<'a> {
    pub fn set_optional_reload(&mut self, optional_reload: Option<OptionalReload>) {
        let u_module = self.sol.u_data.items.get_mut(self.uid).dc_module_mut().unwrap();
        u_module.set_optional_reload(optional_reload);
    }
}
