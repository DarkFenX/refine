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
    pub(in crate::sol::svc) fn restat_fit_added(&mut self, fit_id: &SolFitId) {
        self.restat.add_fit(*fit_id);
    }
    pub(in crate::sol::svc) fn restat_fit_removed(&mut self, fit_id: &SolFitId) {
        self.restat.remove_fit(fit_id);
    }
    pub(in crate::sol::svc) fn restat_item_loaded(&mut self, sol_view: &SolView, item: &SolItem) {}
    pub(in crate::sol::svc) fn restat_item_unloaded(&mut self, sol_view: &SolView, item: &SolItem) {}
    pub(in crate::sol::svc) fn restat_item_state_activated_loaded(&mut self, item: &SolItem, state: &SolItemState) {
        if matches!(state, SolItemState::Online) {
            if let SolItem::Module(module) = item {
                let fit_data = self.restat.get_data_mut(&module.get_fit_id()).unwrap();
                fit_data.mods_online.insert(item.get_id());
            }
        }
    }
    pub(in crate::sol::svc) fn restat_item_state_deactivated_loaded(&mut self, item: &SolItem, state: &SolItemState) {
        if matches!(state, SolItemState::Online) {
            if let SolItem::Module(module) = item {
                let fit_data = self.restat.get_data_mut(&module.get_fit_id()).unwrap();
                fit_data.mods_online.remove(&item.get_id());
            }
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
}
