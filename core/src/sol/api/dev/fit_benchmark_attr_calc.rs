use crate::{
    def::ItemTypeId,
    misc::{AddMode, ModRack, ModuleState, RmMode},
    sol::api::FitMut,
    ud::UEffectUpdates,
};

impl<'a> FitMut<'a> {
    pub fn benchmark_attr_calc(&mut self, type_id: ItemTypeId, iterations: usize) {
        let mut reuse_eupdates = UEffectUpdates::new();
        let ship_key = self.sol.u_data.fits.get(self.key).ship.unwrap();
        // Collect attr keys
        let mut attr_keys = Vec::new();
        for (attr_key, _) in self.sol.svc.iter_item_attr_vals(&self.sol.u_data, ship_key).unwrap() {
            if !attr_keys.contains(&attr_key) {
                attr_keys.push(attr_key);
            }
        }
        // Do 2 passes in case fetching some attributes triggers spawn of new attribs
        for (attr_key, _) in self.sol.svc.iter_item_attr_vals(&self.sol.u_data, ship_key).unwrap() {
            if !attr_keys.contains(&attr_key) {
                attr_keys.push(attr_key);
            }
        }
        for _ in 0..iterations {
            let item_key = self.sol.internal_add_module(
                self.key,
                ModRack::Low,
                AddMode::Equip,
                type_id,
                ModuleState::Online,
                None,
                None,
                &mut reuse_eupdates,
            );
            for attr_key in attr_keys.iter().copied() {
                let _ = self
                    .sol
                    .svc
                    .get_item_attr_val_full(&self.sol.u_data, ship_key, attr_key);
            }
            self.sol
                .internal_remove_module(item_key, RmMode::Free, &mut reuse_eupdates);
            for attr_key in attr_keys.iter().copied() {
                let _ = self
                    .sol
                    .svc
                    .get_item_attr_val_full(&self.sol.u_data, ship_key, attr_key);
            }
        }
    }
}
