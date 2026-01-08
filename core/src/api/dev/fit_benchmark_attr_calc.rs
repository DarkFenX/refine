use crate::{
    api::{AddMode, FitMut, ItemTypeId, ModuleState, RmMode},
    misc::ModRack,
    ud::UEffectUpdates,
};

impl<'a> FitMut<'a> {
    pub fn benchmark_attr_calc(&mut self, type_id: ItemTypeId, iterations: usize) {
        let mut reuse_eupdates = UEffectUpdates::new();
        let ship_uid = self.sol.u_data.fits.get(self.uid).ship.unwrap();
        // Collect attr keys
        let mut attr_rids = Vec::new();
        for (attr_rid, _) in self.sol.svc.iter_item_attr_vals(&self.sol.u_data, ship_uid).unwrap() {
            if !attr_rids.contains(&attr_rid) {
                attr_rids.push(attr_rid);
            }
        }
        // Do 2 passes in case fetching some attributes triggers spawn of new attribs
        for (attr_rid, _) in self.sol.svc.iter_item_attr_vals(&self.sol.u_data, ship_uid).unwrap() {
            if !attr_rids.contains(&attr_rid) {
                attr_rids.push(attr_rid);
            }
        }
        let item_aid = type_id.into_aid();
        for _ in 0..iterations {
            let item_uid = self.sol.internal_add_module(
                self.uid,
                ModRack::Low,
                AddMode::Equip,
                item_aid,
                ModuleState::Online,
                None,
                None,
                &mut reuse_eupdates,
            );
            for attr_rid in attr_rids.iter().copied() {
                let _ = self
                    .sol
                    .svc
                    .get_item_attr_val_full(&self.sol.u_data, ship_uid, attr_rid);
            }
            self.sol
                .internal_remove_module(item_uid, RmMode::Free, &mut reuse_eupdates);
            for attr_rid in attr_rids.iter().copied() {
                let _ = self
                    .sol
                    .svc
                    .get_item_attr_val_full(&self.sol.u_data, ship_uid, attr_rid);
            }
        }
    }
}
