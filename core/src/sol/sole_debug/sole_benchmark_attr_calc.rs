use std::hint::black_box;

use crate::sol::{AddMode, FitId, ItemTypeId, ModRack, SolarSystem, uad::item::ModuleState};

impl SolarSystem {
    pub fn benchmark_attr_calc(&mut self, fit_id: &FitId, type_id: ItemTypeId, iterations: usize) {
        let fit_key = self.uad.fits.key_by_id(fit_id).unwrap();
        let ship_key = self.uad.fits.get(fit_key).ship.unwrap();
        for _ in 0..iterations {
            let item_key = self.add_module_internal(
                fit_key,
                ModRack::Low,
                AddMode::Equip,
                type_id,
                ModuleState::Online,
                None,
                None,
            );
            black_box(self.iter_item_attrs_internal(ship_key).iter().for_each(drop));
            self.remove_module_internal(item_key, crate::RmMode::Free).unwrap();
            black_box(self.iter_item_attrs_internal(ship_key).iter().for_each(drop));
        }
    }
}
