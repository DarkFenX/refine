use std::collections::HashSet;

use crate::{
    ss::{item::SsItem, SsView},
    SsItemId,
};

use super::SolarSystem;

impl SolarSystem {
    pub fn debug_consistency_check(&self) -> bool {
        let view = SsView::new(&self.src, &self.fleets, &self.fits, &self.items);
        let mut used_item_ids = Vec::new();
        // Fleets
        for fleet in self.fleets.iter_fleets() {
            if !fleet.debug_consistency_check(&view) {
                return false;
            }
        }
        // On-solar system items
        for item_id in self.sw_effects.iter() {
            used_item_ids.push(*item_id);
            let item = match self.items.get_item(item_id) {
                Ok(item) => item,
                _ => return false,
            };
            if !matches!(item, SsItem::SwEffect(_)) {
                return false;
            }
        }
        for item_id in self.proj_effects.iter() {
            used_item_ids.push(*item_id);
            let item = match self.items.get_item(item_id) {
                Ok(item) => item,
                _ => return false,
            };
            if !matches!(item, SsItem::ProjEffect(_)) {
                return false;
            }
        }
        // Checks on data we gathered throughout the process
        if check_item_duplicates(&used_item_ids) {
            return false;
        }
        true
    }
}

fn check_item_duplicates(item_ids: &Vec<SsItemId>) -> bool {
    let mut uniq = HashSet::new();
    !item_ids.iter().all(|x| uniq.insert(x))
}
