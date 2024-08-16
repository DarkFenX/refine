use crate::{
    ad,
    defs::{AttrVal, EAttrId, SolFitId, SolItemId},
    sol::{
        fleet::SolFleet,
        item::{SolItem, SolItemState},
        svc::SolSvcs,
        SolView,
    },
};

impl SolSvcs {
    pub(in crate::sol::svc) fn notify_fit_added(&mut self, fit_id: &SolFitId) {
        self.calc_fit_added(fit_id);
    }
    pub(in crate::sol::svc) fn notify_fit_removed(&mut self, fit_id: &SolFitId) {
        self.calc_fit_removed(fit_id);
    }
    pub(in crate::sol::svc) fn notify_fit_added_to_fleet(
        &mut self,
        sol_view: &SolView,
        fleet: &SolFleet,
        fit_id: &SolFitId,
    ) {
        self.calc_fit_added_to_fleet(sol_view, fleet, fit_id);
    }
    pub(in crate::sol::svc) fn notify_fit_removed_from_fleet(
        &mut self,
        sol_view: &SolView,
        fleet: &SolFleet,
        fit_id: &SolFitId,
    ) {
        self.calc_fit_removed_from_fleet(sol_view, fleet, fit_id);
    }
    pub(in crate::sol::svc) fn notify_item_added(&mut self, sol_view: &SolView, item: &SolItem) {
        self.calc_item_added(sol_view, item);
    }
    pub(in crate::sol::svc) fn notify_item_removed(&mut self, sol_view: &SolView, item: &SolItem) {
        self.calc_item_removed(sol_view, item);
    }
    pub(in crate::sol::svc) fn notify_state_activated(
        &mut self,
        sol_view: &SolView,
        item: &SolItem,
        state: &SolItemState,
    ) {
    }
    pub(in crate::sol::svc) fn notify_state_deactivated(
        &mut self,
        sol_view: &SolView,
        item: &SolItem,
        state: &SolItemState,
    ) {
    }
    pub(in crate::sol::svc) fn notify_item_loaded(&mut self, sol_view: &SolView, item: &SolItem) {
        self.calc_item_loaded(sol_view, item);
    }
    pub(in crate::sol::svc) fn notify_item_unloaded(&mut self, sol_view: &SolView, item: &SolItem) {
        self.calc_item_unloaded(sol_view, item);
    }
    pub(in crate::sol::svc) fn notify_item_state_activated_loaded(
        &mut self,
        sol_view: &SolView,
        item: &SolItem,
        state: &SolItemState,
    ) {
    }
    pub(in crate::sol::svc) fn notify_item_state_deactivated_loaded(
        &mut self,
        sol_view: &SolView,
        item: &SolItem,
        state: &SolItemState,
    ) {
    }
    pub(in crate::sol::svc) fn notify_effects_started(
        &mut self,
        sol_view: &SolView,
        item: &SolItem,
        effects: &Vec<ad::ArcEffect>,
    ) {
        self.running_effects
            .effects_started(item.get_id(), effects.iter().map(|v| v.id));
        self.calc_effects_started(sol_view, item, effects);
    }
    pub(in crate::sol::svc) fn notify_effects_stopped(
        &mut self,
        sol_view: &SolView,
        item: &SolItem,
        effects: &Vec<ad::ArcEffect>,
    ) {
        self.calc_effects_stopped(sol_view, item, effects);
        self.running_effects
            .effects_stopped(&item.get_id(), effects.iter().map(|v| &v.id));
    }
    pub(in crate::sol::svc) fn notify_item_projected(
        &mut self,
        sol_view: &SolView,
        projector_item: &SolItem,
        projectee_item: &SolItem,
    ) {
    }
    pub(in crate::sol::svc) fn notify_item_unprojected(
        &mut self,
        sol_view: &SolView,
        projector_item: &SolItem,
        projectee_item: &SolItem,
    ) {
    }
    pub(in crate::sol::svc) fn notify_item_proj_range_changed(
        &mut self,
        sol_view: &SolView,
        projector_item: &SolItem,
        projectee_item: &SolItem,
    ) {
    }
    pub(in crate::sol::svc) fn notify_effect_projected(
        &mut self,
        sol_view: &SolView,
        projector_item: &SolItem,
        effect: &ad::ArcEffect,
        projectee_item: &SolItem,
        range: Option<AttrVal>,
    ) {
        self.calc_effect_projected(sol_view, projector_item, effect, projectee_item, range);
    }
    pub(in crate::sol::svc) fn notify_effect_unprojected(
        &mut self,
        sol_view: &SolView,
        projector_item: &SolItem,
        effect: &ad::ArcEffect,
        projectee_item: &SolItem,
    ) {
        self.calc_effect_unprojected(sol_view, projector_item, effect, projectee_item);
    }
    pub(in crate::sol::svc) fn notify_effect_proj_range_changed(
        &mut self,
        sol_view: &SolView,
        projector_item: &SolItem,
        effect: &ad::ArcEffect,
        projectee_item: &SolItem,
        range: Option<AttrVal>,
    ) {
        self.calc_effect_proj_range_changed(sol_view, projector_item, effect, projectee_item, range);
    }
    pub(in crate::sol::svc) fn notify_attr_val_changed(
        &mut self,
        sol_view: &SolView,
        item_id: &SolItemId,
        attr_id: &EAttrId,
    ) {
        self.calc_attr_value_changed(sol_view, item_id, attr_id);
    }
}
