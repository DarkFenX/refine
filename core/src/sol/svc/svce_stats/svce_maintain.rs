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
    pub(in crate::sol::svc) fn stats_fit_added(&mut self, fit_id: &SolFitId) {}
    pub(in crate::sol::svc) fn stats_fit_removed(&mut self, fit_id: &SolFitId) {}
    pub(in crate::sol::svc) fn stats_item_loaded(&mut self, sol_view: &SolView, item: &SolItem) {}
    pub(in crate::sol::svc) fn stats_item_unloaded(&mut self, sol_view: &SolView, item: &SolItem) {}
    pub(in crate::sol::svc) fn stats_item_state_activated_loaded(&mut self, item: &SolItem, state: &SolItemState) {
        self.stats.mods_online.item_state_activated_loaded(item, state);
    }
    pub(in crate::sol::svc) fn stats_item_state_deactivated_loaded(&mut self, item: &SolItem, state: &SolItemState) {
        self.stats.mods_online.item_state_deactivated_loaded(item, state);
    }
    pub(in crate::sol::svc) fn stats_effects_started(
        &mut self,
        sol_view: &SolView,
        item: &SolItem,
        effects: &Vec<ad::ArcEffect>,
    ) {
    }
    pub(in crate::sol::svc) fn stats_effects_stopped(
        &mut self,
        sol_view: &SolView,
        item: &SolItem,
        effects: &Vec<ad::ArcEffect>,
    ) {
    }
    // Private methods
}
