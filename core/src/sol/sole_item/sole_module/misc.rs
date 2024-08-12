use crate::sol::{fit::SolFit, item::SolModule, item_info::SolModuleInfo, SolModRack, SolarSystem};

impl SolarSystem {
    pub(in crate::sol) fn make_module_info(&self, module: &SolModule) -> SolModuleInfo {
        let charge_info = module.charge_item_id.map(|i| self.get_charge(&i).unwrap());
        SolModuleInfo::from_mod_and_charge(module, charge_info)
    }
    pub(in crate::sol::sole_item::sole_module) fn int_get_fit_module_infos(
        &self,
        fit: &SolFit,
        rack: SolModRack,
    ) -> Vec<SolModuleInfo> {
        let module_ids = match rack {
            SolModRack::High => &fit.mods_high,
            SolModRack::Mid => &fit.mods_mid,
            SolModRack::Low => &fit.mods_low,
        };
        let module_infos = module_ids
            .iter()
            .map(|v| self.make_module_info(self.items.get_item(v).unwrap().get_module().unwrap()))
            .collect();
        module_infos
    }
}
