use crate::api::ModuleMut;

impl<'a> ModuleMut<'a> {
    pub fn set_reload_optionals(&mut self, reload_optionals: Option<bool>) {
        let u_module = self.sol.u_data.items.get_mut(self.uid).dc_module_mut().unwrap();
        u_module.set_reload_optionals(reload_optionals);
    }
}
