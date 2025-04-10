use crate::{
    sol::{
        ItemKey,
        debug::{DebugError, DebugResult},
        uad::{Uad, item::Item},
    },
    util::RSet,
};

impl Uad {
    pub fn consistency_check(&self) -> DebugResult {
        let mut seen_items = Vec::new();
        // Fleets
        for fleet in self.fleets.values() {
            fleet.consistency_check(self)?;
        }
        // Fits
        for fit in self.fits.values() {
            fit.consistency_check(self, &mut seen_items)?;
        }
        // System-wide effects
        for &sw_effect_key in self.sw_effects.iter() {
            seen_items.push(sw_effect_key);
            let item = match self.items.try_get(sw_effect_key) {
                Some(item) => item,
                None => return Err(DebugError {}),
            };
            if !matches!(item, Item::SwEffect(_)) {
                return Err(DebugError {});
            }
            item.consistency_check(self)?;
        }
        // Projected effects
        for &proj_effect_key in self.proj_effects.iter() {
            seen_items.push(proj_effect_key);
            let item = match self.items.try_get(proj_effect_key) {
                Some(item) => item,
                None => return Err(DebugError {}),
            };
            if !matches!(item, Item::ProjEffect(_)) {
                return Err(DebugError {});
            }
            item.consistency_check(self)?;
        }
        // Check if we have any duplicate references to items
        if check_item_duplicates(&seen_items) {
            return Err(DebugError {});
        }
        // Check if we have any unreferenced items
        if !self.items.keys().all(|item_key| seen_items.contains(&item_key)) {
            return Err(DebugError {});
        }
        // Checks for internal container consistency
        self.items.consistency_check()?;
        self.fits.consistency_check()?;
        self.fleets.consistency_check()?;
        Ok(())
    }
}

fn check_item_duplicates(item_ids: &[ItemKey]) -> bool {
    let mut uniq = RSet::new();
    !item_ids.iter().all(|x| uniq.insert(x))
}
