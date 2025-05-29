use crate::{
    ad,
    sol::{
        AttrVal, FitKey, ItemKey,
        svc::{EffectSpec, Svc, misc::AttrSpec},
        uad::{
            Uad,
            fleet::UadFleet,
            item::{UadFighter, UadItem, UadSkill},
        },
    },
    src::Src,
};

impl Svc {
    pub(in crate::sol) fn notify_src_changed(&mut self, src: &Src) {
        self.calc.src_changed(src);
    }
    pub(in crate::sol) fn notify_fit_added(&mut self, fit_key: FitKey) {
        self.calc.fit_added(fit_key);
        self.vast.fit_added(fit_key);
    }
    pub(in crate::sol) fn notify_fit_removed(&mut self, fit_key: FitKey) {
        self.calc.fit_removed(fit_key);
        self.vast.fit_removed(&fit_key);
    }
    pub(in crate::sol) fn notify_fit_added_to_fleet(&mut self, uad: &Uad, fleet: &UadFleet, fit_key: &FitKey) {
        self.calc.fit_added_to_fleet(uad, fleet, fit_key);
    }
    pub(in crate::sol) fn notify_fit_removed_from_fleet(&mut self, uad: &Uad, fleet: &UadFleet, fit_key: &FitKey) {
        self.calc.fit_removed_from_fleet(uad, fleet, fit_key);
    }
    pub(in crate::sol) fn notify_fit_rah_dps_profile_changed(&mut self, uad: &Uad, fit_key: &FitKey) {
        self.calc.fit_rah_dps_profile_changed(uad, fit_key);
    }
    pub(in crate::sol) fn notify_item_added(&mut self, uad: &Uad, item_key: ItemKey, item: &UadItem) {
        self.calc.item_added(uad, item_key, item);
        self.vast.item_added(item_key, item);
    }
    pub(in crate::sol) fn notify_item_removed(&mut self, uad: &Uad, item_key: ItemKey, item: &UadItem) {
        self.calc.item_removed(uad, item_key, item);
        self.vast.item_removed(uad, item_key, item);
    }
    pub(in crate::sol) fn notify_state_activated(&mut self, item_key: ItemKey, item: &UadItem, a_state: &ad::AState) {
        self.vast.item_state_activated(item_key, item, a_state);
    }
    pub(in crate::sol) fn notify_state_deactivated(
        &mut self,
        item_key: &ItemKey,
        item: &UadItem,
        a_state: &ad::AState,
    ) {
        self.vast.item_state_deactivated(item_key, item, a_state);
    }
    pub(in crate::sol) fn notify_item_loaded(&mut self, uad: &Uad, item_key: ItemKey, item: &UadItem) {
        self.calc.item_loaded(uad, item_key, item);
        self.vast.item_loaded(uad, item_key, item);
    }
    pub(in crate::sol) fn notify_item_unloaded(&mut self, uad: &Uad, item_key: ItemKey, item: &UadItem) {
        self.calc.item_unloaded(uad, item_key, item);
        self.vast.item_unloaded(&item_key, item);
    }
    pub(in crate::sol) fn notify_base_attr_value_changed(
        &mut self,
        uad: &Uad,
        item_key: ItemKey,
        a_attr_id: ad::AAttrId,
    ) {
        self.calc.force_attr_value_recalc(uad, AttrSpec { item_key, a_attr_id });
    }
    pub(in crate::sol) fn notify_item_state_activated_loaded(
        &mut self,
        item_key: ItemKey,
        item: &UadItem,
        a_state: &ad::AState,
    ) {
        self.vast.item_state_activated_loaded(item_key, item, a_state);
    }
    pub(in crate::sol) fn notify_item_state_deactivated_loaded(
        &mut self,
        item_key: &ItemKey,
        item: &UadItem,
        a_state: &ad::AState,
    ) {
        self.vast.item_state_deactivated_loaded(item_key, item, a_state);
    }
    pub(in crate::sol) fn notify_effects_started(
        &mut self,
        uad: &Uad,
        item_key: ItemKey,
        item: &UadItem,
        a_effects: &[ad::ArcEffect],
    ) {
        self.calc.effects_started(uad, item_key, item, a_effects);
    }
    pub(in crate::sol) fn notify_effects_stopped(
        &mut self,
        uad: &Uad,
        item_key: ItemKey,
        item: &UadItem,
        a_effects: &[ad::ArcEffect],
    ) {
        self.calc.effects_stopped(uad, item_key, item, a_effects);
    }
    pub(in crate::sol) fn notify_item_projected(&mut self) {}
    pub(in crate::sol) fn notify_item_unprojected(&mut self) {}
    pub(in crate::sol) fn notify_item_proj_range_changed(&mut self) {}
    pub(in crate::sol) fn notify_effect_projected(
        &mut self,
        uad: &Uad,
        projector_item_key: ItemKey,
        projector_item: &UadItem,
        a_effect: &ad::ArcEffect,
        projectee_item_key: ItemKey,
        projectee_item: &UadItem,
        range: Option<AttrVal>,
    ) {
        let projector_espec = EffectSpec {
            item_key: projector_item_key,
            a_effect_id: a_effect.id,
        };
        self.calc
            .effect_projected(uad, projector_espec, projectee_item_key, projectee_item, range);
        self.vast.effect_projected(
            projector_item_key,
            projector_item,
            a_effect,
            projectee_item_key,
            projectee_item,
        );
    }
    pub(in crate::sol) fn notify_effect_unprojected(
        &mut self,
        uad: &Uad,
        projector_item_key: ItemKey,
        projector_item: &UadItem,
        a_effect: &ad::ArcEffect,
        projectee_item_key: ItemKey,
        projectee_item: &UadItem,
    ) {
        let projector_espec = EffectSpec {
            item_key: projector_item_key,
            a_effect_id: a_effect.id,
        };
        self.calc
            .effect_unprojected(uad, projector_espec, projectee_item_key, projectee_item);
        self.vast.effect_unprojected(
            projector_item_key,
            projector_item,
            a_effect,
            projectee_item_key,
            projectee_item,
        );
    }
    pub(in crate::sol) fn notify_effect_proj_range_changed(
        &mut self,
        uad: &Uad,
        projector_item_key: ItemKey,
        a_effect_id: ad::AEffectId,
        projectee_item_key: ItemKey,
        projectee_item: &UadItem,
        range: Option<AttrVal>,
    ) {
        let projector_espec = EffectSpec {
            item_key: projector_item_key,
            a_effect_id,
        };
        self.calc
            .effect_proj_range_changed(uad, projector_espec, projectee_item_key, projectee_item, range);
    }
    pub(in crate::sol) fn notify_sol_sec_zone_changed(&mut self, uad: &Uad) {
        self.calc.sol_sec_zone_changed(uad);
    }
    pub(in crate::sol) fn notify_fighter_count_changed(
        &mut self,
        uad: &Uad,
        fighter_key: ItemKey,
        fighter: &UadFighter,
    ) {
        self.calc.fighter_count_changed(uad, fighter_key);
        self.vast.fighter_count_changed(fighter_key, fighter);
    }
    pub(in crate::sol) fn notify_ship_sec_status_changed(&mut self, uad: &Uad, ship_key: ItemKey) {
        self.calc.ship_sec_status_changed(uad, ship_key);
    }
    pub(in crate::sol) fn notify_skill_level_changed(&mut self, uad: &Uad, skill_key: ItemKey, skill: &UadSkill) {
        self.calc.skill_level_changed(uad, skill_key);
        self.vast.skill_level_changed(uad, skill);
    }
}
