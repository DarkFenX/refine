use crate::{
    ad,
    misc::{AttrSpec, EffectSpec},
    rd,
    src::Src,
    svc::{Svc, SvcCtx},
    uad::{Uad, UadFighter, UadFitKey, UadFleet, UadItem, UadItemKey, UadProjRange, UadSkill},
};

impl Svc {
    pub(crate) fn notify_src_changed(&mut self, src: &Src) {
        self.calc.src_changed(src);
    }
    pub(crate) fn notify_fit_added(&mut self, fit_key: UadFitKey) {
        self.calc.fit_added(fit_key);
        self.vast.fit_added(fit_key);
    }
    pub(crate) fn notify_fit_removed(&mut self, fit_key: UadFitKey) {
        self.calc.fit_removed(fit_key);
        self.vast.fit_removed(&fit_key);
    }
    pub(crate) fn notify_fit_added_to_fleet(&mut self, uad: &Uad, fleet: &UadFleet, fit_key: &UadFitKey) {
        let svc_ctx = SvcCtx::new(uad, &self.eprojs);
        self.calc.fit_added_to_fleet(svc_ctx, fleet, fit_key);
    }
    pub(crate) fn notify_fit_removed_from_fleet(&mut self, uad: &Uad, fleet: &UadFleet, fit_key: &UadFitKey) {
        let svc_ctx = SvcCtx::new(uad, &self.eprojs);
        self.calc.fit_removed_from_fleet(svc_ctx, fleet, fit_key);
    }
    pub(crate) fn notify_fit_rah_dps_profile_changed(&mut self, uad: &Uad, fit_key: &UadFitKey) {
        let svc_ctx = SvcCtx::new(uad, &self.eprojs);
        self.calc.fit_rah_dps_profile_changed(svc_ctx, fit_key);
    }
    pub(crate) fn notify_item_added(&mut self, uad: &Uad, item_key: UadItemKey, item: &UadItem) {
        let svc_ctx = SvcCtx::new(uad, &self.eprojs);
        self.calc.item_added(svc_ctx, item_key, item);
        self.vast.item_added(item_key, item);
    }
    pub(crate) fn notify_item_removed(&mut self, uad: &Uad, item_key: UadItemKey, item: &UadItem) {
        let svc_ctx = SvcCtx::new(uad, &self.eprojs);
        self.calc.item_removed(svc_ctx, item_key, item);
        self.vast.item_removed(uad, item_key, item);
    }
    pub(crate) fn notify_state_activated(&mut self, item_key: UadItemKey, item: &UadItem, a_state: &ad::AState) {
        self.vast.item_state_activated(item_key, item, a_state);
    }
    pub(crate) fn notify_state_deactivated(&mut self, item_key: &UadItemKey, item: &UadItem, a_state: &ad::AState) {
        self.vast.item_state_deactivated(item_key, item, a_state);
    }
    pub(crate) fn notify_item_loaded(&mut self, uad: &Uad, item_key: UadItemKey, item: &UadItem) {
        let svc_ctx = SvcCtx::new(uad, &self.eprojs);
        self.calc.item_loaded(svc_ctx, item_key, item);
        self.vast.item_loaded(uad, item_key, item);
    }
    pub(crate) fn notify_item_unloaded(&mut self, uad: &Uad, item_key: UadItemKey, item: &UadItem) {
        let svc_ctx = SvcCtx::new(uad, &self.eprojs);
        self.calc.item_unloaded(svc_ctx, item_key, item);
        self.vast.item_unloaded(&item_key, item);
    }
    pub(crate) fn notify_base_attr_value_changed(&mut self, uad: &Uad, item_key: UadItemKey, a_attr_id: ad::AAttrId) {
        let svc_ctx = SvcCtx::new(uad, &self.eprojs);
        self.calc
            .force_attr_value_recalc(svc_ctx, AttrSpec::new(item_key, a_attr_id));
    }
    pub(crate) fn notify_item_state_activated_loaded(
        &mut self,
        item_key: UadItemKey,
        item: &UadItem,
        a_state: &ad::AState,
    ) {
        self.vast.item_state_activated_loaded(item_key, item, a_state);
    }
    pub(crate) fn notify_item_state_deactivated_loaded(
        &mut self,
        item_key: &UadItemKey,
        item: &UadItem,
        a_state: &ad::AState,
    ) {
        self.vast.item_state_deactivated_loaded(item_key, item, a_state);
    }
    pub(crate) fn notify_effects_started(
        &mut self,
        uad: &Uad,
        item_key: UadItemKey,
        item: &UadItem,
        r_effects: &[rd::RcEffect],
    ) {
        let svc_ctx = SvcCtx::new(uad, &self.eprojs);
        self.calc.effects_started(svc_ctx, item_key, item, r_effects);
        self.vast.effects_started(item_key, item, r_effects);
    }
    pub(crate) fn notify_effects_stopped(
        &mut self,
        uad: &Uad,
        item_key: UadItemKey,
        item: &UadItem,
        r_effects: &[rd::RcEffect],
    ) {
        let svc_ctx = SvcCtx::new(uad, &self.eprojs);
        self.calc.effects_stopped(svc_ctx, item_key, item, r_effects);
        self.vast.effects_stopped(item_key, item, r_effects);
    }
    pub(crate) fn notify_item_projected(&mut self) {}
    pub(crate) fn notify_item_unprojected(&mut self) {}
    pub(crate) fn notify_item_proj_range_changed(&mut self) {}
    pub(crate) fn notify_effect_projected(
        &mut self,
        uad: &Uad,
        projector_key: UadItemKey,
        projector_item: &UadItem,
        r_effect: &rd::RcEffect,
        projectee_key: UadItemKey,
        projectee_item: &UadItem,
        range: Option<UadProjRange>,
    ) {
        let projector_espec = EffectSpec::new(projector_key, r_effect.get_id());
        self.eprojs.add_range(projector_espec, projectee_key, range);
        let svc_ctx = SvcCtx::new(uad, &self.eprojs);
        self.calc
            .effect_projected(svc_ctx, projector_espec, projectee_key, projectee_item);
        self.vast
            .effect_projected(projector_key, projector_item, r_effect, projectee_key, projectee_item);
    }
    pub(crate) fn notify_effect_unprojected(
        &mut self,
        uad: &Uad,
        projector_key: UadItemKey,
        projector_item: &UadItem,
        r_effect: &rd::RcEffect,
        projectee_key: UadItemKey,
        projectee_item: &UadItem,
    ) {
        let projector_espec = EffectSpec::new(projector_key, r_effect.get_id());
        let svc_ctx = SvcCtx::new(uad, &self.eprojs);
        self.calc
            .effect_unprojected(svc_ctx, projector_espec, projectee_key, projectee_item);
        self.vast
            .effect_unprojected(projector_key, projector_item, r_effect, projectee_key, projectee_item);
        self.eprojs.remove_range(projector_espec, projectee_key);
    }
    pub(crate) fn notify_effect_proj_range_changed(
        &mut self,
        uad: &Uad,
        projector_key: UadItemKey,
        a_effect_id: ad::AEffectId,
        projectee_key: UadItemKey,
        projectee_item: &UadItem,
        range: Option<UadProjRange>,
    ) {
        let projector_espec = EffectSpec::new(projector_key, a_effect_id);
        self.eprojs.change_range(projector_espec, projectee_key, range);
        let svc_ctx = SvcCtx::new(uad, &self.eprojs);
        self.calc
            .effect_proj_range_changed(svc_ctx, projector_espec, projectee_key, projectee_item);
    }
    pub(crate) fn notify_sol_sec_zone_changed(&mut self, uad: &Uad) {
        let svc_ctx = SvcCtx::new(uad, &self.eprojs);
        self.calc.sol_sec_zone_changed(svc_ctx);
    }
    pub(crate) fn notify_fighter_count_changed(&mut self, uad: &Uad, fighter_key: UadItemKey, fighter: &UadFighter) {
        let svc_ctx = SvcCtx::new(uad, &self.eprojs);
        self.calc.fighter_count_changed(svc_ctx, fighter_key);
        self.vast.fighter_count_changed(fighter_key, fighter);
    }
    pub(crate) fn notify_ship_sec_status_changed(&mut self, uad: &Uad, ship_key: UadItemKey) {
        let svc_ctx = SvcCtx::new(uad, &self.eprojs);
        self.calc.ship_sec_status_changed(svc_ctx, ship_key);
    }
    pub(crate) fn notify_skill_level_changed(&mut self, uad: &Uad, skill_key: UadItemKey, skill: &UadSkill) {
        let svc_ctx = SvcCtx::new(uad, &self.eprojs);
        self.calc.skill_level_changed(svc_ctx, skill_key);
        self.vast.skill_level_changed(uad, skill);
    }
}
