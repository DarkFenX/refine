use crate::{
    ad,
    defs::SolFitId,
    sol::{
        item::{SolItem, SolItemState},
        svc::SolSvcs,
        SolView,
    },
};

impl SolSvcs {
    // Modification methods
    pub(in crate::sol::svc) fn restat_fit_added(&mut self, fit_id: &SolFitId) {
        self.restat.add_fit(*fit_id);
    }
    pub(in crate::sol::svc) fn restat_fit_removed(&mut self, fit_id: &SolFitId) {
        self.restat.remove_fit(fit_id);
    }
    pub(in crate::sol::svc) fn restat_item_loaded(&mut self, sol_view: &SolView, item: &SolItem) {}
    pub(in crate::sol::svc) fn restat_item_unloaded(&mut self, sol_view: &SolView, item: &SolItem) {}
    pub(in crate::sol::svc) fn restat_item_state_activated_loaded(&mut self, item: &SolItem, state: &SolItemState) {
        if let Some(fit_id) = item.get_fit_id() {
            let fit_data = self.restat.get_data_mut(&fit_id).unwrap();
            fit_data.mods_online.item_state_activated_loaded(item, state);
        }
    }
    pub(in crate::sol::svc) fn restat_item_state_deactivated_loaded(&mut self, item: &SolItem, state: &SolItemState) {
        if let Some(fit_id) = item.get_fit_id() {
            let fit_data = self.restat.get_data_mut(&fit_id).unwrap();
            fit_data.mods_online.item_state_deactivated_loaded(item, state);
        }
    }
    pub(in crate::sol::svc) fn restat_effects_started(
        &mut self,
        sol_view: &SolView,
        item: &SolItem,
        effects: &Vec<ad::ArcEffect>,
    ) {
    }
    pub(in crate::sol::svc) fn restats_effects_stopped(
        &mut self,
        sol_view: &SolView,
        item: &SolItem,
        effects: &Vec<ad::ArcEffect>,
    ) {
    }
    // Private methods
}
