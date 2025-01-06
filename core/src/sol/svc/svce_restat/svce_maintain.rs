use crate::{
    ad,
    defs::SolFitId,
    ec,
    sol::{
        item::{SolItem, SolItemState},
        svc::SolSvcs,
    },
};

impl SolSvcs {
    pub(in crate::sol::svc) fn restat_fit_added(&mut self, fit_id: &SolFitId) {
        self.restat.add_fit(*fit_id);
    }
    pub(in crate::sol::svc) fn restat_fit_removed(&mut self, fit_id: &SolFitId) {
        self.restat.remove_fit(fit_id);
    }
    pub(in crate::sol::svc) fn restat_item_loaded(&mut self, item: &SolItem) {}
    pub(in crate::sol::svc) fn restat_item_unloaded(&mut self, item: &SolItem) {}
    pub(in crate::sol::svc) fn restat_item_state_activated_loaded(&mut self, item: &SolItem, state: &SolItemState) {}
    pub(in crate::sol::svc) fn restat_item_state_deactivated_loaded(&mut self, item: &SolItem, state: &SolItemState) {}
    pub(in crate::sol::svc) fn restat_effects_started(&mut self, item: &SolItem, effects: &Vec<ad::ArcEffect>) {
        if let SolItem::Module(module) = item {
            for effect in effects {
                if effect.id == ec::effects::ONLINE {
                    let fit_data = self.restat.get_data_mut(&module.get_fit_id()).unwrap();
                    fit_data.mods_online.insert(module.get_id());
                }
            }
        }
    }
    pub(in crate::sol::svc) fn restat_effects_stopped(&mut self, item: &SolItem, effects: &Vec<ad::ArcEffect>) {
        if let SolItem::Module(module) = item {
            for effect in effects {
                if effect.id == ec::effects::ONLINE {
                    let fit_data = self.restat.get_data_mut(&module.get_fit_id()).unwrap();
                    fit_data.mods_online.remove(&module.get_id());
                }
            }
        }
    }
}
