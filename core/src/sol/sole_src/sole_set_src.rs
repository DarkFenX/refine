use itertools::Itertools;

use crate::{
    defs::{EEffectId, SolItemId},
    err::basic::ItemAllocError,
    sol::{
        item::{SolItem, SolShipKind},
        SolView, SolarSystem,
    },
    src::Src,
    util::StMap,
};

impl SolarSystem {
    pub fn set_src(&mut self, mut src: Src) -> Result<(), SetSrcError> {
        // Unregister and remove autocharges first, extracting necessary data. This data might be
        // needed later is source switch fails
        let autocharge_backup = self.extract_autocharges();
        // Process other non-autocharge items, autocharges should've been removed by this point
        let sol_view = &SolView::new(&self.src, &self.fleets, &self.fits, &self.items);
        for item in self.items.iter() {
            if item.is_loaded() {
                self.svcs.unload_item(sol_view, item);
            }
        }
        // Reload items & set new source
        std::mem::swap(&mut self.src, &mut src);
        for item in self.items.iter_mut() {
            item.reload_a_item(&self.src)
        }
        // Update autocharges - as first step because it can fail
        for item_id in self.items.iter().map(|v| v.get_id()).collect_vec() {
            // Undo all the changes we did so far in case it failed
            if let Err(e) = self.update_item_autocharges(&item_id) {
                // Remove autocharges we managed to add to skeleton so far (for items on which
                // update_item_autocharges() method did not fail)
                let mut autocharge_ids = Vec::new();
                for item in self.items.iter_mut() {
                    if let Some(item_autocharges) = item.get_autocharges_mut() {
                        autocharge_ids.extend(item_autocharges.values().map(|v| *v));
                        item_autocharges.clear();
                    }
                }
                for autocharge_id in autocharge_ids {
                    self.items.remove_item(&autocharge_id).unwrap();
                }
                // Set new source & reload regular items
                std::mem::swap(&mut self.src, &mut src);
                for item in self.items.iter_mut() {
                    item.reload_a_item(&self.src)
                }
                // Re-register regular items in services
                let sol_view = &SolView::new(&self.src, &self.fleets, &self.fits, &self.items);
                for item in self.items.iter() {
                    if item.is_loaded() {
                        self.svcs.load_item(sol_view, item);
                    }
                }
                // Move autocharges back
                for (item_id, backup_item_ac_map) in autocharge_backup.into_iter() {
                    for (effect_id, autocharge) in backup_item_ac_map.into_iter() {
                        let autocharge_id = autocharge.get_id();
                        // Skeleton
                        let item_autocharges = self
                            .items
                            .get_item_mut(&item_id)
                            .unwrap()
                            .get_autocharges_mut()
                            .unwrap();
                        item_autocharges.set(effect_id, autocharge_id);
                        self.items.add_item(autocharge);
                        // Services
                        let autocharge = self.items.get_item(&autocharge_id).unwrap();
                        self.svcs.add_item(
                            &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
                            autocharge,
                        );
                    }
                }
                return Err(e.into());
            }
        }
        // Update fit kind
        for fit in self.fits.iter_fits_mut() {
            fit.kind = match fit.ship {
                Some(ship_id) => self.items.get_item(&ship_id).unwrap().get_ship().unwrap().kind,
                None => SolShipKind::default(),
            }
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
        Ok(())
    }
    fn extract_autocharges(&mut self) -> StMap<SolItemId, StMap<EEffectId, SolItem>> {
        let mut backup_ac_map = StMap::new();
        for (item_id, item_autocharge_ids) in self.get_autocharge_id_map().into_iter() {
            let mut backup_item_ac_map = StMap::new();
            for (effect_id, autocharge_id) in item_autocharge_ids.into_iter() {
                // Remove from services
                let autocharge = self.items.get_item(&autocharge_id).unwrap();
                self.svcs.remove_item(
                    &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
                    autocharge,
                );
                // Update skeleton and move the charge item into backup container
                let item_autocharges = self
                    .items
                    .get_item_mut(&item_id)
                    .unwrap()
                    .get_autocharges_mut()
                    .unwrap();
                item_autocharges.remove(&effect_id);
                let autocharge = self.items.remove_item(&autocharge_id).unwrap();
                backup_item_ac_map.insert(effect_id, autocharge);
            }
            backup_ac_map.insert(item_id, backup_item_ac_map);
        }
        backup_ac_map
    }
    fn get_autocharge_id_map(&self) -> StMap<SolItemId, StMap<EEffectId, SolItemId>> {
        let mut backup_ac_id_map = StMap::new();
        for item in self.items.iter() {
            if let Some(item_autocharges) = item.get_autocharges() {
                if item_autocharges.is_empty() {
                    continue;
                }
                let mut backup_item_ac_id_map = StMap::new();
                for (effect_id, autocharge_id) in item_autocharges.iter() {
                    backup_item_ac_id_map.insert(*effect_id, *autocharge_id);
                }
                backup_ac_id_map.insert(item.get_id(), backup_item_ac_id_map);
            }
        }
        backup_ac_id_map
    }
}

#[derive(Debug)]
pub enum SetSrcError {
    ItemIdAllocFailed(ItemAllocError),
}
impl std::error::Error for SetSrcError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemIdAllocFailed(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetSrcError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemIdAllocFailed(e) => e.fmt(f),
        }
    }
}
impl From<ItemAllocError> for SetSrcError {
    fn from(error: ItemAllocError) -> Self {
        Self::ItemIdAllocFailed(error)
    }
}
