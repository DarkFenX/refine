use crate::{
    ad,
    sol::{
        AttrVal, FitId, ItemId,
        svc::Svc,
        uad::{
            Uad,
            fleet::Fleet,
            item::{Fighter, Item, Skill},
        },
    },
    src::Src,
};

impl Svc {
    pub(in crate::sol::svc) fn notify_src_changed(&mut self, src: &Src) {
        self.calc.src_changed(src);
    }
    pub(in crate::sol::svc) fn notify_fit_added(&mut self, fit_id: &FitId) {
        self.calc.fit_added(fit_id);
        self.vast.fit_added(fit_id);
    }
    pub(in crate::sol::svc) fn notify_fit_removed(&mut self, fit_id: &FitId) {
        self.calc.fit_removed(fit_id);
        self.vast.fit_removed(fit_id);
    }
    pub(in crate::sol::svc) fn notify_fit_added_to_fleet(&mut self, uad: &Uad, fleet: &Fleet, fit_id: &FitId) {
        self.calc.fit_added_to_fleet(uad, fleet, fit_id);
    }
    pub(in crate::sol::svc) fn notify_fit_removed_from_fleet(&mut self, uad: &Uad, fleet: &Fleet, fit_id: &FitId) {
        self.calc.fit_removed_from_fleet(uad, fleet, fit_id);
    }
    pub(in crate::sol::svc) fn notify_fit_rah_dmg_profile_changed(&mut self, uad: &Uad, fit_id: &FitId) {
        self.calc.fit_rah_dmg_profile_changed(uad, fit_id);
    }
    pub(in crate::sol::svc) fn notify_item_added(&mut self, uad: &Uad, item: &Item) {
        self.calc.item_added(uad, item);
        self.vast.item_added(item);
    }
    pub(in crate::sol::svc) fn notify_item_removed(&mut self, uad: &Uad, item: &Item) {
        self.calc.item_removed(uad, item);
        self.vast.item_removed(uad, item);
    }
    pub(in crate::sol::svc) fn notify_state_activated(&mut self, item: &Item, a_state: &ad::AState) {
        self.vast.item_state_activated(item, a_state);
    }
    pub(in crate::sol::svc) fn notify_state_deactivated(&mut self, item: &Item, a_state: &ad::AState) {
        self.vast.item_state_deactivated(item, a_state);
    }
    pub(in crate::sol::svc) fn notify_item_loaded(&mut self, uad: &Uad, item: &Item) {
        self.calc.item_loaded(uad, item);
        self.vast.item_loaded(uad, item);
    }
    pub(in crate::sol::svc) fn notify_item_unloaded(&mut self, uad: &Uad, item: &Item) {
        self.calc.item_unloaded(uad, item);
        self.vast.item_unloaded(item);
    }
    pub(in crate::sol::svc) fn notify_base_attr_value_changed(
        &mut self,
        uad: &Uad,
        item_id: &ItemId,
        a_attr_id: &ad::AAttrId,
    ) {
        self.calc.force_attr_value_recalc(uad, item_id, a_attr_id);
    }
    pub(in crate::sol::svc) fn notify_item_state_activated_loaded(&mut self, item: &Item, a_state: &ad::AState) {
        self.vast.item_state_activated_loaded(item, a_state);
    }
    pub(in crate::sol::svc) fn notify_item_state_deactivated_loaded(&mut self, item: &Item, a_state: &ad::AState) {
        self.vast.item_state_deactivated_loaded(item, a_state);
    }
    pub(in crate::sol::svc) fn notify_effects_started(&mut self, uad: &Uad, item: &Item, a_effects: &[ad::ArcEffect]) {
        self.running_effects
            .effects_started(item.get_item_id(), a_effects.iter().map(|v| v.id));
        self.calc.effects_started(uad, item, a_effects);
        self.vast.effects_started(item, a_effects);
    }
    pub(in crate::sol::svc) fn notify_effects_stopped(&mut self, uad: &Uad, item: &Item, a_effects: &[ad::ArcEffect]) {
        self.calc.effects_stopped(uad, item, a_effects);
        self.running_effects
            .effects_stopped(&item.get_item_id(), a_effects.iter().map(|v| &v.id));
        self.vast.effects_stopped(item, a_effects);
    }
    pub(in crate::sol::svc) fn notify_item_projected(
        &mut self,
        _uad: &Uad,
        _projector_item: &Item,
        _projectee_item: &Item,
    ) {
    }
    pub(in crate::sol::svc) fn notify_item_unprojected(
        &mut self,
        _uad: &Uad,
        _projector_item: &Item,
        _projectee_item: &Item,
    ) {
    }
    pub(in crate::sol::svc) fn notify_item_proj_range_changed(
        &mut self,
        _uad: &Uad,
        _projector_item: &Item,
        _projectee_item: &Item,
    ) {
    }
    pub(in crate::sol::svc) fn notify_effect_projected(
        &mut self,
        uad: &Uad,
        projector_item: &Item,
        a_effect: &ad::ArcEffect,
        projectee_item: &Item,
        range: Option<AttrVal>,
    ) {
        self.calc
            .effect_projected(uad, projector_item, a_effect, projectee_item, range);
    }
    pub(in crate::sol::svc) fn notify_effect_unprojected(
        &mut self,
        uad: &Uad,
        projector_item: &Item,
        a_effect: &ad::ArcEffect,
        projectee_item: &Item,
    ) {
        self.calc
            .effect_unprojected(uad, projector_item, a_effect, projectee_item);
    }
    pub(in crate::sol::svc) fn notify_effect_proj_range_changed(
        &mut self,
        uad: &Uad,
        projector_item: &Item,
        effect: &ad::ArcEffect,
        projectee_item: &Item,
        range: Option<AttrVal>,
    ) {
        self.calc
            .effect_proj_range_changed(uad, projector_item, effect, projectee_item, range);
    }
    pub(in crate::sol::svc) fn notify_fighter_count_changed(&mut self, uad: &Uad, fighter: &Fighter) {
        self.calc.fighter_count_changed(uad, &fighter.get_item_id());
        self.vast.fighter_count_changed(fighter);
    }
    pub(in crate::sol::svc) fn notify_skill_level_changed(&mut self, uad: &Uad, skill: &Skill) {
        self.calc.skill_level_changed(uad, &skill.get_item_id());
        self.vast.skill_level_changed(uad, skill);
    }
}
