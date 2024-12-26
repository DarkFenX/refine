use itertools::Itertools;

use crate::{
    ad,
    defs::SolItemId,
    ec,
    sol::{item::SolItem, svc::SolSvcs},
    util::StMap,
};

impl SolSvcs {
    fn calc_rah_effects_started(&mut self, item: &SolItem, effects: &Vec<ad::ArcEffect>) {
        // TODO: set callbacks and emit "attr changed" events for cleared results
        if let SolItem::Module(module) = item {
            if effects.iter().any(|v| v.id == ec::effects::ADAPTIVE_ARMOR_HARDENER) {
                let item_id = module.get_id();
                let fit_id = module.get_fit_id();
                let other_item_ids = self.calc_data.rah.by_fit.get(&fit_id).map(|v| *v).collect_vec();
                for other_item_id in other_item_ids {
                    self.clear_results_for_item(&other_item_id);
                }
                self.calc_data.rah.resonances.insert(item_id, StMap::new());
                self.calc_data.rah.by_fit.add_entry(fit_id, item_id);
            }
        }
    }
    fn calc_rah_effects_stopped(&mut self, item: &SolItem, effects: &Vec<ad::ArcEffect>) {
        // TODO: remove callbacks and emit "attr changed" events for cleared results
        if let SolItem::Module(module) = item {
            if effects.iter().any(|v| v.id == ec::effects::ADAPTIVE_ARMOR_HARDENER) {
                let item_id = module.get_id();
                let fit_id = module.get_fit_id();
                self.calc_data.rah.resonances.remove(&item_id);
                self.calc_data.rah.by_fit.remove_entry(&module.get_fit_id(), &item_id);
                let other_item_ids = self.calc_data.rah.by_fit.get(&fit_id).map(|v| *v).collect_vec();
                for other_item_id in other_item_ids {
                    self.clear_results_for_item(&other_item_id);
                }
            }
        }
    }
    fn calc_rah_attr_value_changed(&mut self) {
        // Args: item ID, attr ID
        // Ship resistance attributes
        // - get item from view, if not ship do nothing, if ship go on
        // - clear results for all RAHs for ship's fit
        // RAH resistance attributes, shift amount
        // - check if item is in main storage, if no do nothing, if yes go on
        // - clear results for all items for its fit
        // RAH cycle time
        // - check if item is in main storage, if no do nothing, if yes go on
        // - if fit has only one RAH (check in fit-to-rah map), do nothing, if more go on
        // - clear results for all items for its fit
    }
    fn calc_rah_dmg_profile_changed(&mut self) {}
    fn clear_results_for_item(&mut self, item_id: &SolItemId) {
        self.calc_data.rah.resonances.get_mut(item_id).unwrap().clear();
    }
}
