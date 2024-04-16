use std::collections::HashSet;

use crate::{
    ss::{item::SsItem, SsView},
    SsItemId,
};

use super::SolarSystem;

impl SolarSystem {
    pub fn debug_consistency_check(&self) -> bool {
        let view = SsView::new(&self.src, &self.fleets, &self.fits, &self.items);
        let mut seen_items = Vec::new();
        // Fleets
        if !self
            .fleets
            .iter_fleets()
            .all(|fleet| fleet.debug_consistency_check(&view))
        {
            return false;
        }
        // Fits
        if !self
            .fits
            .iter_fits()
            .all(|fit| fit.debug_consistency_check(&view, &mut seen_items))
        {
            return false;
        }
        // System-wide effects
        for item_id in self.sw_effects.iter() {
            seen_items.push(*item_id);
            let item = match self.items.get_item(item_id) {
                Ok(item) => item,
                _ => return false,
            };
            if !matches!(item, SsItem::SwEffect(_)) {
                return false;
            }
        }
        // Projected effects
        for item_id in self.proj_effects.iter() {
            seen_items.push(*item_id);
            let item = match self.items.get_item(item_id) {
                Ok(item) => item,
                _ => return false,
            };
            if !matches!(item, SsItem::ProjEffect(_)) {
                return false;
            }
        }
        // Check if we have any duplicate references to items
        if check_item_duplicates(&seen_items) {
            return false;
        }
        // Check if we have any unreferenced items
        if !self.items.iter().all(|item| seen_items.contains(&item.get_id())) {
            return false;
        }
        true
    }
}

fn check_item_duplicates(item_ids: &Vec<SsItemId>) -> bool {
    let mut uniq = HashSet::new();
    !item_ids.iter().all(|x| uniq.insert(x))
}
