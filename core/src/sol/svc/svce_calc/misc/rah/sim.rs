use crate::{
    ad,
    defs::{EAttrId, SolFitId, SolItemId},
    ec,
    sol::{item::SolItem, svc::svce_calc::SolAttrVal},
    util::{StMap, StMapSetL1},
};

struct RahSim {
    resonances: StMap<SolItemId, StMap<EAttrId, SolAttrVal>>,
    by_fit: StMapSetL1<SolFitId, SolItemId>,
}
impl RahSim {
    fn new() -> Self {
        Self {
            resonances: StMap::new(),
            by_fit: StMapSetL1::new(),
        }
    }
    fn handle_effect_started(&mut self, item: &SolItem, effects: &Vec<ad::ArcEffect>) {
        // TODO: set callbacks and emit "attr changed" events for cleared results
        if let SolItem::Module(module) = item {
            if effects.iter().any(|v| v.id == ec::effects::ADAPTIVE_ARMOR_HARDENER) {
                let item_id = module.get_id();
                let fit_id = module.get_fit_id();
                for other_item_id in self.by_fit.get(&fit_id) {
                    self.resonances.get_mut(other_item_id).unwrap().clear();
                }
                self.resonances.insert(item_id, StMap::new());
                self.by_fit.add_entry(fit_id, item_id);
            }
        }
    }
    fn handle_effect_stopped(&mut self, item: &SolItem, effects: &Vec<ad::ArcEffect>) {
        // TODO: remove callbacks and emit "attr changed" events for cleared results
        if let SolItem::Module(module) = item {
            if effects.iter().any(|v| v.id == ec::effects::ADAPTIVE_ARMOR_HARDENER) {
                let item_id = module.get_id();
                let fit_id = module.get_fit_id();
                self.resonances.remove(&item_id);
                self.by_fit.remove_entry(&module.get_fit_id(), &item_id);
                for other_item_id in self.by_fit.get(&fit_id) {
                    self.resonances.get_mut(other_item_id).unwrap().clear();
                }
            }
        }
    }
    fn handle_attr_value_changed(&mut self) {
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
    fn handle_dmg_profile_changed(&mut self) {}
}
