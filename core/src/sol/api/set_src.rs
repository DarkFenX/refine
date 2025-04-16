use itertools::Itertools;

use crate::{
    sol::{
        SolarSystem,
        uad::item::{ShipKind, UadItem},
    },
    src::Src,
    util::RMapVec,
};

impl SolarSystem {
    pub fn set_src(&mut self, mut src: Src) {
        self.remove_autocharges();
        // Process non-autocharge items
        for (item_key, item) in self.uad.items.iter() {
            self.svc.unload_item(&self.uad, item_key, item);
        }
        // Set new source, update source-dependent data in services and reload items
        std::mem::swap(&mut self.uad.src, &mut src);
        self.svc.src_changed(&self.uad.src);
        for item in self.uad.items.values_mut() {
            item.update_a_data(&self.uad.src)
        }
        // Update fit kind
        for fit in self.uad.fits.values_mut() {
            fit.kind = match fit.ship {
                Some(ship_key) => self.uad.items.get(ship_key).get_ship().unwrap().get_kind(),
                None => ShipKind::Unknown,
            }
        }
        // Update autocharges
        for autocharge_carrier_key in self
            .uad
            .items
            .iter()
            .filter(|(_, item)| item.get_autocharges().is_some())
            .map(|(item_key, _)| item_key)
            .collect_vec()
        {
            self.internal_add_item_autocharges(autocharge_carrier_key);
        }
        // Register things in services again
        for (item_key, item) in self.uad.items.iter() {
            match item {
                UadItem::Autocharge(autocharge) => {
                    // Autocharges are new, so we're adding them, not loading
                    self.svc.add_item(&self.uad, item_key, item);
                    // For autocharges also enable outgoing projections
                    for (projectee_item_key, range) in autocharge.get_projs().iter() {
                        self.proj_tracker.reg_projectee(item_key, *projectee_item_key);
                        let projectee_item = self.uad.items.get(*projectee_item_key);
                        self.svc
                            .add_item_projection(&self.uad, item_key, *projectee_item_key, projectee_item, *range);
                    }
                }
                _ => self.svc.load_item(&self.uad, item_key, item),
            }
        }
    }
    fn remove_autocharges(&mut self) {
        let mut autocharge_map = RMapVec::new();
        // Collect data and update what we can right away
        for (item_key, item) in self.uad.items.iter() {
            if let Some(autocharges) = item.get_autocharges() {
                for &autocharge_key in autocharges.values() {
                    let autocharge_item = self.uad.items.get(autocharge_key);
                    let autocharge = autocharge_item.get_autocharge().unwrap();
                    for &projectee_item_key in autocharge.get_projs().iter_projectee_item_keys() {
                        let projectee_item = self.uad.items.get(projectee_item_key);
                        // Update services
                        self.svc
                            .remove_item_projection(&self.uad, autocharge_key, projectee_item_key, projectee_item);
                        // Update user data for autocharge - don't touch data on charge itself,
                        // since charge will be removed later anyway
                        self.proj_tracker.unreg_projectee(&autocharge_key, &projectee_item_key);
                    }
                    // Remove from services
                    self.svc.remove_item(&self.uad, autocharge_key, autocharge_item);
                    autocharge_map.add_entry(item_key, autocharge_key);
                }
            }
        }
        // Update items
        for (item_key, autocharge_keys) in autocharge_map.into_iter() {
            self.uad.items.get_mut(item_key).get_autocharges_mut().unwrap().clear();
            for autocharge_key in autocharge_keys {
                self.uad.items.remove(autocharge_key);
            }
        }
    }
}
