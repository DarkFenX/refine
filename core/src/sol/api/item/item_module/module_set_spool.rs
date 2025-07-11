use crate::{misc::Spool, sol::api::ModuleMut};

impl<'a> ModuleMut<'a> {
    pub fn set_spool(&mut self, spool: Option<Spool>) {
        let uad_module = self.sol.uad.items.get_mut(self.key).get_module_mut().unwrap();
        uad_module.set_spool(spool);
    }
}
