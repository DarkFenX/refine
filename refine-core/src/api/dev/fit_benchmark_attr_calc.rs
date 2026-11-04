use crate::{AddMode, Fit, ItemTypeId, ModRack, ModuleState, RemoveMode};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Fit<'s> {
    pub fn benchmark_attr_calc(&mut self, type_id: ItemTypeId, iterations: usize) {
        let ship_uid = self.core.u_data.fits.get(self.uid).ship.unwrap();
        // Collect attr keys
        let mut attr_rids = Vec::new();
        for (attr_rid, _) in self.core.svc.iter_item_attr_vals(&self.core.u_data, ship_uid).unwrap() {
            if !attr_rids.contains(&attr_rid) {
                attr_rids.push(attr_rid);
            }
        }
        // Do 2 passes in case fetching some attributes triggers spawn of new attribs
        for (attr_rid, _) in self.core.svc.iter_item_attr_vals(&self.core.u_data, ship_uid).unwrap() {
            if !attr_rids.contains(&attr_rid) {
                attr_rids.push(attr_rid);
            }
        }
        let type_aid = type_id.into_aid();
        for _ in 0..iterations {
            let item_uid = self
                .core
                .internal_add_module(
                    self.uid,
                    ModRack::Low,
                    AddMode::Equip,
                    type_aid,
                    ModuleState::Online,
                    None,
                    None,
                )
                .unwrap();
            for attr_rid in attr_rids.iter().copied() {
                let _ = self
                    .core
                    .svc
                    .get_item_attr_val_full(&self.core.u_data, ship_uid, attr_rid);
            }
            self.core.internal_remove_module(item_uid, RemoveMode::Free);
            for attr_rid in attr_rids.iter().copied() {
                let _ = self
                    .core
                    .svc
                    .get_item_attr_val_full(&self.core.u_data, ship_uid, attr_rid);
            }
        }
    }
}
