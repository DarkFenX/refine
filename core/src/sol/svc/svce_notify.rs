use crate::{
    EAttrId, ad,
    defs::{AttrVal, SolFitId, SolItemId},
    sol::{
        svc::SolSvc,
        uad::{
            SolUad,
            fleet::SolFleet,
            item::{SolFighter, SolItem, SolItemState, SolSkill},
        },
    },
    src::Src,
};

impl SolSvc {
    pub(in crate::sol::svc) fn notify_src_changed(&mut self, src: &Src) {
        self.calc.src_changed(src);
    }
    pub(in crate::sol::svc) fn notify_fit_added(&mut self, fit_id: &SolFitId) {
        self.calc.fit_added(fit_id);
        self.vast.fit_added(fit_id);
    }
    pub(in crate::sol::svc) fn notify_fit_removed(&mut self, fit_id: &SolFitId) {
        self.calc.fit_removed(fit_id);
        self.vast.fit_removed(fit_id);
    }
    pub(in crate::sol::svc) fn notify_fit_added_to_fleet(&mut self, uad: &SolUad, fleet: &SolFleet, fit_id: &SolFitId) {
        self.calc.fit_added_to_fleet(uad, fleet, fit_id);
    }
    pub(in crate::sol::svc) fn notify_fit_removed_from_fleet(
        &mut self,
        uad: &SolUad,
        fleet: &SolFleet,
        fit_id: &SolFitId,
    ) {
        self.calc.fit_removed_from_fleet(uad, fleet, fit_id);
    }
    pub(in crate::sol::svc) fn notify_fit_rah_dmg_profile_changed(&mut self, uad: &SolUad, fit_id: &SolFitId) {
        self.calc.fit_rah_dmg_profile_changed(uad, fit_id);
    }
    pub(in crate::sol::svc) fn notify_item_added(&mut self, uad: &SolUad, item: &SolItem) {
        self.calc.item_added(uad, item);
        self.vast.item_added(item);
    }
    pub(in crate::sol::svc) fn notify_item_removed(&mut self, uad: &SolUad, item: &SolItem) {
        self.calc.item_removed(uad, item);
        self.vast.item_removed(uad, item);
    }
    pub(in crate::sol::svc) fn notify_state_activated(&mut self, item: &SolItem, state: &SolItemState) {
        self.vast.item_state_activated(item, state);
    }
    pub(in crate::sol::svc) fn notify_state_deactivated(&mut self, item: &SolItem, state: &SolItemState) {
        self.vast.item_state_deactivated(item, state);
    }
    pub(in crate::sol::svc) fn notify_item_loaded(&mut self, uad: &SolUad, item: &SolItem) {
        self.calc.item_loaded(uad, item);
        self.vast.item_loaded(uad, item);
    }
    pub(in crate::sol::svc) fn notify_item_unloaded(&mut self, uad: &SolUad, item: &SolItem) {
        self.calc.item_unloaded(uad, item);
        self.vast.item_unloaded(item);
    }
    pub(in crate::sol::svc) fn notify_base_attr_value_changed(
        &mut self,
        uad: &SolUad,
        item_id: &SolItemId,
        attr_id: &EAttrId,
    ) {
        self.calc.force_attr_value_recalc(uad, item_id, attr_id);
    }
    pub(in crate::sol::svc) fn notify_item_state_activated_loaded(&mut self, item: &SolItem, state: &SolItemState) {
        self.vast.item_state_activated_loaded(item, state);
    }
    pub(in crate::sol::svc) fn notify_item_state_deactivated_loaded(&mut self, item: &SolItem, state: &SolItemState) {
        self.vast.item_state_deactivated_loaded(item, state);
    }
    pub(in crate::sol::svc) fn notify_effects_started(
        &mut self,
        uad: &SolUad,
        item: &SolItem,
        effects: &[ad::ArcEffect],
    ) {
        self.running_effects
            .effects_started(item.get_id(), effects.iter().map(|v| v.id));
        self.calc.effects_started(uad, item, effects);
        self.vast.effects_started(item, effects);
    }
    pub(in crate::sol::svc) fn notify_effects_stopped(
        &mut self,
        uad: &SolUad,
        item: &SolItem,
        effects: &[ad::ArcEffect],
    ) {
        self.calc.effects_stopped(uad, item, effects);
        self.running_effects
            .effects_stopped(&item.get_id(), effects.iter().map(|v| &v.id));
        self.vast.effects_stopped(item, effects);
    }
    pub(in crate::sol::svc) fn notify_item_projected(
        &mut self,
        _uad: &SolUad,
        _projector_item: &SolItem,
        _projectee_item: &SolItem,
    ) {
    }
    pub(in crate::sol::svc) fn notify_item_unprojected(
        &mut self,
        _uad: &SolUad,
        _projector_item: &SolItem,
        _projectee_item: &SolItem,
    ) {
    }
    pub(in crate::sol::svc) fn notify_item_proj_range_changed(
        &mut self,
        _uad: &SolUad,
        _projector_item: &SolItem,
        _projectee_item: &SolItem,
    ) {
    }
    pub(in crate::sol::svc) fn notify_effect_projected(
        &mut self,
        uad: &SolUad,
        projector_item: &SolItem,
        effect: &ad::ArcEffect,
        projectee_item: &SolItem,
        range: Option<AttrVal>,
    ) {
        self.calc
            .effect_projected(uad, projector_item, effect, projectee_item, range);
    }
    pub(in crate::sol::svc) fn notify_effect_unprojected(
        &mut self,
        uad: &SolUad,
        projector_item: &SolItem,
        effect: &ad::ArcEffect,
        projectee_item: &SolItem,
    ) {
        self.calc
            .effect_unprojected(uad, projector_item, effect, projectee_item);
    }
    pub(in crate::sol::svc) fn notify_effect_proj_range_changed(
        &mut self,
        uad: &SolUad,
        projector_item: &SolItem,
        effect: &ad::ArcEffect,
        projectee_item: &SolItem,
        range: Option<AttrVal>,
    ) {
        self.calc
            .effect_proj_range_changed(uad, projector_item, effect, projectee_item, range);
    }
    pub(in crate::sol::svc) fn notify_fighter_count_changed(&mut self, uad: &SolUad, fighter: &SolFighter) {
        self.calc.fighter_count_changed(uad, &fighter.get_id());
        self.vast.fighter_count_changed(fighter);
    }
    pub(in crate::sol::svc) fn notify_skill_level_changed(&mut self, uad: &SolUad, skill: &SolSkill) {
        self.calc.skill_level_changed(uad, &skill.get_id());
        self.vast.skill_level_changed(uad, skill);
    }
}
