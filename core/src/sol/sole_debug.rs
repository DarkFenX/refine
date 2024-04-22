use crate::{
    defs::SolItemId,
    sol::{item::SolItem, SolView},
    util::{DebugError, DebugResult, StSet},
};

use super::SolarSystem;

impl SolarSystem {
    // This function is intended to be used in tests, to make sure inner state is consistent, i.e.
    // no links broken, mutual references are correct, etc. All the entities which contain data
    // should be checked, and this function should be called from tests, to ensure there are no
    // memory leaks.
    pub fn debug_consistency_check(&self) -> bool {
        let sol_view = SolView::new(&self.src, &self.fleets, &self.fits, &self.items);
        // Check solar system structure
        if self.check_sol_structure(&sol_view).is_err() {
            return false;
        }
        // Check solar system helper data containers
        if self.tgt_tracker.debug_consistency_check(&sol_view).is_err() {
            return false;
        }
        // Check services
        if self.svcs.debug_consistency_check(&sol_view).is_err() {
            return false;
        }
        true
    }
    // Check that entities which define solar system object structure are consistent
    fn check_sol_structure(&self, sol_view: &SolView) -> DebugResult {
        let mut seen_items = Vec::new();
        // Fleets
        for fleet in self.fleets.iter_fleets() {
            fleet.debug_consistency_check(&sol_view)?;
        }
        // Fits
        for fit in self.fits.iter_fits() {
            fit.debug_consistency_check(&sol_view, &mut seen_items)?;
        }
        // System-wide effects
        for item_id in self.sw_effects.iter() {
            seen_items.push(*item_id);
            let item = match self.items.get_item(item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if !matches!(item, SolItem::SwEffect(_)) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(sol_view)?;
        }
        // Projected effects
        for item_id in self.proj_effects.iter() {
            seen_items.push(*item_id);
            let item = match self.items.get_item(item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if !matches!(item, SolItem::ProjEffect(_)) {
                return Err(DebugError::new());
            }
            item.debug_consistency_check(sol_view)?;
        }
        // Check if we have any duplicate references to items
        if check_item_duplicates(&seen_items) {
            return Err(DebugError::new());
        }
        // Check if we have any unreferenced items
        if !self.items.iter().all(|item| seen_items.contains(&item.get_id())) {
            return Err(DebugError::new());
        }
        Ok(())
    }
}

fn check_item_duplicates(item_ids: &Vec<SolItemId>) -> bool {
    let mut uniq = StSet::new();
    !item_ids.iter().all(|x| uniq.insert(x))
}
