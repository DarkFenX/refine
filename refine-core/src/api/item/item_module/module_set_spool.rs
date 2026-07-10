use crate::{api::ModuleMut, misc::Spool};

impl<'a> ModuleMut<'a> {
    pub fn set_spool(&mut self, spool: Option<Spool>) {
        let u_module = self.sol.u_data.items.get_mut(self.uid).dc_module_mut().unwrap();
        u_module.set_spool(spool);
    }
}
