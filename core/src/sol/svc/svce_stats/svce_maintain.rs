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
    pub(in crate::sol::svc) fn stats_item_state_activated_loaded(
        &mut self,
        item: &SolItem,
        state: &SolItemState,
    ) {
        if let Some(fit_id) = item.get_fit_id() {
            match state {
                SolItemState::Online => {
                    let item_id = item.get_id();
                    self.stats.cpu.register_item(fit_id, item_id);
                }
                _ => (),
            }
        }
    }
    pub(in crate::sol::svc) fn stats_item_state_deactivated_loaded(
        &mut self,
        item: &SolItem,
        state: &SolItemState,
    ) {
        if let Some(fit_id) = item.get_fit_id() {
            match state {
                SolItemState::Online => {
                    let item_id = item.get_id();
                    self.stats.cpu.unregister_item(&fit_id, &item_id);
                }
                _ => (),
            }
        }
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
