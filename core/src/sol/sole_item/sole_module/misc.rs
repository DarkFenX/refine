use crate::sol::{info::SolModuleInfo, uad::item::SolModule, SolarSystem};

impl SolarSystem {
    pub(in crate::sol) fn make_module_info(&self, module: &SolModule) -> SolModuleInfo {
        let charge_info = module.get_charge_id().map(|i| self.get_charge(&i).unwrap());
        SolModuleInfo::from_mod_and_charge_with_source(&self.uad.src, module, charge_info)
    }
}
