use std::hint::black_box;

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
            #[allow(clippy::unit_arg)]
            black_box(
                self.sol
                    .svc
                    .iter_item_attr_vals(&self.sol.u_data, ship_key)
                    .unwrap()
                    .for_each(drop),
            );
            self.sol
                .internal_remove_module(item_key, RmMode::Free, &mut reuse_eupdates);
            #[allow(clippy::unit_arg)]
            black_box(
                self.sol
                    .svc
                    .iter_item_attr_vals(&self.sol.u_data, ship_key)
                    .unwrap()
                    .for_each(drop),
            );
        }
    }
}
