use itertools::Itertools;

use crate::{
    sol::{item::SolItem, SolView},
    src::Src,
};

use super::SolarSystem;

impl SolarSystem {
    pub fn set_src(&mut self, src: Src) {
        // Unregister from services
        let sol_view = &SolView::new(&self.src, &self.fleets, &self.fits, &self.items);
        let mut autocharge_ids = Vec::new();
        for item in self.items.iter() {
            match item {
                SolItem::Autocharge(autocharge) => {
                    autocharge_ids.push(autocharge.get_id());
                    self.svcs.remove_item(sol_view, item);
                }
                _ => {
                    if item.is_loaded() {
                        self.svcs.unload_item(sol_view, item);
                    }
                }
            }
        }
        // Remove old autocharges
        for autocharge_id in autocharge_ids.into_iter() {
            self.items.remove_item(&autocharge_id);
        }
        // Reload regular items & set new source
        for item in self.items.iter_mut() {
            item.reload_a_item(&src)
        }
        self.src = src;
        // Update fit kind
        for fit in self.fits.iter_fits_mut() {
            fit.update_fit_kind(&mut self.items);
        }
        // Update autocharges
        for item_id in self.items.iter().map(|v| v.get_id()).collect_vec() {
            self.update_item_autocharges(&item_id);
        }
        // Register things in services again
        let sol_view = &SolView::new(&self.src, &self.fleets, &self.fits, &self.items);
        for item in self.items.iter() {
            match item {
                SolItem::Autocharge(_) => {
                    self.svcs.add_item(sol_view, item);
                }
                _ => {
                    if item.is_loaded() {
                        self.svcs.load_item(sol_view, item);
                    }
                }
            }
        }
    }
}
