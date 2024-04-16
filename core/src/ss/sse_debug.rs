use std::collections::HashSet;

use crate::{
    defs::SsItemId,
    ss::{item::SsItem, SsView},
    util::{DebugError, DebugResult},
};

use super::SolarSystem;

impl SolarSystem {
    // This function is intended to be used in tests, to make sure inner state is consistent, i.e.
    // no links broken, mutual references are correct, etc.
    pub fn debug_consistency_check(&self) -> bool {
        let ss_view = SsView::new(&self.src, &self.fleets, &self.fits, &self.items);
        if self.check_ss_structure(&ss_view).is_err() {
            return false;
        }
        if self.svcs.debug_consistency_check(&ss_view).is_err() {
            return false;
        }
        true
    }
    // Check that entities which define solar system object structure are consistent
    fn check_ss_structure(&self, ss_view: &SsView) -> DebugResult {
        let mut seen_items = Vec::new();
        // Fleets
        for fleet in self.fleets.iter_fleets() {
            fleet.debug_consistency_check(&ss_view)?;
        }
        // Fits
        for fit in self.fits.iter_fits() {
            fit.debug_consistency_check(&ss_view, &mut seen_items)?;
        }
        // System-wide effects
        for item_id in self.sw_effects.iter() {
            seen_items.push(*item_id);
            let item = match self.items.get_item(item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            if !matches!(item, SsItem::SwEffect(_)) {
                return Err(DebugError::new());
            }
        }
        // Projected effects
        for item_id in self.proj_effects.iter() {
            seen_items.push(*item_id);
            let item = match self.items.get_item(item_id) {
                Ok(item) => item,
                _ => return Err(DebugError::new()),
            };
            let proj_effect = match item {
                SsItem::ProjEffect(proj_effect) => proj_effect,
                _ => return Err(DebugError::new()),
            };
            for tgt_item_id in proj_effect.tgts.iter() {
                if ss_view.items.get_item(tgt_item_id).is_err() {
                    return Err(DebugError::new());
                }
            }
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

fn check_item_duplicates(item_ids: &Vec<SsItemId>) -> bool {
    let mut uniq = HashSet::new();
    !item_ids.iter().all(|x| uniq.insert(x))
}
