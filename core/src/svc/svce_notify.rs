use crate::{
    misc::{AttrSpec, EffectSpec},
    rd::{RAttrId, REffectId, RState, RcEffect},
    svc::{Svc, SvcCtx},
    ud::{UData, UFighter, UFitId, UFleet, UItem, UItemId, UProjData, USkill},
};

impl Svc {
    pub(crate) fn notify_fit_added(&mut self, fit_uid: UFitId) {
        self.calc.fit_added(fit_uid);
        self.vast.fit_added(fit_uid);
    }
    pub(crate) fn notify_fit_removed(&mut self, fit_uid: UFitId) {
        self.calc.fit_removed(fit_uid);
        self.vast.fit_removed(&fit_uid);
    }
    pub(crate) fn notify_fit_added_to_fleet(&mut self, u_data: &UData, fleet: &UFleet, fit_uid: UFitId) {
        let svc_ctx = SvcCtx::new(u_data, &self.eff_projs);
        self.calc.fit_added_to_fleet(svc_ctx, fleet, fit_uid);
    }
    pub(crate) fn notify_fit_removed_from_fleet(&mut self, u_data: &UData, fleet: &UFleet, fit_uid: UFitId) {
        let svc_ctx = SvcCtx::new(u_data, &self.eff_projs);
        self.calc.fit_removed_from_fleet(svc_ctx, fleet, fit_uid);
    }
    pub(crate) fn notify_fit_rah_dps_profile_changed(&mut self, u_data: &UData, fit_uid: UFitId) {
        let svc_ctx = SvcCtx::new(u_data, &self.eff_projs);
        self.calc.fit_rah_dps_profile_changed(svc_ctx, fit_uid);
    }
    pub(crate) fn notify_item_added(&mut self, u_data: &UData, item_uid: UItemId, item: &UItem) {
        let svc_ctx = SvcCtx::new(u_data, &self.eff_projs);
        self.calc.item_added(svc_ctx, item_uid, item);
        self.vast.item_added(item_uid, item);
    }
    pub(crate) fn notify_item_removed(&mut self, u_data: &UData, item_uid: UItemId, item: &UItem) {
        let svc_ctx = SvcCtx::new(u_data, &self.eff_projs);
        self.calc.item_removed(svc_ctx, item_uid, item);
        self.vast.item_removed(u_data, item_uid, item);
    }
    pub(crate) fn notify_state_activated(&mut self, item_uid: UItemId, item: &UItem, state: RState) {
        self.vast.item_state_activated(item_uid, item, state);
    }
    pub(crate) fn notify_state_deactivated(&mut self, item_uid: &UItemId, item: &UItem, state: RState) {
        self.vast.item_state_deactivated(item_uid, item, state);
    }
    pub(crate) fn notify_item_loaded(&mut self, u_data: &UData, item_uid: UItemId, item: &UItem) {
        let svc_ctx = SvcCtx::new(u_data, &self.eff_projs);
        self.calc.item_loaded(svc_ctx, item_uid, item);
        self.vast.item_loaded(u_data, item_uid, item);
    }
    pub(crate) fn notify_item_unloaded(&mut self, u_data: &UData, item_uid: UItemId, item: &UItem) {
        let svc_ctx = SvcCtx::new(u_data, &self.eff_projs);
        self.calc.item_unloaded(svc_ctx, item_uid, item);
        self.vast.item_unloaded(&item_uid, item);
    }
    pub(crate) fn notify_base_attr_value_changed(&mut self, u_data: &UData, item_uid: UItemId, attr_rid: RAttrId) {
        let svc_ctx = SvcCtx::new(u_data, &self.eff_projs);
        self.calc
            .force_attr_value_recalc(svc_ctx, AttrSpec::new(item_uid, attr_rid));
    }
    pub(crate) fn notify_item_state_activated_loaded(&mut self, item_uid: UItemId, item: &UItem, state: RState) {
        self.vast.item_state_activated_loaded(item_uid, item, state);
    }
    pub(crate) fn notify_item_state_deactivated_loaded(&mut self, item_uid: &UItemId, item: &UItem, state: RState) {
        self.vast.item_state_deactivated_loaded(item_uid, item, state);
    }
    pub(crate) fn notify_effects_started(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
        item: &UItem,
        effects: &[RcEffect],
    ) {
        let svc_ctx = SvcCtx::new(u_data, &self.eff_projs);
        self.calc.effects_started(svc_ctx, item_uid, item, effects);
        self.vast.effects_started(item_uid, item, effects);
    }
    pub(crate) fn notify_effects_stopped(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
        item: &UItem,
        effects: &[RcEffect],
    ) {
        let svc_ctx = SvcCtx::new(u_data, &self.eff_projs);
        self.calc.effects_stopped(svc_ctx, item_uid, item, effects);
        self.vast.effects_stopped(item_uid, item, effects);
    }
    pub(crate) fn notify_item_projected(&mut self) {}
    pub(crate) fn notify_item_unprojected(&mut self) {}
    pub(crate) fn notify_item_proj_data_changed(&mut self) {}
    pub(crate) fn notify_effect_projected(
        &mut self,
        u_data: &UData,
        projector_uid: UItemId,
        projector_item: &UItem,
        effect: &RcEffect,
        projectee_uid: UItemId,
        projectee_item: &UItem,
        proj_data: Option<UProjData>,
    ) {
        let projector_espec = EffectSpec::new(projector_uid, effect.rid);
        self.eff_projs.add_proj_data(projector_espec, projectee_uid, proj_data);
        let svc_ctx = SvcCtx::new(u_data, &self.eff_projs);
        self.calc
            .effect_projected(svc_ctx, projector_espec, projectee_uid, projectee_item);
        self.vast
            .effect_projected(projector_uid, projector_item, effect, projectee_uid, projectee_item);
    }
    pub(crate) fn notify_effect_unprojected(
        &mut self,
        u_data: &UData,
        projector_uid: UItemId,
        projector_item: &UItem,
        effect: &RcEffect,
        projectee_uid: UItemId,
        projectee_item: &UItem,
    ) {
        let projector_espec = EffectSpec::new(projector_uid, effect.rid);
        let svc_ctx = SvcCtx::new(u_data, &self.eff_projs);
        self.calc
            .effect_unprojected(svc_ctx, projector_espec, projectee_uid, projectee_item);
        self.vast
            .effect_unprojected(projector_uid, projector_item, effect, projectee_uid, projectee_item);
        self.eff_projs.remove_proj_data(projector_espec, projectee_uid);
    }
    pub(crate) fn notify_effect_proj_data_changed(
        &mut self,
        u_data: &UData,
        projector_uid: UItemId,
        effect_rid: REffectId,
        projectee_uid: UItemId,
        projectee_item: &UItem,
        proj_data: Option<UProjData>,
    ) {
        let projector_espec = EffectSpec::new(projector_uid, effect_rid);
        self.eff_projs
            .change_proj_data(projector_espec, projectee_uid, proj_data);
        let svc_ctx = SvcCtx::new(u_data, &self.eff_projs);
        self.calc
            .effect_proj_data_changed(svc_ctx, projector_espec, projectee_uid, projectee_item);
    }
    pub(crate) fn notify_sol_sec_zone_changed(&mut self, u_data: &UData) {
        let svc_ctx = SvcCtx::new(u_data, &self.eff_projs);
        self.calc.sol_sec_zone_changed(svc_ctx);
    }
    pub(crate) fn notify_fighter_count_changed(&mut self, u_data: &UData, fighter_uid: UItemId, fighter: &UFighter) {
        let svc_ctx = SvcCtx::new(u_data, &self.eff_projs);
        self.calc.fighter_count_changed(svc_ctx, fighter_uid);
        self.vast.fighter_count_changed(fighter_uid, fighter);
    }
    pub(crate) fn notify_ship_sec_status_changed(&mut self, u_data: &UData, ship_uid: UItemId) {
        let svc_ctx = SvcCtx::new(u_data, &self.eff_projs);
        self.calc.ship_sec_status_changed(svc_ctx, ship_uid);
    }
    pub(crate) fn notify_skill_level_changed(&mut self, u_data: &UData, skill_uid: UItemId, skill: &USkill) {
        let svc_ctx = SvcCtx::new(u_data, &self.eff_projs);
        self.calc.skill_level_changed(svc_ctx, skill_uid);
        self.vast.skill_level_changed(u_data, skill);
    }
}
