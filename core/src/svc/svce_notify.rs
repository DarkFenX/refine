use crate::{
    ad::AState,
    misc::{AttrSpec, EffectSpec},
    rd::{RAttrKey, REffectKey, RcEffect},
    svc::{Svc, SvcCtx},
    ud::{UData, UFighter, UFitKey, UFleet, UItem, UItemKey, UProjData, USkill},
};

impl Svc {
    pub(crate) fn notify_fit_added(&mut self, fit_key: UFitKey) {
        self.calc.fit_added(fit_key);
        self.vast.fit_added(fit_key);
    }
    pub(crate) fn notify_fit_removed(&mut self, fit_key: UFitKey) {
        self.calc.fit_removed(fit_key);
        self.vast.fit_removed(&fit_key);
    }
    pub(crate) fn notify_fit_added_to_fleet(&mut self, u_data: &UData, fleet: &UFleet, fit_key: UFitKey) {
        let svc_ctx = SvcCtx::new(u_data, &self.eff_projs);
        self.calc.fit_added_to_fleet(svc_ctx, fleet, fit_key);
    }
    pub(crate) fn notify_fit_removed_from_fleet(&mut self, u_data: &UData, fleet: &UFleet, fit_key: UFitKey) {
        let svc_ctx = SvcCtx::new(u_data, &self.eff_projs);
        self.calc.fit_removed_from_fleet(svc_ctx, fleet, fit_key);
    }
    pub(crate) fn notify_fit_rah_dps_profile_changed(&mut self, u_data: &UData, fit_key: UFitKey) {
        let svc_ctx = SvcCtx::new(u_data, &self.eff_projs);
        self.calc.fit_rah_dps_profile_changed(svc_ctx, fit_key);
    }
    pub(crate) fn notify_item_added(&mut self, u_data: &UData, item_key: UItemKey, item: &UItem) {
        let svc_ctx = SvcCtx::new(u_data, &self.eff_projs);
        self.calc.item_added(svc_ctx, item_key, item);
        self.vast.item_added(item_key, item);
    }
    pub(crate) fn notify_item_removed(&mut self, u_data: &UData, item_key: UItemKey, item: &UItem) {
        let svc_ctx = SvcCtx::new(u_data, &self.eff_projs);
        self.calc.item_removed(svc_ctx, item_key, item);
        self.vast.item_removed(u_data, item_key, item);
    }
    pub(crate) fn notify_state_activated(&mut self, item_key: UItemKey, item: &UItem, a_state: &AState) {
        self.vast.item_state_activated(item_key, item, a_state);
    }
    pub(crate) fn notify_state_deactivated(&mut self, item_key: &UItemKey, item: &UItem, a_state: &AState) {
        self.vast.item_state_deactivated(item_key, item, a_state);
    }
    pub(crate) fn notify_item_loaded(&mut self, u_data: &UData, item_key: UItemKey, item: &UItem) {
        let svc_ctx = SvcCtx::new(u_data, &self.eff_projs);
        self.calc.item_loaded(svc_ctx, item_key, item);
        self.vast.item_loaded(u_data, item_key, item);
    }
    pub(crate) fn notify_item_unloaded(&mut self, u_data: &UData, item_key: UItemKey, item: &UItem) {
        let svc_ctx = SvcCtx::new(u_data, &self.eff_projs);
        self.calc.item_unloaded(svc_ctx, item_key, item);
        self.vast.item_unloaded(&item_key, item);
    }
    pub(crate) fn notify_base_attr_value_changed(&mut self, u_data: &UData, item_key: UItemKey, attr_key: RAttrKey) {
        let svc_ctx = SvcCtx::new(u_data, &self.eff_projs);
        self.calc
            .force_attr_value_recalc(svc_ctx, AttrSpec::new(item_key, attr_key));
    }
    pub(crate) fn notify_item_state_activated_loaded(&mut self, item_key: UItemKey, item: &UItem, a_state: &AState) {
        self.vast.item_state_activated_loaded(item_key, item, a_state);
    }
    pub(crate) fn notify_item_state_deactivated_loaded(&mut self, item_key: &UItemKey, item: &UItem, a_state: &AState) {
        self.vast.item_state_deactivated_loaded(item_key, item, a_state);
    }
    pub(crate) fn notify_effects_started(
        &mut self,
        u_data: &UData,
        item_key: UItemKey,
        item: &UItem,
        effects: &[RcEffect],
    ) {
        let svc_ctx = SvcCtx::new(u_data, &self.eff_projs);
        self.calc.effects_started(svc_ctx, item_key, item, effects);
        self.vast.effects_started(item_key, item, effects);
    }
    pub(crate) fn notify_effects_stopped(
        &mut self,
        u_data: &UData,
        item_key: UItemKey,
        item: &UItem,
        effects: &[RcEffect],
    ) {
        let svc_ctx = SvcCtx::new(u_data, &self.eff_projs);
        self.calc.effects_stopped(svc_ctx, item_key, item, effects);
        self.vast.effects_stopped(item_key, item, effects);
    }
    pub(crate) fn notify_item_projected(&mut self) {}
    pub(crate) fn notify_item_unprojected(&mut self) {}
    pub(crate) fn notify_item_proj_data_changed(&mut self) {}
    pub(crate) fn notify_effect_projected(
        &mut self,
        u_data: &UData,
        projector_key: UItemKey,
        projector_item: &UItem,
        effect: &RcEffect,
        projectee_key: UItemKey,
        projectee_item: &UItem,
        proj_data: Option<UProjData>,
    ) {
        let projector_espec = EffectSpec::new(projector_key, effect.key);
        self.eff_projs.add_proj_data(projector_espec, projectee_key, proj_data);
        let svc_ctx = SvcCtx::new(u_data, &self.eff_projs);
        self.calc
            .effect_projected(svc_ctx, projector_espec, projectee_key, projectee_item);
        self.vast
            .effect_projected(projector_key, projector_item, effect, projectee_key, projectee_item);
    }
    pub(crate) fn notify_effect_unprojected(
        &mut self,
        u_data: &UData,
        projector_key: UItemKey,
        projector_item: &UItem,
        effect: &RcEffect,
        projectee_key: UItemKey,
        projectee_item: &UItem,
    ) {
        let projector_espec = EffectSpec::new(projector_key, effect.key);
        let svc_ctx = SvcCtx::new(u_data, &self.eff_projs);
        self.calc
            .effect_unprojected(svc_ctx, projector_espec, projectee_key, projectee_item);
        self.vast
            .effect_unprojected(projector_key, projector_item, effect, projectee_key, projectee_item);
        self.eff_projs.remove_proj_data(projector_espec, projectee_key);
    }
    pub(crate) fn notify_effect_proj_data_changed(
        &mut self,
        u_data: &UData,
        projector_key: UItemKey,
        effect_key: REffectKey,
        projectee_key: UItemKey,
        projectee_item: &UItem,
        proj_data: Option<UProjData>,
    ) {
        let projector_espec = EffectSpec::new(projector_key, effect_key);
        self.eff_projs
            .change_proj_data(projector_espec, projectee_key, proj_data);
        let svc_ctx = SvcCtx::new(u_data, &self.eff_projs);
        self.calc
            .effect_proj_data_changed(svc_ctx, projector_espec, projectee_key, projectee_item);
    }
    pub(crate) fn notify_sol_sec_zone_changed(&mut self, u_data: &UData) {
        let svc_ctx = SvcCtx::new(u_data, &self.eff_projs);
        self.calc.sol_sec_zone_changed(svc_ctx);
    }
    pub(crate) fn notify_fighter_count_changed(&mut self, u_data: &UData, fighter_key: UItemKey, fighter: &UFighter) {
        let svc_ctx = SvcCtx::new(u_data, &self.eff_projs);
        self.calc.fighter_count_changed(svc_ctx, fighter_key);
        self.vast.fighter_count_changed(fighter_key, fighter);
    }
    pub(crate) fn notify_ship_sec_status_changed(&mut self, u_data: &UData, ship_key: UItemKey) {
        let svc_ctx = SvcCtx::new(u_data, &self.eff_projs);
        self.calc.ship_sec_status_changed(svc_ctx, ship_key);
    }
    pub(crate) fn notify_skill_level_changed(&mut self, u_data: &UData, skill_key: UItemKey, skill: &USkill) {
        let svc_ctx = SvcCtx::new(u_data, &self.eff_projs);
        self.calc.skill_level_changed(svc_ctx, skill_key);
        self.vast.skill_level_changed(u_data, skill);
    }
}
