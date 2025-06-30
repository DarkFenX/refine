use std::hint::black_box;

use crate::{
    def::ItemTypeId,
    misc::{AddMode, ModRack, ModuleState, RmMode},
    sol::api::FitMut,
};

impl<'a> FitMut<'a> {
    pub fn benchmark_attr_calc(&mut self, type_id: ItemTypeId, iterations: usize) {
        let ship_key = self.sol.uad.fits.get(self.key).ship.unwrap();
        for _ in 0..iterations {
            let item_key = self.sol.internal_add_module(
                self.key,
                ModRack::Low,
                AddMode::Equip,
                type_id,
                ModuleState::Online,
                None,
                None,
            );
            #[allow(clippy::unit_arg)]
            black_box(
                self.sol
                    .svc
                    .iter_item_attr_vals(&self.sol.uad, ship_key)
                    .unwrap()
                    .for_each(drop),
            );
            self.sol.internal_remove_module(item_key, RmMode::Free);
            #[allow(clippy::unit_arg)]
            black_box(
                self.sol
                    .svc
                    .iter_item_attr_vals(&self.sol.uad, ship_key)
                    .unwrap()
                    .for_each(drop),
            );
        }
    }
}
