use crate::{
    sol::{
        ItemId,
        debug::{DebugError, DebugResult},
        uad::{Uad, item::Item},
    },
    util::RSet,
};

impl Uad {
    pub fn debug_consistency_check(&self) -> DebugResult {
        let mut seen_items = Vec::new();
        // Fleets
        for fleet in self.fleets.iter_fleets() {
            fleet.debug_consistency_check(self)?;
        }
        // Fits
        for fit in self.fits.iter_fits() {
            fit.debug_consistency_check(self, &mut seen_items)?;
        }
        // System-wide effects
        for item_id in self.sw_effects.iter() {
            seen_items.push(*item_id);
            let item = match self.items.get_by_id(item_id) {
                Ok(item) => item,
                _ => return Err(DebugError {}),
            };
            if !matches!(item, Item::SwEffect(_)) {
                return Err(DebugError {});
            }
            item.debug_consistency_check(self)?;
        }
        // Projected effects
        for item_id in self.proj_effects.iter() {
            seen_items.push(*item_id);
            let item = match self.items.get_by_id(item_id) {
                Ok(item) => item,
                _ => return Err(DebugError {}),
            };
            if !matches!(item, Item::ProjEffect(_)) {
                return Err(DebugError {});
            }
            item.debug_consistency_check(self)?;
        }
        // Check if we have any duplicate references to items
        if check_item_duplicates(&seen_items) {
            return Err(DebugError {});
        }
        // Check if we have any unreferenced items
        if !self.items.iter().all(|item| seen_items.contains(&item.get_item_id())) {
            return Err(DebugError {});
        }
        Ok(())
    }
}

fn check_item_duplicates(item_ids: &[ItemId]) -> bool {
    let mut uniq = RSet::new();
    !item_ids.iter().all(|x| uniq.insert(x))
}
