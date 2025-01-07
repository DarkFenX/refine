use itertools::Itertools;

use crate::{
    sol::{
        uad::item::{SolItem, SolShipKind},
        SolarSystem,
    },
    src::Src,
    util::StMapVecL1,
};

impl SolarSystem {
    pub fn set_src(&mut self, mut src: Src) {
        self.remove_autocharges();
        // Process non-autocharge items
        for item in self.uad.items.iter() {
            self.svc.unload_item(&self.uad, item);
        }
        // Set new source, update source-dependent data in services and reload items
        std::mem::swap(&mut self.uad.src, &mut src);
        self.svc.src_changed(&self.uad.src);
        for item in self.uad.items.iter_mut() {
            item.update_a_data(&self.uad.src)
        }
        // Update fit kind
        for fit in self.uad.fits.iter_fits_mut() {
            fit.kind = match fit.ship {
                Some(ship_id) => self
                    .uad
                    .items
                    .get_item(&ship_id)
                    .unwrap()
                    .get_ship()
                    .unwrap()
                    .get_kind(),
                None => SolShipKind::Unknown,
            }
        }
        // Update autocharges
        for autocharge_carrier_id in self
            .uad
            .items
            .iter()
            .filter(|v| v.get_autocharges().is_some())
            .map(|v| v.get_id())
            .collect_vec()
        {
            self.add_item_autocharges(&autocharge_carrier_id);
        }
        // Register things in services again
        for item in self.uad.items.iter() {
            match item {
                SolItem::Autocharge(autocharge) => {
                    // Autocharges are new, so we're adding them, not loading
                    self.svc.add_item(&self.uad, item);
                    // For autocharges also enable outgoing projections
                    for (projectee_item_id, range) in autocharge.get_projs().iter() {
                        self.proj_tracker.reg_projectee(autocharge.get_id(), *projectee_item_id);
                        let projectee_item = self.uad.items.get_item(projectee_item_id).unwrap();
                        self.svc.add_item_projection(&self.uad, item, projectee_item, *range);
                    }
                }
                _ => self.svc.load_item(&self.uad, item),
            }
        }
    }
    fn remove_autocharges(&mut self) {
        let mut autocharge_map = StMapVecL1::new();
        // Collect data and update what we can right away
        for item in self.uad.items.iter() {
            if let Some(autocharges) = item.get_autocharges() {
                for autocharge_id in autocharges.values() {
                    let autocharge_item = self.uad.items.get_item(autocharge_id).unwrap();
                    let autocharge = autocharge_item.get_autocharge().unwrap();
                    for projectee_item_id in autocharge.get_projs().iter_items() {
                        let projectee_item = self.uad.items.get_item(projectee_item_id).unwrap();
                        // Update services
                        self.svc
                            .remove_item_projection(&self.uad, autocharge_item, projectee_item);
                        // Update skeleton for autocharge - don't touch data on charge itself, since
                        // charge will be removed later anyway
                        self.proj_tracker.unreg_projectee(&autocharge_id, projectee_item_id);
                    }
                    // Remove from services
                    self.svc.remove_item(&self.uad, autocharge_item);
                    autocharge_map.add_entry(item.get_id(), *autocharge_id);
                }
            }
        }
        // Update items
        for (item_id, autocharge_ids) in autocharge_map.into_iter() {
            self.uad
                .items
                .get_item_mut(&item_id)
                .unwrap()
                .get_autocharges_mut()
                .unwrap()
                .clear();
            for autocharge_id in autocharge_ids {
                self.uad.items.remove_item(&autocharge_id);
            }
        }
    }
}
